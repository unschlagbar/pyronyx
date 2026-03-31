// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_swapchain";
pub const SPEC_VERSION: u32 = 70;

pub trait SwapchainDevice {
    fn create_swapchain(
        &self,
        create_info: &SwapchainCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SwapchainKHR, Error>;

    fn destroy_swapchain(&self, swapchain: SwapchainKHR, allocator: Option<&AllocationCallbacks>);

    fn get_swapchain_images(&self, swapchain: SwapchainKHR) -> Result<Vec<Image>, Error>;

    fn acquire_next_image(
        &self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
    ) -> Result<u32, Error>;
}

impl SwapchainDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSwapchainKHR.html>
    #[inline]
    fn create_swapchain(
        &self,
        create_info: &SwapchainCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SwapchainKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_swapchain
                .as_ref()
                .unwrap()
                .create_swapchain_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySwapchainKHR.html>
    #[inline]
    fn destroy_swapchain(&self, swapchain: SwapchainKHR, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self
                .fns()
                .khr_swapchain
                .as_ref()
                .unwrap()
                .destroy_swapchain_khr)(
                self.handle, swapchain, allocator.map_or(null(), from_ref)
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainImagesKHR.html>
    #[inline]
    fn get_swapchain_images(&self, swapchain: SwapchainKHR) -> Result<Vec<Image>, Error> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .khr_swapchain
                .as_ref()
                .unwrap()
                .get_swapchain_images_khr)(self.handle, swapchain, count, data)
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireNextImageKHR.html>
    #[inline]
    fn acquire_next_image(
        &self,
        swapchain: SwapchainKHR,
        timeout: u64,
        semaphore: Semaphore,
        fence: Fence,
    ) -> Result<u32, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_swapchain
                .as_ref()
                .unwrap()
                .acquire_next_image_khr)(
                self.handle,
                swapchain,
                timeout,
                semaphore,
                fence,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait SwapchainQueue {
    fn present(&self, present_info: &PresentInfoKHR) -> Result<(), Error>;
}

impl SwapchainQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueuePresentKHR.html>
    #[inline]
    fn present(&self, present_info: &PresentInfoKHR) -> Result<(), Error> {
        unsafe {
            (self.fns().khr_swapchain.as_ref().unwrap().queue_present_khr)(
                self.handle,
                present_info,
            )
        }
        .result()
    }
}
