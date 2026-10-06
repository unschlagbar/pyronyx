// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Promoted to [`VK_KHR_device_fault`](crate::khr::device_fault)
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_device_fault";
pub const SPEC_VERSION: u32 = 2;

pub trait DeviceFaultDevice {
    fn get_fault_info(
        &self,
        fault_counts: *mut DeviceFaultCountsEXT,
        fault_info: &mut DeviceFaultInfoEXT<'_>,
    ) -> Result<()>;
}

impl DeviceFaultDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html>
    #[inline]
    fn get_fault_info(
        &self,
        fault_counts: *mut DeviceFaultCountsEXT,
        fault_info: &mut DeviceFaultInfoEXT<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_device_fault
            .get_device_fault_info_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, fault_counts, fault_info) }.result()
    }
}
