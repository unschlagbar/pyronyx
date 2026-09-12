// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_SEC_ubm_surface";
pub const SPEC_VERSION: u32 = 1;

pub trait UbmSurfaceInstance {
    fn create_ubm_surface(
        &self,
        create_info: &UbmSurfaceCreateInfoSEC,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR>;
}

impl UbmSurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateUbmSurfaceSEC.html>
    #[inline]
    fn create_ubm_surface(
        &self,
        create_info: &UbmSurfaceCreateInfoSEC,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .sec_ubm_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_ubm_surface_sec;

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

pub trait UbmSurfacePhysicalDevice {
    fn get_ubm_presentation_support(
        &self,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> Bool32;
}

impl UbmSurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceUbmPresentationSupportSEC.html>
    #[inline]
    fn get_ubm_presentation_support(
        &self,
        queue_family_index: u32,
        device: *mut ubm_device,
    ) -> Bool32 {
        let call = self
            .fns()
            .sec_ubm_surface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_ubm_presentation_support_sec;

        unsafe { (call)(self.handle, queue_family_index, device) }
    }
}
