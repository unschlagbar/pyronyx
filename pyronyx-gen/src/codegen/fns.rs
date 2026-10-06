use std::collections::{HashMap, HashSet};

use heck::ToSnakeCase;
use indexmap::IndexSet;

use crate::{
    codegen::{find_assert_fn, is_byte_slice_fn, rust_member, simple_rust_member},
    parse::registry::VkCommand,
};

#[derive(Debug)]
pub enum CleanParam {
    Single {
        name: String,
        ty: String,
        convert: bool,
    },
    Slice {
        slice_name: String, // "submits"
        elem_ty: String,    // "VkSubmitInfo"
        count_name: String, // "submit_count"
        mutable: bool,
        /// Length taken from another parameter's field, checked with `assert_eq!`
        /// (e.g. `allocate_info.descriptor_set_count`).
        len_assert: Option<String>,
    },
    CountForSlice {
        name: String,
        ty: String,
        slice_name: String,
        mutable: bool,
    },
    Output {
        ty: String,
        convert: bool,
    },
    SelfHandle,
}

pub fn analyze_params(
    cmd: &VkCommand,
    imports: &mut IndexSet<&'static str>,
    lifetimes: &HashSet<String>,
) -> Vec<CleanParam> {
    let params = &cmd.params;

    let mut count_to_slices: HashMap<String, Vec<String>> = HashMap::new();

    for param in params {
        if let Some(len) = &param.len {
            let count_name = len.split(',').next().unwrap_or(len);

            let is_param = params.iter().any(|p| p.name == count_name);
            if is_param {
                count_to_slices
                    .entry(count_name.to_string())
                    .or_default()
                    .push(param.name.clone());
            }
        }

        match param.ty.as_str() {
            _ => match param.ty.split(' ').next_back().unwrap() {
                "c_char" => {
                    imports.insert("use core::ffi::c_char;");
                }
                "c_int" => {
                    imports.insert("use core::ffi::c_int;");
                }
                _ => (),
            },
        };
    }

    let mut result = Vec::new();

    let mut ps = HashSet::new();

    let mut mut_lens = HashSet::new();

    for (i, param) in params.iter().enumerate() {
        if i == 0 {
            result.push(CleanParam::SelfHandle);
            continue;
        }

        if let Some(slice_names) = count_to_slices.get(&param.name) {
            result.push(CleanParam::CountForSlice {
                name: simple_rust_member(&param.name),
                slice_name: rust_member(slice_names.first().unwrap(), &mut ps),
                ty: param.ty.to_string(),
                mutable: false,
            });
            continue;
        }
        let is_last = i == params.len() - 1;

        if let Some(len) = &param.len {
            let count_name = len.split(',').next().unwrap_or(len);
            let is_param_count = params.iter().any(|p| p.name == count_name);

            if is_param_count && param.pointer_depth > 0 {
                let mutable = !param.is_const && param.pointer_depth > 0;
                let elem_ty = slice_rust_like(extract_pointer(&param.ty)).to_string();

                result.push(CleanParam::Slice {
                    slice_name: rust_member(&param.name, &mut ps),
                    elem_ty,
                    count_name: simple_rust_member(count_name),
                    mutable,
                    len_assert: None,
                });

                if mutable {
                    mut_lens.insert(simple_rust_member(len));
                }
                continue;
            }

            // `len` is a field of another parameter, e.g. "pAllocateInfo->descriptorSetCount":
            // a slice whose length is asserted against that field.
            if let Some((owner, field)) = count_name.split_once("->")
                && params.iter().any(|p| p.name == owner)
                && param.pointer_depth > 0
            {
                let elem_ty = slice_rust_like(extract_pointer(&param.ty)).to_string();
                result.push(CleanParam::Slice {
                    slice_name: rust_member(&param.name, &mut ps),
                    elem_ty,
                    count_name: String::new(),
                    mutable: !param.is_const,
                    len_assert: Some(format!(
                        "{}.{}",
                        simple_rust_member(owner),
                        field.to_snake_case()
                    )),
                });
                continue;
            }
        } else if find_assert_fn(&cmd.name).is_some() && !param.is_const && param.pointer_depth > 0
        {
            // param has no `len` attribute at all but belongs to a slice-assert fn.
            let elem_ty = slice_rust_like(extract_pointer(&param.ty)).to_string();
            result.push(CleanParam::Slice {
                slice_name: rust_member(&param.name, &mut ps),
                elem_ty,
                count_name: String::new(),
                mutable: true,
                len_assert: None,
            });
            continue;
        }

        let is_output = param.pointer_depth > 0 && !param.is_const;

        // An untyped `void*` out-param is an opaque caller-provided buffer, not a
        // value we can `MaybeUninit`-initialise. BYTE_SLICE_FNS says which of them are
        // byte buffers (`&mut [u8]`); the rest pass the raw pointer through.
        let is_opaque_output = extract_pointer(&param.ty) == "c_void";

        if is_opaque_output && is_output && is_byte_slice_fn(&cmd.name) {
            result.push(CleanParam::Slice {
                slice_name: rust_member(&param.name, &mut ps),
                elem_ty: "u8".to_string(),
                count_name: String::new(),
                mutable: true,
                len_assert: None,
            });
            continue;
        }

        if is_last
            && is_output
            && !is_opaque_output
            && (cmd.return_type == "vkResult" || cmd.return_type == "c_void")
        {
            let (ty, convert) = rust_like(extract_pointer(&param.ty), imports);

            // The driver reads `sType` and walks `pNext` of an extensible out-struct, so the
            // caller passes an initialised (and possibly chained) one instead of getting
            // uninitialised memory filled.
            if param.extensible {
                result.push(CleanParam::Single {
                    name: rust_member(&param.name, &mut ps),
                    ty: format!("&mut {}", result_ty(ty, lifetimes)),
                    convert: false,
                });
                continue;
            }

            result.push(CleanParam::Output {
                ty: result_ty(ty, lifetimes),
                convert,
            });
            imports.insert("use core::mem::MaybeUninit;");
            continue;
        }

        if param.optional && param.is_const && param.pointer_depth > 0 {
            let (ty, convert) = rust_like(extract_pointer(&param.ty), imports);
            result.push(CleanParam::Single {
                name: rust_member(&param.name, &mut ps),
                ty: format!("Option<&{}>", ty),
                convert,
            });
            imports.insert("use core::ptr::{null, from_ref};");
            continue;
        }

        let (ty, convert) = rust_like(&param.ty, imports);

        result.push(CleanParam::Single {
            name: rust_member(&param.name, &mut ps),
            ty: covert_pointer(ty),
            convert,
        });
    }

    for mut_slice in mut_lens {
        'a: for param in &mut result {
            match param {
                CleanParam::CountForSlice {
                    mutable,
                    name: _name,
                    ..
                } => {
                    if _name == &mut_slice {
                        *mutable = true;
                        break 'a;
                    }
                }
                _ => (),
            }
        }
    }

    result
}

fn extract_pointer(ty: &str) -> &str {
    ty.strip_prefix("*const ")
        .unwrap_or(ty.strip_prefix("*mut ").unwrap_or(ty))
}

fn result_ty(ty: &str, lifetimes: &HashSet<String>) -> String {
    let clean = ty.split(' ').next_back().unwrap();
    if lifetimes.contains(clean) {
        format!("{ty}<'_>")
    } else {
        ty.to_string()
    }
}

fn covert_pointer(ty: &str) -> String {
    ty.replace("*const ", "&").trim().to_string()
}

fn rust_like<'a>(ty: &'a str, imports: &mut IndexSet<&'static str>) -> (&'a str, bool) {
    match ty {
        "Bool32" => ("bool", true),
        "c_void" => {
            imports.insert("use core::ffi::c_void;");
            ("c_void", false)
        }
        "*mut c_void" => {
            imports.insert("use core::ffi::c_void;");
            ("*mut c_void", false)
        }
        "*const c_void" => {
            imports.insert("use core::ffi::c_void;");
            ("*const c_void", false)
        }
        ty => (ty, false),
    }
}

fn slice_rust_like<'a>(ty: &'a str) -> &'a str {
    match ty {
        "c_void" => "u8",
        other => other,
    }
}
