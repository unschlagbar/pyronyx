// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_KHR_calibrated_timestamps";
pub const SPEC_VERSION: u32 = 1;

pub trait CalibratedTimestampsPhysicalDevice {
    fn get_calibrateable_time_domains(&self, time_domains: &mut [TimeDomainKHR]) -> Result<()>;
    fn get_calibrateable_time_domains_len(&self) -> Result<usize>;
}

impl CalibratedTimestampsPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>
    ///
    /// Call [`get_calibrateable_time_domains_len()`][`Self::get_calibrateable_time_domains_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_calibrateable_time_domains(&self, time_domains: &mut [TimeDomainKHR]) -> Result<()> {
        let mut time_domain_count = time_domains.len() as u32;
        let call = self
            .fns()
            .khr_calibrated_timestamps
            .get_physical_device_calibrateable_time_domains_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                &mut time_domain_count,
                time_domains.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_calibrateable_time_domains`][`Self::get_calibrateable_time_domains`].
    #[inline]
    fn get_calibrateable_time_domains_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_calibrated_timestamps
                .get_physical_device_calibrateable_time_domains_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}

pub trait CalibratedTimestampsDevice {
    fn get_calibrated_timestamps(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [u64],
    ) -> Result<u64>;
}

impl CalibratedTimestampsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html>
    #[inline]
    fn get_calibrated_timestamps(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [u64],
    ) -> Result<u64> {
        assert_eq!(timestamp_infos.len(), timestamps.len());
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_calibrated_timestamps
            .get_calibrated_timestamps_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                timestamp_infos.len() as u32,
                timestamp_infos.as_ptr(),
                timestamps.as_mut_ptr(),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
