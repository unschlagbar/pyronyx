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
    ) -> Result<ExecutionGraphPipelineScratchSizeAMDX<'_>>;

    fn get_execution_graph_pipeline_node_index(
        &self,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> Result<u32>;

    fn create_execution_graph_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ExecutionGraphPipelineCreateInfoAMDX],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<()>;
}

impl ShaderEnqueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineScratchSizeAMDX.html>
    #[inline]
    fn get_execution_graph_pipeline_scratch_size(
        &self,
        execution_graph: Pipeline,
    ) -> Result<ExecutionGraphPipelineScratchSizeAMDX<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_execution_graph_pipeline_scratch_size_amdx;

        unsafe { (call)(self.handle, execution_graph, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetExecutionGraphPipelineNodeIndexAMDX.html>
    #[inline]
    fn get_execution_graph_pipeline_node_index(
        &self,
        execution_graph: Pipeline,
        node_info: &PipelineShaderStageNodeCreateInfoAMDX,
    ) -> Result<u32> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_execution_graph_pipeline_node_index_amdx;

        unsafe { (call)(self.handle, execution_graph, node_info, out.as_mut_ptr()) }
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
    ) -> Result<()> {
        assert_eq!(create_infos.len(), pipelines.len());
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_execution_graph_pipelines_amdx;

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
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn initialize_graph_scratch_memory(
        &self,
        execution_graph: Pipeline,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
    ) {
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .initialize_graph_scratch_memory_amdx;

        unsafe { (call)(self.handle, execution_graph, scratch, scratch_size) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphAMDX.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn dispatch_graph(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) {
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .dispatch_graph_amdx;

        unsafe { (call)(self.handle, scratch, scratch_size, count_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectAMDX.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn dispatch_graph_indirect(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: &DispatchGraphCountInfoAMDX,
    ) {
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .dispatch_graph_indirect_amdx;

        unsafe { (call)(self.handle, scratch, scratch_size, count_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchGraphIndirectCountAMDX.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn dispatch_graph_indirect_count(
        &self,
        scratch: DeviceAddress,
        scratch_size: DeviceSize,
        count_info: DeviceAddress,
    ) {
        let call = self
            .fns()
            .amdx_shader_enqueue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .dispatch_graph_indirect_count_amdx;

        unsafe { (call)(self.handle, scratch, scratch_size, count_info) };
    }
}
