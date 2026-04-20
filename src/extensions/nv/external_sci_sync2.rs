// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_external_sci_sync2";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalSciSync2Device {
    fn get_fence_sci_sync_fence(
        &self,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
    ) -> Result<c_void>;

    fn get_fence_sci_sync_obj(
        &self,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
    ) -> Result<c_void>;

    fn import_fence_sci_sync_fence(
        &self,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> Result<()>;

    fn import_fence_sci_sync_obj(
        &self,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> Result<()>;

    fn create_semaphore_sci_sync_pool(
        &self,
        create_info: &SemaphoreSciSyncPoolCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SemaphoreSciSyncPoolNV>;

    fn destroy_semaphore_sci_sync_pool(
        &self,
        semaphore_pool: SemaphoreSciSyncPoolNV,
        allocator: Option<&AllocationCallbacks>,
    );
}

impl ExternalSciSync2Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceSciSyncFenceNV.html>
    #[inline]
    fn get_fence_sci_sync_fence(
        &self,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
    ) -> Result<c_void> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_fence_sci_sync_fence_nv;

        unsafe { (call)(self.handle, get_sci_sync_handle_info, out.as_mut_ptr()) }
            .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceSciSyncObjNV.html>
    #[inline]
    fn get_fence_sci_sync_obj(
        &self,
        get_sci_sync_handle_info: &FenceGetSciSyncInfoNV,
    ) -> Result<c_void> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_fence_sci_sync_obj_nv;

        unsafe { (call)(self.handle, get_sci_sync_handle_info, out.as_mut_ptr()) }
            .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceSciSyncFenceNV.html>
    #[inline]
    fn import_fence_sci_sync_fence(
        &self,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .import_fence_sci_sync_fence_nv;

        unsafe { (call)(self.handle, import_fence_sci_sync_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceSciSyncObjNV.html>
    #[inline]
    fn import_fence_sci_sync_obj(
        &self,
        import_fence_sci_sync_info: &ImportFenceSciSyncInfoNV,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .import_fence_sci_sync_obj_nv;

        unsafe { (call)(self.handle, import_fence_sci_sync_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphoreSciSyncPoolNV.html>
    #[inline]
    fn create_semaphore_sci_sync_pool(
        &self,
        create_info: &SemaphoreSciSyncPoolCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SemaphoreSciSyncPoolNV> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_semaphore_sci_sync_pool_nv;

        unsafe {
            (call)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphoreSciSyncPoolNV.html>
    #[inline]
    fn destroy_semaphore_sci_sync_pool(
        &self,
        semaphore_pool: SemaphoreSciSyncPoolNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_semaphore_sci_sync_pool_nv;

        unsafe {
            (call)(
                self.handle,
                semaphore_pool,
                allocator.map_or(null(), from_ref),
            )
        };
    }
}

pub trait ExternalSciSync2PhysicalDevice {
    fn get_sci_sync_attributes(
        &self,
        sci_sync_attributes_info: &SciSyncAttributesInfoNV,
        attributes: NvSciSyncAttrList,
    ) -> Result<()>;
}

impl ExternalSciSync2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSciSyncAttributesNV.html>
    #[inline]
    fn get_sci_sync_attributes(
        &self,
        sci_sync_attributes_info: &SciSyncAttributesInfoNV,
        attributes: NvSciSyncAttrList,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_external_sci_sync2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_sci_sync_attributes_nv;

        unsafe { (call)(self.handle, sci_sync_attributes_info, attributes) }.result()
    }
}
