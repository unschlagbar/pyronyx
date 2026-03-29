// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_HUAWEI_invocation_mask";
pub const SPEC_VERSION: u32 = 1;

pub trait InvocationMaskCommandBuffer {
    fn bind_invocation_mask(&self, image_view: ImageView, image_layout: ImageLayout);
}

impl InvocationMaskCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindInvocationMaskHUAWEI.html>
    #[inline]
    fn bind_invocation_mask(&self, image_view: ImageView, image_layout: ImageLayout) {
        unsafe {
            (self
                .fns()
                .huawei_invocation_mask
                .as_ref()
                .unwrap()
                .bind_invocation_mask_huawei)(self.handle, image_view, image_layout)
        };
    }
}
