// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_mesh_shader";
pub const SPEC_VERSION: u32 = 1;

pub trait MeshShaderCommandBuffer {
    fn draw_mesh_tasks(&self, task_count: u32, first_task: u32);

    fn draw_mesh_tasks_indirect(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    );

    fn draw_mesh_tasks_indirect_count(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    );
}

impl MeshShaderCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksNV.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_mesh_tasks(&self, task_count: u32, first_task: u32) {
        let call = self
            .fns()
            .nv_mesh_shader
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .draw_mesh_tasks_nv;

        unsafe { (call)(self.handle, task_count, first_task) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_mesh_tasks_indirect(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        let call = self
            .fns()
            .nv_mesh_shader
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .draw_mesh_tasks_indirect_nv;

        unsafe { (call)(self.handle, buffer, offset, draw_count, stride) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_mesh_tasks_indirect_count(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        let call = self
            .fns()
            .nv_mesh_shader
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .draw_mesh_tasks_indirect_count_nv;

        unsafe {
            (call)(
                self.handle,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        };
    }
}
