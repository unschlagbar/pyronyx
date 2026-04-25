// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
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
    ) -> Result<()>;

    fn copy_acceleration_structure_to_memory(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result<()>;

    fn copy_memory_to_acceleration_structure(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result<()>;

    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> Result<()>;

    fn get_acceleration_structure_compatibility(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR;

    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureKHR>;

    fn build_acceleration_structures(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> Result<()>;

    fn get_acceleration_structure_address(
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
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_acceleration_structure_khr;

        unsafe {
            (call)(
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
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_acceleration_structure_khr;

        unsafe { (call)(self.handle, deferred_operation, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyAccelerationStructureToMemoryKHR.html>
    #[inline]
    fn copy_acceleration_structure_to_memory(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_acceleration_structure_to_memory_khr;

        unsafe { (call)(self.handle, deferred_operation, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToAccelerationStructureKHR.html>
    #[inline]
    fn copy_memory_to_acceleration_structure(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_memory_to_acceleration_structure_khr;

        unsafe { (call)(self.handle, deferred_operation, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteAccelerationStructuresPropertiesKHR.html>
    #[inline]
    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        data: &mut [u8],
        stride: usize,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .write_acceleration_structures_properties_khr;

        unsafe {
            (call)(
                self.handle,
                acceleration_structures.len() as u32,
                acceleration_structures.as_ptr(),
                query_type,
                data.len() as usize,
                data.as_mut_ptr().cast(),
                stride,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceAccelerationStructureCompatibilityKHR.html>
    #[inline]
    fn get_acceleration_structure_compatibility(
        &self,
        version_info: &AccelerationStructureVersionInfoKHR,
    ) -> AccelerationStructureCompatibilityKHR {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_device_acceleration_structure_compatibility_khr;

        unsafe {
            (call)(self.handle, version_info, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureKHR.html>
    #[inline]
    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_acceleration_structure_khr;

        unsafe {
            (call)(
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
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) -> Result<()> {
        assert_eq!(infos.len(), build_range_infos.len());
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_acceleration_structures_khr;

        unsafe {
            (call)(
                self.handle,
                deferred_operation,
                infos.len() as u32,
                infos.as_ptr(),
                build_range_infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureDeviceAddressKHR.html>
    #[inline]
    fn get_acceleration_structure_address(
        &self,
        info: &AccelerationStructureDeviceAddressInfoKHR,
    ) -> DeviceAddress {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_acceleration_structure_device_address_khr;

        unsafe { (call)(self.handle, info) }
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
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_acceleration_structure_build_sizes_khr;

        unsafe {
            (call)(
                self.handle,
                build_type,
                build_info,
                max_primitive_counts.map_or(null(), from_ref),
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
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
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    );

    fn build_acceleration_structures_indirect(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[u32],
        max_primitive_counts: &[*const u32],
    );
}

impl AccelerationStructureCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_acceleration_structure(&self, info: &CopyAccelerationStructureInfoKHR) {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_acceleration_structure_khr;

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureToMemoryKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_acceleration_structure_to_memory(
        &self,
        info: &CopyAccelerationStructureToMemoryInfoKHR,
    ) {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_acceleration_structure_to_memory_khr;

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToAccelerationStructureKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_to_acceleration_structure(
        &self,
        info: &CopyMemoryToAccelerationStructureInfoKHR,
    ) {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_memory_to_acceleration_structure_khr;

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureKHR],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .write_acceleration_structures_properties_khr;

        unsafe {
            (call)(
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
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn build_acceleration_structures(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        build_range_infos: &[*const AccelerationStructureBuildRangeInfoKHR],
    ) {
        assert_eq!(infos.len(), build_range_infos.len());
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_acceleration_structures_khr;

        unsafe {
            (call)(
                self.handle,
                infos.len() as u32,
                infos.as_ptr(),
                build_range_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructuresIndirectKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn build_acceleration_structures_indirect(
        &self,
        infos: &[AccelerationStructureBuildGeometryInfoKHR],
        indirect_device_addresses: &[DeviceAddress],
        indirect_strides: &[u32],
        max_primitive_counts: &[*const u32],
    ) {
        assert_eq!(infos.len(), indirect_device_addresses.len());
        assert_eq!(infos.len(), indirect_strides.len());
        assert_eq!(infos.len(), max_primitive_counts.len());
        let call = self
            .fns()
            .khr_acceleration_structure
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_acceleration_structures_indirect_khr;

        unsafe {
            (call)(
                self.handle,
                infos.len() as u32,
                infos.as_ptr(),
                indirect_device_addresses.as_ptr(),
                indirect_strides.as_ptr(),
                max_primitive_counts.as_ptr(),
            )
        };
    }
}
