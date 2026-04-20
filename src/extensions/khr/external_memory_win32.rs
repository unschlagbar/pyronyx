// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_external_memory_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryWin32Device {
    fn get_memory_win32_handle(
        &self,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> Result<HANDLE>;

    fn get_memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
    ) -> Result<MemoryWin32HandlePropertiesKHR<'_>>;
}

impl ExternalMemoryWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html>
    #[inline]
    fn get_memory_win32_handle(
        &self,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> Result<HANDLE> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_external_memory_win32
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_win32_handle_khr;

        unsafe { (call)(self.handle, get_win32_handle_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>
    #[inline]
    fn get_memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
    ) -> Result<MemoryWin32HandlePropertiesKHR<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_external_memory_win32
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_win32_handle_properties_khr;

        unsafe { (call)(self.handle, handle_type, handle, out.as_mut_ptr()) }.init_on_success(out)
    }
}
