// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_local_dimming_amd;

        unsafe { (call)(self.handle, swap_chain, local_dimming_enable as _) };
    }
}
