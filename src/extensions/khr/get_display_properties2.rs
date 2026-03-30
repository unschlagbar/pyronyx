// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_KHR_get_display_properties2";
pub const SPEC_VERSION: u32 = 1;

pub trait GetDisplayProperties2PhysicalDevice {
    fn get_display_properties2(
        &self,
        properties: &mut [DisplayProperties2KHR],
    ) -> Result<(), Error>;

    fn get_display_plane_properties2(
        &self,
        properties: &mut [DisplayPlaneProperties2KHR],
    ) -> Result<(), Error>;

    fn get_display_mode_properties2(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModeProperties2KHR],
    ) -> Result<(), Error>;

    fn get_display_plane_capabilities2(
        &self,
        display_plane_info: &DisplayPlaneInfo2KHR,
    ) -> Result<DisplayPlaneCapabilities2KHR<'_>, Error>;
}

impl GetDisplayProperties2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html>
    #[inline]
    fn get_display_properties2(
        &self,
        properties: &mut [DisplayProperties2KHR],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .unwrap()
                .get_physical_device_display_properties2_khr)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>
    #[inline]
    fn get_display_plane_properties2(
        &self,
        properties: &mut [DisplayPlaneProperties2KHR],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .unwrap()
                .get_physical_device_display_plane_properties2_khr)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html>
    #[inline]
    fn get_display_mode_properties2(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModeProperties2KHR],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .unwrap()
                .get_display_mode_properties2_khr)(
                self.handle,
                display,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html>
    #[inline]
    fn get_display_plane_capabilities2(
        &self,
        display_plane_info: &DisplayPlaneInfo2KHR,
    ) -> Result<DisplayPlaneCapabilities2KHR<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .unwrap()
                .get_display_plane_capabilities2_khr)(
                self.handle,
                display_plane_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
