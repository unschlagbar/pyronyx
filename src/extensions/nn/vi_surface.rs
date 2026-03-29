// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_NN_vi_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait ViSurfaceInstance {
    fn create_vi_surface(
        &self,
        create_info: &ViSurfaceCreateInfoNN,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl ViSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html>
    #[inline]
    fn create_vi_surface(
        &self,
        create_info: &ViSurfaceCreateInfoNN,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nn_vi_surface
                .as_ref()
                .unwrap()
                .create_vi_surface_nn)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
