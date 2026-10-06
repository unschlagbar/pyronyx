// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_get_surface_capabilities2`](crate::khr::get_surface_capabilities2) + [`VK_KHR_swapchain`](crate::khr::swapchain)
pub const NAME: &CStr = c"VK_AMD_display_native_hdr";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplayNativeHdrDevice {
    fn set_local_dimming(&self, swap_chain: SwapchainKHR, local_dimming_enable: bool);
}

impl DisplayNativeHdrDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetLocalDimmingAMD.html>
    #[inline]
    fn set_local_dimming(&self, swap_chain: SwapchainKHR, local_dimming_enable: bool) {
        let call = self
            .fns()
            .amd_display_native_hdr
            .set_local_dimming_amd
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swap_chain, local_dimming_enable as _) };
    }
}
