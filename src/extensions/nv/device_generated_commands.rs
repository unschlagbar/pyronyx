// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_device_generated_commands";
pub const SPEC_VERSION: u32 = 3;

pub trait DeviceGeneratedCommandsCommandBuffer {
    fn execute_generated_commands(
        &self,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV,
    );

    fn preprocess_generated_commands(&self, generated_commands_info: &GeneratedCommandsInfoNV);

    fn bind_pipeline_shader_group(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    );
}

impl DeviceGeneratedCommandsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsNV.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Action`, `Indirect action`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn execute_generated_commands(
        &self,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoNV,
    ) {
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands
                .as_ref()
                .unwrap()
                .execute_generated_commands_nv)(
                self.handle,
                is_preprocessed as _,
                generated_commands_info,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Action`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn preprocess_generated_commands(&self, generated_commands_info: &GeneratedCommandsInfoNV) {
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands
                .as_ref()
                .unwrap()
                .preprocess_generated_commands_nv)(self.handle, generated_commands_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipelineShaderGroupNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_pipeline_shader_group(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        pipeline: Pipeline,
        group_index: u32,
    ) {
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands
                .as_ref()
                .unwrap()
                .bind_pipeline_shader_group_nv)(
                self.handle,
                pipeline_bind_point,
                pipeline,
                group_index,
            )
        };
    }
}

pub trait DeviceGeneratedCommandsDevice {
    fn get_generated_commands_memory_requirements(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoNV,
    ) -> MemoryRequirements2<'_>;

    fn create_indirect_commands_layout(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<IndirectCommandsLayoutNV, Error>;

    fn destroy_indirect_commands_layout(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&AllocationCallbacks>,
    );
}

impl DeviceGeneratedCommandsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsNV.html>
    #[inline]
    fn get_generated_commands_memory_requirements(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoNV,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands
                .as_ref()
                .unwrap()
                .get_generated_commands_memory_requirements_nv)(
                self.handle, info, out.as_mut_ptr()
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutNV.html>
    #[inline]
    fn create_indirect_commands_layout(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<IndirectCommandsLayoutNV, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands
                .as_ref()
                .unwrap()
                .create_indirect_commands_layout_nv)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutNV.html>
    #[inline]
    fn destroy_indirect_commands_layout(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .nv_device_generated_commands
                .as_ref()
                .unwrap()
                .destroy_indirect_commands_layout_nv)(
                self.handle,
                indirect_commands_layout,
                allocator.map_or(null(), from_ref),
            )
        };
    }
}
