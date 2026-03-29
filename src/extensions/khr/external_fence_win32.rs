// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_external_fence_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalFenceWin32Device {
    fn get_fence_win32_handle(
        &self,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    ) -> Result<HANDLE, vkResult>;

    fn import_fence_win32_handle(
        &self,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> Result<(), vkResult>;
}

impl ExternalFenceWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html>
    #[inline]
    fn get_fence_win32_handle(
        &self,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    ) -> Result<HANDLE, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_fence_win32
                .as_ref()
                .unwrap()
                .get_fence_win32_handle_khr)(
                self.handle, get_win32_handle_info, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html>
    #[inline]
    fn import_fence_win32_handle(
        &self,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_external_fence_win32
                .as_ref()
                .unwrap()
                .import_fence_win32_handle_khr)(
                self.handle, import_fence_win32_handle_info
            )
        }
        .result()
    }
}
