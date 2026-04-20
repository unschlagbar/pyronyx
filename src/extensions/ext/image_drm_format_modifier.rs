// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_image_drm_format_modifier";
pub const SPEC_VERSION: u32 = 2;

pub trait ImageDrmFormatModifierDevice {
    fn get_image_drm_format_modifier_properties(
        &self,
        image: Image,
    ) -> Result<ImageDrmFormatModifierPropertiesEXT<'_>>;
}

impl ImageDrmFormatModifierDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>
    #[inline]
    fn get_image_drm_format_modifier_properties(
        &self,
        image: Image,
    ) -> Result<ImageDrmFormatModifierPropertiesEXT<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_image_drm_format_modifier
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_image_drm_format_modifier_properties_ext;

        unsafe { (call)(self.handle, image, out.as_mut_ptr()) }.init_on_success(out)
    }
}
