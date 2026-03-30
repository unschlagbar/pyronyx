// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_performance_query";
pub const SPEC_VERSION: u32 = 1;

pub trait PerformanceQueryPhysicalDevice {
    fn enumerate_queue_family_performance_query_counters(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterKHR],
        counter_descriptions: &mut [PerformanceCounterDescriptionKHR],
    ) -> Result<(), Error>;

    fn get_queue_family_performance_query_passes(
        &self,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32;
}

impl PerformanceQueryPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    #[inline]
    fn enumerate_queue_family_performance_query_counters(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterKHR],
        counter_descriptions: &mut [PerformanceCounterDescriptionKHR],
    ) -> Result<(), Error> {
        assert_eq!(counters.len(), counter_descriptions.len());
        unsafe {
            (self
                .fns()
                .khr_performance_query
                .as_ref()
                .unwrap()
                .enumerate_physical_device_queue_family_performance_query_counters_khr)(
                self.handle,
                queue_family_index,
                counters.len() as *mut u32,
                counters.as_mut_ptr(),
                counter_descriptions.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    #[inline]
    fn get_queue_family_performance_query_passes(
        &self,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_performance_query
                .as_ref()
                .unwrap()
                .get_physical_device_queue_family_performance_query_passes_khr)(
                self.handle,
                performance_query_create_info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }
}

pub trait PerformanceQueryDevice {
    fn acquire_profiling_lock(&self, info: &AcquireProfilingLockInfoKHR) -> Result<(), Error>;

    fn release_profiling_lock(&self);
}

impl PerformanceQueryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html>
    #[inline]
    fn acquire_profiling_lock(&self, info: &AcquireProfilingLockInfoKHR) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_performance_query
                .as_ref()
                .unwrap()
                .acquire_profiling_lock_khr)(self.handle, info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html>
    #[inline]
    fn release_profiling_lock(&self) {
        unsafe {
            (self
                .fns()
                .khr_performance_query
                .as_ref()
                .unwrap()
                .release_profiling_lock_khr)(self.handle)
        };
    }
}
