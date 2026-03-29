// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_acceleration_structure";
pub const SPEC_VERSION: u32 = 13;

pub trait AccelerationStructureDevice {
    fn destroy_acceleration_structure(
        &self,
        acceleration_structure: AccelerationStructureKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn copy_acceleration_structure(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> Result<(), vkResult>;

    fn copy_acceleration_structure_to_memory(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result<(), vkResult>;

    fn copy_memory_to_acceleration_structure(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result<(), vkResult>;

    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [c_void],
        stride: usize,
    ) -> Result<(), vkResult>;

    fn get_device_acceleration_structure_compatibility(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR;

    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureKHR, vkResult>;

    fn build_acceleration_structures(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        pp_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> Result<(), vkResult>;

    fn get_acceleration_structure_device_address(
        &self,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress;

    fn get_acceleration_structure_build_sizes(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: Option<&u32>,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_>;
}

impl AccelerationStructureDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureKHR.html>
    #[inline]
    fn destroy_acceleration_structure(
        &self,
        acceleration_structure: AccelerationStructureKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .destroy_acceleration_structure_khr)(
                self.handle,
                acceleration_structure,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureKHR.html>
    #[inline]
    fn copy_acceleration_structure(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureInfoKHR,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .copy_acceleration_structure_khr)(self.handle, deferred_operation, info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    fn copy_acceleration_structure_to_memory(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .copy_acceleration_structure_to_memory_khr)(
                self.handle, deferred_operation, info
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    fn copy_memory_to_acceleration_structure(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .copy_memory_to_acceleration_structure_khr)(
                self.handle, deferred_operation, info
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [c_void],
        stride: usize,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .write_acceleration_structures_properties_khr)(
                self.handle,
                acceleration_structures.len() as u32,
                acceleration_structures.as_ptr(),
                query_type,
                data.len() as usize,
                data.as_mut_ptr(),
                stride,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    #[inline]
    fn get_device_acceleration_structure_compatibility(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .get_device_acceleration_structure_compatibility_khr)(
                self.handle,
                version_info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html>
    #[inline]
    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .create_acceleration_structure_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildAccelerationStructuresKHR.html>
    #[inline]
    fn build_acceleration_structures(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        pp_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> Result<(), vkResult> {
        assert_eq!(infos.len(), pp_build_range_infos.len());
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .build_acceleration_structures_khr)(
                self.handle,
                deferred_operation,
                infos.len() as u32,
                infos.as_ptr(),
                pp_build_range_infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html>
    #[inline]
    fn get_acceleration_structure_device_address(
        &self,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .get_acceleration_structure_device_address_khr)(self.handle, info)
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureBuildSizesKHR.html>
    #[inline]
    fn get_acceleration_structure_build_sizes(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &AccelerationStructureBuildGeometryInfoKHR,
        max_primitive_counts: Option<&u32>,
    ) -> AccelerationStructureBuildSizesInfoKHR<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .get_acceleration_structure_build_sizes_khr)(
                self.handle,
                build_type,
                build_info,
                max_primitive_counts.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }
}

pub trait AccelerationStructureCommandBuffer {
    fn copy_acceleration_structure(&self, info: &CopyAccelerationStructureInfoKHR);

    fn copy_acceleration_structure_to_memory(
        &self,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    );

    fn copy_memory_to_acceleration_structure(
        &self,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    );

    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );

    fn build_acceleration_structures(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        pp_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    );

    fn build_acceleration_structures_indirect(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[u32],
        pp_max_primitive_counts: &[*const u32],
    );
}

impl AccelerationStructureCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html>
    #[inline]
    fn copy_acceleration_structure(&self, info: &CopyAccelerationStructureInfoKHR) {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .copy_acceleration_structure_khr)(self.handle, info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    fn copy_acceleration_structure_to_memory(
        &self,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .copy_acceleration_structure_to_memory_khr)(self.handle, info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    fn copy_memory_to_acceleration_structure(
        &self,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .copy_memory_to_acceleration_structure_khr)(self.handle, info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .write_acceleration_structures_properties_khr)(
                self.handle,
                acceleration_structures.len() as u32,
                acceleration_structures.as_ptr(),
                query_type,
                query_pool,
                first_query,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresKHR.html>
    #[inline]
    fn build_acceleration_structures(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        pp_build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) {
        assert_eq!(infos.len(), pp_build_range_infos.len());
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .build_acceleration_structures_khr)(
                self.handle,
                infos.len() as u32,
                infos.as_ptr(),
                pp_build_range_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    #[inline]
    fn build_acceleration_structures_indirect(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[u32],
        pp_max_primitive_counts: &[*const u32],
    ) {
        assert_eq!(infos.len(), indirect_device_addresses.len());
        assert_eq!(infos.len(), indirect_strides.len());
        assert_eq!(infos.len(), pp_max_primitive_counts.len());
        unsafe {
            (self
                .fns()
                .khr_acceleration_structure
                .as_ref()
                .unwrap()
                .build_acceleration_structures_indirect_khr)(
                self.handle,
                infos.len() as u32,
                infos.as_ptr(),
                indirect_device_addresses.as_ptr(),
                indirect_strides.as_ptr(),
                pp_max_primitive_counts.as_ptr(),
            )
        };
    }
}
