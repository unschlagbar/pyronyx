// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: (Vulkan 1.2 or [`VK_KHR_timeline_semaphore`](crate::khr::timeline_semaphore)) + ([`VK_KHR_present_id`](crate::khr::present_id) or [`VK_KHR_present_id2`](crate::khr::present_id2))
pub const NAME: &CStr = c"VK_NV_low_latency2";
pub const SPEC_VERSION: u32 = 2;

pub trait LowLatency2Device {
    fn set_latency_sleep_mode(
        &self,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> Result<()>;

    fn latency_sleep(&self, swapchain: SwapchainKHR, sleep_info: &LatencySleepInfoNV)
    -> Result<()>;

    fn set_latency_marker(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV,
    );

    fn get_latency_timings(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &mut GetLatencyMarkerInfoNV<'_>,
    );
}

impl LowLatency2Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html>
    #[inline]
    fn set_latency_sleep_mode(
        &self,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_low_latency2
            .set_latency_sleep_mode_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, sleep_mode_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html>
    #[inline]
    fn latency_sleep(
        &self,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_low_latency2
            .latency_sleep_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, sleep_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html>
    #[inline]
    fn set_latency_marker(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV,
    ) {
        let call = self
            .fns()
            .nv_low_latency2
            .set_latency_marker_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, latency_marker_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html>
    #[inline]
    fn get_latency_timings(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &mut GetLatencyMarkerInfoNV<'_>,
    ) {
        let call = self
            .fns()
            .nv_low_latency2
            .get_latency_timings_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, latency_marker_info) };
    }
}

pub trait LowLatency2Queue {
    fn notify_out_of_band(&self, queue_type_info: &OutOfBandQueueTypeInfoNV);
}

impl LowLatency2Queue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html>
    #[inline]
    fn notify_out_of_band(&self, queue_type_info: &OutOfBandQueueTypeInfoNV) {
        let call = self
            .fns()
            .nv_low_latency2
            .queue_notify_out_of_band_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, queue_type_info) };
    }
}
