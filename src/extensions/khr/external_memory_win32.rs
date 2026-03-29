// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_external_memory_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryWin32Device {
    fn get_memory_win32_handle(
        &self,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> Result<HANDLE, vkResult>;

    fn get_memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
    ) -> Result<MemoryWin32HandlePropertiesKHR<'_>, vkResult>;
}

impl ExternalMemoryWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleKHR.html>
    #[inline]
    fn get_memory_win32_handle(
        &self,
        get_win32_handle_info: &MemoryGetWin32HandleInfoKHR,
    ) -> Result<HANDLE, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_memory_win32
                .as_ref()
                .unwrap()
                .get_memory_win32_handle_khr)(
                self.handle, get_win32_handle_info, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>
    #[inline]
    fn get_memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
    ) -> Result<MemoryWin32HandlePropertiesKHR<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_memory_win32
                .as_ref()
                .unwrap()
                .get_memory_win32_handle_properties_khr)(
                self.handle,
                handle_type,
                handle,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
