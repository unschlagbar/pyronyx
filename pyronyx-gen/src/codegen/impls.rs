use std::collections::{HashMap, HashSet};

use heck::ToSnakeCase;
use indexmap::IndexSet;

use super::fns::{CleanParam, analyze_params};
use super::{Writer, file_header};
use crate::codegen::bitflags::value_to_const_name;
use crate::codegen::commands::{fn_field, fn_sig};
use crate::codegen::{
    HAND_WRITTEN_FNS, RETURNS_SUBOPTIMAL, VEC_FNS, find_assert_fn, find_len_fn, is_byte_slice_fn,
    rust_name, simple_rust_member,
};
use crate::parse::commands::Param;
use crate::parse::registry::{Depends, Registry, RenderPass, VkCommand};

#[derive(Clone, Copy, Eq, Hash, PartialEq, Debug)]
pub enum ImplTarget {
    Instance,
    PhysicalDevice,
    Device,
    Queue,
    CommandBuffer,
}

impl ImplTarget {
    pub fn from_first_param(params: &[Param]) -> Option<Self> {
        let first_ty = params.first()?.ty.as_str();
        match first_ty {
            "vkInstance" => Some(Self::Instance),
            "vkPhysicalDevice" => Some(Self::PhysicalDevice),
            "vkDevice" => Some(Self::Device),
            "vkQueue" => Some(Self::Queue),
            "vkCommandBuffer" => Some(Self::CommandBuffer),
            _ => None,
        }
    }

    pub fn struct_name(&self) -> &str {
        match self {
            Self::Instance => "Instance",
            Self::PhysicalDevice => "PhysicalDevice",
            Self::Device => "Device",
            Self::Queue => "Queue",
            Self::CommandBuffer => "CommandBuffer",
        }
    }

    fn handle_field(&self) -> &str {
        "self.handle"
    }
}

// ── Shared signature representation ──────────────────────────────────────────

/// All pieces of a computed function signature, produced by [`compute_fn_signature`].
/// Both [`write_command_signature`] (trait definitions) and [`write_command_wrapper`]
/// (impl blocks) derive their signature from this struct so they can never drift apart.
struct SignatureParts {
    /// The Rust-style method name (e.g. `submit`, not `vkQueueSubmit`).
    wrapper_name: String,
    /// Full parameter list including the leading `&self` token.
    sig_params: Vec<String>,
    /// Return type string including the leading ` -> `, or empty for `()`.
    ret_str: String,
    /// Resolved output type (used to build the function body).
    output_ty: Option<String>,
    /// Element type of a vec-style output slice, if any.
    vec_out: Option<String>,
}

/// Computes the complete [`SignatureParts`] for a Vulkan command.
///
/// `vec` indicates whether the command is a vec-reader (see `VEC_FNS`).
/// `imports` is extended with any `use` statements required by the signature.
fn compute_fn_signature(
    target: &ImplTarget,
    cmd: &VkCommand,
    params: &[CleanParam],
    vec: bool,
    imports: &mut IndexSet<&'static str>,
    lifetimes: &HashSet<String>,
    len_fn: bool,
) -> SignatureParts {
    let struct_name = target.struct_name();
    let wrapper_name = fn_sig(&cmd.name, struct_name);

    // Collected during the sig_params pass so the output_ty pass can use it.
    let mut vec_out: Option<String> = None;

    let sig_params: Vec<String> = params
        .iter()
        .filter_map(|p| match p {
            CleanParam::SelfHandle => Some("&self".to_string()),

            CleanParam::Single { name, ty, .. } => Some(format!("{name}: {ty}")),

            CleanParam::Slice {
                slice_name,
                elem_ty,
                mutable,
                ..
            } => {
                if *mutable {
                    if vec {
                        // Mutable slice becomes a Vec<T> return value instead of a parameter.
                        vec_out = Some(elem_ty.clone());
                        if cmd.return_type == "vkResult" {
                            imports.insert("use crate::utils::read_into_vec_result;");
                        } else {
                            imports.insert("use crate::utils::read_into_vec;");
                        }
                        None
                    } else if len_fn {
                        None
                    } else {
                        Some(format!("{slice_name}: &mut [{elem_ty}]"))
                    }
                } else {
                    Some(format!("{slice_name}: &[{elem_ty}]"))
                }
            }

            // Count parameters are derived from the slice length at call-site.
            CleanParam::CountForSlice { .. } => None,
            // Output parameters become the return value, not a parameter.
            CleanParam::Output { .. } => None,
        })
        .collect();

    let output_ty = params.iter().find_map(|p| match p {
        CleanParam::Output { ty, .. } => Some(ty.clone()),
        _ => {
            if vec {
                vec_out.as_ref().map(|out| {
                    if lifetimes.contains(out) {
                        format!("Vec<{out}<'_>>")
                    } else {
                        format!("Vec<{out}>")
                    }
                })
            } else {
                None
            }
        }
    });

    let ret = if len_fn {
        match cmd.return_type.as_str() {
            "vkResult" => "Result<usize>".to_string(),
            "c_void" => "usize".to_string(),
            other => rust_name(other),
        }
    } else {
        match (cmd.return_type.as_str(), &output_ty) {
            ("vkResult", Some(ty)) if RETURNS_SUBOPTIMAL.contains(&cmd.name.as_str()) => {
                format!("Result<Suboptimal<{ty}>>")
            }
            ("vkResult", Some(ty)) => format!("Result<{ty}>"),
            ("vkResult", None) if RETURNS_SUBOPTIMAL.contains(&cmd.name.as_str()) => {
                "Result<Suboptimal<()>>".to_string()
            }
            ("vkResult", None) => "Result<()>".to_string(),
            ("c_void", Some(ty)) => ty.to_string(),
            ("c_void", None) => String::new(),
            (other, _) => rust_name(other),
        }
    };

    let ret_str = if ret.is_empty() {
        String::new()
    } else {
        format!(" -> {ret}")
    };

    SignatureParts {
        wrapper_name,
        sig_params,
        ret_str,
        output_ty,
        vec_out,
    }
}

// ── Code generation ───────────────────────────────────────────────────────────

pub fn generate(registry: &Registry, out_path: &str, lifetimes: &HashSet<String>) {
    let mut by_target: HashMap<ImplTarget, Vec<&VkCommand>> = HashMap::new();

    for (_, cmd) in &registry.commands {
        if cmd.alias.is_some() {
            continue;
        }
        if let Some(target) = cmd.target {
            by_target.entry(target).or_default().push(cmd);
        }
    }

    for (handle_ty, cmds) in &by_target {
        let mut w = Writer::new();
        file_header(&mut w, "generated impls");

        let mut imports = IndexSet::new();

        let mut inner_w = Writer::new();
        write_impl_block(&mut inner_w, handle_ty, cmds, &mut imports, lifetimes);

        w.ln("use crate::vk::*;");
        for import in imports {
            w.ln(import);
        }
        w.blank();
        w.ln(&inner_w.into_string());

        w.save(format!(
            "{out_path}_{}.rs",
            handle_ty.struct_name().to_snake_case()
        ));
    }
}

pub fn write_impl_block(
    w: &mut Writer,
    target: &ImplTarget,
    cmds: &[&VkCommand],
    imports: &mut IndexSet<&'static str>,
    lifetimes: &HashSet<String>,
) {
    let target_name = target.struct_name();
    w.ln(&format!("impl {target_name} {{"));

    for cmd in cmds {
        // Extension impls live in their own extension files.
        if HAND_WRITTEN_FNS
            .iter()
            .any(|(tar, name)| *tar == target_name && name == &cmd.name)
        {
            continue;
        }
        if cmd.extension.is_none() {
            write_command_wrapper(w, target, cmd, "pub ", imports, lifetimes);
        }
    }

    w.ln("}");
    w.blank();
}

/// Writes a complete method wrapper (doc comment + `#[inline]` + signature + body)
/// into an `impl` block.
///
/// The signature portion is built via [`compute_fn_signature`], the same helper
/// used by [`write_command_signature`], so trait definitions and impl methods
/// always stay in sync.
pub fn write_command_wrapper(
    w: &mut Writer,
    target: &ImplTarget,
    cmd: &VkCommand,
    pre_key: &str,
    imports: &mut IndexSet<&'static str>,
    lifetimes: &HashSet<String>,
) {
    let params = analyze_params(cmd, imports, lifetimes);

    let fn_name = fn_field(&cmd.name);
    let vtable = vtable_for_version(cmd);
    let error = if matches!(cmd.table_name(), Depends::Core(_)) {
        ".expect(Self::CORE_LOAD_ERROR)"
    } else {
        ".expect(Self::EXT_LOAD_ERROR)"
    };
    let unwrap = if cmd.option_member() { error } else { "" };
    let vec = VEC_FNS.iter().any(|f| f == &cmd.name);

    let is_mut_slice_fn = find_len_fn(&cmd.name).is_some();

    // Warn at codegen time about any function that exposes a mutable output slice
    // but hasn't been placed in one of the three handling lists yet.
    // This fires for newly-encountered commands so you can decide which category fits:
    //   • VEC_FNS               – double-call pattern, returns Vec<T>
    //   • MUT_SLICE_ASSERT_FNS  – length comes from a struct field; assert is emitted
    //   • MUT_SLICE_FNS         – true mut-slice input; a `{fn}_len` companion is generated
    //
    // Functions like vkCreateGraphicsPipelines are intentionally excluded: their mutable
    // slice shares a CountForSlice param with an input slice, so by_count already emits
    // the assert_eq! — no extra categorisation needed.
    let has_uncategorised_mut_slice = !vec
        && find_assert_fn(&cmd.name).is_none()
        && find_len_fn(&cmd.name).is_none()
        // BYTE_SLICE_FNS have no size param at all — the caller sizes the buffer from
        // the spec, so there is nothing to assert against.
        && !is_byte_slice_fn(&cmd.name)
        && params.iter().any(|p| match p {
            // Only flag slices whose _count_name is empty — meaning no CountForSlice
            // partner exists and by_count won't emit an assert for them.
            CleanParam::Slice {
                mutable: true,
                count_name: _count_name,
                ..
            } => _count_name.is_empty(),
            _ => false,
        });
    if has_uncategorised_mut_slice {
        println!(
            "`{}` has a mutable output slice but is not listed in \
             VEC_FNS, ASSERT_FNS, or LEN_FNS — add it to one of the three lists.",
            cmd.name
        );
    }

    let slice_assert = find_assert_fn(&cmd.name);

    // ── Normal path: derive signature from compute_fn_signature ──────────────
    let sig = compute_fn_signature(target, cmd, &params, vec, imports, lifetimes, false);

    w.ln(&format!(
        "/// <https://docs.vulkan.org/refpages/latest/refpages/source/{}.html>",
        cmd.name
    ));
    if is_mut_slice_fn {
        w.ln("///");
        w.ln(&format!(
            "/// Call [`{}_len()`][`Self::{}_len()`] to query the number of elements to pass to `out`.",
            sig.wrapper_name,
            sig.wrapper_name,
        ));
    }
    write_cmd_docs(w, cmd);
    w.ln("#[inline]");
    w.ln(&format!(
        "{pre_key}fn {}({}){} {{",
        sig.wrapper_name,
        sig.sig_params.join(", "),
        sig.ret_str,
    ));

    write_fn_body(
        w,
        target,
        cmd,
        &params,
        &sig.output_ty,
        &vtable,
        unwrap,
        &fn_name,
        vec,
        sig.vec_out.as_deref(),
        slice_assert,
    );

    w.ln("}");
    w.blank();

    if is_mut_slice_fn {
        write_mut_slice_len_companion(
            w, imports, target, cmd, &params, &vtable, unwrap, &fn_name, pre_key,
        );
    }
}

/// Writes a `{fn_name}_len` companion method for functions listed in [`MUT_SLICE_FNS`].
///
/// The companion has the same parameters as the main method **minus** the mutable slice
/// itself, returns `usize`, and its body simply evaluates `count_expr as usize`.
/// This lets callers query the required buffer size without having to know which
/// struct field or parameter holds the count.
fn write_mut_slice_len_companion(
    w: &mut Writer,
    imports: &mut IndexSet<&'static str>,
    target: &ImplTarget,
    cmd: &VkCommand,
    params: &[CleanParam],
    vtable: &str,
    unwrap: &str,
    field: &str,
    pre_key: &str,
) {
    imports.insert("use core::mem::MaybeUninit;");
    let struct_name = target.struct_name();
    let wrapper_name = fn_sig(&cmd.name, struct_name);

    // Same params as the main fn, but the mutable slice is omitted because the
    // caller doesn't need to supply a buffer just to ask for the size.
    let sig_params: Vec<String> = params
        .iter()
        .filter_map(|p| match p {
            CleanParam::SelfHandle => Some("&self".to_string()),
            CleanParam::Single { name, ty, .. } => Some(format!("{name}: {ty}")),
            // The mutable slice is what we're sizing — skip it.
            CleanParam::Slice { mutable: true, .. } => None,
            // Immutable slices are kept (they may be needed to derive the count).
            CleanParam::Slice {
                slice_name,
                elem_ty,
                ..
            } => Some(format!("{slice_name}: &[{elem_ty}]")),
            // Count params are always derived, never appear in wrapper signatures.
            CleanParam::CountForSlice { .. } => None,
            // Output params are not relevant here.
            CleanParam::Output { .. } => None,
        })
        .collect();

    // The count is whatever width the C signature says (`uint32_t*` or `size_t*`);
    // `out` must match it exactly or `assume_init` reads bytes the driver never wrote.
    let mut count_ty = "usize".to_string();

    let call_args: Vec<String> = params
        .iter()
        .flat_map(|p| match p {
            CleanParam::SelfHandle => Some(target.handle_field().to_string()),

            CleanParam::Single { name, ty, convert } => {
                let name = simple_rust_member(name);
                let statement = if ty.starts_with("Option<&mut") {
                    format!("{name}.map_or(null_mut(), from_mut)")
                } else if ty.starts_with("Option<&") {
                    format!("{name}.map_or(null(), from_ref)")
                } else {
                    name.clone()
                };
                if *convert {
                    Some(format!("{statement} as _"))
                } else {
                    Some(statement)
                }
            }

            CleanParam::Slice {
                slice_name,
                mutable,
                ..
            } => Some(if *mutable {
                imports.insert("use core::ptr;");
                format!("ptr::null_mut()")
            } else {
                format!("{slice_name}.as_ptr()")
            }),

            CleanParam::Output { .. } => None,

            CleanParam::CountForSlice {
                ty,
                slice_name,
                mutable,
                ..
            } => {
                if *mutable {
                    count_ty = ty.strip_prefix("*mut ").unwrap_or("usize").to_string();
                    Some("out.as_mut_ptr()".to_string())
                } else {
                    Some(format!("{slice_name}.len() as {ty}"))
                }
            }
        })
        .collect();

    // Wrap the raw call according to return type and output shape.
    let widen = if count_ty == "usize" { "" } else { " as usize" };
    let call = match cmd.return_type.as_str() {
        "vkResult" => {
            let map = if widen.is_empty() {
                String::new()
            } else {
                format!(".map(|v| v{widen})")
            };
            format!(
                "unsafe {{ ({vtable}.{field}{unwrap})({}) }}.init_on_success(out){map}",
                call_args.join(", ")
            )
        }
        "c_void" => {
            format!(
                "unsafe {{ ({vtable}.{field}{unwrap})({});\nout.assume_init(){widen} }}",
                call_args.join(", ")
            )
        }
        d => unreachable!("{}", d),
    };

    let out = match cmd.return_type.as_str() {
        "vkResult" => "Result<usize>",
        "c_void" => "usize",
        _ => unreachable!(),
    };

    w.ln(&format!(
        "/// Returns the required slice length for Call [`{wrapper_name}`][`Self::{wrapper_name}`]."
    ));
    w.ln("#[inline]");
    w.ln(&format!(
        "{pre_key}fn {wrapper_name}_len({}) -> {out} {{",
        sig_params.join(", "),
    ));
    w.ln(&format!(
        "let mut out: MaybeUninit<{count_ty}> = MaybeUninit::uninit();"
    ));
    w.ln(&call);
    w.ln("}");
    w.blank();
}

/// Writes a bare method signature (no body) for use in a trait definition.
///
/// Calls [`compute_fn_signature`] — the same helper used by [`write_command_wrapper`] —
/// so trait signatures and impl signatures are always identical.
pub fn write_command_signature(
    w: &mut Writer,
    target: &ImplTarget,
    cmd: &VkCommand,
    lifetimes: &HashSet<String>,
) {
    let mut imports = IndexSet::new();
    let params = analyze_params(cmd, &mut imports, lifetimes);
    let vec = VEC_FNS.iter().any(|f| f == &cmd.name);
    let sig = compute_fn_signature(target, cmd, &params, vec, &mut imports, lifetimes, false);

    w.ln(&format!(
        "fn {}({}){}; ",
        sig.wrapper_name,
        sig.sig_params.join(", "),
        sig.ret_str,
    ));

    if find_len_fn(&cmd.name).is_some() {
        let sig = compute_fn_signature(target, cmd, &params, vec, &mut imports, lifetimes, true);

        w.ln(&format!(
            "fn {}_len({}){}; ",
            sig.wrapper_name,
            sig.sig_params.join(", "),
            sig.ret_str,
        ));
    }
    w.blank();
}

fn write_fn_body(
    w: &mut Writer,
    target: &ImplTarget,
    cmd: &VkCommand,
    params: &[CleanParam],
    output_ty: &Option<String>,
    vtable: &str,
    unwrap: &str,
    field: &str,
    vec: bool,
    vec_out: Option<&str>,
    slice_assert: Option<&str>,
) {
    let mut by_count: HashMap<&str, Vec<&str>> = HashMap::new();
    for p in params {
        if let CleanParam::Slice {
            slice_name,
            count_name,
            ..
        } = p
        {
            by_count
                .entry(count_name.as_str())
                .or_default()
                .push(slice_name.as_str());
        }
    }
    for slices in by_count.values() {
        if slices.len() >= 2 {
            // Emit pairwise: first.len() == second.len() == …
            let first = slices[0];
            for other in &slices[1..] {
                w.ln(&format!("assert_eq!({first}.len(), {other}.len());"));
            }
        }
    }
    if let Some(ty) = output_ty
        && !vec
    {
        w.ln(&format!(
            "let mut out = MaybeUninit::new(Default::default());"
        ));
    }

    let mut convert_out = false;

    let call_args: Vec<String> = params
        .iter()
        .map(|p| match p {
            CleanParam::SelfHandle => target.handle_field().to_string(),

            CleanParam::Single { name, ty, convert } => {
                let name = simple_rust_member(name);
                let statement = if ty.starts_with("Option<&mut") {
                    format!("{name}.map_or(null_mut(), from_mut)")
                } else if ty.starts_with("Option<&") {
                    format!("{name}.map_or(null(), from_ref)")
                } else {
                    name.clone()
                };
                if *convert {
                    format!("{statement} as _")
                } else {
                    statement
                }
            }

            CleanParam::Slice {
                slice_name,
                mutable,
                elem_ty,
                ..
            } => {
                if vec {
                    if elem_ty == "u8" {
                        "data.cast()".to_string()
                    } else {
                        "data".to_string()
                    }
                } else if *mutable {
                    if let Some(assert) = slice_assert
                        && !assert.is_empty()
                    {
                        w.ln(&format!(
                            "assert_eq!({slice_name}.len(), {assert} as usize);"
                        ));
                    }
                    if elem_ty == "u8" {
                        format!("{slice_name}.as_mut_ptr().cast()")
                    } else {
                        format!("{slice_name}.as_mut_ptr()")
                    }
                } else {
                    if elem_ty == "u8" {
                        format!("{slice_name}.as_ptr().cast()")
                    } else {
                        format!("{slice_name}.as_ptr()")
                    }
                }
            }

            CleanParam::Output { convert, .. } => {
                convert_out = *convert;
                "out.as_mut_ptr()".to_string()
            }

            CleanParam::CountForSlice {
                name,
                ty,
                slice_name,
                ..
            } => {
                if vec {
                    "count".to_string()
                } else if let Some(pointee) = ty.strip_prefix("*mut ") {
                    // Enumerate-style count: the driver reads the slice capacity through
                    // this pointer and writes back how many elements it filled, so it needs
                    // a real local. Casting `len` straight to a pointer would hand the
                    // driver the length *as an address*.
                    // `len()` is already a `usize`; only non-`usize` counts need the cast.
                    let value = if pointee == "usize" {
                        format!("{slice_name}.len()")
                    } else {
                        format!("{slice_name}.len() as {pointee}")
                    };
                    w.ln(&format!("let mut {name} = {value};"));
                    format!("&mut {name}")
                } else if ty == "usize" {
                    format!("{slice_name}.len()")
                } else {
                    format!("{slice_name}.len() as {ty}")
                }
            }
        })
        .collect();

    w.ln(&format!("let call = {vtable}.{field}{unwrap};\n"));

    let mut bool_out = false;

    // Wrap the raw call according to return type and output shape.
    let call = match (cmd.return_type.as_str(), output_ty) {
        ("vkResult", Some(ty)) => {
            if ty == "bool" {
                bool_out = true;
            }

            if vec {
                if ty == "Vec<u8>" {
                    format!(
                        "read_into_vec_result(|count, data: *mut u8| unsafe {{ (call)({}) }})",
                        call_args.join(", ")
                    )
                } else {
                    format!(
                        "read_into_vec_result(|count, data| unsafe {{ (call)({}) }})",
                        call_args.join(", ")
                    )
                }
            } else if RETURNS_SUBOPTIMAL.contains(&cmd.name.as_str()) {
                format!(
                    "unsafe {{ (call)({}) }}.init_on_success_or_suboptimal(out)",
                    call_args.join(", ")
                )
            } else {
                format!(
                    "unsafe {{ (call)({}) }}.init_on_success(out)",
                    call_args.join(", ")
                )
            }
        }
        ("vkResult", None) => {
            if RETURNS_SUBOPTIMAL.contains(&cmd.name.as_str()) {
                format!(
                    "unsafe {{ (call)({}) }}.result_or_suboptimal()",
                    call_args.join(", ")
                )
            } else {
                format!("unsafe {{ (call)({}) }}.result()", call_args.join(", "))
            }
        }
        ("c_void", Some(ty)) => {
            if vec {
                format!(
                    "read_into_vec(|count, data| unsafe {{ (call)({}) }})",
                    call_args.join(", ")
                )
            } else {
                // `out` is a `MaybeUninit<Bool32>` here (its type is inferred from the
                // raw function pointer), so it needs the same `!= 0` conversion the
                // `vkResult` path gets via `bool_out`.
                let init = if ty == "bool" {
                    "out.assume_init() != 0"
                } else {
                    "out.assume_init()"
                };
                format!("unsafe {{ (call)({});\n{init} }}", call_args.join(", "))
            }
        }
        ("c_void", None) => format!("unsafe {{ (call)({}) }};", call_args.join(", ")),
        _ => format!("unsafe {{ (call)({}) }}", call_args.join(", ")),
    };

    let _ = vec_out; // already consumed via output_ty; retained for future use
    if bool_out {
        w.ln(&format!("{call}.map(|v| v != 0)"))
    } else {
        w.ln(&call)
    };
}

fn vtable_for_version(cmd: &VkCommand) -> String {
    let version = cmd.table_name();
    let unwrap = if matches!(version, Depends::Ext(_)) {
        ".as_ref().expect(Self::EXT_LOAD_ERROR)"
    } else {
        ""
    };
    let version = version.to_string().to_snake_case();
    format!("self.fns().{}{unwrap}", version)
}

fn write_cmd_docs(w: &mut Writer, cmd: &VkCommand) {
    if cmd.renderpass == RenderPass::None {
        return;
    }

    w.ln("///");
    if cmd.conditional_rendering {
        w.ln("/// Affected by Conditional Rendering.");
    }
    let queues = cmd
        .queues
        .iter()
        .map(|n| format!("`{}`", value_to_const_name("VK_QUEUE", n)))
        .collect::<Vec<_>>()
        .join(", ");
    w.ln(&format!("/// Queues types: {queues}."));

    let tasks = cmd
        .tasks
        .iter()
        .map(|t| format!("`{}`", t.print()))
        .collect::<Vec<_>>()
        .join(", ");
    w.ln(&format!("/// Task: {tasks}."));

    let renderpass = cmd.renderpass.print();
    w.ln(&format!("{renderpass}."));

    let cmd_buffer_level = cmd
        .cmd_buffer_level
        .split(",")
        .map(|c| format!("`{c}`"))
        .collect::<Vec<_>>()
        .join(", ");
    w.ln(&format!("/// Command buffer level: {cmd_buffer_level}."));
}
