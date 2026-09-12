// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Instance`
pub const NAME: &CStr = c"VK_KHR_get_display_properties2";
pub const SPEC_VERSION: u32 = 1;

pub trait GetDisplayProperties2PhysicalDevice {
    fn get_display_properties2(&self, properties: &mut [DisplayProperties2KHR]) -> Result<()>;
    fn get_display_properties2_len(&self) -> Result<usize>;

    fn get_display_plane_properties2(
        &self,
        properties: &mut [DisplayPlaneProperties2KHR],
    ) -> Result<()>;
    fn get_display_plane_properties2_len(&self) -> Result<usize>;

    fn get_display_mode_properties2(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModeProperties2KHR],
    ) -> Result<()>;
    fn get_display_mode_properties2_len(&self, display: DisplayKHR) -> Result<usize>;

    fn get_display_plane_capabilities2(
        &self,
        display_plane_info: &DisplayPlaneInfo2KHR,
    ) -> Result<DisplayPlaneCapabilities2KHR<'_>>;
}

impl GetDisplayProperties2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayProperties2KHR.html>
    ///
    /// Call [`get_display_properties2_len()`][`Self::get_display_properties2_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_properties2(&self, properties: &mut [DisplayProperties2KHR]) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_get_display_properties2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_display_properties2_khr;

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_display_properties2`][`Self::get_display_properties2`].
    #[inline]
    fn get_display_properties2_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_physical_device_display_properties2_khr)(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlaneProperties2KHR.html>
    ///
    /// Call [`get_display_plane_properties2_len()`][`Self::get_display_plane_properties2_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_plane_properties2(
        &self,
        properties: &mut [DisplayPlaneProperties2KHR],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_get_display_properties2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_display_plane_properties2_khr;

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_display_plane_properties2`][`Self::get_display_plane_properties2`].
    #[inline]
    fn get_display_plane_properties2_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_physical_device_display_plane_properties2_khr)(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModeProperties2KHR.html>
    ///
    /// Call [`get_display_mode_properties2_len()`][`Self::get_display_mode_properties2_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_mode_properties2(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModeProperties2KHR],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_get_display_properties2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_display_mode_properties2_khr;

        unsafe {
            (call)(
                self.handle,
                display,
                &mut property_count,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_display_mode_properties2`][`Self::get_display_mode_properties2`].
    #[inline]
    fn get_display_mode_properties2_len(&self, display: DisplayKHR) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_get_display_properties2
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_display_mode_properties2_khr)(
                self.handle,
                display,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilities2KHR.html>
    #[inline]
    fn get_display_plane_capabilities2(
        &self,
        display_plane_info: &DisplayPlaneInfo2KHR,
    ) -> Result<DisplayPlaneCapabilities2KHR<'_>> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .khr_get_display_properties2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_display_plane_capabilities2_khr;

        unsafe { (call)(self.handle, display_plane_info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
