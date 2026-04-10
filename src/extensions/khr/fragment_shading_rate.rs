// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
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
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_fragment_shading_rate(
        &self,
        fragment_size: &Extent2D,
        combiner_ops: FragmentShadingRateCombinerOpKHR,
    ) {
        let call = self
            .fns()
            .khr_fragment_shading_rate
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_fragment_shading_rate_khr;

        unsafe { (call)(self.handle, fragment_size, combiner_ops) };
    }
}

pub trait FragmentShadingRatePhysicalDevice {
    fn get_fragment_shading_rates(
        &self,
        fragment_shading_rates: &mut [PhysicalDeviceFragmentShadingRateKHR],
    ) -> Result<(), Error>;
}

impl FragmentShadingRatePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFragmentShadingRatesKHR.html>
    #[inline]
    fn get_fragment_shading_rates(
        &self,
        fragment_shading_rates: &mut [PhysicalDeviceFragmentShadingRateKHR],
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .khr_fragment_shading_rate
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_fragment_shading_rates_khr;

        unsafe {
            (call)(
                self.handle,
                fragment_shading_rates.len() as *mut u32,
                fragment_shading_rates.as_mut_ptr(),
            )
        }
        .result()
    }
}
