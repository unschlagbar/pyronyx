// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

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
        unsafe {
            (self
                .fns()
                .nv_partitioned_acceleration_structure
                .as_ref()
                .unwrap()
                .get_partitioned_acceleration_structures_build_sizes_nv)(
                self.handle,
                info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
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
    #[inline]
    fn build_partitioned_acceleration_structures(
        &self,
        build_info: &BuildPartitionedAccelerationStructureInfoNV,
    ) {
        unsafe {
            (self
                .fns()
                .nv_partitioned_acceleration_structure
                .as_ref()
                .unwrap()
                .build_partitioned_acceleration_structures_nv)(self.handle, build_info)
        };
    }
}
