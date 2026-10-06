// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_QCOM_queue_perf_hint";
pub const SPEC_VERSION: u32 = 1;

pub trait QueuePerfHintQueue {
    fn set_perf_hint(&self, perf_hint_info: &PerfHintInfoQCOM) -> Result<()>;
}

impl QueuePerfHintQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerfHintQCOM.html>
    #[inline]
    fn set_perf_hint(&self, perf_hint_info: &PerfHintInfoQCOM) -> Result<()> {
        let call = self
            .fns()
            .qcom_queue_perf_hint
            .queue_set_perf_hint_qcom
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, perf_hint_info) }.result()
    }
}
