// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_metal_objects";
pub const SPEC_VERSION: u32 = 2;

pub trait MetalObjectsDevice {
    fn export_metal_objects(&self, metal_objects_info: &mut ExportMetalObjectsInfoEXT<'_>);
}

impl MetalObjectsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html>
    #[inline]
    fn export_metal_objects(&self, metal_objects_info: &mut ExportMetalObjectsInfoEXT<'_>) {
        let call = self
            .fns()
            .ext_metal_objects
            .export_metal_objects_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, metal_objects_info) };
    }
}
