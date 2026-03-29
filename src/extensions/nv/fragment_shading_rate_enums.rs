// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

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
    #[inline]
    fn set_fragment_shading_rate_enum(
        &self,
        shading_rate: FragmentShadingRateNV,
        combiner_ops: FragmentShadingRateCombinerOpKHR,
    ) {
        unsafe {
            (self
                .fns()
                .nv_fragment_shading_rate_enums
                .as_ref()
                .unwrap()
                .set_fragment_shading_rate_enum_nv)(
                self.handle, shading_rate, combiner_ops
            )
        };
    }
}
