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
    ) -> Result<()>;
    fn enumerate_queue_family_performance_query_counters_len(
        &self,
        queue_family_index: u32,
    ) -> Result<usize>;

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
    ) -> Result<()> {
        assert_eq!(counters.len(), counter_descriptions.len());
        let call = self
            .fns()
            .khr_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .enumerate_physical_device_queue_family_performance_query_counters_khr;

        unsafe {
            (call)(
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
    ) -> Result<usize> {
        let mut out: MaybeUninit<usize> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_performance_query
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .enumerate_physical_device_queue_family_performance_query_counters_khr)(
                self.handle,
                queue_family_index,
                out.as_mut_ptr() as *mut u32,
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR.html>
    #[inline]
    fn get_queue_family_performance_query_passes(
        &self,
        performance_query_create_info: &QueryPoolPerformanceCreateInfoKHR,
    ) -> u32 {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_queue_family_performance_query_passes_khr;

        unsafe {
            (call)(self.handle, performance_query_create_info, out.as_mut_ptr());
            out.assume_init()
        }
    }
}

pub trait PerformanceQueryDevice {
    fn acquire_profiling_lock(&self, info: &AcquireProfilingLockInfoKHR) -> Result<()>;

    fn release_profiling_lock(&self);
}

impl PerformanceQueryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireProfilingLockKHR.html>
    #[inline]
    fn acquire_profiling_lock(&self, info: &AcquireProfilingLockInfoKHR) -> Result<()> {
        let call = self
            .fns()
            .khr_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .acquire_profiling_lock_khr;

        unsafe { (call)(self.handle, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseProfilingLockKHR.html>
    #[inline]
    fn release_profiling_lock(&self) {
        let call = self
            .fns()
            .khr_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .release_profiling_lock_khr;

        unsafe { (call)(self.handle) };
    }
}
