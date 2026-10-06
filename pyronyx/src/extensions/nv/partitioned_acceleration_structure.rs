// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_acceleration_structure`](crate::khr::acceleration_structure)
pub const NAME: &CStr = c"VK_NV_partitioned_acceleration_structure";
pub const SPEC_VERSION: u32 = 1;

pub trait PartitionedAccelerationStructureDevice {
    fn get_partitioned_acceleration_structures_build_sizes(
        &self,
        info: &PartitionedAccelerationStructureInstancesInputNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    );
}

impl PartitionedAccelerationStructureDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>
    #[inline]
    fn get_partitioned_acceleration_structures_build_sizes(
        &self,
        info: &PartitionedAccelerationStructureInstancesInputNV,
        size_info: &mut AccelerationStructureBuildSizesInfoKHR<'_>,
    ) {
        let call = self
            .fns()
            .nv_partitioned_acceleration_structure
            .get_partitioned_acceleration_structures_build_sizes_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, size_info) };
    }
}

pub trait PartitionedAccelerationStructureCommandBuffer {
    fn build_partitioned_acceleration_structures(
        &self,
        build_info: &BuildPartitionedAccelerationStructureInfoNV,
    );
}

impl PartitionedAccelerationStructureCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildPartitionedAccelerationStructuresNV.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn build_partitioned_acceleration_structures(
        &self,
        build_info: &BuildPartitionedAccelerationStructureInfoNV,
    ) {
        let call = self
            .fns()
            .nv_partitioned_acceleration_structure
            .build_partitioned_acceleration_structures_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, build_info) };
    }
}
