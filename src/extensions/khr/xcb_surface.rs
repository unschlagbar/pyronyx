// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_xcb_surface";
pub const SPEC_VERSION: u32 = 6;

pub trait XcbSurfacePhysicalDevice {
    fn get_xcb_presentation_support(
        &self,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32;
}

impl XcbSurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceXcbPresentationSupportKHR.html>
    #[inline]
    fn get_xcb_presentation_support(
        &self,
        queue_family_index: u32,
        connection: *mut xcb_connection_t,
        visual_id: xcb_visualid_t,
    ) -> Bool32 {
        unsafe {
            (self
                .fns()
                .khr_xcb_surface
                .as_ref()
                .unwrap()
                .get_physical_device_xcb_presentation_support_khr)(
                self.handle,
                queue_family_index,
                connection,
                visual_id,
            )
        }
    }
}

pub trait XcbSurfaceInstance {
    fn create_xcb_surface(
        &self,
        create_info: &XcbSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl XcbSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>
    #[inline]
    fn create_xcb_surface(
        &self,
        create_info: &XcbSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_xcb_surface
                .as_ref()
                .unwrap()
                .create_xcb_surface_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
