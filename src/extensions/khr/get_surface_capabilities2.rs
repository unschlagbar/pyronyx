// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Instance`
pub const NAME: &CStr = c"VK_KHR_get_surface_capabilities2";
pub const SPEC_VERSION: u32 = 1;

pub trait GetSurfaceCapabilities2PhysicalDevice {
    fn get_surface_capabilities2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<SurfaceCapabilities2KHR<'_>, Error>;

    fn get_surface_formats2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: &mut [SurfaceFormat2KHR],
    ) -> Result<(), Error>;
}

impl GetSurfaceCapabilities2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    #[inline]
    fn get_surface_capabilities2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<SurfaceCapabilities2KHR<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_get_surface_capabilities2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_surface_capabilities2_khr;

        unsafe { (call)(self.handle, surface_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    #[inline]
    fn get_surface_formats2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: &mut [SurfaceFormat2KHR],
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .khr_get_surface_capabilities2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_surface_formats2_khr;

        unsafe {
            (call)(
                self.handle,
                surface_info,
                surface_formats.len() as *mut u32,
                surface_formats.as_mut_ptr(),
            )
        }
        .result()
    }
}
