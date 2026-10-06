// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
///
/// Promoted to core in Vulkan 1.1
///
/// Requires: [`VK_KHR_device_group_creation`](crate::khr::device_group_creation)
pub const NAME: &CStr = c"VK_KHR_device_group";
pub const SPEC_VERSION: u32 = 4;

pub trait DeviceGroupDevice {
    fn get_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> Result<()>;

    fn get_group_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR>;

    fn acquire_next_image2(
        &self,
        acquire_info: &AcquireNextImageInfoKHR,
    ) -> Result<Suboptimal<u32>>;
}

impl DeviceGroupDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPresentCapabilitiesKHR.html>
    #[inline]
    fn get_group_present_capabilities(
        &self,
        device_group_present_capabilities: &mut DeviceGroupPresentCapabilitiesKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_device_group
            .get_device_group_present_capabilities_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, device_group_present_capabilities) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupSurfacePresentModesKHR.html>
    #[inline]
    fn get_group_surface_present_modes(
        &self,
        surface: SurfaceKHR,
    ) -> Result<DeviceGroupPresentModeFlagsKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_device_group
            .get_device_group_surface_present_modes_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, surface, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImage2KHR.html>
    #[inline]
    fn acquire_next_image2(
        &self,
        acquire_info: &AcquireNextImageInfoKHR,
    ) -> Result<Suboptimal<u32>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_device_group
            .acquire_next_image2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, acquire_info, out.as_mut_ptr()) }
            .init_on_success_or_suboptimal(out)
    }
}

pub trait DeviceGroupPhysicalDevice {
    fn get_present_rectangles(&self, surface: SurfaceKHR, rects: &mut [Rect2D]) -> Result<()>;
    fn get_present_rectangles_len(&self, surface: SurfaceKHR) -> Result<usize>;
}

impl DeviceGroupPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDevicePresentRectanglesKHR.html>
    ///
    /// Call [`get_present_rectangles_len()`][`Self::get_present_rectangles_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_present_rectangles(&self, surface: SurfaceKHR, rects: &mut [Rect2D]) -> Result<()> {
        let mut rect_count = rects.len() as u32;
        let call = self
            .fns()
            .khr_device_group
            .get_physical_device_present_rectangles_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, surface, &mut rect_count, rects.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_present_rectangles`][`Self::get_present_rectangles`].
    #[inline]
    fn get_present_rectangles_len(&self, surface: SurfaceKHR) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_device_group
                .get_physical_device_present_rectangles_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                surface,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
