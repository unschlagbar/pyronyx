// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_wayland_surface";
pub const SPEC_VERSION: u32 = 6;

pub trait WaylandSurfaceInstance {
    fn create_wayland_surface(
        &self,
        create_info: &WaylandSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error>;
}

impl WaylandSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWaylandSurfaceKHR.html>
    #[inline]
    fn create_wayland_surface(
        &self,
        create_info: &WaylandSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_wayland_surface
                .as_ref()
                .unwrap()
                .create_wayland_surface_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait WaylandSurfacePhysicalDevice {
    fn get_wayland_presentation_support(
        &self,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32;
}

impl WaylandSurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWaylandPresentationSupportKHR.html>
    #[inline]
    fn get_wayland_presentation_support(
        &self,
        queue_family_index: u32,
        display: *mut wl_display,
    ) -> Bool32 {
        unsafe {
            (self
                .fns()
                .khr_wayland_surface
                .as_ref()
                .unwrap()
                .get_physical_device_wayland_presentation_support_khr)(
                self.handle,
                queue_family_index,
                display,
            )
        }
    }
}
