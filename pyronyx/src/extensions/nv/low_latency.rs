// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_NV_low_latency2` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_low_latency";
pub const SPEC_VERSION: u32 = 2;

pub trait LowLatencyDevice {
    fn set_latency_sleep_mode_legacy(
        &self,
        low_latency_mode: bool,
        low_latency_boost: bool,
        minimum_interval_us: u32,
    );

    fn latency_sleep_legacy(&self, signal_semaphore: Semaphore, value: u64);

    fn set_latency_marker_legacy(&self, frame_id: u64, marker: u32);

    fn get_latency_timings_legacy(&self, timings: *mut c_void);

    fn get_sleep_status_legacy(&self) -> bool;

    fn shutdown_latency_legacy(&self);
}

impl LowLatencyDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencySleepModeLegacyNV.html>
    #[inline]
    fn set_latency_sleep_mode_legacy(
        &self,
        low_latency_mode: bool,
        low_latency_boost: bool,
        minimum_interval_us: u32,
    ) {
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_latency_sleep_mode_legacy_nv;

        unsafe {
            (call)(
                self.handle,
                low_latency_mode as _,
                low_latency_boost as _,
                minimum_interval_us,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkLatencySleepLegacyNV.html>
    #[inline]
    fn latency_sleep_legacy(&self, signal_semaphore: Semaphore, value: u64) {
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .latency_sleep_legacy_nv;

        unsafe { (call)(self.handle, signal_semaphore, value) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLatencyMarkerLegacyNV.html>
    #[inline]
    fn set_latency_marker_legacy(&self, frame_id: u64, marker: u32) {
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_latency_marker_legacy_nv;

        unsafe { (call)(self.handle, frame_id, marker) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetLatencyTimingsLegacyNV.html>
    #[inline]
    fn get_latency_timings_legacy(&self, timings: *mut c_void) {
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_latency_timings_legacy_nv;

        unsafe { (call)(self.handle, timings) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSleepStatusLegacyNV.html>
    #[inline]
    fn get_sleep_status_legacy(&self) -> bool {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_sleep_status_legacy_nv;

        unsafe {
            (call)(self.handle, out.as_mut_ptr());
            out.assume_init() != 0
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkShutdownLatencyDeviceLegacyNV.html>
    #[inline]
    fn shutdown_latency_legacy(&self) {
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .shutdown_latency_device_legacy_nv;

        unsafe { (call)(self.handle) };
    }
}

pub trait LowLatencyQueue {
    fn notify_out_of_band_legacy(&self, queue_type: u32);
}

impl LowLatencyQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueNotifyOutOfBandLegacyNV.html>
    #[inline]
    fn notify_out_of_band_legacy(&self, queue_type: u32) {
        let call = self
            .fns()
            .nv_low_latency
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .queue_notify_out_of_band_legacy_nv;

        unsafe { (call)(self.handle, queue_type) };
    }
}
