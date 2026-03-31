// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_present_wait";
pub const SPEC_VERSION: u32 = 1;

pub trait PresentWaitDevice {
    fn wait_for_present(
        &self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> Result<(), Error>;
}

impl PresentWaitDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresentKHR.html>
    #[inline]
    fn wait_for_present(
        &self,
        swapchain: SwapchainKHR,
        present_id: u64,
        timeout: u64,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_present_wait
                .as_ref()
                .unwrap()
                .wait_for_present_khr)(self.handle, swapchain, present_id, timeout)
        }
        .result()
    }
}
