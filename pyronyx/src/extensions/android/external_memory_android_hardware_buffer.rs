// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: (([`VK_KHR_sampler_ycbcr_conversion`](crate::khr::sampler_ycbcr_conversion) + [`VK_KHR_external_memory`](crate::khr::external_memory) + [`VK_KHR_dedicated_allocation`](crate::khr::dedicated_allocation)) or Vulkan 1.1) + [`VK_EXT_queue_family_foreign`](crate::ext::queue_family_foreign)
pub const NAME: &CStr = c"VK_ANDROID_external_memory_android_hardware_buffer";
pub const SPEC_VERSION: u32 = 5;

pub trait ExternalMemoryAndroidHardwareBufferDevice {
    fn get_android_hardware_buffer_properties(
        &self,
        buffer: &AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> Result<()>;

    fn get_memory_android_hardware_buffer(
        &self,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> Result<*mut AHardwareBuffer>;
}

impl ExternalMemoryAndroidHardwareBufferDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    #[inline]
    fn get_android_hardware_buffer_properties(
        &self,
        buffer: &AHardwareBuffer,
        properties: &mut AndroidHardwareBufferPropertiesANDROID<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .android_external_memory_android_hardware_buffer
            .get_android_hardware_buffer_properties_android
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, buffer, properties) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html>
    #[inline]
    fn get_memory_android_hardware_buffer(
        &self,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> Result<*mut AHardwareBuffer> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .android_external_memory_android_hardware_buffer
            .get_memory_android_hardware_buffer_android
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
