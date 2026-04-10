// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_FUCHSIA_external_semaphore";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalSemaphoreDevice {
    fn get_semaphore_zircon_handle(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t, Error>;

    fn import_semaphore_zircon_handle(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result<(), Error>;
}

impl ExternalSemaphoreDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreZirconHandleFUCHSIA.html>
    #[inline]
    fn get_semaphore_zircon_handle(
        &self,
        get_zircon_handle_info: &SemaphoreGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .fuchsia_external_semaphore
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_semaphore_zircon_handle_fuchsia;

        unsafe { (call)(self.handle, get_zircon_handle_info, out.as_mut_ptr()) }
            .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreZirconHandleFUCHSIA.html>
    #[inline]
    fn import_semaphore_zircon_handle(
        &self,
        import_semaphore_zircon_handle_info: &ImportSemaphoreZirconHandleInfoFUCHSIA,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .fuchsia_external_semaphore
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .import_semaphore_zircon_handle_fuchsia;

        unsafe { (call)(self.handle, import_semaphore_zircon_handle_info) }.result()
    }
}
