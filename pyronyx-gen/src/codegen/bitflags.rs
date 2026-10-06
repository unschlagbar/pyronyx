use heck::ToSnakeCase;

use super::{Writer, file_header};
use crate::{
    codegen::{VENDORS, const_name, enums::better_pascal_case, rust_name},
    parse::{
        registry::{Registry, VkEnum},
        types::TypeKind,
    },
};
use std::collections::HashSet;

pub fn generate(registry: &Registry, out_path: &str) {
    let mut w = Writer::new();
    file_header(&mut w, "vk/bitflags.rs");
    w.ln("#![allow(non_upper_case_globals)]");
    w.ln("use crate::{vk::types::{Flags, Flags64}, vk_bitflags_wrapped};");
    w.blank();

    for ty in registry.types.values() {
        let (bits_enum, repr) = match &ty.kind {
            TypeKind::Bitmask { bits_enum, repr } => (bits_enum, repr),
            _ => continue,
        };

        if let Some(en) = bits_enum
            && let Some(en) = registry.enums.get(en)
        {
            write_bitflag_struct(&mut w, en, repr);
        } else {
            write_empty_bitflags(&mut w, &ty.name, repr);
        };
    }

    w.save(out_path);
}

fn write_bitflag_struct(w: &mut Writer, en: &VkEnum, repr: &str) {
    let struct_name = rust_name(&en.name);

    w.ln(&format!(
        "/// <https://docs.vulkan.org/refpages/latest/refpages/source/{}.html>",
        en.name
    ));
    w.ln("#[repr(transparent)]");
    w.ln("#[derive(Clone, Copy, Hash, PartialEq, Eq)]");
    w.ln(&format!("pub struct {struct_name}(pub(crate) {repr});"));
    w.ln(&format!("vk_bitflags_wrapped!({struct_name}, {repr});"));
    w.blank();

    if !en.values.is_empty() {
        w.ln(&format!("impl {struct_name} {{"));

        let mut seen_names: HashSet<String> = HashSet::new();

        for v in &en.values {
            let const_name = value_to_const_name(&en.name, &v.name);
            if !seen_names.insert(const_name.clone()) {
                continue;
            }

            if let Some(c) = &v.comment {
                w.ln(&format!("/// {c}"));
            }

            let literal = to_binary_literal(v.value as u64);
            w.ln(&format!("pub const {const_name}: Self = Self({literal});"));
        }

        w.ln("}");
        w.blank();
    }
}

fn write_empty_bitflags(w: &mut Writer, name: &str, repr: &str) {
    w.ln(&format!(
        "/// <https://docs.vulkan.org/refpages/latest/refpages/source/{name}.html>",
    ));
    w.ln(&format!("pub type {} = {repr};", rust_name(name)));
    w.blank();
}

/// Converts a raw Vulkan enum value name into its Rust constant / variant name.
///
/// Examples:
/// - `"VK_ACCESS_INDIRECT_COMMAND_READ_BIT"` under `"VkAccessFlagBits"` → `"IndirectCommandRead"`
/// - `"VK_ACCESS_INDIRECT_COMMAND_READ_BIT_KHR"` → `"IndirectCommandReadKHR"`
/// - `"VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_IMPLICIT_DESTINATIONS_NV"`
///   under `"VkClusterAccelerationStructureOpModeNV"` → `"ImplicitDestinations"`
pub fn value_to_const_name(enum_name: &str, vk_name: &str) -> String {
    let prefix = enum_name_to_screaming_prefix(enum_name);
    let without_prefix = vk_name
        .strip_prefix(&prefix)
        .unwrap_or_else(|| const_name(vk_name));

    // Determine whether the enum type itself carries a vendor suffix (e.g. "NV").
    // When it does, we omit the per-value vendor suffix from the output name
    // (it would just be redundant noise).
    let temp = enum_name.to_snake_case().to_uppercase();
    let (_, enum_suffix) = split_vendor_suffix(&temp);

    let (name, vendor) = split_vendor_suffix(without_prefix);

    let clean = name.strip_suffix("_BIT").unwrap_or(name);
    let clean = clean.strip_suffix("_BIT_").unwrap_or(clean);
    let clean = move_digits_to_end(&better_pascal_case(clean));

    if enum_suffix.is_empty() {
        format!("{clean}{}", vendor.to_uppercase())
    } else {
        clean
    }
}

/// Returns the screaming-snake prefix that all Vulkan value names belonging to
/// `enum_name` share, **without** the vendor suffix.
///
/// This is the single shared implementation used by both enum and bitflag
/// codegen so they always agree on how to strip value-name prefixes.
///
/// | `enum_name`                               | result                                    |
/// |-------------------------------------------|-------------------------------------------|
/// | `VkFormat`                                | `"VK_FORMAT_"`                            |
/// | `VkImageLayout`                           | `"VK_IMAGE_LAYOUT_"`                      |
/// | `VkClusterAccelerationStructureOpModeNV`  | `"VK_CLUSTER_ACCELERATION_STRUCTURE_OP_MODE_"` |
/// | `VkAccessFlagBits`                        | `"VK_ACCESS_"`                            |
/// | `VkAccessFlagBits2KHR`  (Sync2)           | `"VK_ACCESS_2_"`                          |
pub fn enum_name_to_screaming_prefix(name: &str) -> String {
    // For FlagBits enums, strip "FlagBits" and capture any version digit that
    // immediately follows (e.g. the "2" in "VkAccessFlagBits2KHR").
    let (base, version_suffix) = match name.find("FlagBits") {
        Some(i) => {
            let after = &name[i + "FlagBits".len()..];
            let digits: String = after.chars().take_while(|c| c.is_ascii_digit()).collect();
            let s = if digits.is_empty() {
                String::new()
            } else {
                format!("_{digits}")
            };
            (&name[..i], s)
        }
        None => (name, String::new()),
    };

    let screaming = format!("{base}{version_suffix}")
        .to_snake_case()
        .to_uppercase();

    // Strip the vendor suffix (e.g. "_NV", "_KHR") from the enum name so it
    // does not become part of the value-name prefix.  split_vendor_suffix
    // leaves a trailing "_" when a suffix was present; we normalise to always
    // end with exactly one "_".
    let (without_vendor, _) = split_vendor_suffix(&screaming);
    format!("{}_", without_vendor.trim_end_matches('_'))
}

/// Splits a SCREAMING_SNAKE name into `(base, vendor_suffix)`.
///
/// Examples:
/// - `"INDIRECT_COMMAND_READ_BIT_KHR"` → `("INDIRECT_COMMAND_READ_BIT_", "KHR")`  (wait, actually it's `(&name[..n-3], "KHR")`)
/// - `"INDIRECT_COMMAND_READ_BIT"` → `("INDIRECT_COMMAND_READ_BIT", "")`
pub fn split_vendor_suffix(name: &str) -> (&str, &str) {
    for vendor in VENDORS {
        if name.ends_with(&vendor.to_uppercase()) {
            return (&name[..name.len() - vendor.len()], vendor);
        }
    }
    (name, "")
}

/// Moves a leading digit run to the end of a PascalCase identifier so that the
/// result is a valid Rust identifier.
///
/// - `"8Bit"` → `"Type8Bit"` (single leading digit → prefix with "Type")
/// - `"R8G8"` → `"R8G8"` (no leading digit → unchanged)
pub fn move_digits_to_end(s: &str) -> String {
    if s.chars().next().is_some_and(|c| c.is_ascii_digit()) {
        return format!("Type{s}");
    }
    let split_index = s
        .char_indices()
        .find(|(_, c)| !c.is_ascii_digit())
        .map(|(i, _)| i)
        .unwrap_or(s.len());

    let (digits, rest) = s.split_at(split_index);
    format!("{}{}", rest, digits)
}

/// Formats a `u64` as a grouped binary literal (`0b1_0000_0000`).
fn to_binary_literal(value: u64) -> String {
    if value == 0 {
        return "0".to_string();
    }

    let bits = format!("{:b}", value);

    let grouped: String = bits
        .chars()
        .rev()
        .enumerate()
        .flat_map(|(i, c)| {
            if i > 0 && i % 4 == 0 {
                vec!['_', c]
            } else {
                vec![c]
            }
        })
        .collect::<String>()
        .chars()
        .rev()
        .collect();

    format!("0b{grouped}")
}
