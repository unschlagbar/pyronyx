// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_attachment_feedback_loop_enable_ext;

        unsafe { (call)(self.handle, aspect_mask) };
    }
}
