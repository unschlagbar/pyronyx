// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_device_fault";
pub const SPEC_VERSION: u32 = 2;

pub trait DeviceFaultDevice {
    fn get_device_fault_info(
        &self,
        fault_counts: *mut DeviceFaultCountsEXT,
    ) -> Result<DeviceFaultInfoEXT<'_>, Error>;
}

impl DeviceFaultDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html>
    #[inline]
    fn get_device_fault_info(
        &self,
        fault_counts: *mut DeviceFaultCountsEXT,
    ) -> Result<DeviceFaultInfoEXT<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_device_fault
                .as_ref()
                .unwrap()
                .get_device_fault_info_ext)(self.handle, fault_counts, out.as_mut_ptr())
        }
        .init_on_success(out)
    }
}
