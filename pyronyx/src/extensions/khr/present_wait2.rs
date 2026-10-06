// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_surface_capabilities2`](crate::khr::get_surface_capabilities2) + [`VK_KHR_surface`](crate::khr::surface) + [`VK_KHR_swapchain`](crate::khr::swapchain) + [`VK_KHR_present_id2`](crate::khr::present_id2)
pub const NAME: &CStr = c"VK_KHR_present_wait2";
pub const SPEC_VERSION: u32 = 1;

pub trait PresentWait2Device {
    fn wait_for_present2(
        &self,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR,
    ) -> Result<()>;
}

impl PresentWait2Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html>
    #[inline]
    fn wait_for_present2(
        &self,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_present_wait2
            .wait_for_present2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, present_wait2_info) }.result()
    }
}
