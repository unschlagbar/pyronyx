// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
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
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_cluster(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        let call = self
            .fns()
            .huawei_cluster_culling_shader
            .draw_cluster_huawei
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, group_count_x, group_count_y, group_count_z) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawClusterIndirectHUAWEI.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_cluster_indirect(&self, buffer: Buffer, offset: DeviceSize) {
        let call = self
            .fns()
            .huawei_cluster_culling_shader
            .draw_cluster_indirect_huawei
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, buffer, offset) };
    }
}
