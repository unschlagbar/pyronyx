// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_EXT_present_timing";
pub const SPEC_VERSION: u32 = 3;

pub trait PresentTimingDevice {
    fn set_swapchain_present_timing_queue_size(
        &self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> Result<(), vkResult>;

    fn get_swapchain_timing_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    ) -> Result<u64, vkResult>;

    fn get_swapchain_time_domain_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    ) -> Result<u64, vkResult>;

    fn get_past_presentation_timing(
        &self,
        past_presentation_timing_info: &PastPresentationTimingInfoEXT,
    ) -> Result<PastPresentationTimingPropertiesEXT<'_>, vkResult>;
}

impl PresentTimingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetSwapchainPresentTimingQueueSizeEXT.html>
    #[inline]
    fn set_swapchain_present_timing_queue_size(
        &self,
        swapchain: SwapchainKHR,
        size: u32,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .ext_present_timing
                .as_ref()
                .unwrap()
                .set_swapchain_present_timing_queue_size_ext)(
                self.handle, swapchain, size
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainTimingPropertiesEXT.html>
    #[inline]
    fn get_swapchain_timing_properties(
        &self,
        swapchain: SwapchainKHR,
        swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    ) -> Result<u64, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_present_timing
                .as_ref()
                .unwrap()
                .get_swapchain_timing_properties_ext)(
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
    ) -> Result<u64, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_present_timing
                .as_ref()
                .unwrap()
                .get_swapchain_time_domain_properties_ext)(
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
    ) -> Result<PastPresentationTimingPropertiesEXT<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_present_timing
                .as_ref()
                .unwrap()
                .get_past_presentation_timing_ext)(
                self.handle,
                past_presentation_timing_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
