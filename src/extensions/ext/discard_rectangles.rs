// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_discard_rectangles";
pub const SPEC_VERSION: u32 = 2;

pub trait DiscardRectanglesCommandBuffer {
    fn set_discard_rectangle(&self, first_discard_rectangle: u32, discard_rectangles: &[Rect2D]);

    fn set_discard_rectangle_enable(&self, discard_rectangle_enable: bool);

    fn set_discard_rectangle_mode(&self, discard_rectangle_mode: DiscardRectangleModeEXT);
}

impl DiscardRectanglesCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_discard_rectangle(&self, first_discard_rectangle: u32, discard_rectangles: &[Rect2D]) {
        unsafe {
            (self
                .fns()
                .ext_discard_rectangles
                .as_ref()
                .unwrap()
                .set_discard_rectangle_ext)(
                self.handle,
                first_discard_rectangle,
                discard_rectangles.len() as u32,
                discard_rectangles.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleEnableEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_discard_rectangle_enable(&self, discard_rectangle_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_discard_rectangles
                .as_ref()
                .unwrap()
                .set_discard_rectangle_enable_ext)(
                self.handle, discard_rectangle_enable as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_discard_rectangle_mode(&self, discard_rectangle_mode: DiscardRectangleModeEXT) {
        unsafe {
            (self
                .fns()
                .ext_discard_rectangles
                .as_ref()
                .unwrap()
                .set_discard_rectangle_mode_ext)(self.handle, discard_rectangle_mode)
        };
    }
}
