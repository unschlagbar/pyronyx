// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_int;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: [`VK_KHR_external_memory`](crate::khr::external_memory) or Vulkan 1.1
pub const NAME: &CStr = c"VK_KHR_external_memory_fd";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryFdDevice {
    fn get_memory_fd(&self, get_fd_info: &MemoryGetFdInfoKHR) -> Result<c_int>;

    fn get_memory_fd_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR<'_>,
    ) -> Result<()>;
}

impl ExternalMemoryFdDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdKHR.html>
    #[inline]
    fn get_memory_fd(&self, get_fd_info: &MemoryGetFdInfoKHR) -> Result<c_int> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_external_memory_fd
            .get_memory_fd_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, get_fd_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryFdPropertiesKHR.html>
    #[inline]
    fn get_memory_fd_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        fd: c_int,
        memory_fd_properties: &mut MemoryFdPropertiesKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_external_memory_fd
            .get_memory_fd_properties_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, handle_type, fd, memory_fd_properties) }.result()
    }
}
