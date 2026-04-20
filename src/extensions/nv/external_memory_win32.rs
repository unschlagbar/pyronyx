// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_KHR_external_memory_win32` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_external_memory_win32";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryWin32Device {
    fn get_memory_win32_handle(
        &self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<HANDLE>;
}

impl ExternalMemoryWin32Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryWin32HandleNV.html>
    #[inline]
    fn get_memory_win32_handle(
        &self,
        memory: DeviceMemory,
        handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<HANDLE> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_memory_win32
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_win32_handle_nv;

        unsafe { (call)(self.handle, memory, handle_type, out.as_mut_ptr()) }.init_on_success(out)
    }
}
