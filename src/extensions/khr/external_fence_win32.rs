// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_external_fence_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalFenceWin32Device {
    fn get_fence_win32_handle(
        &self,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    ) -> Result<HANDLE>;

    fn import_fence_win32_handle(
        &self,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> Result<()>;
}

impl ExternalFenceWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceWin32HandleKHR.html>
    #[inline]
    fn get_fence_win32_handle(
        &self,
        get_win32_handle_info: &FenceGetWin32HandleInfoKHR,
    ) -> Result<HANDLE> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_external_fence_win32
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_fence_win32_handle_khr;

        unsafe { (call)(self.handle, get_win32_handle_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkImportFenceWin32HandleKHR.html>
    #[inline]
    fn import_fence_win32_handle(
        &self,
        import_fence_win32_handle_info: &ImportFenceWin32HandleInfoKHR,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_external_fence_win32
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .import_fence_win32_handle_khr;

        unsafe { (call)(self.handle, import_fence_win32_handle_info) }.result()
    }
}
