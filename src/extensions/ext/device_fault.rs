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
    fn get_fault_info(
        &self,
        fault_counts: *mut DeviceFaultCountsEXT,
    ) -> Result<DeviceFaultInfoEXT<'_>>;
}

impl DeviceFaultDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultInfoEXT.html>
    #[inline]
    fn get_fault_info(
        &self,
        fault_counts: *mut DeviceFaultCountsEXT,
    ) -> Result<DeviceFaultInfoEXT<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_device_fault
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_device_fault_info_ext;

        unsafe { (call)(self.handle, fault_counts, out.as_mut_ptr()) }.init_on_success(out)
    }
}
