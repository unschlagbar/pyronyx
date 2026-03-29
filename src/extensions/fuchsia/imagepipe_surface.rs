// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_FUCHSIA_imagepipe_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait ImagepipeSurfaceInstance {
    fn create_image_pipe_surface(
        &self,
        create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl ImagepipeSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImagePipeSurfaceFUCHSIA.html>
    #[inline]
    fn create_image_pipe_surface(
        &self,
        create_info: &ImagePipeSurfaceCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .fuchsia_imagepipe_surface
                .as_ref()
                .unwrap()
                .create_image_pipe_surface_fuchsia)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
