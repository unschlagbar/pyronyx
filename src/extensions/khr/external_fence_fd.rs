// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_int;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_external_fence_fd";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalFenceFdDevice {
    fn get_fence_fd(&self, get_fd_info: &FenceGetFdInfoKHR) -> Result<c_int>;

    fn import_fence_fd(&self, import_fence_fd_info: &ImportFenceFdInfoKHR) -> Result<()>;
}

impl ExternalFenceFdDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html>
    #[inline]
    fn get_fence_fd(&self, get_fd_info: &FenceGetFdInfoKHR) -> Result<c_int> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_external_fence_fd
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_fence_fd_khr;

        unsafe { (call)(self.handle, get_fd_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html>
    #[inline]
    fn import_fence_fd(&self, import_fence_fd_info: &ImportFenceFdInfoKHR) -> Result<()> {
        let call = self
            .fns()
            .khr_external_fence_fd
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .import_fence_fd_khr;

        unsafe { (call)(self.handle, import_fence_fd_info) }.result()
    }
}
