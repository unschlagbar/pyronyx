// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_color_write_enable";
pub const SPEC_VERSION: u32 = 1;

pub trait ColorWriteEnableCommandBuffer {
    fn set_color_write_enable(&self, color_write_enables: &[Bool32]);
}

impl ColorWriteEnableCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteEnableEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_color_write_enable(&self, color_write_enables: &[Bool32]) {
        let call = self
            .fns()
            .ext_color_write_enable
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_color_write_enable_ext;

        unsafe {
            (call)(
                self.handle,
                color_write_enables.len() as u32,
                color_write_enables.as_ptr(),
            )
        };
    }
}
