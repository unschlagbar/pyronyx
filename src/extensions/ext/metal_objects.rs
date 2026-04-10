// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_metal_objects";
pub const SPEC_VERSION: u32 = 2;

pub trait MetalObjectsDevice {
    fn export_metal_objects(&self) -> ExportMetalObjectsInfoEXT<'_>;
}

impl MetalObjectsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkExportMetalObjectsEXT.html>
    #[inline]
    fn export_metal_objects(&self) -> ExportMetalObjectsInfoEXT<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_metal_objects
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .export_metal_objects_ext;

        unsafe {
            (call)(self.handle, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
