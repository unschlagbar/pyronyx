// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_present_timing";
pub const SPEC_VERSION: u32 = 3;

pub trait PresentTimingDevice {
    fn set_swapchain_present_timing_queue_size(
        &self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> Result<()>;

    fn get_swapchain_timing_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    ) -> Result<u64>;

    fn get_swapchain_time_domain_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    ) -> Result<u64>;

    fn get_past_presentation_timing(
        &self,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT,
    ) -> Result<PastPresentationTimingPropertiesEXT<'_>>;
}

impl PresentTimingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    #[inline]
    fn set_swapchain_present_timing_queue_size(
        &self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_present_timing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_swapchain_present_timing_queue_size_ext;

        unsafe { (call)(self.handle, swapchain, size) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html>
    #[inline]
    fn get_swapchain_timing_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    ) -> Result<u64> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_present_timing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_swapchain_timing_properties_ext;

        unsafe {
            (call)(
                self.handle,
                swapchain,
                swapchain_timing_properties,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimeDomainPropertiesEXT.html>
    #[inline]
    fn get_swapchain_time_domain_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    ) -> Result<u64> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_present_timing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_swapchain_time_domain_properties_ext;

        unsafe {
            (call)(
                self.handle,
                swapchain,
                swapchain_time_domain_properties,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingEXT.html>
    #[inline]
    fn get_past_presentation_timing(
        &self,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT,
    ) -> Result<PastPresentationTimingPropertiesEXT<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_present_timing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_past_presentation_timing_ext;

        unsafe { (call)(self.handle, past_presentation_timing_info, out.as_mut_ptr()) }
            .init_on_success(out)
    }
}
