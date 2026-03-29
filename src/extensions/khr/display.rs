// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_display";
pub const SPEC_VERSION: u32 = 23;

pub trait DisplayInstance {
    fn create_display_plane_surface(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl DisplayInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html>
    #[inline]
    fn create_display_plane_surface(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .create_display_plane_surface_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait DisplayPhysicalDevice {
    fn get_display_properties(
        &self,
        properties: &mut [DisplayPropertiesKHR],
    ) -> Result<(), vkResult>;

    fn get_display_plane_properties(
        &self,
        properties: &mut [DisplayPlanePropertiesKHR],
    ) -> Result<(), vkResult>;

    fn get_display_plane_supported_displays(
        &self,
        plane_index: u32,
        displays: &mut [DisplayKHR],
    ) -> Result<(), vkResult>;

    fn get_display_mode_properties(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModePropertiesKHR],
    ) -> Result<(), vkResult>;

    fn create_display_mode(
        &self,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DisplayModeKHR, vkResult>;

    fn get_display_plane_capabilities(
        &self,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> Result<DisplayPlaneCapabilitiesKHR, vkResult>;
}

impl DisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    #[inline]
    fn get_display_properties(
        &self,
        properties: &mut [DisplayPropertiesKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .get_physical_device_display_properties_khr)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    #[inline]
    fn get_display_plane_properties(
        &self,
        properties: &mut [DisplayPlanePropertiesKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .get_physical_device_display_plane_properties_khr)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    #[inline]
    fn get_display_plane_supported_displays(
        &self,
        plane_index: u32,
        displays: &mut [DisplayKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .get_display_plane_supported_displays_khr)(
                self.handle,
                plane_index,
                displays.len() as *mut u32,
                displays.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html>
    #[inline]
    fn get_display_mode_properties(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModePropertiesKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .get_display_mode_properties_khr)(
                self.handle,
                display,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html>
    #[inline]
    fn create_display_mode(
        &self,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DisplayModeKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .create_display_mode_khr)(
                self.handle,
                display,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html>
    #[inline]
    fn get_display_plane_capabilities(
        &self,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> Result<DisplayPlaneCapabilitiesKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .unwrap()
                .get_display_plane_capabilities_khr)(
                self.handle,
                mode,
                plane_index,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
