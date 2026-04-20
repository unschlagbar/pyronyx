// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_NN_vi_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait ViSurfaceInstance {
    fn create_vi_surface(
        &self,
        create_info: &ViSurfaceCreateInfoNN,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR>;
}

impl ViSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateViSurfaceNN.html>
    #[inline]
    fn create_vi_surface(
        &self,
        create_info: &ViSurfaceCreateInfoNN,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nn_vi_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_vi_surface_nn;

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
