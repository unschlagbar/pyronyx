// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated impls
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ptr::{from_ref, null};

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>
    #[inline]
    pub fn destroy(&self, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_instance.unwrap())(
                self.handle,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>
    #[inline]
    pub fn enumerate_physical_device_groups(
        &self,
        physical_device_group_properties: &mut [PhysicalDeviceGroupProperties],
    ) -> Result<(), vkResult> {
        unsafe {
            (self.fns().v1_1.enumerate_physical_device_groups.unwrap())(
                self.handle,
                physical_device_group_properties.len() as *mut u32,
                physical_device_group_properties.as_mut_ptr(),
            )
        }
        .result()
    }
}
