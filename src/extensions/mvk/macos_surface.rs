// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_EXT_metal_surface` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_MVK_macos_surface";
pub const SPEC_VERSION: u32 = 3;

pub trait MacosSurfaceInstance {
    fn create_mac_os_surface(
        &self,
        create_info: &MacOSSurfaceCreateInfoMVK,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error>;
}

impl MacosSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMacOSSurfaceMVK.html>
    #[inline]
    fn create_mac_os_surface(
        &self,
        create_info: &MacOSSurfaceCreateInfoMVK,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .mvk_macos_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_mac_os_surface_mvk;

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
