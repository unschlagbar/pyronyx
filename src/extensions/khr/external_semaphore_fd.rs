// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_int;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_external_semaphore_fd";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalSemaphoreFdDevice {
    fn get_semaphore_fd(&self, get_fd_info: &SemaphoreGetFdInfoKHR) -> Result<c_int, vkResult>;

    fn import_semaphore_fd(
        &self,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> Result<(), vkResult>;
}

impl ExternalSemaphoreFdDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html>
    #[inline]
    fn get_semaphore_fd(&self, get_fd_info: &SemaphoreGetFdInfoKHR) -> Result<c_int, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_semaphore_fd
                .as_ref()
                .unwrap()
                .get_semaphore_fd_khr)(self.handle, get_fd_info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html>
    #[inline]
    fn import_semaphore_fd(
        &self,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_external_semaphore_fd
                .as_ref()
                .unwrap()
                .import_semaphore_fd_khr)(self.handle, import_semaphore_fd_info)
        }
        .result()
    }
}
