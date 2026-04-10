// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_KHR_external_memory_capabilities` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Instance`
pub const NAME: &CStr = c"VK_NV_external_memory_capabilities";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryCapabilitiesPhysicalDevice {
    fn get_external_image_format_properties(
        &self,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<ExternalImageFormatPropertiesNV, Error>;
}

impl ExternalMemoryCapabilitiesPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalImageFormatPropertiesNV.html>
    #[inline]
    fn get_external_image_format_properties(
        &self,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
        external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    ) -> Result<ExternalImageFormatPropertiesNV, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_memory_capabilities
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_external_image_format_properties_nv;

        unsafe {
            (call)(
                self.handle,
                format,
                ty,
                tiling,
                usage,
                flags,
                external_handle_type,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
