// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_partitioned_acceleration_structure";
pub const SPEC_VERSION: u32 = 1;

pub trait PartitionedAccelerationStructureDevice {
    fn get_partitioned_acceleration_structures_build_sizes(
        &self,
        info: &PartitionedAccelerationStructureInstancesInputNV,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_>;
}

impl PartitionedAccelerationStructureDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPartitionedAccelerationStructuresBuildSizesNV.html>
    #[inline]
    fn get_partitioned_acceleration_structures_build_sizes(
        &self,
        info: &PartitionedAccelerationStructureInstancesInputNV,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_partitioned_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_partitioned_acceleration_structures_build_sizes_nv;

        unsafe {
            (call)(self.handle, info, out.as_mut_ptr());
            out.assume_init()
        }
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_partitioned_acceleration_structures_nv;

        unsafe { (call)(self.handle, build_info) };
    }
}
