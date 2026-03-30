// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_EXT_direct_mode_display";
pub const SPEC_VERSION: u32 = 1;

pub trait DirectModeDisplayPhysicalDevice {
    fn release_display(&self, display: DisplayKHR) -> Result<(), Error>;
}

impl DirectModeDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html>
    #[inline]
    fn release_display(&self, display: DisplayKHR) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .ext_direct_mode_display
                .as_ref()
                .unwrap()
                .release_display_ext)(self.handle, display)
        }
        .result()
    }
}
