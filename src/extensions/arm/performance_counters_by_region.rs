// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_ARM_performance_counters_by_region";
pub const SPEC_VERSION: u32 = 1;

pub trait PerformanceCountersByRegionPhysicalDevice {
    fn enumerate_queue_family_performance_counters_by_region(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterARM],
        counter_descriptions: &mut [PerformanceCounterDescriptionARM],
    ) -> Result<(), Error>;
}

impl PerformanceCountersByRegionPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM.html>
    #[inline]
    fn enumerate_queue_family_performance_counters_by_region(
        &self,
        queue_family_index: u32,
        counters: &mut [PerformanceCounterARM],
        counter_descriptions: &mut [PerformanceCounterDescriptionARM],
    ) -> Result<(), Error> {
        assert_eq!(counters.len(), counter_descriptions.len());
        unsafe {
            (self
                .fns()
                .arm_performance_counters_by_region
                .as_ref()
                .unwrap()
                .enumerate_physical_device_queue_family_performance_counters_by_region_arm)(
                self.handle,
                queue_family_index,
                counters.len() as *mut u32,
                counters.as_mut_ptr(),
                counter_descriptions.as_mut_ptr(),
            )
        }
        .result()
    }
}
