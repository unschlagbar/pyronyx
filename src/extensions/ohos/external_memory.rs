// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_OHOS_external_memory";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryDevice {
    fn get_native_buffer_properties(
        &self,
        buffer: &OH_NativeBuffer,
    ) -> Result<NativeBufferPropertiesOHOS<'_>>;

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
    ) -> Result<NativeBufferPropertiesOHOS<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ohos_external_memory
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_native_buffer_properties_ohos;

        unsafe { (call)(self.handle, buffer, out.as_mut_ptr()) }.init_on_success(out)
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_native_buffer_ohos;

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
