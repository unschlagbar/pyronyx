// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_low_latency2";
pub const SPEC_VERSION: u32 = 2;

pub trait LowLatency2Device {
    fn set_latency_sleep_mode(
        &self,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> Result<(), Error>;

    fn latency_sleep(
        &self,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV,
    ) -> Result<(), Error>;

    fn set_latency_marker(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV,
    );

    fn get_latency_timings(&self, swapchain: SwapchainKHR) -> GetLatencyMarkerInfoNV<'_>;
}

impl LowLatency2Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeNV.html>
    #[inline]
    fn set_latency_sleep_mode(
        &self,
        swapchain: SwapchainKHR,
        sleep_mode_info: &LatencySleepModeInfoNV,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_low_latency2
                .as_ref()
                .unwrap()
                .set_latency_sleep_mode_nv)(self.handle, swapchain, sleep_mode_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepNV.html>
    #[inline]
    fn latency_sleep(
        &self,
        swapchain: SwapchainKHR,
        sleep_info: &LatencySleepInfoNV,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_low_latency2
                .as_ref()
                .unwrap()
                .latency_sleep_nv)(self.handle, swapchain, sleep_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerNV.html>
    #[inline]
    fn set_latency_marker(
        &self,
        swapchain: SwapchainKHR,
        latency_marker_info: &SetLatencyMarkerInfoNV,
    ) {
        unsafe {
            (self
                .fns()
                .nv_low_latency2
                .as_ref()
                .unwrap()
                .set_latency_marker_nv)(self.handle, swapchain, latency_marker_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsNV.html>
    #[inline]
    fn get_latency_timings(&self, swapchain: SwapchainKHR) -> GetLatencyMarkerInfoNV<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_low_latency2
                .as_ref()
                .unwrap()
                .get_latency_timings_nv)(self.handle, swapchain, out.as_mut_ptr());
            out.assume_init()
        }
    }
}

pub trait LowLatency2Queue {
    fn notify_out_of_band(&self, queue_type_info: &OutOfBandQueueTypeInfoNV);
}

impl LowLatency2Queue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandNV.html>
    #[inline]
    fn notify_out_of_band(&self, queue_type_info: &OutOfBandQueueTypeInfoNV) {
        unsafe {
            (self
                .fns()
                .nv_low_latency2
                .as_ref()
                .unwrap()
                .queue_notify_out_of_band_nv)(self.handle, queue_type_info)
        };
    }
}
