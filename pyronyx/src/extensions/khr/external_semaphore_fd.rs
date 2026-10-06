// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_int;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: [`VK_KHR_external_semaphore`](crate::khr::external_semaphore) or Vulkan 1.1
pub const NAME: &CStr = c"VK_KHR_external_semaphore_fd";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalSemaphoreFdDevice {
    fn get_semaphore_fd(&self, get_fd_info: &SemaphoreGetFdInfoKHR) -> Result<c_int>;

    fn import_semaphore_fd(
        &self,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> Result<()>;
}

impl ExternalSemaphoreFdDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreFdKHR.html>
    #[inline]
    fn get_semaphore_fd(&self, get_fd_info: &SemaphoreGetFdInfoKHR) -> Result<c_int> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_external_semaphore_fd
            .get_semaphore_fd_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, get_fd_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportSemaphoreFdKHR.html>
    #[inline]
    fn import_semaphore_fd(
        &self,
        import_semaphore_fd_info: &ImportSemaphoreFdInfoKHR,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_external_semaphore_fd
            .import_semaphore_fd_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, import_semaphore_fd_info) }.result()
    }
}
