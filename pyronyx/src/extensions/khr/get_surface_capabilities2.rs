// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Instance`
///
/// Requires: [`VK_KHR_surface`](crate::khr::surface)
pub const NAME: &CStr = c"VK_KHR_get_surface_capabilities2";
pub const SPEC_VERSION: u32 = 1;

pub trait GetSurfaceCapabilities2PhysicalDevice {
    fn get_surface_capabilities2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: &mut SurfaceCapabilities2KHR<'_>,
    ) -> Result<()>;

    fn get_surface_formats2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: &mut [SurfaceFormat2KHR],
    ) -> Result<()>;
    fn get_surface_formats2_len(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<usize>;
}

impl GetSurfaceCapabilities2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceCapabilities2KHR.html>
    #[inline]
    fn get_surface_capabilities2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_capabilities: &mut SurfaceCapabilities2KHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_get_surface_capabilities2
            .get_physical_device_surface_capabilities2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, surface_info, surface_capabilities) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSurfaceFormats2KHR.html>
    ///
    /// Call [`get_surface_formats2_len()`][`Self::get_surface_formats2_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_surface_formats2(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
        surface_formats: &mut [SurfaceFormat2KHR],
    ) -> Result<()> {
        let mut surface_format_count = surface_formats.len() as u32;
        let call = self
            .fns()
            .khr_get_surface_capabilities2
            .get_physical_device_surface_formats2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                surface_info,
                &mut surface_format_count,
                surface_formats.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_surface_formats2`][`Self::get_surface_formats2`].
    #[inline]
    fn get_surface_formats2_len(
        &self,
        surface_info: &PhysicalDeviceSurfaceInfo2KHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_get_surface_capabilities2
                .get_physical_device_surface_formats2_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                surface_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
