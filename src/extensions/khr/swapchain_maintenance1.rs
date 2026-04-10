// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_swapchain_maintenance1";
pub const SPEC_VERSION: u32 = 1;

pub trait SwapchainMaintenance1Device {
    fn release_swapchain_images(
        &self,
        release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> Result<(), Error>;
}

impl SwapchainMaintenance1Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseSwapchainImagesKHR.html>
    #[inline]
    fn release_swapchain_images(
        &self,
        release_info: &ReleaseSwapchainImagesInfoKHR,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .khr_swapchain_maintenance1
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .release_swapchain_images_khr;

        unsafe { (call)(self.handle, release_info) }.result()
    }
}
