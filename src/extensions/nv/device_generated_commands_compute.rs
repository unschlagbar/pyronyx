// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
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
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands_compute
                .as_ref()
                .unwrap()
                .update_pipeline_indirect_buffer_nv)(
                self.handle, pipeline_bind_point, pipeline
            )
        };
    }
}

pub trait DeviceGeneratedCommandsComputeDevice {
    fn get_pipeline_indirect_memory_requirements(
        &self,
        create_info: &ComputePipelineCreateInfo,
    ) -> MemoryRequirements2<'_>;

    fn get_pipeline_indirect_device_address(
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
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands_compute
                .as_ref()
                .unwrap()
                .get_pipeline_indirect_memory_requirements_nv)(
                self.handle,
                create_info,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineIndirectDeviceAddressNV.html>
    #[inline]
    fn get_pipeline_indirect_device_address(
        &self,
        info: &PipelineIndirectDeviceAddressInfoNV,
    ) -> DeviceAddress {
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands_compute
                .as_ref()
                .unwrap()
                .get_pipeline_indirect_device_address_nv)(self.handle, info)
        }
    }
}
