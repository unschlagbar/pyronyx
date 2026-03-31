// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_QNX_screen_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait ScreenSurfaceInstance {
    fn create_screen_surface(
        &self,
        create_info: &ScreenSurfaceCreateInfoQNX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error>;
}

impl ScreenSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateScreenSurfaceQNX.html>
    #[inline]
    fn create_screen_surface(
        &self,
        create_info: &ScreenSurfaceCreateInfoQNX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .qnx_screen_surface
                .as_ref()
                .unwrap()
                .create_screen_surface_qnx)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait ScreenSurfacePhysicalDevice {
    fn get_screen_presentation_support(
        &self,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32;
}

impl ScreenSurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceScreenPresentationSupportQNX.html>
    #[inline]
    fn get_screen_presentation_support(
        &self,
        queue_family_index: u32,
        window: *mut _screen_window,
    ) -> Bool32 {
        unsafe {
            (self
                .fns()
                .qnx_screen_surface
                .as_ref()
                .unwrap()
                .get_physical_device_screen_presentation_support_qnx)(
                self.handle,
                queue_family_index,
                window,
            )
        }
    }
}
