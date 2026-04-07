// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_full_screen_exclusive";
pub const SPEC_VERSION: u32 = 4;

pub trait FullScreenExclusivePhysicalDevice {
    fn get_surface_present_modes2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<Vec<PresentModeKHR>, Error>;
}

impl FullScreenExclusivePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModes2EXT.html>
    #[inline]
    fn get_surface_present_modes2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<Vec<PresentModeKHR>, Error> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .ext_full_screen_exclusive
                .as_ref()
                .unwrap()
                .get_physical_device_surface_present_modes2_ext)(
                self.handle,
                surface_info,
                count,
                data,
            )
        })
    }
}

pub trait FullScreenExclusiveDevice {
    fn get_group_surface_present_modes2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR, Error>;

    fn acquire_full_screen_exclusive_mode(&self, swapchain: SwapchainKHR) -> Result<(), Error>;

    fn release_full_screen_exclusive_mode(&self, swapchain: SwapchainKHR) -> Result<(), Error>;
}

impl FullScreenExclusiveDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModes2EXT.html>
    #[inline]
    fn get_group_surface_present_modes2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_full_screen_exclusive
                .as_ref()
                .unwrap()
                .get_device_group_surface_present_modes2_ext)(
                self.handle,
                surface_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireFullScreenExclusiveModeEXT.html>
    #[inline]
    fn acquire_full_screen_exclusive_mode(&self, swapchain: SwapchainKHR) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .ext_full_screen_exclusive
                .as_ref()
                .unwrap()
                .acquire_full_screen_exclusive_mode_ext)(self.handle, swapchain)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseFullScreenExclusiveModeEXT.html>
    #[inline]
    fn release_full_screen_exclusive_mode(&self, swapchain: SwapchainKHR) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .ext_full_screen_exclusive
                .as_ref()
                .unwrap()
                .release_full_screen_exclusive_mode_ext)(self.handle, swapchain)
        }
        .result()
    }
}
