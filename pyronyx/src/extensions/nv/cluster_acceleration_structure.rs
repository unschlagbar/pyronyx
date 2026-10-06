// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_acceleration_structure`](crate::khr::acceleration_structure)
pub const NAME: &CStr = c"VK_NV_cluster_acceleration_structure";
pub const SPEC_VERSION: u32 = 4;

pub trait ClusterAccelerationStructureDevice {
    fn get_cluster_acceleration_structure_build_sizes(
        &self,
        info: &ClusterAccelerationStructureInputInfoNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    );
}

impl ClusterAccelerationStructureDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html>
    #[inline]
    fn get_cluster_acceleration_structure_build_sizes(
        &self,
        info: &ClusterAccelerationStructureInputInfoNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        let call = self
            .fns()
            .nv_cluster_acceleration_structure
            .get_cluster_acceleration_structure_build_sizes_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, size_info) };
    }
}

pub trait ClusterAccelerationStructureCommandBuffer {
    fn build_cluster_acceleration_structure_indirect(
        &self,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    );
}

impl ClusterAccelerationStructureCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildClusterAccelerationStructureIndirectNV.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn build_cluster_acceleration_structure_indirect(
        &self,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    ) {
        let call = self
            .fns()
            .nv_cluster_acceleration_structure
            .build_cluster_acceleration_structure_indirect_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, command_infos) };
    }
}
