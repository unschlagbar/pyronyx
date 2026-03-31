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
    ) -> Result<NativeBufferPropertiesOHOS<'_>, Error>;

    fn get_memory_native_buffer(
        &self,
        info: &MemoryGetNativeBufferInfoOHOS,
    ) -> Result<*mut OH_NativeBuffer, Error>;
}

impl ExternalMemoryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetNativeBufferPropertiesOHOS.html>
    #[inline]
    fn get_native_buffer_properties(
        &self,
        buffer: &OH_NativeBuffer,
    ) -> Result<NativeBufferPropertiesOHOS<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ohos_external_memory
                .as_ref()
                .unwrap()
                .get_native_buffer_properties_ohos)(
                self.handle, buffer, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryNativeBufferOHOS.html>
    #[inline]
    fn get_memory_native_buffer(
        &self,
        info: &MemoryGetNativeBufferInfoOHOS,
    ) -> Result<*mut OH_NativeBuffer, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ohos_external_memory
                .as_ref()
                .unwrap()
                .get_memory_native_buffer_ohos)(self.handle, info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }
}
