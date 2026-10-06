use crate::codegen::const_name;
use crate::parse::c_to_rust;
use crate::parse::enums::parse_extension_enum;
use crate::parse::registry::{Extension, Feature, Registry, RequireBlock, VkConstant};
use roxmltree::Node;

pub fn parse_feature(node: Node, registry: &mut Registry) {
    let name = node.attribute("name").unwrap_or("").to_string();
    let version = node
        .attribute("number")
        .map(|n| format!("v{}", n.replace(".", "_")));

    let mut feature = Feature {
        name: name.clone(),
        version: version.clone(),
        required_types: vec![],
        required_commands: vec![],
    };

    for require in node.children().filter(|n| n.has_tag_name("require")) {
        for child in require.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "type" => feature
                    .required_types
                    .push(child.attribute("name").unwrap_or("").to_string()),
                "command" => feature
                    .required_commands
                    .push(child.attribute("name").unwrap_or("").to_string()),
                "enum" => parse_extension_enum(&child, &mut registry.enums, None),
                _ => {}
            }
        }
    }

    for cmd in &feature.required_commands {
        if let Some(name) = registry.commands.get_mut(cmd) {
            name.version = version.clone();
        }
    }

    registry.features.push(feature);
}

pub fn parse_extension(node: Node, registry: &mut Registry) {
    let name = node.attribute("name").unwrap_or("").to_string();
    let number: i64 = node
        .attribute("number")
        .and_then(|n| n.parse().ok())
        .unwrap_or(1);

    let deprecated_by = node.attribute("deprecatedby").map(str::to_string);
    let promoted_to = node.attribute("promotedto").map(str::to_string);
    let depends = node.attribute("depends").map(str::to_string);
    let typ = node.attribute("type").unwrap_or("").to_string();

    let mut spec_version: u32 = 0;
    for require in node.children().filter(|n| n.has_tag_name("require")) {
        for child in require.children().filter(|n| n.has_tag_name("enum")) {
            if child
                .attribute("name")
                .map_or(false, |n| n.ends_with("_SPEC_VERSION"))
            {
                if let Some(val) = child.attribute("value") {
                    spec_version = val.parse().unwrap_or(0);
                }
                break;
            }
        }
    }

    let mut ext = Extension {
        name: name.clone(),
        number,
        spec_version,
        requires: node
            .attribute("requires")
            .map(|r| r.split(',').map(str::to_string).collect())
            .unwrap_or_default(),
        require_blocks: vec![],
        deprecated_by,
        promoted_to,
        depends,
        disabled: node.attribute("supported") == Some("disabled"),
        typ,
    };

    for require in node.children().filter(|n| n.has_tag_name("require")) {
        let mut block = RequireBlock {
            feature_guard: require.attribute("feature").map(str::to_string),
            types: vec![],
            commands: vec![],
        };

        for child in require.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "type" => block
                    .types
                    .push(child.attribute("name").unwrap_or("").to_string()),
                "command" => {
                    let cmd_name = child.attribute("name").unwrap_or("");
                    if let Some(cmd) = registry.commands.get_mut(cmd_name) {
                        cmd.extension = Some(name.clone());
                    }
                    block.commands.push(cmd_name.to_string());
                }
                "enum" => {
                    parse_extension_enum_with_number(&child, number, &name, &mut registry.enums);
                }
                _ => {}
            }
        }
        ext.require_blocks.push(block);
    }

    registry.extensions.push(ext);
}

fn parse_extension_enum_with_number(
    node: &Node,
    extnumber: i64,
    extension_name: &str,
    enums: &mut indexmap::IndexMap<String, crate::parse::registry::VkEnum>,
) {
    use crate::parse::registry::VkEnumValue;

    let name = node.attribute("name").unwrap_or("");
    let extends = match node.attribute("extends") {
        Some(e) => e,
        None => return,
    };
    let en = match enums.get_mut(extends) {
        Some(e) => e,
        None => return,
    };

    let value = if let Some(val) = node.attribute("value") {
        val.trim().parse().unwrap_or(0)
    } else if let Some(bitpos) = node.attribute("bitpos") {
        1i64 << bitpos.parse::<u32>().unwrap_or(0)
    } else if let Some(offset) = node.attribute("offset") {
        let offset: i64 = offset.parse().unwrap_or(0);
        let ext_nr: i64 = node
            .attribute("extnumber")
            .and_then(|n| n.parse().ok())
            .unwrap_or(extnumber);
        let base = 1_000_000_000 + (ext_nr - 1) * 1000 + offset;
        if node.attribute("dir") == Some("-") {
            -base
        } else {
            base
        }
    } else {
        return;
    };

    en.values.push(VkEnumValue {
        name: name.to_string(),
        value,
        comment: node.attribute("comment").map(str::to_string),
        extension: Some(extension_name.to_string()),
    });
}

pub fn parse_video_extensions(node: Node, reg: &mut Registry) {
    for ext in node.children().filter(|n| n.has_tag_name("extension")) {
        if ext.attribute("supported") == Some("disabled") {
            continue;
        }

        for require in ext.children().filter(|n| n.has_tag_name("require")) {
            for child in require.children().filter(|n| n.has_tag_name("enum")) {
                let name = match child.attribute("name") {
                    Some(n) => const_name(n),
                    None => continue,
                };
                let value = match child.attribute("value") {
                    Some(v) => v,
                    None => continue,
                };
                let ty = match child.attribute("type") {
                    Some(t) => t,
                    None => continue,
                };

                let rust_ty = c_to_rust(ty);
                let rust_value = if value.starts_with("0x") || value.starts_with("0X") {
                    format!("0x{:X}", u64::from_str_radix(&value[2..], 16).unwrap_or(0))
                } else {
                    value.to_string()
                };

                reg.constants.push(VkConstant {
                    name: name.to_string(),
                    ty: rust_ty.to_string(),
                    value: rust_value,
                    comment: child.attribute("comment").map(str::to_string),
                });
            }
        }
    }
}
