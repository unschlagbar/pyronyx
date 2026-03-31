// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_int;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_external_memory_fd";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryFdDevice {
    fn get_memory_fd(&self, get_fd_info: &MemoryGetFdInfoKHR) -> Result<c_int, Error>;

    fn get_memory_fd_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
    ) -> Result<MemoryFdPropertiesKHR<'_>, Error>;
}

impl ExternalMemoryFdDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html>
    #[inline]
    fn get_memory_fd(&self, get_fd_info: &MemoryGetFdInfoKHR) -> Result<c_int, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_memory_fd
                .as_ref()
                .unwrap()
                .get_memory_fd_khr)(self.handle, get_fd_info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html>
    #[inline]
    fn get_memory_fd_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
    ) -> Result<MemoryFdPropertiesKHR<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_external_memory_fd
                .as_ref()
                .unwrap()
                .get_memory_fd_properties_khr)(
                self.handle, handle_type, fd, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}
