#![allow(unused)]
use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use crate::{
    codegen::{impls::ImplTarget, rust_name},
    parse::{commands, enums, extensions, types},
};
use heck::ToPascalCase;
use indexmap::IndexMap;

#[derive(Debug, Clone)]
pub struct VkType {
    pub name: String,
    pub alias: Option<String>,
    pub kind: types::TypeKind,
    pub comment: Option<String>,
}

#[derive(Debug, Clone)]
pub struct VkEnum {
    pub bitwidth_64: bool,
    pub name: String,
    pub is_bitmask: bool,
    pub values: Vec<VkEnumValue>,
}

#[derive(Debug, Clone)]
pub struct VkEnumValue {
    pub name: String,
    pub value: i64,
    pub comment: Option<String>,
    pub extension: Option<String>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub struct VkCommand {
    pub name: String,
    pub return_type: String,
    pub target: Option<ImplTarget>,
    pub params: Vec<commands::Param>,
    pub alias: Option<String>,
    pub success_codes: Vec<String>,
    pub error_codes: Vec<String>,
    pub version: Option<String>,
    pub extension: Option<String>,

    pub conditional_rendering: bool,
    pub renderpass: RenderPass,
    pub queues: Vec<String>,
    pub cmd_buffer_level: String,
    pub tasks: Vec<Task>,
}

#[derive(Debug, Clone, Hash, PartialEq, Eq, Default)]
pub enum RenderPass {
    Both,
    Inside,
    Outside,
    #[default]
    None,
}

impl RenderPass {
    pub fn from_str(str: &str) -> Self {
        match str {
            "both" => Self::Both,
            "inside" => Self::Inside,
            "outside" => Self::Outside,
            "" => Self::None,
            other => panic!("Not supported Renderpass {}", other),
        }
    }

    pub fn print(&self) -> &'static str {
        match self {
            RenderPass::Both => "/// Use inside and outside `RenderPass`",
            RenderPass::Inside => "/// Use inside `RenderPass`",
            RenderPass::Outside => "/// Use outside `RenderPass`",
            RenderPass::None => "",
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
pub enum Task {
    State,
    Action,
    Synchronization,
    Indirection,
}

impl Task {
    pub fn from_str(str: &str) -> Self {
        match str {
            "state" => Self::State,
            "action" => Self::Action,
            "indirection" => Self::Indirection,
            "synchronization" => Self::Synchronization,
            other => panic!("Not supported Task {}", other),
        }
    }

    pub fn print(&self) -> &'static str {
        match self {
            Task::State => "Vulkan state access",
            Task::Action => "Executes GPU work",
            Task::Synchronization => "Synchronization",
            Task::Indirection => "Executes indirect GPU work",
        }
    }
}

impl VkCommand {
    pub fn version(&self) -> &str {
        self.version.as_deref().unwrap_or("Ext")
    }

    pub fn table_name(&self) -> Depends {
        if let Some(version) = &self.version {
            Depends::Core(version.to_string())
        } else if let Some(ext) = &self.extension {
            Depends::Ext(ext.strip_prefix("VK_").unwrap_or(ext).to_pascal_case())
        } else {
            Depends::Undefined
        }
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord)]
pub enum Depends {
    Core(String),
    Ext(String),
    Undefined,
}

impl fmt::Display for Depends {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Depends::Core(v) | Depends::Ext(v) => f.write_str(v),
            Depends::Undefined => f.write_str("U"),
        }
    }
}

#[derive(Debug, Clone)]
pub struct VkConstant {
    pub name: String,
    pub ty: String,
    pub value: String,
    pub comment: Option<String>,
}

#[derive(Debug, Default)]
pub struct Feature {
    pub name: String,
    pub version: Option<String>,
    pub required_types: Vec<String>,
    pub required_commands: Vec<String>,
}

#[derive(Debug, Default)]
pub struct Extension {
    pub name: String,
    pub number: i64,
    pub spec_version: u32,
    pub disabled: bool,
    pub deprecated_by: Option<String>,
    /// `VK_VERSION_x_y` or the extension this one was promoted to.
    pub promoted_to: Option<String>,
    /// Dependency expression: `+` is AND, `,` is OR, parentheses group.
    pub depends: Option<String>,
    pub requires: Vec<String>,
    pub require_blocks: Vec<RequireBlock>,
    /// Device | Instance
    pub typ: String,
}

#[derive(Debug, Default)]
pub struct RequireBlock {
    pub feature_guard: Option<String>,
    pub types: Vec<String>,
    pub commands: Vec<String>,
}

// ── Registry ──────────────────────────────────────────────────────────────────

#[derive(Default)]
pub struct Registry {
    pub types: IndexMap<String, VkType>,
    pub stypes: IndexMap<String, (String, bool)>,
    pub enums: IndexMap<String, VkEnum>,
    pub commands: IndexMap<String, VkCommand>,
    pub constants: Vec<VkConstant>,
    pub features: Vec<Feature>,
    pub extensions: Vec<Extension>,
    pub next_chains: HashMap<String, bool>,
}

impl Registry {
    pub fn parse_video(xml: &str, lifetimes: &mut HashSet<String>) -> Self {
        let doc = roxmltree::Document::parse(xml).expect("video.xml parse error");
        let root = doc.root_element();
        let mut reg = Registry::default();

        for child in root.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "types" => types::parse_into(&child, &mut reg, lifetimes),
                "enums" => enums::parse_into(&child, &mut reg),
                "commands" => commands::parse_into(&child, &mut reg),
                "extensions" => extensions::parse_video_extensions(child, &mut reg),
                _ => (),
            }
        }

        for child in root.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "feature" => extensions::parse_feature(child, &mut reg),
                "extensions" => {
                    for ext in child.children().filter(|n| n.has_tag_name("extension")) {
                        extensions::parse_extension(ext, &mut reg);
                    }
                }
                _ => {}
            }
        }

        reg.resolve_command_aliases();
        reg
    }

    pub fn parse(xml: &str, lifetimes: &mut HashSet<String>) -> Self {
        let doc = roxmltree::Document::parse(xml).expect("vk.xml parse error");
        let root = doc.root_element();
        let mut reg = Registry::default();

        for child in root.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "types" => types::parse_into(&child, &mut reg, lifetimes),
                "enums" => enums::parse_into(&child, &mut reg),
                "commands" => commands::parse_into(&child, &mut reg),
                _ => (),
            }
        }

        for child in root.children().filter(|n| n.is_element()) {
            match child.tag_name().name() {
                "feature" => extensions::parse_feature(child, &mut reg),
                "extensions" => {
                    for ext in child.children().filter(|n| n.has_tag_name("extension")) {
                        extensions::parse_extension(ext, &mut reg);
                    }
                }
                _ => {}
            }
        }

        reg.resolve_command_aliases();
        reg.remove_disabled_extensions();
        reg
    }

    fn remove_disabled_extensions(&mut self) {
        let disabled: HashSet<String> = self
            .extensions
            .iter()
            .filter(|e| e.disabled)
            .map(|e| e.name.clone())
            .collect();

        if disabled.is_empty() {
            return;
        }

        // Collect everything claimed by enabled extensions and core features so
        // we don't accidentally remove shared types/commands.
        let mut enabled_types: HashSet<&str> = HashSet::new();
        let mut enabled_commands: HashSet<&str> = HashSet::new();

        for ext in &self.extensions {
            if !ext.disabled {
                for block in &ext.require_blocks {
                    enabled_types.extend(block.types.iter().map(String::as_str));
                    enabled_commands.extend(block.commands.iter().map(String::as_str));
                }
            }
        }
        for feature in &self.features {
            enabled_types.extend(feature.required_types.iter().map(String::as_str));
            enabled_commands.extend(feature.required_commands.iter().map(String::as_str));
        }

        // Remove types and commands that only disabled extensions require.
        for ext in &self.extensions {
            if !ext.disabled {
                continue;
            }
            for block in &ext.require_blocks {
                for ty in &block.types {
                    if !enabled_types.contains(ty.as_str()) {
                        self.types.shift_remove(ty);
                    }
                }
                for cmd in &block.commands {
                    if !enabled_commands.contains(cmd.as_str()) {
                        self.commands.shift_remove(cmd);
                    }
                }
            }
        }

        // Remove enum values that were injected by disabled extensions.
        for en in self.enums.values_mut() {
            en.values.retain(|v| {
                v.extension
                    .as_ref()
                    .map_or(true, |ext| !disabled.contains(ext))
            });
        }
    }

    fn resolve_command_aliases(&mut self) {
        let aliases: Vec<(String, String)> = self
            .commands
            .iter()
            .filter_map(|(name, cmd)| cmd.alias.as_ref().map(|a| (name.clone(), a.clone())))
            .collect();

        for (alias_name, original_name) in aliases {
            if let Some(original) = self.commands.get(&original_name).cloned() {
                self.commands.insert(
                    alias_name.clone(),
                    VkCommand {
                        name: alias_name,
                        alias: Some(original_name),
                        ..original
                    },
                );
            }
        }
    }
}
