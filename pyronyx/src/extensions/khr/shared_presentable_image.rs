// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_swapchain`](crate::khr::swapchain) + [`VK_KHR_get_surface_capabilities2`](crate::khr::get_surface_capabilities2) + ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1)
pub const NAME: &CStr = c"VK_KHR_shared_presentable_image";
pub const SPEC_VERSION: u32 = 1;

pub trait SharedPresentableImageDevice {
    fn get_swapchain_status(&self, swapchain: SwapchainKHR) -> Result<()>;
}

impl SharedPresentableImageDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html>
    #[inline]
    fn get_swapchain_status(&self, swapchain: SwapchainKHR) -> Result<()> {
        let call = self
            .fns()
            .khr_shared_presentable_image
            .get_swapchain_status_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain) }.result()
    }
}
