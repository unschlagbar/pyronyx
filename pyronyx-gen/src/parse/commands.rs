use crate::codegen::impls::ImplTarget;
use crate::parse::c_to_rust;
use crate::parse::registry::{Registry, Task, VkCommand};
use crate::{codegen::rust_name, parse::registry::RenderPass};
use indexmap::{IndexMap, IndexSet};
use roxmltree::Node;

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct Param {
    pub name: String,
    pub ty: String,
    pub optional: bool,
    pub len: Option<String>,
    pub pointer_depth: u32,
    pub is_const: bool,
    /// Points to a struct starting with `sType`/`pNext`, so it must be initialised
    /// (and may be extended) by the caller.
    pub extensible: bool,
}

pub fn parse_into(commands_node: &Node, reg: &mut Registry) {
    for node in commands_node
        .children()
        .filter(|n| n.has_tag_name("command"))
    {
        if let Some(alias) = node.attribute("alias") {
            let name = node.attribute("name").unwrap_or("").to_string();
            if name.is_empty() {
                continue;
            }
            reg.commands.insert(
                name.clone(),
                VkCommand {
                    name,
                    return_type: alias.to_string(),
                    params: vec![],
                    target: None,
                    alias: Some(alias.to_string()),
                    success_codes: vec![],
                    error_codes: vec![],
                    version: None,
                    extension: None,
                    conditional_rendering: false,
                    renderpass: RenderPass::None,
                    queues: vec![],
                    cmd_buffer_level: String::new(),
                    tasks: vec![],
                },
            );
            continue;
        }

        let proto = match node.children().find(|n| n.has_tag_name("proto")) {
            Some(p) => p,
            None => continue,
        };

        let (return_type, _, name) = parse_member(proto);
        if name.is_empty() {
            continue;
        }

        let mut other_params = IndexSet::new();

        let params: Vec<Param> = node
            .children()
            .filter(|n| n.has_tag_name("param"))
            .filter_map(|p| parse_param(p, &mut other_params, &reg.stypes))
            .collect();

        let target = ImplTarget::from_first_param(&params);

        let success_codes = node
            .attribute("successcodes")
            .map(|s| s.split(',').map(str::to_string).collect())
            .unwrap_or_default();

        let error_codes = node
            .attribute("errorcodes")
            .map(|s| s.split(',').map(str::to_string).collect())
            .unwrap_or_default();

        let queues = node
            .attribute("queues")
            .map(|s| s.split(',').map(str::to_string).collect())
            .unwrap_or_default();

        let tasks = node
            .attribute("tasks")
            .map(|s| s.split(',').map(Task::from_str).collect())
            .unwrap_or_default();

        let renderpass = node
            .attribute("renderpass")
            .map(RenderPass::from_str)
            .unwrap_or_default();

        let cmd_buffer_level = node
            .attribute("cmdbufferlevel")
            .unwrap_or_default()
            .to_string();

        let conditional_rendering =
            node.attribute("conditionalrendering").unwrap_or_default() == "true";

        reg.commands.insert(
            name.clone(),
            VkCommand {
                name,
                return_type,
                params,
                target,
                alias: None,
                success_codes,
                error_codes,
                version: None,
                extension: None,
                conditional_rendering,
                renderpass,
                queues,
                cmd_buffer_level,
                tasks,
            },
        );
    }
}

pub fn parse_param(
    node: Node,
    other_params: &mut IndexSet<String>,
    stypes: &IndexMap<String, (String, bool)>,
) -> Option<Param> {
    let (full_ty, base_ty, name) = parse_member(node);
    if other_params.contains(&name) {
        return None;
    } else {
        other_params.insert(name.clone());
    }

    let is_const = full_ty.contains("const");
    let pointer_depth = full_ty.chars().filter(|&c| c == '*').count() as u32;
    let rust_base = rust_name(&base_ty);

    let ty = match pointer_depth {
        0 => rust_base,
        1 => {
            if is_const {
                format!("*const {rust_base}")
            } else {
                format!("*mut {rust_base}")
            }
        }
        2 => {
            if is_const {
                format!("*const *const {rust_base}")
            } else {
                format!("*mut *mut {rust_base}")
            }
        }
        _ => format!("*mut {rust_base}"),
    };

    Some(Param {
        name,
        ty,
        optional: node
            .attribute("optional")
            .map(|v| v.contains("true"))
            .unwrap_or(false),
        len: node.attribute("len").map(str::to_string),
        pointer_depth,
        is_const,
        extensible: pointer_depth > 0 && stypes.contains_key(&base_ty.to_lowercase()),
    })
}

fn parse_member(node: Node) -> (String, String, String) {
    let mut full_ty = String::new();
    let mut ty = String::new();
    let mut name = String::new();
    let mut past_name = false;

    for child in node.children() {
        match child.tag_name().name() {
            "type" => {
                ty = c_to_rust(child.text().unwrap_or("")).to_string();
                if !past_name {
                    full_ty.push_str(&ty);
                }
            }
            "name" => {
                name = child.text().unwrap_or("").trim().to_string();
                past_name = true;
            }
            "" => {
                if let Some(text) = child.text()
                    && !past_name
                {
                    full_ty.push_str(text);
                }
            }
            _ => {}
        }
    }

    (full_ty.trim().to_string(), ty, name)
}
