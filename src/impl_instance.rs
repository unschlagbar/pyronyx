// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated impls
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::{from_ref, null};

impl Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyInstance.html>
    #[inline]
    pub fn destroy(&self, allocator: Option<&AllocationCallbacks>) {
        let call = self
            .fns()
            .v1_0
            .destroy_instance
            .expect(Self::CORE_LOAD_ERROR);

        unsafe { (call)(self.handle, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceGroups.html>
    ///
    /// Call [`enumerate_physical_device_groups_len()`][`Self::enumerate_physical_device_groups_len()`] to query the number of elements to pass to `out`.
    #[inline]
    pub fn enumerate_physical_device_groups(
        &self,
        physical_device_group_properties: &mut [PhysicalDeviceGroupProperties],
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .v1_1
            .enumerate_physical_device_groups
            .expect(Self::CORE_LOAD_ERROR);

        unsafe {
            (call)(
                self.handle,
                physical_device_group_properties.len() as *mut u32,
                physical_device_group_properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`enumerate_physical_device_groups`][`Self::enumerate_physical_device_groups`].
    #[inline]
    pub fn enumerate_physical_device_groups_len(&self) -> Result<usize, Error> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .enumerate_physical_device_groups
                .expect(Self::CORE_LOAD_ERROR))(
                self.handle, out.as_mut_ptr(), ptr::null_mut()
            )
        }
        .init_on_success(out)
        .map(|o| o as usize)
    }
}
