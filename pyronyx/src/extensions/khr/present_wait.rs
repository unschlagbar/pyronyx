// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_swapchain`](crate::khr::swapchain) + [`VK_KHR_present_id`](crate::khr::present_id)
pub const NAME: &CStr = c"VK_KHR_present_wait";
pub const SPEC_VERSION: u32 = 1;

pub trait PresentWaitDevice {
    fn wait_for_present(
        &self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> Result<()>;
}

impl PresentWaitDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html>
    #[inline]
    fn wait_for_present(
        &self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_present_wait
            .wait_for_present_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, present_id, timeout) }.result()
    }
}
