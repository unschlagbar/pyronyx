// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_primitive_restart_index";
pub const SPEC_VERSION: u32 = 1;

pub trait PrimitiveRestartIndexCommandBuffer {
    fn set_primitive_restart_index(&self, primitive_restart_index: u32);
}

impl PrimitiveRestartIndexCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartIndexEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_primitive_restart_index(&self, primitive_restart_index: u32) {
        let call = self
            .fns()
            .ext_primitive_restart_index
            .set_primitive_restart_index_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, primitive_restart_index) };
    }
}
