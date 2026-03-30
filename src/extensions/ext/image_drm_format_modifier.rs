// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_EXT_image_drm_format_modifier";
pub const SPEC_VERSION: u32 = 2;

pub trait ImageDrmFormatModifierDevice {
    fn get_image_drm_format_modifier_properties(
        &self,
        image: Image,
    ) -> Result<ImageDrmFormatModifierPropertiesEXT<'_>, Error>;
}

impl ImageDrmFormatModifierDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageDrmFormatModifierPropertiesEXT.html>
    #[inline]
    fn get_image_drm_format_modifier_properties(
        &self,
        image: Image,
    ) -> Result<ImageDrmFormatModifierPropertiesEXT<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_image_drm_format_modifier
                .as_ref()
                .unwrap()
                .get_image_drm_format_modifier_properties_ext)(
                self.handle, image, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}
