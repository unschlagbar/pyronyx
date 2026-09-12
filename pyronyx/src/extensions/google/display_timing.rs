// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_GOOGLE_display_timing";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplayTimingDevice {
    fn get_refresh_cycle_duration(
        &self,
        swapchain: SwapchainKHR,
    ) -> Result<RefreshCycleDurationGOOGLE>;

    fn get_past_presentation_timing(
        &self,
        swapchain: SwapchainKHR,
        presentation_timings: &mut [PastPresentationTimingGOOGLE],
    ) -> Result<()>;
    fn get_past_presentation_timing_len(&self, swapchain: SwapchainKHR) -> Result<usize>;
}

impl DisplayTimingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRefreshCycleDurationGOOGLE.html>
    #[inline]
    fn get_refresh_cycle_duration(
        &self,
        swapchain: SwapchainKHR,
    ) -> Result<RefreshCycleDurationGOOGLE> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .google_display_timing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_refresh_cycle_duration_google;

        unsafe { (call)(self.handle, swapchain, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPastPresentationTimingGOOGLE.html>
    ///
    /// Call [`get_past_presentation_timing_len()`][`Self::get_past_presentation_timing_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_past_presentation_timing(
        &self,
        swapchain: SwapchainKHR,
        presentation_timings: &mut [PastPresentationTimingGOOGLE],
    ) -> Result<()> {
        let mut presentation_timing_count = presentation_timings.len() as u32;
        let call = self
            .fns()
            .google_display_timing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_past_presentation_timing_google;

        unsafe {
            (call)(
                self.handle,
                swapchain,
                &mut presentation_timing_count,
                presentation_timings.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_past_presentation_timing`][`Self::get_past_presentation_timing`].
    #[inline]
    fn get_past_presentation_timing_len(&self, swapchain: SwapchainKHR) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .google_display_timing
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_past_presentation_timing_google)(
                self.handle,
                swapchain,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
