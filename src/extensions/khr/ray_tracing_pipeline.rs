// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_ray_tracing_pipeline";
pub const SPEC_VERSION: u32 = 1;

pub trait RayTracingPipelineCommandBuffer {
    fn trace_rays(
        &self,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    );

    fn trace_rays_indirect(
        &self,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    );

    fn set_ray_tracing_pipeline_stack_size(&self, pipeline_stack_size: u32);
}

impl RayTracingPipelineCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn trace_rays(
        &self,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        width: u32,
        height: u32,
        depth: u32,
    ) {
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .trace_rays_khr;

        unsafe {
            (call)(
                self.handle,
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                width,
                height,
                depth,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirectKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn trace_rays_indirect(
        &self,
        raygen_shader_binding_table: &StridedDeviceAddressRegionKHR,
        miss_shader_binding_table: &StridedDeviceAddressRegionKHR,
        hit_shader_binding_table: &StridedDeviceAddressRegionKHR,
        callable_shader_binding_table: &StridedDeviceAddressRegionKHR,
        indirect_device_address: DeviceAddress,
    ) {
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .trace_rays_indirect_khr;

        unsafe {
            (call)(
                self.handle,
                raygen_shader_binding_table,
                miss_shader_binding_table,
                hit_shader_binding_table,
                callable_shader_binding_table,
                indirect_device_address,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRayTracingPipelineStackSizeKHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Vulkan state access`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_ray_tracing_pipeline_stack_size(&self, pipeline_stack_size: u32) {
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_ray_tracing_pipeline_stack_size_khr;

        unsafe { (call)(self.handle, pipeline_stack_size) };
    }
}

pub trait RayTracingPipelineDevice {
    fn get_ray_tracing_shader_group_handles(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> Result<()>;

    fn get_ray_tracing_capture_replay_shader_group_handles(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> Result<()>;

    fn create_ray_tracing_pipelines(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<()>;

    fn get_ray_tracing_shader_group_stack_size(
        &self,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize;
}

impl RayTracingPipelineDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupHandlesKHR.html>
    #[inline]
    fn get_ray_tracing_shader_group_handles(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_ray_tracing_shader_group_handles_khr;

        unsafe {
            (call)(
                self.handle,
                pipeline,
                first_group,
                group_count,
                data.len() as usize,
                data.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingCaptureReplayShaderGroupHandlesKHR.html>
    #[inline]
    fn get_ray_tracing_capture_replay_shader_group_handles(
        &self,
        pipeline: Pipeline,
        first_group: u32,
        group_count: u32,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_ray_tracing_capture_replay_shader_group_handles_khr;

        unsafe {
            (call)(
                self.handle,
                pipeline,
                first_group,
                group_count,
                data.len() as usize,
                data.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRayTracingPipelinesKHR.html>
    #[inline]
    fn create_ray_tracing_pipelines(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[RayTracingPipelineCreateInfoKHR],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<()> {
        assert_eq!(create_infos.len(), pipelines.len());
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_ray_tracing_pipelines_khr;

        unsafe {
            (call)(
                self.handle,
                deferred_operation,
                pipeline_cache,
                create_infos.len() as u32,
                create_infos.as_ptr(),
                allocator.map_or(null(), from_ref),
                pipelines.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRayTracingShaderGroupStackSizeKHR.html>
    #[inline]
    fn get_ray_tracing_shader_group_stack_size(
        &self,
        pipeline: Pipeline,
        group: u32,
        group_shader: ShaderGroupShaderKHR,
    ) -> DeviceSize {
        let call = self
            .fns()
            .khr_ray_tracing_pipeline
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_ray_tracing_shader_group_stack_size_khr;

        unsafe { (call)(self.handle, pipeline, group, group_shader) }
    }
}
