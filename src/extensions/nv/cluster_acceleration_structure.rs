// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_NV_cluster_acceleration_structure";
pub const SPEC_VERSION: u32 = 4;

pub trait ClusterAccelerationStructureDevice {
    fn get_cluster_acceleration_structure_build_sizes(
        &self,
        info: &ClusterAccelerationStructureInputInfoNV,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_>;
}

impl ClusterAccelerationStructureDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetClusterAccelerationStructureBuildSizesNV.html>
    #[inline]
    fn get_cluster_acceleration_structure_build_sizes(
        &self,
        info: &ClusterAccelerationStructureInputInfoNV,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_cluster_acceleration_structure
                .as_ref()
                .unwrap()
                .get_cluster_acceleration_structure_build_sizes_nv)(
                self.handle,
                info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
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
    #[inline]
    fn build_cluster_acceleration_structure_indirect(
        &self,
        command_infos: &ClusterAccelerationStructureCommandsInfoNV,
    ) {
        unsafe {
            (self
                .fns()
                .nv_cluster_acceleration_structure
                .as_ref()
                .unwrap()
                .build_cluster_acceleration_structure_indirect_nv)(
                self.handle, command_infos
            )
        };
    }
}
