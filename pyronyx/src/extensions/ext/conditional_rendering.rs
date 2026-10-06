// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_conditional_rendering";
pub const SPEC_VERSION: u32 = 2;

pub trait ConditionalRenderingCommandBuffer {
    fn begin_conditional_rendering(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    );

    fn end_conditional_rendering(&self);
}

impl ConditionalRenderingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_conditional_rendering(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) {
        let call = self
            .fns()
            .ext_conditional_rendering
            .begin_conditional_rendering_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, conditional_rendering_begin) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_conditional_rendering(&self) {
        let call = self
            .fns()
            .ext_conditional_rendering
            .end_conditional_rendering_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle) };
    }
}
