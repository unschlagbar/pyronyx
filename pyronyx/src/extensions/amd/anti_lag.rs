// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
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
            .anti_lag_update_amd
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, data) };
    }
}
