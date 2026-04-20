// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_KHR_xcb_surface";
pub const SPEC_VERSION: u32 = 6;

pub trait XcbSurfaceInstance {
    fn create_xcb_surface(
        &self,
        create_info: &XcbSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR>;
}

impl XcbSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateXcbSurfaceKHR.html>
    #[inline]
    fn create_xcb_surface(
        &self,
        create_info: &XcbSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_xcb_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_xcb_surface_khr;

        unsafe {
            (call)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

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
        let call = self
            .fns()
            .khr_xcb_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_xcb_presentation_support_khr;

        unsafe { (call)(self.handle, queue_family_index, connection, visual_id) }
    }
}
