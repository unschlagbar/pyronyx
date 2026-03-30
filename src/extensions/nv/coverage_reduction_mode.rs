// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_NV_coverage_reduction_mode";
pub const SPEC_VERSION: u32 = 1;

pub trait CoverageReductionModePhysicalDevice {
    fn get_supported_framebuffer_mixed_samples_combinations(
        &self,
        combinations: &mut [FramebufferMixedSamplesCombinationNV],
    ) -> Result<(), Error>;
}

impl CoverageReductionModePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    #[inline]
    fn get_supported_framebuffer_mixed_samples_combinations(
        &self,
        combinations: &mut [FramebufferMixedSamplesCombinationNV],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_coverage_reduction_mode
                .as_ref()
                .unwrap()
                .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
                self.handle,
                combinations.len() as *mut u32,
                combinations.as_mut_ptr(),
            )
        }
        .result()
    }
}
