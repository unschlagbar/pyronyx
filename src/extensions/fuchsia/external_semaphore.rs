// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_FUCHSIA_external_semaphore";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalSemaphoreDevice {
    fn get_semaphore_zircon_handle(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t, vkResult>;

    fn import_semaphore_zircon_handle(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result<(), vkResult>;
}

impl ExternalSemaphoreDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html>
    #[inline]
    fn get_semaphore_zircon_handle(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .fuchsia_external_semaphore
                .as_ref()
                .unwrap()
                .get_semaphore_zircon_handle_fuchsia)(
                self.handle,
                get_zircon_handle_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html>
    #[inline]
    fn import_semaphore_zircon_handle(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .fuchsia_external_semaphore
                .as_ref()
                .unwrap()
                .import_semaphore_zircon_handle_fuchsia)(
                self.handle,
                import_semaphore_zircon_handle_info,
            )
        }
        .result()
    }
}
