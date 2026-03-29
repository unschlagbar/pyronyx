// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_ANDROID_external_memory_android_hardware_buffer";
pub const SPEC_VERSION: u32 = 5;

pub trait ExternalMemoryAndroidHardwareBufferDevice {
    fn get_android_hardware_buffer_properties(
        &self,
        buffer: &AHardwareBuffer,
    ) -> Result<AndroidHardwareBufferPropertiesANDROID<'_>, vkResult>;

    fn get_memory_android_hardware_buffer(
        &self,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> Result<*mut AHardwareBuffer, vkResult>;
}

impl ExternalMemoryAndroidHardwareBufferDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAndroidHardwareBufferPropertiesANDROID.html>
    #[inline]
    fn get_android_hardware_buffer_properties(
        &self,
        buffer: &AHardwareBuffer,
    ) -> Result<AndroidHardwareBufferPropertiesANDROID<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .android_external_memory_android_hardware_buffer
                .as_ref()
                .unwrap()
                .get_android_hardware_buffer_properties_android)(
                self.handle,
                buffer,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryAndroidHardwareBufferANDROID.html>
    #[inline]
    fn get_memory_android_hardware_buffer(
        &self,
        info: &MemoryGetAndroidHardwareBufferInfoANDROID,
    ) -> Result<*mut AHardwareBuffer, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .android_external_memory_android_hardware_buffer
                .as_ref()
                .unwrap()
                .get_memory_android_hardware_buffer_android)(
                self.handle, info, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}
