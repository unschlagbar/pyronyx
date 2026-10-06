// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;

/// Type: `Device`
///
/// Requires: [`VK_KHR_external_memory`](crate::khr::external_memory) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_external_memory_host";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryHostDevice {
    fn get_memory_host_pointer_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        host_pointer: &c_void,
        memory_host_pointer_properties: &mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> Result<()>;
}

impl ExternalMemoryHostDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html>
    #[inline]
    fn get_memory_host_pointer_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        host_pointer: &c_void,
        memory_host_pointer_properties: &mut MemoryHostPointerPropertiesEXT<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_external_memory_host
            .get_memory_host_pointer_properties_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                handle_type,
                host_pointer,
                memory_host_pointer_properties,
            )
        }
        .result()
    }
}
