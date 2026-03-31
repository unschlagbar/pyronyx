// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_device_group";
pub const SPEC_VERSION: u32 = 4;

pub trait DeviceGroupDevice {
    fn get_device_group_present_capabilities(
        &self,
    ) -> Result<DeviceGroupPresentCapabilitiesKHR<'_>, Error>;

    fn get_device_group_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR, Error>;

    fn acquire_next_image2(&self, acquire_info: &AcquireNextImageInfoKHR) -> Result<u32, Error>;
}

impl DeviceGroupDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    #[inline]
    fn get_device_group_present_capabilities(
        &self,
    ) -> Result<DeviceGroupPresentCapabilitiesKHR<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_device_group
                .as_ref()
                .unwrap()
                .get_device_group_present_capabilities_khr)(
                self.handle, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html>
    #[inline]
    fn get_device_group_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_device_group
                .as_ref()
                .unwrap()
                .get_device_group_surface_present_modes_khr)(
                self.handle, surface, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html>
    #[inline]
    fn acquire_next_image2(&self, acquire_info: &AcquireNextImageInfoKHR) -> Result<u32, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_device_group
                .as_ref()
                .unwrap()
                .acquire_next_image2_khr)(self.handle, acquire_info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }
}

pub trait DeviceGroupPhysicalDevice {
    fn get_present_rectangles(
        &self,
        surface: SurfaceKHR,
        rects: &mut [Rect2D],
    ) -> Result<(), Error>;
}

impl DeviceGroupPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html>
    #[inline]
    fn get_present_rectangles(
        &self,
        surface: SurfaceKHR,
        rects: &mut [Rect2D],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_device_group
                .as_ref()
                .unwrap()
                .get_physical_device_present_rectangles_khr)(
                self.handle,
                surface,
                rects.len() as *mut u32,
                rects.as_mut_ptr(),
            )
        }
        .result()
    }
}
