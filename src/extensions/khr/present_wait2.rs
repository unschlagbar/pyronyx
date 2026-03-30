// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_KHR_present_wait2";
pub const SPEC_VERSION: u32 = 1;

pub trait PresentWait2Device {
    fn wait_for_present2(
        &self,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR,
    ) -> Result<(), Error>;
}

impl PresentWait2Device for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForPresent2KHR.html>
    #[inline]
    fn wait_for_present2(
        &self,
        swapchain: SwapchainKHR,
        present_wait2_info: &PresentWait2InfoKHR,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_present_wait2
                .as_ref()
                .unwrap()
                .wait_for_present2_khr)(self.handle, swapchain, present_wait2_info)
        }
        .result()
    }
}
