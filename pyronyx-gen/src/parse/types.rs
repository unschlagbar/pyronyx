#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use crate::{
    codegen::{rust_name, types},
    parse::{
        c_to_rust,
        commands::{Param, parse_param},
        registry::{Registry, VkConstant, VkType},
    },
};
use heck::{ToLowerCamelCase, ToSnakeCase};
use indexmap::{IndexMap, IndexSet};
use roxmltree::Node;

#[derive(Debug, Clone)]
pub enum TypeKind {
    BaseType {
        inner: String,
    },
    Handle {
        dispatchable: bool,
        parent: Option<String>,
    },
    Bitmask {
        bits_enum: Option<String>,
        repr: String,
    },
    Alias {
        alias: String,
        lifetime: bool,
    },
    Struct {
        members: Vec<Member>,
        extends: Option<String>,
        returned_only: bool,
        lifetime: bool,
    },
    Union {
        lifetime: bool,
        members: Vec<Member>,
    },
    FuncPointer {
        return_type: String,
        params: Vec<Param>,
    },
    Requires {
        requires: Option<String>,
    },
}

#[derive(Debug, Clone)]
pub struct Member {
    pub name: String,
    pub ty: String,
    pub full_ty: String,
    pub optional: bool,
    pub len: Option<String>,
    pub is_const: bool,
    pub pointer_depth: u32,
    pub array_dims: Vec<String>,
    pub comment: String,
    /// Bitfield width (`<name>foo</name>:24` in the registry), if any.
    pub bits: Option<u32>,
}

impl Member {
    fn parse(node: Node) -> Self {
        parse_member(node)
    }
    fn has_lifetime(&self) -> bool {
        self.pointer_depth != 0
    }

    pub fn has_type_lifetime(&self, types: &IndexMap<String, VkType>) -> bool {
        if let Some(ty) = types.get(&self.ty) {
            match ty.kind {
                TypeKind::Struct { lifetime, .. } => lifetime,
                TypeKind::Union { lifetime, .. } => lifetime,
                _ => false,
            }
        } else {
            false
        }
    }
}

impl Eq for Member {}

impl PartialEq for Member {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl Hash for Member {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub fn parse_into(types_node: &Node, reg: &mut Registry, lifetimes: &mut HashSet<String>) {
    for node in types_node.children().filter(|n| n.has_tag_name("type")) {
        // ── Alias ───────────────────────────────────────────────────────────
        if let Some(alias) = node.attribute("alias") {
            let name = match node.attribute("name") {
                Some(n) => n.to_string(),
                None => continue,
            };

            reg.types.insert(
                name.clone(),
                VkType {
                    name,
                    alias: Some(alias.to_string()),
                    kind: TypeKind::Alias {
                        lifetime: false,
                        alias: alias.to_string(),
                    },
                    comment: node.attribute("comment").map(str::to_string),
                },
            );
            continue;
        }

        let category = node.attribute("category").unwrap_or("");

        let name = match node.attribute("name") {
            Some(n) => n.to_string(),
            None => {
                if category == "funcpointer" {
                    let proto_name = node
                        .children()
                        .find(|n| n.has_tag_name("proto"))
                        .and_then(|proto| proto.children().find(|n| n.has_tag_name("name")))
                        .and_then(|n| n.text());

                    match proto_name {
                        Some(n) => n.to_string(),
                        None => continue,
                    }
                } else {
                    match node.children().find(|n| n.has_tag_name("name")) {
                        Some(n) => n.text().unwrap_or("").to_string(),
                        None => continue,
                    }
                }
            }
        };

        if name.is_empty() {
            continue;
        }

        if category == "define" {
            if let Some(c) = try_parse_define(&node, &name) {
                reg.constants.push(c);
            }
            continue;
        }

        let kind = match category {
            "basetype" => {
                let mut inner = node
                    .children()
                    .find(|n| n.has_tag_name("type"))
                    .and_then(|n| n.text())
                    .unwrap_or("u32")
                    .to_string();

                if inner == "void" {
                    // HACK: this can only happen if the actual type is void*
                    inner.push('*');
                }

                TypeKind::BaseType { inner }
            }

            "handle" => {
                let inner_text = node
                    .children()
                    .find(|n| n.has_tag_name("type"))
                    .and_then(|n| n.text())
                    .unwrap_or("");
                let dispatchable = inner_text == "VK_DEFINE_HANDLE";
                TypeKind::Handle {
                    dispatchable,
                    parent: node.attribute("parent").map(str::to_string),
                }
            }

            "bitmask" => {
                let bits_enum = node
                    .attribute("requires")
                    .or_else(|| node.attribute("bitvalues"))
                    .map(str::to_string);

                let repr = node
                    .children()
                    .find(|n| n.has_tag_name("type"))
                    .and_then(|n| n.text())
                    .map(|t| rust_name(t.trim()))
                    .unwrap_or_else(|| "Flags".to_string());

                TypeKind::Bitmask { bits_enum, repr }
            }

            "struct" => {
                let members =
                    parse_members_dedup(node.children().filter(|n| n.has_tag_name("member")));

                if members.iter().any(|m| &m.name == "sType") {
                    reg.stypes
                        .insert(name.to_lowercase(), (name.clone(), false));
                }

                TypeKind::Struct {
                    lifetime: members.iter().any(|m| m.has_lifetime()),
                    members,
                    extends: node.attribute("structextends").map(str::to_string),
                    returned_only: node.attribute("returnedonly") == Some("true"),
                }
            }

            "union" => {
                let members =
                    parse_members_dedup(node.children().filter(|n| n.has_tag_name("member")));

                TypeKind::Union {
                    lifetime: false,
                    members,
                }
            }

            "funcpointer" => {
                let return_type = node
                    .children()
                    .find(|n| n.has_tag_name("proto"))
                    .and_then(|proto| proto.children().find(|n| n.has_tag_name("type")))
                    .and_then(|n| n.text())
                    .unwrap_or("c_void")
                    .to_string();

                let mut other_params = IndexSet::new();

                let params = node
                    .children()
                    .filter(|n| n.has_tag_name("param"))
                    .filter_map(|p| parse_param(p, &mut other_params))
                    .collect();

                TypeKind::FuncPointer {
                    return_type,
                    params,
                }
            }

            _ => TypeKind::Requires {
                requires: node.attribute("requires").map(str::to_string),
            },
        };

        reg.types.insert(
            name.clone(),
            VkType {
                name,
                alias: None,
                kind,
                comment: node.attribute("comment").map(str::to_string),
            },
        );
    }

    reg.types
        .iter()
        .filter_map(|(name, ty)| match &ty.kind {
            TypeKind::Struct { lifetime: true, .. } => Some(name.clone()),
            TypeKind::Union { lifetime: true, .. } => Some(name.clone()),
            TypeKind::Alias { lifetime: true, .. } => Some(name.clone()),
            _ => None,
        })
        .for_each(|g| {
            lifetimes.insert(rust_name(&g));
        });

    // Resolve Lifetimes
    loop {
        let mut changed = false;

        for ty in reg.types.values_mut() {
            match &mut ty.kind {
                TypeKind::Struct {
                    members, lifetime, ..
                } => {
                    if *lifetime {
                        continue;
                    }

                    let needs = members
                        .iter()
                        .any(|m| lifetimes.contains(&rust_name(&m.ty)));

                    if needs {
                        lifetimes.insert(rust_name(&ty.name));
                        *lifetime = true;
                        changed = true;
                    }
                }
                TypeKind::Union {
                    members, lifetime, ..
                } => {
                    if *lifetime {
                        continue;
                    }

                    let needs = members
                        .iter()
                        .any(|m| lifetimes.contains(&rust_name(&m.ty)));

                    if needs {
                        lifetimes.insert(rust_name(&ty.name));

                        *lifetime = true;
                        changed = true;
                    }
                }
                TypeKind::Alias { lifetime, alias } => {
                    if *lifetime {
                        continue;
                    }

                    let needs = lifetimes.contains(&rust_name(alias.as_str()));

                    if needs {
                        lifetimes.insert(rust_name(&ty.name));

                        *lifetime = true;
                        changed = true;
                    }
                }
                _ => {}
            }
        }

        if !changed {
            break;
        }
    }

    // Resolve next chains
    for ty in reg.types.values() {
        if let TypeKind::Struct {
            extends, members, ..
        } = &ty.kind
            && let Some(extends) = extends
        {
            let mutable = !&members
                .iter()
                .find(|m| m.name.as_str() == "pNext")
                .unwrap()
                .is_const;

            for extend in extends.split(",") {
                reg.next_chains.insert(extend.to_string(), mutable);
            }
        }
    }
}

fn parse_members_dedup<'a>(nodes: impl Iterator<Item = Node<'a, 'a>>) -> Vec<Member> {
    let mut seen = IndexSet::new();

    for node in nodes {
        let member = Member::parse(node);
        seen.insert(member);
    }

    seen.into_iter().collect()
}

pub fn parse_member(node: Node) -> Member {
    let mut full_ty = String::new();
    let mut ty = String::new();
    let mut name = String::new();
    let mut array_dims = parse_array_dimensions(node);
    let mut past_name = false;
    let mut comment = String::new();
    let mut bits = None;

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
            "comment" => {
                comment = child.text().unwrap_or("").trim().to_string();
            }
            "" => {
                if let Some(text) = child.text() {
                    if !past_name {
                        full_ty.push_str(text);
                    } else if let Some(width) = text.trim().strip_prefix(':') {
                        bits = width.trim().parse().ok();
                    }
                }
            }
            _ => {}
        }
    }

    let pointer_depth = full_ty.chars().filter(|&c| c == '*').count() as u32;
    Member {
        optional: node
            .attribute("optional")
            .map(|v| v.contains("true"))
            .unwrap_or(false),
        len: node.attribute("len").map(str::to_string),
        is_const: full_ty.contains("const"),
        pointer_depth,
        full_ty: full_ty.trim().to_string(),
        ty,
        name,
        array_dims,
        comment,
        bits,
    }
}

fn parse_funcpointer_params(node: &Node) -> Vec<(String, String)> {
    let mut params = Vec::new();
    let mut types = node.children().filter(|n| n.has_tag_name("type"));
    let mut names = node.children().filter(|n| n.has_tag_name("name"));

    names.next();

    types.next();
    for ty_node in types {
        let ty = ty_node.text().unwrap_or("").to_string();
        let name = names
            .next()
            .and_then(|n| n.text())
            .unwrap_or("")
            .to_string();
        if !ty.is_empty() {
            params.push((ty, name));
        }
    }
    params
}

fn try_parse_define(node: &Node, name: &str) -> Option<VkConstant> {
    let text = node.text().unwrap_or("");

    let after = text.split(name).nth(1)?.trim();
    let value_str = after.split_whitespace().next()?;

    let value: u64 = value_str.trim().parse().ok()?;

    Some(VkConstant {
        name: name.to_string(),
        ty: "u32".to_string(),
        value: value.to_string(),
        comment: node.attribute("comment").map(str::to_string),
    })
}

// 1. Neue Hilfsfunktion hinzufügen (am besten neben parse_member_node)
fn parse_array_dimensions(member_node: Node) -> Vec<String> {
    let mut dims = Vec::new();

    let mut after_name = false;
    for child in member_node.children() {
        if !after_name {
            if child.is_element() && child.tag_name().name() == "name" {
                after_name = true;
            }
            continue;
        }

        // Text-Node with [3][4] or [6] or "["
        if child.is_text() {
            let text = child.text().unwrap_or_default();
            let mut chars = text.chars().peekable();

            while let Some(c) = chars.next() {
                if c == '[' {
                    let mut expr = String::new();
                    while let Some(&next) = chars.peek() {
                        if next == ']' {
                            break;
                        }
                        expr.push(chars.next().unwrap());
                    }
                    if chars.next() == Some(']') {
                        let expr = expr.trim();
                        if !expr.is_empty() {
                            dims.push(expr.to_string());
                        }
                    }
                }
            }
        }
        // <enum>VK_UUID_SIZE</enum> directly in array (z. B. [ <enum>...</enum> ])
        else if child.is_element()
            && child.tag_name().name() == "enum"
            && let Some(enum_text) = child.text()
        {
            let trimmed = enum_text.trim();
            if !trimmed.is_empty() {
                dims.push(trimmed.to_string());
            }
        }
    }

    dims
}
