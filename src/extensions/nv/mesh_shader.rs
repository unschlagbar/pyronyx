// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

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
    #[inline]
    fn draw_mesh_tasks(&self, task_count: u32, first_task: u32) {
        unsafe {
            (self
                .fns()
                .nv_mesh_shader
                .as_ref()
                .unwrap()
                .draw_mesh_tasks_nv)(self.handle, task_count, first_task)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectNV.html>
    #[inline]
    fn draw_mesh_tasks_indirect(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self
                .fns()
                .nv_mesh_shader
                .as_ref()
                .unwrap()
                .draw_mesh_tasks_indirect_nv)(
                self.handle, buffer, offset, draw_count, stride
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCountNV.html>
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
        unsafe {
            (self
                .fns()
                .nv_mesh_shader
                .as_ref()
                .unwrap()
                .draw_mesh_tasks_indirect_count_nv)(
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
