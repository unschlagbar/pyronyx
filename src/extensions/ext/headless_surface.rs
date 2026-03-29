// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_headless_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait HeadlessSurfaceInstance {
    fn create_headless_surface(
        &self,
        create_info: &HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl HeadlessSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateHeadlessSurfaceEXT.html>
    #[inline]
    fn create_headless_surface(
        &self,
        create_info: &HeadlessSurfaceCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_headless_surface
                .as_ref()
                .unwrap()
                .create_headless_surface_ext)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
