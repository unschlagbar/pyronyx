// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_clip_space_w_scaling";
pub const SPEC_VERSION: u32 = 1;

pub trait ClipSpaceWScalingCommandBuffer {
    fn set_viewport_w_scaling(
        &self,
        first_viewport: u32,
        viewport_w_scalings: &[ViewportWScalingNV],
    );
}

impl ClipSpaceWScalingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_viewport_w_scaling(
        &self,
        first_viewport: u32,
        viewport_w_scalings: &[ViewportWScalingNV],
    ) {
        unsafe {
            (self
                .fns()
                .nv_clip_space_w_scaling
                .as_ref()
                .unwrap()
                .set_viewport_w_scaling_nv)(
                self.handle,
                first_viewport,
                viewport_w_scalings.len() as u32,
                viewport_w_scalings.as_ptr(),
            )
        };
    }
}
