// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_external_memory_metal";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryMetalDevice {
    fn get_memory_metal_handle(
        &self,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
    ) -> Result<*mut c_void>;

    fn get_memory_metal_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: &c_void,
    ) -> Result<MemoryMetalHandlePropertiesEXT<'_>>;
}

impl ExternalMemoryMetalDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandleEXT.html>
    #[inline]
    fn get_memory_metal_handle(
        &self,
        get_metal_handle_info: &MemoryGetMetalHandleInfoEXT,
    ) -> Result<*mut c_void> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_external_memory_metal
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_metal_handle_ext;

        unsafe { (call)(self.handle, get_metal_handle_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryMetalHandlePropertiesEXT.html>
    #[inline]
    fn get_memory_metal_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: &c_void,
    ) -> Result<MemoryMetalHandlePropertiesEXT<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_external_memory_metal
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_metal_handle_properties_ext;

        unsafe { (call)(self.handle, handle_type, handle, out.as_mut_ptr()) }.init_on_success(out)
    }
}
