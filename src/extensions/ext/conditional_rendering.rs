// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_EXT_conditional_rendering";
pub const SPEC_VERSION: u32 = 2;

pub trait ConditionalRenderingCommandBuffer {
    fn begin_conditional_rendering(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    );

    fn end_conditional_rendering(&self);
}

impl ConditionalRenderingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRenderingEXT.html>
    #[inline]
    fn begin_conditional_rendering(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfoEXT,
    ) {
        unsafe {
            (self
                .fns()
                .ext_conditional_rendering
                .as_ref()
                .unwrap()
                .begin_conditional_rendering_ext)(
                self.handle, conditional_rendering_begin
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndConditionalRenderingEXT.html>
    #[inline]
    fn end_conditional_rendering(&self) {
        unsafe {
            (self
                .fns()
                .ext_conditional_rendering
                .as_ref()
                .unwrap()
                .end_conditional_rendering_ext)(self.handle)
        };
    }
}
