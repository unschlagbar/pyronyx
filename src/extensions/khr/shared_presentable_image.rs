// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_shared_presentable_image";
pub const SPEC_VERSION: u32 = 1;

pub trait SharedPresentableImageDevice {
    fn get_swapchain_status(&self, swapchain: SwapchainKHR) -> Result<(), Error>;
}

impl SharedPresentableImageDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainStatusKHR.html>
    #[inline]
    fn get_swapchain_status(&self, swapchain: SwapchainKHR) -> Result<(), Error> {
        let call = self
            .fns()
            .khr_shared_presentable_image
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_swapchain_status_khr;

        unsafe { (call)(self.handle, swapchain) }.result()
    }
}
