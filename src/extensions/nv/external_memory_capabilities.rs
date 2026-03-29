// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

#[deprecated = "This extension is deprecated. Use `VK_KHR_external_memory_capabilities` instead."]
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
    ) -> Result<ExternalImageFormatPropertiesNV, vkResult>;
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
    ) -> Result<ExternalImageFormatPropertiesNV, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_external_memory_capabilities
                .as_ref()
                .unwrap()
                .get_physical_device_external_image_format_properties_nv)(
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
