// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_EXT_color_write_enable";
pub const SPEC_VERSION: u32 = 1;

pub trait ColorWriteEnableCommandBuffer {
    fn set_color_write_enable(&self, color_write_enables: &[Bool32]);
}

impl ColorWriteEnableCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html>
    #[inline]
    fn set_color_write_enable(&self, color_write_enables: &[Bool32]) {
        unsafe {
            (self
                .fns()
                .ext_color_write_enable
                .as_ref()
                .unwrap()
                .set_color_write_enable_ext)(
                self.handle,
                color_write_enables.len() as u32,
                color_write_enables.as_ptr(),
            )
        };
    }
}
