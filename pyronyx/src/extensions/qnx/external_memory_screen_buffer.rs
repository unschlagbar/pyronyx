// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: (([`VK_KHR_sampler_ycbcr_conversion`](crate::khr::sampler_ycbcr_conversion) + [`VK_KHR_external_memory`](crate::khr::external_memory) + [`VK_KHR_dedicated_allocation`](crate::khr::dedicated_allocation)) or Vulkan 1.1) + [`VK_EXT_queue_family_foreign`](crate::ext::queue_family_foreign)
pub const NAME: &CStr = c"VK_QNX_external_memory_screen_buffer";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryScreenBufferDevice {
    fn get_screen_buffer_properties(
        &self,
        buffer: &_screen_buffer,
        properties: &mut ScreenBufferPropertiesQNX<'_>,
    ) -> Result<()>;
}

impl ExternalMemoryScreenBufferDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html>
    #[inline]
    fn get_screen_buffer_properties(
        &self,
        buffer: &_screen_buffer,
        properties: &mut ScreenBufferPropertiesQNX<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .qnx_external_memory_screen_buffer
            .get_screen_buffer_properties_qnx
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, buffer, properties) }.result()
    }
}
