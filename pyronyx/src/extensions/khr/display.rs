// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_KHR_display";
pub const SPEC_VERSION: u32 = 23;

pub trait DisplayPhysicalDevice {
    fn get_display_properties(&self, properties: &mut [DisplayPropertiesKHR]) -> Result<()>;
    fn get_display_properties_len(&self) -> Result<usize>;

    fn get_display_plane_properties(
        &self,
        properties: &mut [DisplayPlanePropertiesKHR],
    ) -> Result<()>;
    fn get_display_plane_properties_len(&self) -> Result<usize>;

    fn get_display_plane_supported_displays(
        &self,
        plane_index: u32,
        displays: &mut [DisplayKHR],
    ) -> Result<()>;
    fn get_display_plane_supported_displays_len(&self, plane_index: u32) -> Result<usize>;

    fn get_display_mode_properties(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModePropertiesKHR],
    ) -> Result<()>;
    fn get_display_mode_properties_len(&self, display: DisplayKHR) -> Result<usize>;

    fn create_display_mode(
        &self,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DisplayModeKHR>;

    fn get_display_plane_capabilities(
        &self,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> Result<DisplayPlaneCapabilitiesKHR>;
}

impl DisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPropertiesKHR.html>
    ///
    /// Call [`get_display_properties_len()`][`Self::get_display_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_properties(&self, properties: &mut [DisplayPropertiesKHR]) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_display_properties_khr;

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_display_properties`][`Self::get_display_properties`].
    #[inline]
    fn get_display_properties_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_physical_device_display_properties_khr)(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDisplayPlanePropertiesKHR.html>
    ///
    /// Call [`get_display_plane_properties_len()`][`Self::get_display_plane_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_plane_properties(
        &self,
        properties: &mut [DisplayPlanePropertiesKHR],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_display_plane_properties_khr;

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_display_plane_properties`][`Self::get_display_plane_properties`].
    #[inline]
    fn get_display_plane_properties_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_physical_device_display_plane_properties_khr)(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneSupportedDisplaysKHR.html>
    ///
    /// Call [`get_display_plane_supported_displays_len()`][`Self::get_display_plane_supported_displays_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_plane_supported_displays(
        &self,
        plane_index: u32,
        displays: &mut [DisplayKHR],
    ) -> Result<()> {
        let mut display_count = displays.len() as u32;
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_display_plane_supported_displays_khr;

        unsafe {
            (call)(
                self.handle,
                plane_index,
                &mut display_count,
                displays.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_display_plane_supported_displays`][`Self::get_display_plane_supported_displays`].
    #[inline]
    fn get_display_plane_supported_displays_len(&self, plane_index: u32) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_display_plane_supported_displays_khr)(
                self.handle,
                plane_index,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayModePropertiesKHR.html>
    ///
    /// Call [`get_display_mode_properties_len()`][`Self::get_display_mode_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_display_mode_properties(
        &self,
        display: DisplayKHR,
        properties: &mut [DisplayModePropertiesKHR],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_display_mode_properties_khr;

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

    /// Returns the required slice length for Call [`get_display_mode_properties`][`Self::get_display_mode_properties`].
    #[inline]
    fn get_display_mode_properties_len(&self, display: DisplayKHR) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_display
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_display_mode_properties_khr)(
                self.handle,
                display,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayModeKHR.html>
    #[inline]
    fn create_display_mode(
        &self,
        display: DisplayKHR,
        create_info: &DisplayModeCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DisplayModeKHR> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_display_mode_khr;

        unsafe {
            (call)(
                self.handle,
                display,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDisplayPlaneCapabilitiesKHR.html>
    #[inline]
    fn get_display_plane_capabilities(
        &self,
        mode: DisplayModeKHR,
        plane_index: u32,
    ) -> Result<DisplayPlaneCapabilitiesKHR> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_display_plane_capabilities_khr;

        unsafe { (call)(self.handle, mode, plane_index, out.as_mut_ptr()) }.init_on_success(out)
    }
}

pub trait DisplayInstance {
    fn create_display_plane_surface(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR>;
}

impl DisplayInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDisplayPlaneSurfaceKHR.html>
    #[inline]
    fn create_display_plane_surface(
        &self,
        create_info: &DisplaySurfaceCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SurfaceKHR> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .khr_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_display_plane_surface_khr;

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
