use std::{collections::HashSet, fs};

use super::{Writer, file_header};
use crate::{
    codegen::{
        HAND_WRITTEN_FNS, const_name,
        impls::{ImplTarget, write_command_signature, write_command_wrapper},
    },
    parse::{
        self,
        registry::{Extension, Registry},
    },
};
use heck::ToPascalCase;
use indexmap::{IndexMap, IndexSet};
use parse::registry::VkCommand;

pub fn generate(registry: &Registry, out_path: &str, lifetimes: &HashSet<String>) {
    let mut extensions = IndexMap::new();
    for extension in &registry.extensions {
        if extension.disabled {
            continue;
        }
        let name = const_name(&extension.name).to_string();
        let vendor = name.split_once('_').unwrap().0;

        if !extensions.contains_key(vendor) {
            extensions.insert(vendor.to_string(), Vec::new());
        }

        let folder = extensions.get_mut(vendor).unwrap();
        folder.push(extension.name.to_string());
    }

    let mut outer_mod_rs = Writer::new();

    for (name, vendor_ext) in extensions.into_iter() {
        let lower_name = name.to_lowercase();
        let folder_name = format!("{out_path}/{lower_name}");

        let mut mod_rs = Writer::new();

        for extension in vendor_ext {
            let name = move_digits_to_end(const_name(&extension).split_once('_').unwrap().1);

            generate_extension(
                &mut mod_rs,
                registry,
                extension,
                &name,
                &format!("{folder_name}/{name}.rs"),
                lifetimes,
            );
        }

        if mod_rs.len() != 0 {
            outer_mod_rs.ln(&format!("pub mod {lower_name};"));
            if fs::read_dir(&folder_name).is_err() {
                fs::create_dir(&folder_name).unwrap();
            }
            mod_rs.save(format!("{folder_name}/mod.rs"));
        }
    }
    outer_mod_rs.save(format!("{out_path}/mod.rs"));
}

fn generate_extension(
    mod_rs: &mut Writer,
    registry: &Registry,
    extension: String,
    orig_name: &str,
    path: &str,
    lifetimes: &HashSet<String>,
) {
    let name = orig_name.to_pascal_case();
    let mut w = Writer::new();

    file_header(&mut w, "generated extensions");
    w.blank();

    let len = w.len();
    generate_impl(
        registry, extension, name, orig_name, &mut w, mod_rs, lifetimes,
    );

    if w.len() != len {
        w.save(path);
        mod_rs.ln(&format!("pub mod {orig_name};"));
    }
}

pub fn generate_impl(
    registry: &Registry,
    extension: String,
    name: String,
    orig_name: &str,
    w: &mut Writer,
    mod_rs: &mut Writer,
    lifetimes: &HashSet<String>,
) {
    let mut by_target: IndexMap<ImplTarget, Vec<&VkCommand>> = IndexMap::new();

    for (_, cmd) in &registry.commands {
        if cmd.alias.is_some() || cmd.extension.as_ref() != Some(&extension) {
            continue;
        }
        if let Some(target) = cmd.target {
            by_target.entry(target).or_default().push(cmd);
        }
    }
    let mut inner_w = Writer::new();

    if by_target.is_empty() {
        if let Some(ext) = registry.extensions.iter().find(|e| e.name == extension) {
            if let Some(deprecated) = &ext.deprecated_by {
                mod_rs.ln(&format!(
                    "#[deprecated = \"This extension is deprecated. Use `{}` instead.\"]",
                    deprecated
                ));
            }
            mod_rs.ln(&format!("pub mod {orig_name}{{"));
            mod_rs.ln("use core::ffi::CStr;\n
            ");
            write_ext_consts(mod_rs, ext, registry);
            mod_rs.ln("}");
        }
        return;
    } else {
        if let Some(ext) = registry.extensions.iter().find(|e| e.name == extension) {
            write_ext_consts(&mut inner_w, ext, registry);

            if let Some(deprecated) = &ext.deprecated_by {
                w.ln(&format!(
                    "#![deprecated = \"This extension is deprecated. Use `{}` instead.\"]",
                    deprecated
                ));
            }
        }
    };

    let mut imports = IndexSet::new();
    imports.insert("use core::ffi::CStr;");
    w.ln("use crate::vk::*;");

    for (target, cmds) in by_target {
        inner_w.blank();

        let target_name = target.struct_name();
        let tar_str = target_name; // du hattest needs_target = true

        inner_w.ln(&format!("pub trait {name}{tar_str} {{"));
        for cmd in &cmds {
            if !HAND_WRITTEN_FNS
                .iter()
                .any(|(tar, n)| *tar == target_name && n == &cmd.name)
            {
                write_command_signature(&mut inner_w, &target, cmd, lifetimes);
            }
        }
        inner_w.ln("}");

        inner_w.blank();
        inner_w.ln(&format!("impl {name}{tar_str} for {} {{", target_name));

        for cmd in &cmds {
            if !HAND_WRITTEN_FNS
                .iter()
                .any(|(tar, n)| *tar == target_name && n == &cmd.name)
            {
                write_command_wrapper(&mut inner_w, &target, cmd, "", &mut imports, lifetimes);
            }
        }

        inner_w.ln("}");
    }

    for import in imports {
        w.ln(import);
    }
    w.blank();
    w.ln(&inner_w.into_string());
}

fn move_digits_to_end(s: &str) -> String {
    s.char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| {
            if i == 0 {
                s.to_string()
            } else {
                let (digits, rest) = s.split_at(i);
                format!(
                    "{}_{}",
                    rest.strip_prefix("_").unwrap_or(rest),
                    digits.strip_suffix("_").unwrap_or(digits)
                )
            }
        })
        .unwrap_or(s.to_string())
}

/// `NAME` and `SPEC_VERSION`, documented with the extension's type and promotion.
fn write_ext_consts(w: &mut Writer, ext: &Extension, registry: &Registry) {
    if !ext.typ.is_empty() {
        w.ln(&format!("/// Type: `{}`", ext.typ.to_pascal_case()));
    }
    if let Some(promoted) = &ext.promoted_to {
        w.ln("///");
        match promoted.strip_prefix("VK_VERSION_") {
            Some(version) => w.ln(&format!(
                "/// Promoted to core in Vulkan {}",
                version.replace('_', ".")
            )),
            None => w.ln(&format!("/// Promoted to {}", ext_link(promoted, registry))),
        }
    }
    if let Some(depends) = &ext.depends {
        w.ln("///");
        w.ln(&format!("/// Requires: {}", depends_doc(depends, registry)));
    }
    w.ln(&format!("pub const NAME: &CStr = c\"{}\";", ext.name));
    w.ln(&format!(
        "pub const SPEC_VERSION: u32 = {};",
        ext.spec_version
    ));
}

/// `VK_KHR_swapchain+(VK_KHR_get_physical_device_properties2,VK_VERSION_1_1)` →
/// `` [`VK_KHR_swapchain`](…) + ([`VK_KHR_get_physical_device_properties2`](…) or Vulkan 1.1) ``
fn depends_doc(depends: &str, registry: &Registry) -> String {
    let mut out = String::new();
    let mut name = String::new();
    let flush = |name: &mut String, out: &mut String| {
        if name.is_empty() {
            return;
        }
        match name.strip_prefix("VK_VERSION_") {
            Some(version) => out.push_str(&format!("Vulkan {}", version.replace('_', "."))),
            None => out.push_str(&ext_link(name, registry)),
        }
        name.clear();
    };
    for c in depends.chars() {
        match c {
            '+' | ',' | '(' | ')' => {
                flush(&mut name, &mut out);
                out.push_str(match c {
                    '+' => " + ",
                    ',' => " or ",
                    '(' => "(",
                    _ => ")",
                });
            }
            _ => name.push(c),
        }
    }
    flush(&mut name, &mut out);
    out
}

/// Doc link to an extension's module, or plain code when it has none (e.g. disabled ones).
fn ext_link(name: &str, registry: &Registry) -> String {
    if registry
        .extensions
        .iter()
        .any(|e| e.name == name && !e.disabled)
    {
        format!("[`{name}`]({})", module_path(name))
    } else {
        format!("`{name}`")
    }
}

/// `VK_KHR_get_physical_device_properties2` → `crate::khr::get_physical_device_properties2`
fn module_path(name: &str) -> String {
    let (vendor, name) = const_name(name).split_once('_').unwrap();
    format!(
        "crate::{}::{}",
        vendor.to_lowercase(),
        move_digits_to_end(name)
    )
}
