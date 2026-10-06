// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_multi_draw";
pub const SPEC_VERSION: u32 = 1;

pub trait MultiDrawCommandBuffer {
    fn draw_multi(
        &self,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    );

    fn draw_multi_indexed(
        &self,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    );
}

impl MultiDrawCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_multi(
        &self,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        let call = self
            .fns()
            .ext_multi_draw
            .draw_multi_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                vertex_info.len() as u32,
                vertex_info.as_ptr(),
                instance_count,
                first_instance,
                stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_multi_indexed(
        &self,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    ) {
        let call = self
            .fns()
            .ext_multi_draw
            .draw_multi_indexed_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                index_info.len() as u32,
                index_info.as_ptr(),
                instance_count,
                first_instance,
                stride,
                vertex_offset.map_or(null(), from_ref),
            )
        };
    }
}
