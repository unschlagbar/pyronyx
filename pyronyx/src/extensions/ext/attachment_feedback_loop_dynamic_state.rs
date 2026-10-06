// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_EXT_attachment_feedback_loop_layout`](crate::ext::attachment_feedback_loop_layout)
pub const NAME: &CStr = c"VK_EXT_attachment_feedback_loop_dynamic_state";
pub const SPEC_VERSION: u32 = 1;

pub trait AttachmentFeedbackLoopDynamicStateCommandBuffer {
    fn set_attachment_feedback_loop_enable(&self, aspect_mask: ImageAspectFlags);
}

impl AttachmentFeedbackLoopDynamicStateCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAttachmentFeedbackLoopEnableEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_attachment_feedback_loop_enable(&self, aspect_mask: ImageAspectFlags) {
        let call = self
            .fns()
            .ext_attachment_feedback_loop_dynamic_state
            .set_attachment_feedback_loop_enable_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, aspect_mask) };
    }
}
