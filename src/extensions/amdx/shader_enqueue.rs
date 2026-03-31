// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_AMDX_shader_enqueue";
pub const SPEC_VERSION: u32 = 2;

pub trait ShaderEnqueueDevice {
    fn get_execution_graph_pipeline_scratch_size(
        &self,
        execution_graph: Pipeline,
    ) -> Result<ExecutionGraphPipelineScratchSizeAMDX<'_>, Error>;

    fn get_execution_graph_pipeline_node_index(
        &self,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> Result<u32, Error>;

    fn create_execution_graph_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), Error>;
}

impl ShaderEnqueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html>
    #[inline]
    fn get_execution_graph_pipeline_scratch_size(
        &self,
        execution_graph: Pipeline,
    ) -> Result<ExecutionGraphPipelineScratchSizeAMDX<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .get_execution_graph_pipeline_scratch_size_amdx)(
                self.handle,
                execution_graph,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html>
    #[inline]
    fn get_execution_graph_pipeline_node_index(
        &self,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> Result<u32, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .get_execution_graph_pipeline_node_index_amdx)(
                self.handle,
                execution_graph,
                node_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExecutionGraphPipelinesAMDX.html>
    #[inline]
    fn create_execution_graph_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), Error> {
        assert_eq!(create_infos.len(), pipelines.len());
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .create_execution_graph_pipelines_amdx)(
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

pub trait ShaderEnqueueCommandBuffer {
    fn initialize_graph_scratch_memory(
        &self,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    );

    fn dispatch_graph(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    );

    fn dispatch_graph_indirect(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    );

    fn dispatch_graph_indirect_count(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    );
}

impl ShaderEnqueueCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInitializeGraphScratchMemoryAMDX.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Action`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn initialize_graph_scratch_memory(
        &self,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .initialize_graph_scratch_memory_amdx)(
                self.handle,
                execution_graph,
                scratch,
                scratch_size,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Action`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn dispatch_graph(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .dispatch_graph_amdx)(self.handle, scratch, scratch_size, count_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Action`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn dispatch_graph_indirect(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) {
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .dispatch_graph_indirect_amdx)(
                self.handle, scratch, scratch_size, count_info
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Action`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn dispatch_graph_indirect_count(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        unsafe {
            (self
                .fns()
                .amdx_shader_enqueue
                .as_ref()
                .unwrap()
                .dispatch_graph_indirect_count_amdx)(
                self.handle, scratch, scratch_size, count_info
            )
        };
    }
}
