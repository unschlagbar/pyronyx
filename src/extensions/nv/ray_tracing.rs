// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_KHR_ray_tracing_pipeline` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_ray_tracing";
pub const SPEC_VERSION: u32 = 3;

pub trait RayTracingDevice {
    fn compile_deferred(&self, pipeline: Pipeline, shader: u32) -> Result<()>;

    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureNV>;

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
    ) -> Result<()>;

    fn get_acceleration_structure_handle(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [c_void],
    ) -> Result<()>;

    fn create_ray_tracing_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoNV],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<()>;
}

impl RayTracingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCompileDeferredNV.html>
    #[inline]
    fn compile_deferred(&self, pipeline: Pipeline, shader: u32) -> Result<()> {
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .compile_deferred_nv;

        unsafe { (call)(self.handle, pipeline, shader) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructureNV.html>
    #[inline]
    fn create_acceleration_structure(
        &self,
        create_info: &AccelerationStructureCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureNV> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_acceleration_structure_nv;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyAccelerationStructureNV.html>
    #[inline]
    fn destroy_acceleration_structure(
        &self,
        acceleration_structure: AccelerationStructureNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_acceleration_structure_nv;

        unsafe {
            (call)(
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
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_acceleration_structure_memory_requirements_nv;

        unsafe {
            (call)(self.handle, info, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindAccelerationStructureMemoryNV.html>
    #[inline]
    fn bind_acceleration_structure_memory(
        &self,
        bind_infos: &[BindAccelerationStructureMemoryInfoNV],
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .bind_acceleration_structure_memory_nv;

        unsafe { (call)(self.handle, bind_infos.len() as u32, bind_infos.as_ptr()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureHandleNV.html>
    #[inline]
    fn get_acceleration_structure_handle(
        &self,
        acceleration_structure: AccelerationStructureNV,
        data: &mut [c_void],
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_acceleration_structure_handle_nv;

        unsafe {
            (call)(
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
    ) -> Result<()> {
        assert_eq!(create_infos.len(), pipelines.len());
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_ray_tracing_pipelines_nv;

        unsafe {
            (call)(
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
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_acceleration_structure(
        &self,
        dst: AccelerationStructureNV,
        src: AccelerationStructureNV,
        mode: CopyAccelerationStructureModeKHR,
    ) {
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_acceleration_structure_nv;

        unsafe { (call)(self.handle, dst, src, mode) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteAccelerationStructuresPropertiesNV.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn write_acceleration_structures_properties(
        &self,
        acceleration_structures: &[AccelerationStructureNV],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .write_acceleration_structures_properties_nv;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildAccelerationStructureNV.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
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
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_acceleration_structure_nv;

        unsafe {
            (call)(
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
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
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
        let call = self
            .fns()
            .nv_ray_tracing
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .trace_rays_nv;

        unsafe {
            (call)(
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
