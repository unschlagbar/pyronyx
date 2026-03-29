// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_QNX_external_memory_screen_buffer";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryScreenBufferDevice {
    fn get_screen_buffer_properties(
        &self,
        buffer: &_screen_buffer,
    ) -> Result<ScreenBufferPropertiesQNX<'_>, vkResult>;
}

impl ExternalMemoryScreenBufferDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetScreenBufferPropertiesQNX.html>
    #[inline]
    fn get_screen_buffer_properties(
        &self,
        buffer: &_screen_buffer,
    ) -> Result<ScreenBufferPropertiesQNX<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .qnx_external_memory_screen_buffer
                .as_ref()
                .unwrap()
                .get_screen_buffer_properties_qnx)(self.handle, buffer, out.as_mut_ptr())
        }
        .init_on_success(out)
    }
}
