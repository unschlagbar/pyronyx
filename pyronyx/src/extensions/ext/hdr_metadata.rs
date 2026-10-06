// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_swapchain`](crate::khr::swapchain)
pub const NAME: &CStr = c"VK_EXT_hdr_metadata";
pub const SPEC_VERSION: u32 = 3;

pub trait HdrMetadataDevice {
    fn set_hdr_metadata(&self, swapchains: &[SwapchainKHR], metadata: &[HdrMetadataEXT]);
}

impl HdrMetadataDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetHdrMetadataEXT.html>
    #[inline]
    fn set_hdr_metadata(&self, swapchains: &[SwapchainKHR], metadata: &[HdrMetadataEXT]) {
        assert_eq!(swapchains.len(), metadata.len());
        let call = self
            .fns()
            .ext_hdr_metadata
            .set_hdr_metadata_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                swapchains.len() as u32,
                swapchains.as_ptr(),
                metadata.as_ptr(),
            )
        };
    }
}
