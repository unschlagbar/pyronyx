// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_int;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_external_fence_fd";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalFenceFdDevice {
    fn get_fence_fd(&self, get_fd_info: &FenceGetFdInfoKHR) -> Result<c_int, vkResult>;

    fn import_fence_fd(&self, import_fence_fd_info: &ImportFenceFdInfoKHR) -> Result<(), vkResult>;
}

impl ExternalFenceFdDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceFdKHR.html>
    #[inline]
    fn get_fence_fd(&self, get_fd_info: &FenceGetFdInfoKHR) -> Result<c_int, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_fence_fd
                .as_ref()
                .unwrap()
                .get_fence_fd_khr)(self.handle, get_fd_info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceFdKHR.html>
    #[inline]
    fn import_fence_fd(&self, import_fence_fd_info: &ImportFenceFdInfoKHR) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_external_fence_fd
                .as_ref()
                .unwrap()
                .import_fence_fd_khr)(self.handle, import_fence_fd_info)
        }
        .result()
    }
}
