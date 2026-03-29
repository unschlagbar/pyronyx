// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_KHR_object_refresh";
pub const SPEC_VERSION: u32 = 1;

pub trait ObjectRefreshPhysicalDevice {
    fn get_refreshable_object_types(
        &self,
        refreshable_object_types: &mut [ObjectType],
    ) -> Result<(), vkResult>;
}

impl ObjectRefreshPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceRefreshableObjectTypesKHR.html>
    #[inline]
    fn get_refreshable_object_types(
        &self,
        refreshable_object_types: &mut [ObjectType],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_object_refresh
                .as_ref()
                .unwrap()
                .get_physical_device_refreshable_object_types_khr)(
                self.handle,
                refreshable_object_types.len() as *mut u32,
                refreshable_object_types.as_mut_ptr(),
            )
        }
        .result()
    }
}

pub trait ObjectRefreshCommandBuffer {
    fn refresh_objects(&self, refresh_objects: &RefreshObjectListKHR);
}

impl ObjectRefreshCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdRefreshObjectsKHR.html>
    #[inline]
    fn refresh_objects(&self, refresh_objects: &RefreshObjectListKHR) {
        unsafe {
            (self
                .fns()
                .khr_object_refresh
                .as_ref()
                .unwrap()
                .refresh_objects_khr)(self.handle, refresh_objects)
        };
    }
}
