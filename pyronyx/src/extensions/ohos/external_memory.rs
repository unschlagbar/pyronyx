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
pub const NAME: &CStr = c"VK_OHOS_external_memory";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryDevice {
    fn get_native_buffer_properties(
        &self,
        buffer: &OH_NativeBuffer,
        properties: &mut NativeBufferPropertiesOHOS<'_>,
    ) -> Result<()>;

    fn get_memory_native_buffer(
        &self,
        info: &MemoryGetNativeBufferInfoOHOS,
    ) -> Result<*mut OH_NativeBuffer>;
}

impl ExternalMemoryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html>
    #[inline]
    fn get_native_buffer_properties(
        &self,
        buffer: &OH_NativeBuffer,
        properties: &mut NativeBufferPropertiesOHOS<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .ohos_external_memory
            .get_native_buffer_properties_ohos
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, buffer, properties) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html>
    #[inline]
    fn get_memory_native_buffer(
        &self,
        info: &MemoryGetNativeBufferInfoOHOS,
    ) -> Result<*mut OH_NativeBuffer> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ohos_external_memory
            .get_memory_native_buffer_ohos
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
