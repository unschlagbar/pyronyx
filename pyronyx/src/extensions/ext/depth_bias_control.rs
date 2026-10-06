// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_depth_bias_control";
pub const SPEC_VERSION: u32 = 1;

pub trait DepthBiasControlCommandBuffer {
    fn set_depth_bias2(&self, depth_bias_info: &DepthBiasInfoEXT);
}

impl DepthBiasControlCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias2EXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_depth_bias2(&self, depth_bias_info: &DepthBiasInfoEXT) {
        let call = self
            .fns()
            .ext_depth_bias_control
            .set_depth_bias2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, depth_bias_info) };
    }
}
