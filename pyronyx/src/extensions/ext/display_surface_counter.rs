// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Instance`
///
/// Requires: [`VK_KHR_display`](crate::khr::display)
pub const NAME: &CStr = c"VK_EXT_display_surface_counter";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplaySurfaceCounterPhysicalDevice {
    fn get_surface_capabilities2(
        &self,
        surface: SurfaceKHR,
        surface_capabilities: &mut SurfaceCapabilities2EXT<'_>,
    ) -> Result<()>;
}

impl DisplaySurfaceCounterPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    #[inline]
    fn get_surface_capabilities2(
        &self,
        surface: SurfaceKHR,
        surface_capabilities: &mut SurfaceCapabilities2EXT<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_display_surface_counter
            .get_physical_device_surface_capabilities2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, surface, surface_capabilities) }.result()
    }
}
