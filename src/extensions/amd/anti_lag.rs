// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_AMD_anti_lag";
pub const SPEC_VERSION: u32 = 1;

pub trait AntiLagDevice {
    fn anti_lag_update(&self, data: &AntiLagDataAMD);
}

impl AntiLagDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAntiLagUpdateAMD.html>
    #[inline]
    fn anti_lag_update(&self, data: &AntiLagDataAMD) {
        let call = self
            .fns()
            .amd_anti_lag
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .anti_lag_update_amd;

        unsafe { (call)(self.handle, data) };
    }
}
