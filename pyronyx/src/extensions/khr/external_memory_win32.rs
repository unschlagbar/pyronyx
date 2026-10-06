// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: [`VK_KHR_external_memory`](crate::khr::external_memory) or Vulkan 1.1
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
        memory_win32_handle_properties: &mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> Result<()>;
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
            .get_memory_win32_handle_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, get_win32_handle_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandlePropertiesKHR.html>
    #[inline]
    fn get_memory_win32_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: HANDLE,
        memory_win32_handle_properties: &mut MemoryWin32HandlePropertiesKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_external_memory_win32
            .get_memory_win32_handle_properties_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                handle_type,
                handle,
                memory_win32_handle_properties,
            )
        }
        .result()
    }
}
