// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_display_swapchain";
pub const SPEC_VERSION: u32 = 10;

pub trait DisplaySwapchainDevice {
    fn create_shared_swapchains(
        &self,
        create_infos: &[SwapchainCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        swapchains: &mut [SwapchainKHR],
    ) -> Result<(), Error>;
}

impl DisplaySwapchainDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSharedSwapchainsKHR.html>
    #[inline]
    fn create_shared_swapchains(
        &self,
        create_infos: &[SwapchainCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        swapchains: &mut [SwapchainKHR],
    ) -> Result<(), Error> {
        assert_eq!(create_infos.len(), swapchains.len());
        unsafe {
            (self
                .fns()
                .khr_display_swapchain
                .as_ref()
                .unwrap()
                .create_shared_swapchains_khr)(
                self.handle,
                create_infos.len() as u32,
                create_infos.as_ptr(),
                allocator.map_or(null(), from_ref),
                swapchains.as_mut_ptr(),
            )
        }
        .result()
    }
}
