// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_xlib_surface";
pub const SPEC_VERSION: u32 = 6;

pub trait XlibSurfaceInstance {
    fn create_xlib_surface(
        &self,
        create_info: &XlibSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl XlibSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXlibSurfaceKHR.html>
    #[inline]
    fn create_xlib_surface(
        &self,
        create_info: &XlibSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_xlib_surface
                .as_ref()
                .unwrap()
                .create_xlib_surface_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait XlibSurfacePhysicalDevice {
    fn get_xlib_presentation_support(
        &self,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32;
}

impl XlibSurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXlibPresentationSupportKHR.html>
    #[inline]
    fn get_xlib_presentation_support(
        &self,
        queue_family_index: u32,
        dpy: *mut Display,
        visual_id: VisualID,
    ) -> Bool32 {
        unsafe {
            (self
                .fns()
                .khr_xlib_surface
                .as_ref()
                .unwrap()
                .get_physical_device_xlib_presentation_support_khr)(
                self.handle,
                queue_family_index,
                dpy,
                visual_id,
            )
        }
    }
}
