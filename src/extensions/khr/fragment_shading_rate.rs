// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_KHR_fragment_shading_rate";
pub const SPEC_VERSION: u32 = 2;

pub trait FragmentShadingRateCommandBuffer {
    fn set_fragment_shading_rate(
        &self,
        fragment_size: &Extent2D,
        combiner_ops: FragmentShadingRateCombinerOpKHR,
    );
}

impl FragmentShadingRateCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFragmentShadingRateKHR.html>
    #[inline]
    fn set_fragment_shading_rate(
        &self,
        fragment_size: &Extent2D,
        combiner_ops: FragmentShadingRateCombinerOpKHR,
    ) {
        unsafe {
            (self
                .fns()
                .khr_fragment_shading_rate
                .as_ref()
                .unwrap()
                .set_fragment_shading_rate_khr)(self.handle, fragment_size, combiner_ops)
        };
    }
}

pub trait FragmentShadingRatePhysicalDevice {
    fn get_fragment_shading_rates(
        &self,
        fragment_shading_rates: &mut [PhysicalDeviceFragmentShadingRateKHR],
    ) -> Result<(), vkResult>;
}

impl FragmentShadingRatePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    #[inline]
    fn get_fragment_shading_rates(
        &self,
        fragment_shading_rates: &mut [PhysicalDeviceFragmentShadingRateKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_fragment_shading_rate
                .as_ref()
                .unwrap()
                .get_physical_device_fragment_shading_rates_khr)(
                self.handle,
                fragment_shading_rates.len() as *mut u32,
                fragment_shading_rates.as_mut_ptr(),
            )
        }
        .result()
    }
}
