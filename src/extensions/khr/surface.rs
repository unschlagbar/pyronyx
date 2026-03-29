// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_surface";
pub const SPEC_VERSION: u32 = 25;

pub trait SurfaceInstance {
    fn destroy_surface(&self, surface: SurfaceKHR, allocator: Option<&AllocationCallbacks>);
}

impl SurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySurfaceKHR.html>
    #[inline]
    fn destroy_surface(&self, surface: SurfaceKHR, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().khr_surface.as_ref().unwrap().destroy_surface_khr)(
                self.handle,
                surface,
                allocator.map_or(null(), from_ref),
            )
        };
    }
}

pub trait SurfacePhysicalDevice {
    fn get_surface_support(
        &self,
        queue_family_index: u32,
        surface: SurfaceKHR,
    ) -> Result<bool, vkResult>;

    fn get_surface_capabilities(
        &self,
        surface: SurfaceKHR,
    ) -> Result<SurfaceCapabilitiesKHR, vkResult>;

    fn get_surface_formats(&self, surface: SurfaceKHR) -> Result<Vec<SurfaceFormatKHR>, vkResult>;

    fn get_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> Result<Vec<PresentModeKHR>, vkResult>;
}

impl SurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceSupportKHR.html>
    #[inline]
    fn get_surface_support(
        &self,
        queue_family_index: u32,
        surface: SurfaceKHR,
    ) -> Result<bool, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_surface
                .as_ref()
                .unwrap()
                .get_physical_device_surface_support_khr)(
                self.handle,
                queue_family_index,
                surface,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
        .map(|v| v != 0)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilitiesKHR.html>
    #[inline]
    fn get_surface_capabilities(
        &self,
        surface: SurfaceKHR,
    ) -> Result<SurfaceCapabilitiesKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_surface
                .as_ref()
                .unwrap()
                .get_physical_device_surface_capabilities_khr)(
                self.handle,
                surface,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormatsKHR.html>
    #[inline]
    fn get_surface_formats(&self, surface: SurfaceKHR) -> Result<Vec<SurfaceFormatKHR>, vkResult> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .khr_surface
                .as_ref()
                .unwrap()
                .get_physical_device_surface_formats_khr)(
                self.handle, surface, count, data
            )
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfacePresentModesKHR.html>
    #[inline]
    fn get_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> Result<Vec<PresentModeKHR>, vkResult> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .khr_surface
                .as_ref()
                .unwrap()
                .get_physical_device_surface_present_modes_khr)(
                self.handle, surface, count, data
            )
        })
    }
}
