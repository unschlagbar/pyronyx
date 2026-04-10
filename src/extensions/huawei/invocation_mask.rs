// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_HUAWEI_invocation_mask";
pub const SPEC_VERSION: u32 = 1;

pub trait InvocationMaskCommandBuffer {
    fn bind_invocation_mask(&self, image_view: ImageView, image_layout: ImageLayout);
}

impl InvocationMaskCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Vulkan state access`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_invocation_mask(&self, image_view: ImageView, image_layout: ImageLayout) {
        let call = self
            .fns()
            .huawei_invocation_mask
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .bind_invocation_mask_huawei;

        unsafe { (call)(self.handle, image_view, image_layout) };
    }
}
