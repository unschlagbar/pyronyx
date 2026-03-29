// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

#[deprecated = "This extension is deprecated. Use `VK_NV_external_sci_sync2` instead."]
pub const NAME: &CStr = c"VK_NV_external_sci_sync";
pub const SPEC_VERSION: u32 = 2;

pub trait ExternalSciSyncDevice {
    fn get_semaphore_sci_sync_obj(
        &self,
        get_sci_sync_info: &SemaphoreGetSciSyncInfoNV,
    ) -> Result<c_void, vkResult>;

    fn import_semaphore_sci_sync_obj(
        &self,
        import_semaphore_sci_sync_info: &ImportSemaphoreSciSyncInfoNV,
    ) -> Result<(), vkResult>;
}

impl ExternalSciSyncDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreSciSyncObjNV.html>
    #[inline]
    fn get_semaphore_sci_sync_obj(
        &self,
        get_sci_sync_info: &SemaphoreGetSciSyncInfoNV,
    ) -> Result<c_void, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_external_sci_sync
                .as_ref()
                .unwrap()
                .get_semaphore_sci_sync_obj_nv)(
                self.handle, get_sci_sync_info, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreSciSyncObjNV.html>
    #[inline]
    fn import_semaphore_sci_sync_obj(
        &self,
        import_semaphore_sci_sync_info: &ImportSemaphoreSciSyncInfoNV,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .nv_external_sci_sync
                .as_ref()
                .unwrap()
                .import_semaphore_sci_sync_obj_nv)(
                self.handle, import_semaphore_sci_sync_info
            )
        }
        .result()
    }
}
