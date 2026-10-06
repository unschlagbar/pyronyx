// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_object_refresh";
pub const SPEC_VERSION: u32 = 1;

pub trait ObjectRefreshCommandBuffer {
    fn refresh_objects(&self, refresh_objects: &RefreshObjectListKHR);
}

impl ObjectRefreshCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdRefreshObjectsKHR.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn refresh_objects(&self, refresh_objects: &RefreshObjectListKHR) {
        let call = self
            .fns()
            .khr_object_refresh
            .refresh_objects_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, refresh_objects) };
    }
}

pub trait ObjectRefreshPhysicalDevice {
    fn get_refreshable_object_types(
        &self,
        refreshable_object_types: &mut [ObjectType],
    ) -> Result<()>;
    fn get_refreshable_object_types_len(&self) -> Result<usize>;
}

impl ObjectRefreshPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceRefreshableObjectTypesKHR.html>
    ///
    /// Call [`get_refreshable_object_types_len()`][`Self::get_refreshable_object_types_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_refreshable_object_types(
        &self,
        refreshable_object_types: &mut [ObjectType],
    ) -> Result<()> {
        let mut refreshable_object_type_count = refreshable_object_types.len() as u32;
        let call = self
            .fns()
            .khr_object_refresh
            .get_physical_device_refreshable_object_types_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                &mut refreshable_object_type_count,
                refreshable_object_types.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_refreshable_object_types`][`Self::get_refreshable_object_types`].
    #[inline]
    fn get_refreshable_object_types_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_object_refresh
                .get_physical_device_refreshable_object_types_khr
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
