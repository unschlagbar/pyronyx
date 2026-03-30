// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_directfb_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait DirectfbSurfaceInstance {
    fn create_direct_fb_surface(
        &self,
        create_info: &DirectFBSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error>;
}

impl DirectfbSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDirectFBSurfaceEXT.html>
    #[inline]
    fn create_direct_fb_surface(
        &self,
        create_info: &DirectFBSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_directfb_surface
                .as_ref()
                .unwrap()
                .create_direct_fb_surface_ext)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait DirectfbSurfacePhysicalDevice {
    fn get_direct_fb_presentation_support(
        &self,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32;
}

impl DirectfbSurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDirectFBPresentationSupportEXT.html>
    #[inline]
    fn get_direct_fb_presentation_support(
        &self,
        queue_family_index: u32,
        dfb: *mut IDirectFB,
    ) -> Bool32 {
        unsafe {
            (self
                .fns()
                .ext_directfb_surface
                .as_ref()
                .unwrap()
                .get_physical_device_direct_fb_presentation_support_ext)(
                self.handle,
                queue_family_index,
                dfb,
            )
        }
    }
}
