// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_custom_resolve";
pub const SPEC_VERSION: u32 = 1;

pub trait CustomResolveCommandBuffer {
    fn begin_custom_resolve(&self, begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT>);
}

impl CustomResolveCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginCustomResolveEXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_custom_resolve(&self, begin_custom_resolve_info: Option<&BeginCustomResolveInfoEXT>) {
        unsafe {
            (self
                .fns()
                .ext_custom_resolve
                .as_ref()
                .unwrap()
                .begin_custom_resolve_ext)(
                self.handle,
                begin_custom_resolve_info.map_or(null(), from_ref),
            )
        };
    }
}
