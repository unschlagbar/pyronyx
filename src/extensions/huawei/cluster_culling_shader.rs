// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_HUAWEI_cluster_culling_shader";
pub const SPEC_VERSION: u32 = 3;

pub trait ClusterCullingShaderCommandBuffer {
    fn draw_cluster(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32);

    fn draw_cluster_indirect(&self, buffer: Buffer, offset: DeviceSize);
}

impl ClusterCullingShaderCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterHUAWEI.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Action`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_cluster(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            (self
                .fns()
                .huawei_cluster_culling_shader
                .as_ref()
                .unwrap()
                .draw_cluster_huawei)(
                self.handle, group_count_x, group_count_y, group_count_z
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Action`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_cluster_indirect(&self, buffer: Buffer, offset: DeviceSize) {
        unsafe {
            (self
                .fns()
                .huawei_cluster_culling_shader
                .as_ref()
                .unwrap()
                .draw_cluster_indirect_huawei)(self.handle, buffer, offset)
        };
    }
}
