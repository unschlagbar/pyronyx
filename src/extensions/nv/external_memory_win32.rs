// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

#[deprecated = "This extension is deprecated. Use `VK_KHR_external_memory_win32` instead."]
pub const NAME: &CStr = c"VK_NV_external_memory_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryWin32Device {
    fn get_memory_win32_handle(
        &self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<HANDLE, Error>;
}

impl ExternalMemoryWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html>
    #[inline]
    fn get_memory_win32_handle(
        &self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<HANDLE, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_external_memory_win32
                .as_ref()
                .unwrap()
                .get_memory_win32_handle_nv)(
                self.handle, memory, handle_type, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}
