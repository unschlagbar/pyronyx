// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_GOOGLE_display_timing";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplayTimingDevice {
    fn get_refresh_cycle_duration(
        &self,
        swapchain: SwapchainKHR,
    ) -> Result<RefreshCycleDurationGOOGLE, vkResult>;

    fn get_past_presentation_timing(
        &self,
        swapchain: SwapchainKHR,
        presentation_timings: &mut [PastPresentationTimingGOOGLE],
    ) -> Result<(), vkResult>;
}

impl DisplayTimingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html>
    #[inline]
    fn get_refresh_cycle_duration(
        &self,
        swapchain: SwapchainKHR,
    ) -> Result<RefreshCycleDurationGOOGLE, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .google_display_timing
                .as_ref()
                .unwrap()
                .get_refresh_cycle_duration_google)(
                self.handle, swapchain, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html>
    #[inline]
    fn get_past_presentation_timing(
        &self,
        swapchain: SwapchainKHR,
        presentation_timings: &mut [PastPresentationTimingGOOGLE],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .google_display_timing
                .as_ref()
                .unwrap()
                .get_past_presentation_timing_google)(
                self.handle,
                swapchain,
                presentation_timings.len() as *mut u32,
                presentation_timings.as_mut_ptr(),
            )
        }
        .result()
    }
}
