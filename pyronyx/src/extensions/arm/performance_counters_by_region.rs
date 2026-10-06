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
pub const NAME: &CStr = c"VK_ARM_performance_counters_by_region";
pub const SPEC_VERSION: u32 = 1;

pub trait PerformanceCountersByRegionPhysicalDevice {
    fn enumerate_queue_family_performance_counters_by_region(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterARM],
        counter_descriptions: &mut [PerformanceCounterDescriptionARM],
    ) -> Result<()>;
    fn enumerate_queue_family_performance_counters_by_region_len(
        &self,
        queue_family_index: u32,
    ) -> Result<usize>;
}

impl PerformanceCountersByRegionPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html>
    ///
    /// Call [`enumerate_queue_family_performance_counters_by_region_len()`][`Self::enumerate_queue_family_performance_counters_by_region_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn enumerate_queue_family_performance_counters_by_region(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterARM],
        counter_descriptions: &mut [PerformanceCounterDescriptionARM],
    ) -> Result<()> {
        assert_eq!(counters.len(), counter_descriptions.len());
        let mut counter_count = counters.len() as u32;
        let call = self
            .fns()
            .arm_performance_counters_by_region
            .enumerate_physical_device_queue_family_performance_counters_by_region_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                queue_family_index,
                &mut counter_count,
                counters.as_mut_ptr(),
                counter_descriptions.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`enumerate_queue_family_performance_counters_by_region`][`Self::enumerate_queue_family_performance_counters_by_region`].
    #[inline]
    fn enumerate_queue_family_performance_counters_by_region_len(
        &self,
        queue_family_index: u32,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_performance_counters_by_region
                .enumerate_physical_device_queue_family_performance_counters_by_region_arm
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                queue_family_index,
                out.as_mut_ptr(),
                ptr::null_mut(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
