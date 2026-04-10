// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_hdr_metadata_ext;

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
