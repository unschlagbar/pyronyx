// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_NV_external_sci_sync2` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;

/// Type: `Device`
///
/// Requires: Vulkan 1.1
pub const NAME: &CStr = c"VK_NV_external_sci_sync";
pub const SPEC_VERSION: u32 = 2;

pub trait ExternalSciSyncDevice {
    fn get_semaphore_sci_sync_obj(
        &self,
        get_sci_sync_info: &SemaphoreGetSciSyncInfoNV,
        handle: *mut c_void,
    ) -> Result<()>;

    fn import_semaphore_sci_sync_obj(
        &self,
        import_semaphore_sci_sync_info: &ImportSemaphoreSciSyncInfoNV,
    ) -> Result<()>;
}

impl ExternalSciSyncDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreSciSyncObjNV.html>
    #[inline]
    fn get_semaphore_sci_sync_obj(
        &self,
        get_sci_sync_info: &SemaphoreGetSciSyncInfoNV,
        handle: *mut c_void,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_external_sci_sync
            .get_semaphore_sci_sync_obj_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, get_sci_sync_info, handle) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreSciSyncObjNV.html>
    #[inline]
    fn import_semaphore_sci_sync_obj(
        &self,
        import_semaphore_sci_sync_info: &ImportSemaphoreSciSyncInfoNV,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_external_sci_sync
            .import_semaphore_sci_sync_obj_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, import_semaphore_sci_sync_info) }.result()
    }
}
