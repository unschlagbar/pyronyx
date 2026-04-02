// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_performance_query";
pub const SPEC_VERSION: u32 = 1;

pub trait PerformanceQueryPhysicalDevice {
    fn enumerate_queue_family_performance_query_counters(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterKHR],
        counter_descriptions: &mut [PerformanceCounterDescriptionKHR],
    ) -> Result<(), Error>;
    fn enumerate_queue_family_performance_query_counters_len(
        &self,
        queue_family_index: u32,
    ) -> Result<usize, Error>;

    fn get_queue_family_performance_query_passes(
        &self,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32;
}

impl PerformanceQueryPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR.html>
    ///
    /// Call [`enumerate_queue_family_performance_query_counters_len()`][`Self::enumerate_queue_family_performance_query_counters_len()`] to query the number of elements to pass to `out`.
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

    /// Returns the required slice length for Call [`enumerate_queue_family_performance_query_counters`][`Self::enumerate_queue_family_performance_query_counters`].
    #[inline]
    fn enumerate_queue_family_performance_query_counters_len(
        &self,
        queue_family_index: u32,
    ) -> Result<usize, Error> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_performance_query
                .as_ref()
                .unwrap()
                .enumerate_physical_device_queue_family_performance_query_counters_khr)(
                self.handle,
                queue_family_index,
                out.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|o| o as usize)
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
            );
            out.assume_init()
        }
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
