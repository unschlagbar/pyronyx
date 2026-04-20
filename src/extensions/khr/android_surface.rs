// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_KHR_android_surface";
pub const SPEC_VERSION: u32 = 6;

pub trait AndroidSurfaceInstance {
    fn create_android_surface(
        &self,
        create_info: &AndroidSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR>;
}

impl AndroidSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAndroidSurfaceKHR.html>
    #[inline]
    fn create_android_surface(
        &self,
        create_info: &AndroidSurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_android_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_android_surface_khr;

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
