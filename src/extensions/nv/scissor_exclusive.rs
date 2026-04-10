// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_scissor_exclusive";
pub const SPEC_VERSION: u32 = 2;

pub trait ScissorExclusiveCommandBuffer {
    fn set_exclusive_scissor(&self, first_exclusive_scissor: u32, exclusive_scissors: &[Rect2D]);

    fn set_exclusive_scissor_enable(
        &self,
        first_exclusive_scissor: u32,
        exclusive_scissor_enables: &[Bool32],
    );
}

impl ScissorExclusiveCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_exclusive_scissor(&self, first_exclusive_scissor: u32, exclusive_scissors: &[Rect2D]) {
        let call = self
            .fns()
            .nv_scissor_exclusive
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_exclusive_scissor_nv;

        unsafe {
            (call)(
                self.handle,
                first_exclusive_scissor,
                exclusive_scissors.len() as u32,
                exclusive_scissors.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExclusiveScissorEnableNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_exclusive_scissor_enable(
        &self,
        first_exclusive_scissor: u32,
        exclusive_scissor_enables: &[Bool32],
    ) {
        let call = self
            .fns()
            .nv_scissor_exclusive
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_exclusive_scissor_enable_nv;

        unsafe {
            (call)(
                self.handle,
                first_exclusive_scissor,
                exclusive_scissor_enables.len() as u32,
                exclusive_scissor_enables.as_ptr(),
            )
        };
    }
}
