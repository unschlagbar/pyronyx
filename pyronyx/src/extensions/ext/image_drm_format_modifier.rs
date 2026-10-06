// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: ((([`VK_KHR_bind_memory2`](crate::khr::bind_memory2) + [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) + [`VK_KHR_sampler_ycbcr_conversion`](crate::khr::sampler_ycbcr_conversion)) or Vulkan 1.1) + [`VK_KHR_image_format_list`](crate::khr::image_format_list)) or Vulkan 1.2
pub const NAME: &CStr = c"VK_EXT_image_drm_format_modifier";
pub const SPEC_VERSION: u32 = 2;

pub trait ImageDrmFormatModifierDevice {
    fn get_image_drm_format_modifier_properties(
        &self,
        image: Image,
        properties: &mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> Result<()>;
}

impl ImageDrmFormatModifierDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>
    #[inline]
    fn get_image_drm_format_modifier_properties(
        &self,
        image: Image,
        properties: &mut ImageDrmFormatModifierPropertiesEXT<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_image_drm_format_modifier
            .get_image_drm_format_modifier_properties_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, image, properties) }.result()
    }
}
