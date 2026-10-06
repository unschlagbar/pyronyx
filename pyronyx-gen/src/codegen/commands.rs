use std::collections::{HashMap, HashSet};

use super::{Writer, file_header};
use crate::{
    codegen::{VENDORS, rust_member, rust_name},
    parse::{
        self,
        registry::{Depends, Registry},
    },
};
use heck::ToSnakeCase;
use indexmap::{IndexMap, IndexSet};
use parse::registry::VkCommand;

pub fn generate(registry: &Registry, out_path: &str, table_out_path: &str) {
    let mut w = Writer::new();
    file_header(&mut w, "vk/commands.rs");
    w.ln("#![allow(non_camel_case_types)]");
    w.ln("#![allow(unused)]");
    w.ln("use super::types::*;");
    w.ln("use super::platform_types::*;");
    w.ln("use super::enums::*;");
    w.ln("use super::bitflags::*;");
    w.ln("use core::ffi::{c_void, c_char, c_int};");
    w.ln("use crate::utils::to_option;");
    w.blank();

    for (_, cmd) in &registry.commands {
        write_pfn_type(&mut w, cmd);
        w.blank();
    }
    w.save(out_path);

    let mut w = Writer::new();
    file_header(&mut w, "vk/commands.rs");
    w.ln("use super::vk::*;");
    w.ln("use core::ffi::{CStr, c_void, c_char};");
    w.ln("use crate::utils::to_option;");

    let mut instance_table = IndexSet::new();
    let mut device_table = IndexSet::new();

    write_dispatch_groups(
        &mut w,
        "InstanceFn",
        registry,
        &mut instance_table,
        true,
        is_instance_command,
    );

    write_dispatch_groups(
        &mut w,
        "PhysicalDeviceFn",
        registry,
        &mut instance_table,
        true,
        is_physical_device_command,
    );

    write_dispatch_groups(
        &mut w,
        "DeviceFn",
        registry,
        &mut device_table,
        false,
        is_device_command,
    );

    write_dispatch_groups(
        &mut w,
        "QueueFn",
        registry,
        &mut device_table,
        false,
        is_queue_command,
    );

    write_dispatch_groups(
        &mut w,
        "CommandBufferFn",
        registry,
        &mut device_table,
        false,
        is_cmd_command,
    );

    write_dispatch_table(&mut w, "InstanceVTable", instance_table);
    write_dispatch_table(&mut w, "DeviceVTable", device_table);

    w.save(table_out_path);
}

fn write_pfn_type(w: &mut Writer, cmd: &VkCommand) {
    let mut ps = HashSet::new();

    let params = cmd
        .params
        .iter()
        .map(|p| format!("{}: {}", rust_member(&p.name, &mut ps), &p.ty))
        .collect::<Vec<_>>()
        .join(", ");

    let ret = if cmd.return_type == "c_void" {
        String::new()
    } else {
        format!(" -> {}", rust_name(&cmd.return_type))
    };
    w.ln(&format!(
        "pub type {name} = unsafe extern \"system\" fn({params}){ret};",
        name = cmd.name,
    ));
}

fn version_const_name(depends_str: &str) -> String {
    let version_part = depends_str.strip_prefix("v").unwrap_or(depends_str);
    format!("API_VERSION_{}", version_part)
}

/// Writes one function table (`InstanceFn`, `DeviceFn`, …) and its per-version /
/// per-extension sub-tables.
///
/// Every command gets exactly one `Option` slot; aliases share their target's slot (like
/// `vulkan.hpp`'s dynamic loader). A slot is filled by the first enabled provider that
/// exposes it: its core version first, then any enabled extension that requires the command
/// or one of its aliases (e.g. `vkCmdDrawIndirectCount` ← `VK_KHR_draw_indirect_count`,
/// `vkReleaseSwapchainImagesKHR` ← `VK_EXT_swapchain_maintenance1`).
fn write_dispatch_groups(
    w: &mut Writer,
    struct_name: &str,
    registry: &Registry,
    groups: &mut IndexSet<String>,
    instance_level: bool,
    filter: impl Fn(&VkCommand) -> bool,
) {
    let mut tables: IndexMap<Depends, Vec<&VkCommand>> = IndexMap::new();
    for cmd in registry.commands.values() {
        if cmd.alias.is_none() && !cmd.name.ends_with("ProcAddr") && filter(cmd) {
            tables.entry(cmd.table_name()).or_default().push(cmd);
        }
    }
    if tables.is_empty() {
        return;
    }
    tables.sort_keys();
    groups.insert(struct_name.to_string());

    // command name → (table field, slot field)
    let mut slots: HashMap<&str, (String, String)> = HashMap::new();
    for (depends, cmds) in &tables {
        let table = depends.to_string().to_snake_case();
        for cmd in cmds {
            slots.insert(&cmd.name, (table.clone(), fn_field(&cmd.name)));
        }
    }

    // Extensions whose type (instance / device) differs from this table's can't be checked
    // against the `extensions` passed to `load`, e.g. device-level commands of
    // `VK_EXT_debug_utils`. Their commands are looked up unconditionally instead.
    let mut always = Vec::new();
    let mut by_extension = Vec::new();
    for ext in registry.extensions.iter().filter(|e| !e.disabled) {
        let mut fills: IndexMap<&(String, String), &str> = IndexMap::new();
        for name in ext.require_blocks.iter().flat_map(|b| &b.commands) {
            if let Some(slot) = slots.get(canonical(registry, name)) {
                fills.entry(slot).or_insert(name);
            }
        }
        if fills.is_empty() {
            continue;
        }
        if (ext.typ == "instance") == instance_level {
            by_extension.push((&ext.name, fills));
        } else {
            always.push(fills);
        }
    }

    // Number of providers per slot, the core version counting as one.
    let mut providers: HashMap<&(String, String), usize> = HashMap::new();
    for (name, slot) in &slots {
        if !matches!(registry.commands[*name].table_name(), Depends::Ext(_)) {
            providers.insert(slot, 1);
        }
    }
    for fills in always
        .iter()
        .chain(by_extension.iter().map(|(_, fills)| fills))
    {
        for slot in fills.keys() {
            *providers.entry(slot).or_default() += 1;
        }
    }

    w.ln("#[derive(Clone)]");
    w.ln(&format!("pub struct {struct_name} {{"));
    for depends in tables.keys() {
        w.ln(&format!(
            "    pub {}: {struct_name}{depends},",
            depends.to_string().to_snake_case()
        ));
    }
    w.ln("}");
    w.blank();

    w.ln(&format!("impl {struct_name} {{"));
    w.ln("    /// A table with no functions loaded; every call through it panics.");
    w.ln("    pub const EMPTY: Self = Self {");
    for depends in tables.keys() {
        let field = depends.to_string().to_snake_case();
        w.ln(&format!("        {field}: {struct_name}{depends}::EMPTY,"));
    }
    w.ln("    };");
    w.blank();
    w.ln("    pub fn load<F: FnMut(&CStr) -> *const c_void>(");
    w.ln("        mut loader: F,");
    w.ln("        api_version: u32,");
    w.ln("        extensions: &[*const c_char],");
    w.ln("    ) -> Self {");
    w.ln("        let mut out = Self {");
    for depends in tables.keys() {
        let field = depends.to_string().to_snake_case();
        let sub = format!("{struct_name}{depends}");
        match depends {
            Depends::Core(v) if v != "v1_0" => {
                let ver_const = version_const_name(v);
                w.ln(&format!(
                    "            {field}: if api_version >= {ver_const} {{ {sub}::load(&mut loader) }} else {{ {sub}::EMPTY }},"
                ));
            }
            Depends::Ext(_) => w.ln(&format!("            {field}: {sub}::EMPTY,")),
            _ => w.ln(&format!("            {field}: {sub}::load(&mut loader),")),
        }
    }
    w.ln("        };");
    for fills in &always {
        write_fills(w, fills, &providers);
    }
    w.ln("        for &ext in extensions {");
    w.ln("            match unsafe { CStr::from_ptr(ext) }.to_bytes() {");
    for (ext_name, fills) in &by_extension {
        w.ln(&format!("                b\"{ext_name}\" => {{"));
        write_fills(w, fills, &providers);
        w.ln("                }");
    }
    w.ln("                _ => (),");
    w.ln("            }");
    w.ln("        }");
    w.ln("        out");
    w.ln("    }");
    w.ln("}");
    w.blank();

    for (depends, cmds) in &tables {
        let sub = format!("{struct_name}{depends}");
        w.ln("#[derive(Clone, Default)]");
        w.ln(&format!("pub struct {sub} {{"));
        for cmd in cmds {
            w.ln(&format!(
                "    pub {}: Option<{}>,",
                fn_field(&cmd.name),
                cmd.name
            ));
        }
        w.ln("}");
        w.blank();

        w.ln(&format!("impl {sub} {{"));
        w.ln("    pub const EMPTY: Self = Self {");
        for cmd in cmds {
            w.ln(&format!("        {}: None,", fn_field(&cmd.name)));
        }
        w.ln("    };");
        if !matches!(depends, Depends::Ext(_)) {
            w.blank();
            w.ln("    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {");
            w.ln("        Self {");
            for cmd in cmds {
                w.ln(&format!(
                    r#"            {}: to_option(loader(c"{}")),"#,
                    fn_field(&cmd.name),
                    cmd.name
                ));
            }
            w.ln("        }");
            w.ln("    }");
        }
        w.ln("}");
        w.blank();
    }
}

/// Loads each slot under the name this provider exposes it as. A slot shared with other
/// providers is only filled while still empty, so a later provider can't replace (or null)
/// an earlier one.
fn write_fills(
    w: &mut Writer,
    fills: &IndexMap<&(String, String), &str>,
    providers: &HashMap<&(String, String), usize>,
) {
    for (&slot, name) in fills {
        let (table, field) = slot;
        let load = format!(r#"out.{table}.{field} = to_option(loader(c"{name}"));"#);
        if providers[slot] > 1 {
            w.ln(&format!("if out.{table}.{field}.is_none() {{ {load} }}"));
        } else {
            w.ln(&load);
        }
    }
}

/// Follows `alias` links to the command that owns the function slot.
fn canonical<'a>(registry: &'a Registry, mut name: &'a str) -> &'a str {
    while let Some(target) = registry.commands.get(name).and_then(|c| c.alias.as_deref()) {
        name = target;
    }
    name
}

fn write_dispatch_table(w: &mut Writer, struct_name: &str, groups: IndexSet<String>) {
    w.ln("#[derive(Clone)]");
    w.ln(&format!("pub struct {struct_name} {{"));
    for group in &groups {
        w.ln(&format!("    pub {}: {},", fn_field(group), group));
    }
    w.ln("}");
    w.blank();

    w.ln(&format!("impl {struct_name} {{"));
    w.ln("    pub fn load<F: FnMut(&CStr) -> *const c_void>(");
    w.ln("        mut loader: F,");
    w.ln("        api_version: u32,");
    w.ln("        extensions: &[*const c_char],");
    w.ln("    ) -> Self {");
    w.ln("debug_assert!(api_version >= API_VERSION_1_0);");
    w.ln("Self {");
    for group in &groups {
        w.ln(&format!(
            r#"{field}: {name}::load(&mut loader, api_version, extensions),"#,
            field = fn_field(group),
            name = group,
        ));
    }
    w.ln("        }");
    w.ln("    }");
    w.ln("}");
    w.blank();
}

/// "vkCreateInstance" → "create_instance"
pub fn fn_field(vk_name: &str) -> String {
    let name = vk_name.strip_prefix("vk").unwrap_or(vk_name);
    let name = name.strip_prefix("Cmd").unwrap_or(name);
    let name = name.replace("Fn", "");
    name.to_snake_case()
}

/// "vkCreateInstance" → "create_instance"
pub fn fn_sig(vk_name: &str, self_name: &str) -> String {
    let name =
        if (vk_name.contains("Buffer") || vk_name.contains("Image") || vk_name.contains("Memory"))
            && self_name == "Device"
        {
            vk_name
        } else {
            &vk_name.replace(self_name, "")
        };
    let name = name.strip_prefix("vk").unwrap_or(name);
    let name = name.strip_prefix("Cmd").unwrap_or(name);
    let name = name.replace("Fn", "");
    let mut rname = name.as_str();
    for vendor in VENDORS {
        if let Some(new) = name.strip_suffix(&vendor.to_uppercase()) {
            rname = new;
            break;
        }
    }
    rname.to_snake_case()
}

fn is_instance_command(cmd: &VkCommand) -> bool {
    cmd.params
        .first()
        .map(|p| p.ty == "vkInstance")
        .unwrap_or(false)
}

fn is_physical_device_command(cmd: &VkCommand) -> bool {
    cmd.params
        .first()
        .map(|p| &p.ty == "vkPhysicalDevice")
        .unwrap_or(false)
}

fn is_device_command(cmd: &VkCommand) -> bool {
    cmd.params
        .first()
        .map(|p| &p.ty == "vkDevice")
        .unwrap_or(false)
}

fn is_queue_command(cmd: &VkCommand) -> bool {
    cmd.params
        .first()
        .map(|p| &p.ty == "vkQueue")
        .unwrap_or(false)
}

fn is_cmd_command(cmd: &VkCommand) -> bool {
    cmd.params
        .first()
        .map(|p| &p.ty == "vkCommandBuffer")
        .unwrap_or(false)
}
