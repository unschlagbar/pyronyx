// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_EXT_display_surface_counter";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplaySurfaceCounterPhysicalDevice {
    fn get_surface_capabilities2(
        &self,
        surface: SurfaceKHR,
    ) -> Result<SurfaceCapabilities2EXT<'_>, Error>;
}

impl DisplaySurfaceCounterPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2EXT.html>
    #[inline]
    fn get_surface_capabilities2(
        &self,
        surface: SurfaceKHR,
    ) -> Result<SurfaceCapabilities2EXT<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_display_surface_counter
                .as_ref()
                .unwrap()
                .get_physical_device_surface_capabilities2_ext)(
                self.handle,
                surface,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
