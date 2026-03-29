// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_win32_surface";
pub const SPEC_VERSION: u32 = 6;

pub trait Win32SurfacePhysicalDevice {
    fn get_win32_presentation_support(&self, queue_family_index: u32) -> Bool32;
}

impl Win32SurfacePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceWin32PresentationSupportKHR.html>
    #[inline]
    fn get_win32_presentation_support(&self, queue_family_index: u32) -> Bool32 {
        unsafe {
            (self
                .fns()
                .khr_win32_surface
                .as_ref()
                .unwrap()
                .get_physical_device_win32_presentation_support_khr)(
                self.handle,
                queue_family_index,
            )
        }
    }
}

pub trait Win32SurfaceInstance {
    fn create_win32_surface(
        &self,
        create_info: &Win32SurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult>;
}

impl Win32SurfaceInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateWin32SurfaceKHR.html>
    #[inline]
    fn create_win32_surface(
        &self,
        create_info: &Win32SurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_win32_surface
                .as_ref()
                .unwrap()
                .create_win32_surface_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
