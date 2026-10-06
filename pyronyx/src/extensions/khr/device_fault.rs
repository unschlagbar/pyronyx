// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_KHR_device_fault";
pub const SPEC_VERSION: u32 = 1;

pub trait DeviceFaultDevice {
    fn get_fault_reports(&self, timeout: u64, fault_info: &mut [DeviceFaultInfoKHR]) -> Result<()>;
    fn get_fault_reports_len(&self, timeout: u64) -> Result<usize>;

    fn get_fault_debug_info(&self, debug_info: &mut DeviceFaultDebugInfoKHR<'_>) -> Result<()>;
}

impl DeviceFaultDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultReportsKHR.html>
    ///
    /// Call [`get_fault_reports_len()`][`Self::get_fault_reports_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_fault_reports(&self, timeout: u64, fault_info: &mut [DeviceFaultInfoKHR]) -> Result<()> {
        let mut fault_counts = fault_info.len() as u32;
        let call = self
            .fns()
            .khr_device_fault
            .get_device_fault_reports_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                timeout,
                &mut fault_counts,
                fault_info.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_fault_reports`][`Self::get_fault_reports`].
    #[inline]
    fn get_fault_reports_len(&self, timeout: u64) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_device_fault
                .get_device_fault_reports_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                timeout,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceFaultDebugInfoKHR.html>
    #[inline]
    fn get_fault_debug_info(&self, debug_info: &mut DeviceFaultDebugInfoKHR<'_>) -> Result<()> {
        let call = self
            .fns()
            .khr_device_fault
            .get_device_fault_debug_info_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, debug_info) }.result()
    }
}
