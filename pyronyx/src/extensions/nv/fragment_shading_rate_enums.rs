// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_fragment_shading_rate`](crate::khr::fragment_shading_rate)
pub const NAME: &CStr = c"VK_NV_fragment_shading_rate_enums";
pub const SPEC_VERSION: u32 = 1;

pub trait FragmentShadingRateEnumsCommandBuffer {
    fn set_fragment_shading_rate_enum(
        &self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: FragmentShadingRateCombinerOpKHR,
    );
}

impl FragmentShadingRateEnumsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateEnumNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_fragment_shading_rate_enum(
        &self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: FragmentShadingRateCombinerOpKHR,
    ) {
        let call = self
            .fns()
            .nv_fragment_shading_rate_enums
            .set_fragment_shading_rate_enum_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, shading_rate, combiner_ops) };
    }
}
