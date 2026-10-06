// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_NV_device_generated_commands`](crate::nv::device_generated_commands)
pub const NAME: &CStr = c"VK_NV_device_generated_commands_compute";
pub const SPEC_VERSION: u32 = 2;

pub trait DeviceGeneratedCommandsComputeCommandBuffer {
    fn update_pipeline_indirect_buffer(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    );
}

impl DeviceGeneratedCommandsComputeCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdatePipelineIndirectBufferNV.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn update_pipeline_indirect_buffer(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
    ) {
        let call = self
            .fns()
            .nv_device_generated_commands_compute
            .update_pipeline_indirect_buffer_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, pipeline_bind_point, pipeline) };
    }
}

pub trait DeviceGeneratedCommandsComputeDevice {
    fn get_pipeline_indirect_memory_requirements(
        &self,
        create_info: &ComputePipelineCreateInfo,
        memory_requirements: &mut MemoryRequirements2<'_>,
    );

    fn get_pipeline_indirect_address(
        &self,
        info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress;
}

impl DeviceGeneratedCommandsComputeDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectMemoryRequirementsNV.html>
    #[inline]
    fn get_pipeline_indirect_memory_requirements(
        &self,
        create_info: &ComputePipelineCreateInfo,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        let call = self
            .fns()
            .nv_device_generated_commands_compute
            .get_pipeline_indirect_memory_requirements_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, create_info, memory_requirements) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html>
    #[inline]
    fn get_pipeline_indirect_address(
        &self,
        info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        let call = self
            .fns()
            .nv_device_generated_commands_compute
            .get_pipeline_indirect_device_address_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) }
    }
}
