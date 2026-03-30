// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_calibrated_timestamps";
pub const SPEC_VERSION: u32 = 1;

pub trait CalibratedTimestampsPhysicalDevice {
    fn get_calibrateable_time_domains(
        &self,
        time_domains: &mut [TimeDomainKHR],
    ) -> Result<(), Error>;
}

impl CalibratedTimestampsPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCalibrateableTimeDomainsKHR.html>
    #[inline]
    fn get_calibrateable_time_domains(
        &self,
        time_domains: &mut [TimeDomainKHR],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_calibrated_timestamps
                .as_ref()
                .unwrap()
                .get_physical_device_calibrateable_time_domains_khr)(
                self.handle,
                time_domains.len() as *mut u32,
                time_domains.as_mut_ptr(),
            )
        }
        .result()
    }
}

pub trait CalibratedTimestampsDevice {
    fn get_calibrated_timestamps(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [u64],
    ) -> Result<u64, Error>;
}

impl CalibratedTimestampsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCalibratedTimestampsKHR.html>
    #[inline]
    fn get_calibrated_timestamps(
        &self,
        timestamp_infos: &[CalibratedTimestampInfoKHR],
        timestamps: &mut [u64],
    ) -> Result<u64, Error> {
        assert_eq!(timestamp_infos.len(), timestamps.len());
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_calibrated_timestamps
                .as_ref()
                .unwrap()
                .get_calibrated_timestamps_khr)(
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
