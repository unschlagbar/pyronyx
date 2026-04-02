// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_coverage_reduction_mode";
pub const SPEC_VERSION: u32 = 1;

pub trait CoverageReductionModePhysicalDevice {
    fn get_supported_framebuffer_mixed_samples_combinations(
        &self,
        combinations: &mut [FramebufferMixedSamplesCombinationNV],
    ) -> Result<(), Error>;
    fn get_supported_framebuffer_mixed_samples_combinations_len(&self) -> Result<usize, Error>;
}

impl CoverageReductionModePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV.html>
    ///
    /// Call [`get_supported_framebuffer_mixed_samples_combinations_len()`][`Self::get_supported_framebuffer_mixed_samples_combinations_len()`] to query the number of elements to pass to `out`.
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

    /// Returns the required slice length for Call [`get_supported_framebuffer_mixed_samples_combinations`][`Self::get_supported_framebuffer_mixed_samples_combinations`].
    #[inline]
    fn get_supported_framebuffer_mixed_samples_combinations_len(&self) -> Result<usize, Error> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_coverage_reduction_mode
                .as_ref()
                .unwrap()
                .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv)(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|o| o as usize)
    }
}
