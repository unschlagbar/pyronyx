// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_ARM_data_graph";
pub const SPEC_VERSION: u32 = 1;

pub trait DataGraphDevice {
    fn create_data_graph_pipelines(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[DataGraphPipelineCreateInfoARM],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), Error>;

    fn create_data_graph_pipeline_session(
        &self,
        create_info: &DataGraphPipelineSessionCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DataGraphPipelineSessionARM, Error>;

    fn get_data_graph_pipeline_session_bind_point_requirements(
        &self,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirements: &mut [DataGraphPipelineSessionBindPointRequirementARM],
    ) -> Result<(), Error>;

    fn get_data_graph_pipeline_session_memory_requirements(
        &self,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
    ) -> MemoryRequirements2<'_>;

    fn bind_data_graph_pipeline_session_memory(
        &self,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM],
    ) -> Result<(), Error>;

    fn destroy_data_graph_pipeline_session(
        &self,
        session: DataGraphPipelineSessionARM,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_data_graph_pipeline_available_properties(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyARM],
    ) -> Result<(), Error>;

    fn get_data_graph_pipeline_properties(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM],
    ) -> Result<(), Error>;
}

impl DataGraphDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelinesARM.html>
    #[inline]
    fn create_data_graph_pipelines(
        &self,
        deferred_operation: DeferredOperationKHR,
        pipeline_cache: PipelineCache,
        create_infos: &[DataGraphPipelineCreateInfoARM],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), Error> {
        assert_eq!(create_infos.len(), pipelines.len());
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .create_data_graph_pipelines_arm)(
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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDataGraphPipelineSessionARM.html>
    #[inline]
    fn create_data_graph_pipeline_session(
        &self,
        create_info: &DataGraphPipelineSessionCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DataGraphPipelineSessionARM, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .create_data_graph_pipeline_session_arm)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionBindPointRequirementsARM.html>
    #[inline]
    fn get_data_graph_pipeline_session_bind_point_requirements(
        &self,
        info: &DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirements: &mut [DataGraphPipelineSessionBindPointRequirementARM],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .get_data_graph_pipeline_session_bind_point_requirements_arm)(
                self.handle,
                info,
                bind_point_requirements.len() as *mut u32,
                bind_point_requirements.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineSessionMemoryRequirementsARM.html>
    #[inline]
    fn get_data_graph_pipeline_session_memory_requirements(
        &self,
        info: &DataGraphPipelineSessionMemoryRequirementsInfoARM,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .get_data_graph_pipeline_session_memory_requirements_arm)(
                self.handle,
                info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindDataGraphPipelineSessionMemoryARM.html>
    #[inline]
    fn bind_data_graph_pipeline_session_memory(
        &self,
        bind_infos: &[BindDataGraphPipelineSessionMemoryInfoARM],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .bind_data_graph_pipeline_session_memory_arm)(
                self.handle,
                bind_infos.len() as u32,
                bind_infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDataGraphPipelineSessionARM.html>
    #[inline]
    fn destroy_data_graph_pipeline_session(
        &self,
        session: DataGraphPipelineSessionARM,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .destroy_data_graph_pipeline_session_arm)(
                self.handle,
                session,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelineAvailablePropertiesARM.html>
    #[inline]
    fn get_data_graph_pipeline_available_properties(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyARM],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .get_data_graph_pipeline_available_properties_arm)(
                self.handle,
                pipeline_info,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDataGraphPipelinePropertiesARM.html>
    #[inline]
    fn get_data_graph_pipeline_properties(
        &self,
        pipeline_info: &DataGraphPipelineInfoARM,
        properties: &mut [DataGraphPipelinePropertyQueryResultARM],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .get_data_graph_pipeline_properties_arm)(
                self.handle,
                pipeline_info,
                properties.len() as u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }
}

pub trait DataGraphCommandBuffer {
    fn dispatch_data_graph(
        &self,
        session: DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM>,
    );
}

impl DataGraphCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchDataGraphARM.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `DataGraphARM`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn dispatch_data_graph(
        &self,
        session: DataGraphPipelineSessionARM,
        info: Option<&DataGraphPipelineDispatchInfoARM>,
    ) {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .dispatch_data_graph_arm)(
                self.handle, session, info.map_or(null(), from_ref)
            )
        };
    }
}

pub trait DataGraphPhysicalDevice {
    fn get_queue_family_data_graph_properties(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &mut [QueueFamilyDataGraphPropertiesARM],
    ) -> Result<(), Error>;

    fn get_queue_family_data_graph_processing_engine_properties(
        &self,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    ) -> QueueFamilyDataGraphProcessingEnginePropertiesARM<'_>;
}

impl DataGraphPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM.html>
    #[inline]
    fn get_queue_family_data_graph_properties(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &mut [QueueFamilyDataGraphPropertiesARM],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .get_physical_device_queue_family_data_graph_properties_arm)(
                self.handle,
                queue_family_index,
                queue_family_data_graph_properties.len() as *mut u32,
                queue_family_data_graph_properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM.html>
    #[inline]
    fn get_queue_family_data_graph_processing_engine_properties(
        &self,
        queue_family_data_graph_processing_engine_info: &PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM,
    ) -> QueueFamilyDataGraphProcessingEnginePropertiesARM<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_data_graph
                .as_ref()
                .unwrap()
                .get_physical_device_queue_family_data_graph_processing_engine_properties_arm)(
                self.handle,
                queue_family_data_graph_processing_engine_info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }
}
