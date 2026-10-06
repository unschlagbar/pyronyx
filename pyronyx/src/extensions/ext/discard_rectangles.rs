// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
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
        let call = self
            .fns()
            .ext_discard_rectangles
            .set_discard_rectangle_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
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
        let call = self
            .fns()
            .ext_discard_rectangles
            .set_discard_rectangle_enable_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, discard_rectangle_enable as _) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDiscardRectangleModeEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_discard_rectangle_mode(&self, discard_rectangle_mode: DiscardRectangleModeEXT) {
        let call = self
            .fns()
            .ext_discard_rectangles
            .set_discard_rectangle_mode_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, discard_rectangle_mode) };
    }
}
