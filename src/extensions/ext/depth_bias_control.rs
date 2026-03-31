// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_depth_bias_control";
pub const SPEC_VERSION: u32 = 1;

pub trait DepthBiasControlCommandBuffer {
    fn set_depth_bias2(&self, depth_bias_info: &DepthBiasInfoEXT);
}

impl DepthBiasControlCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html>
    #[inline]
    fn set_depth_bias2(&self, depth_bias_info: &DepthBiasInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_depth_bias_control
                .as_ref()
                .unwrap()
                .set_depth_bias2_ext)(self.handle, depth_bias_info)
        };
    }
}
