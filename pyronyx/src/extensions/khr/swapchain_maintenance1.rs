// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_swapchain`](crate::khr::swapchain) + [`VK_KHR_surface_maintenance1`](crate::khr::surface_maintenance1) + ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1)
pub const NAME: &CStr = c"VK_KHR_swapchain_maintenance1";
pub const SPEC_VERSION: u32 = 1;

pub trait SwapchainMaintenance1Device {
    fn release_swapchain_images(&self, release_info: &ReleaseSwapchainImagesInfoKHR) -> Result<()>;
}

impl SwapchainMaintenance1Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html>
    #[inline]
    fn release_swapchain_images(&self, release_info: &ReleaseSwapchainImagesInfoKHR) -> Result<()> {
        let call = self
            .fns()
            .khr_swapchain_maintenance1
            .release_swapchain_images_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, release_info) }.result()
    }
}
