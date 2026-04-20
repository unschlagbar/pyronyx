// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Instance`
pub const NAME: &CStr = c"VK_EXT_direct_mode_display";
pub const SPEC_VERSION: u32 = 1;

pub trait DirectModeDisplayPhysicalDevice {
    fn release_display(&self, display: DisplayKHR) -> Result<()>;
}

impl DirectModeDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseDisplayEXT.html>
    #[inline]
    fn release_display(&self, display: DisplayKHR) -> Result<()> {
        let call = self
            .fns()
            .ext_direct_mode_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .release_display_ext;

        unsafe { (call)(self.handle, display) }.result()
    }
}
