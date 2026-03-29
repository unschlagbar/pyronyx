// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

#[deprecated = "This extension is deprecated. Use `VK_KHR_ray_tracing_pipeline` instead."]
pub const NAME: &CStr = c"VK_NV_ray_tracing";
pub const SPEC_VERSION: u32 = 3;

pub trait RayTracingDevice {
    fn compile_deferred(&self, pipeline: Pipeline, shader: u32) -> Result<(), vkResult>;

    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureNV, vkResult>;

    fn destroy_acceleration_structure(
        &self,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_acceleration_structure_memory_requirements(
        &self,
        info: &AccelerationStructureMemoryRequirementsInfoNV,
    ) -> MemoryRequirements2KHR<'_>;

    fn bind_acceleration_structure_memory(
        &self,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> Result<(), vkResult>;

    fn get_acceleration_structure_handle(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [c_void],
    ) -> Result<(), vkResult>;

    fn create_ray_tracing_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), vkResult>;
}

impl RayTracingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html>
    #[inline]
    fn compile_deferred(&self, pipeline: Pipeline, shader: u32) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .compile_deferred_nv)(self.handle, pipeline, shader)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html>
    #[inline]
    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureNV, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .create_acceleration_structure_nv)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html>
    #[inline]
    fn destroy_acceleration_structure(
        &self,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .destroy_acceleration_structure_nv)(
                self.handle,
                acceleration_structure,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureMemoryRequirementsNV.html>
    #[inline]
    fn get_acceleration_structure_memory_requirements(
        &self,
        info: &AccelerationStructureMemoryRequirementsInfoNV,
    ) -> MemoryRequirements2KHR<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .get_acceleration_structure_memory_requirements_nv)(
                self.handle,
                info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html>
    #[inline]
    fn bind_acceleration_structure_memory(
        &self,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .bind_acceleration_structure_memory_nv)(
                self.handle,
                bind_infos.len() as u32,
                bind_infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html>
    #[inline]
    fn get_acceleration_structure_handle(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [c_void],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .get_acceleration_structure_handle_nv)(
                self.handle,
                acceleration_structure,
                data.len() as usize,
                data.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesNV.html>
    #[inline]
    fn create_ray_tracing_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), vkResult> {
        assert_eq!(create_infos.len(), pipelines.len());
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .create_ray_tracing_pipelines_nv)(
                self.handle,
                pipeline_cache,
                create_infos.len() as u32,
                create_infos.as_ptr(),
                allocator.map_or(null(), from_ref),
                pipelines.as_mut_ptr(),
            )
        }
        .result()
    }
}

pub trait RayTracingCommandBuffer {
    fn copy_acceleration_structure(
        &self,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    );

    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );

    fn build_acceleration_structure(
        &self,
        info: &AccelerationStructureInfoNV,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: bool,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    );

    fn trace_rays(
        &self,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    );
}

impl RayTracingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyAccelerationStructureNV.html>
    #[inline]
    fn copy_acceleration_structure(
        &self,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .copy_acceleration_structure_nv)(self.handle, dst, src, mode)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    #[inline]
    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .write_acceleration_structures_properties_nv)(
                self.handle,
                acceleration_structures.len() as u32,
                acceleration_structures.as_ptr(),
                query_type,
                query_pool,
                first_query,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html>
    #[inline]
    fn build_acceleration_structure(
        &self,
        info: &AccelerationStructureInfoNV,
        instance_data: Buffer,
        instance_offset: DeviceSize,
        update: bool,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        scratch: Buffer,
        scratch_offset: DeviceSize,
    ) {
        unsafe {
            (self
                .fns()
                .nv_ray_tracing
                .as_ref()
                .unwrap()
                .build_acceleration_structure_nv)(
                self.handle,
                info,
                instance_data,
                instance_offset,
                update as _,
                dst,
                src,
                scratch,
                scratch_offset,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysNV.html>
    #[inline]
    fn trace_rays(
        &self,
        raygen_shader_binding_table_buffer: Buffer,
        raygen_shader_binding_offset: DeviceSize,
        miss_shader_binding_table_buffer: Buffer,
        miss_shader_binding_offset: DeviceSize,
        miss_shader_binding_stride: DeviceSize,
        hit_shader_binding_table_buffer: Buffer,
        hit_shader_binding_offset: DeviceSize,
        hit_shader_binding_stride: DeviceSize,
        callable_shader_binding_table_buffer: Buffer,
        callable_shader_binding_offset: DeviceSize,
        callable_shader_binding_stride: DeviceSize,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        unsafe {
            (self.fns().nv_ray_tracing.as_ref().unwrap().trace_rays_nv)(
                self.handle,
                raygen_shader_binding_table_buffer,
                raygen_shader_binding_offset,
                miss_shader_binding_table_buffer,
                miss_shader_binding_offset,
                miss_shader_binding_stride,
                hit_shader_binding_table_buffer,
                hit_shader_binding_offset,
                hit_shader_binding_stride,
                callable_shader_binding_table_buffer,
                callable_shader_binding_offset,
                callable_shader_binding_stride,
                width,
                height,
                depth,
            )
        };
    }
}
