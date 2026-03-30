// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_external_semaphore_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalSemaphoreWin32Device {
    fn get_semaphore_win32_handle(
        &self,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
    ) -> Result<HANDLE, Error>;

    fn import_semaphore_win32_handle(
        &self,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result<(), Error>;
}

impl ExternalSemaphoreWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreWin32HandleKHR.html>
    #[inline]
    fn get_semaphore_win32_handle(
        &self,
        get_win32_handle_info: &SemaphoreGetWin32HandleInfoKHR,
    ) -> Result<HANDLE, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_semaphore_win32
                .as_ref()
                .unwrap()
                .get_semaphore_win32_handle_khr)(
                self.handle,
                get_win32_handle_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreWin32HandleKHR.html>
    #[inline]
    fn import_semaphore_win32_handle(
        &self,
        import_semaphore_win32_handle_info: &ImportSemaphoreWin32HandleInfoKHR,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_external_semaphore_win32
                .as_ref()
                .unwrap()
                .import_semaphore_win32_handle_khr)(
                self.handle, import_semaphore_win32_handle_info
            )
        }
        .result()
    }
}
