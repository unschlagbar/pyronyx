// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_external_memory_host";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryHostDevice {
    fn get_memory_host_pointer_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        host_pointer: &c_void,
    ) -> Result<MemoryHostPointerPropertiesEXT<'_>, Error>;
}

impl ExternalMemoryHostDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryHostPointerPropertiesEXT.html>
    #[inline]
    fn get_memory_host_pointer_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        host_pointer: &c_void,
    ) -> Result<MemoryHostPointerPropertiesEXT<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_external_memory_host
                .as_ref()
                .unwrap()
                .get_memory_host_pointer_properties_ext)(
                self.handle,
                handle_type,
                host_pointer,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
