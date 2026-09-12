// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_device_fault";
pub const SPEC_VERSION: u32 = 1;

pub trait DeviceFaultDevice {
    fn get_fault_reports(&self, timeout: u64) -> Result<Vec<DeviceFaultInfoKHR<'_>>>;

    fn get_fault_debug_info(&self) -> Result<DeviceFaultDebugInfoKHR<'_>>;
}

impl DeviceFaultDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html>
    #[inline]
    fn get_fault_reports(&self, timeout: u64) -> Result<Vec<DeviceFaultInfoKHR<'_>>> {
        let call = self
            .fns()
            .khr_device_fault
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_device_fault_reports_khr;

        read_into_vec_result(|count, data| unsafe { (call)(self.handle, timeout, count, data) })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html>
    #[inline]
    fn get_fault_debug_info(&self) -> Result<DeviceFaultDebugInfoKHR<'_>> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .khr_device_fault
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_device_fault_debug_info_khr;

        unsafe { (call)(self.handle, out.as_mut_ptr()) }.init_on_success(out)
    }
}
