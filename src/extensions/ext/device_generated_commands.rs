// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_device_generated_commands";
pub const SPEC_VERSION: u32 = 1;

pub trait DeviceGeneratedCommandsCommandBuffer {
    fn execute_generated_commands(
        &self,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoEXT,
    );

    fn preprocess_generated_commands(
        &self,
        generated_commands_info: &GeneratedCommandsInfoEXT,
        state_command_buffer: vkCommandBuffer,
    );
}

impl DeviceGeneratedCommandsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteGeneratedCommandsEXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Executes indirect GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn execute_generated_commands(
        &self,
        is_preprocessed: bool,
        generated_commands_info: &GeneratedCommandsInfoEXT,
    ) {
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .execute_generated_commands_ext;

        unsafe { (call)(self.handle, is_preprocessed as _, generated_commands_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPreprocessGeneratedCommandsEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn preprocess_generated_commands(
        &self,
        generated_commands_info: &GeneratedCommandsInfoEXT,
        state_command_buffer: vkCommandBuffer,
    ) {
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .preprocess_generated_commands_ext;

        unsafe { (call)(self.handle, generated_commands_info, state_command_buffer) };
    }
}

pub trait DeviceGeneratedCommandsDevice {
    fn get_generated_commands_memory_requirements(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT,
    ) -> MemoryRequirements2<'_>;

    fn create_indirect_commands_layout(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<IndirectCommandsLayoutEXT>;

    fn destroy_indirect_commands_layout(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: Option<&AllocationCallbacks>,
    );

    fn create_indirect_execution_set(
        &self,
        create_info: &IndirectExecutionSetCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<IndirectExecutionSetEXT>;

    fn destroy_indirect_execution_set(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: Option<&AllocationCallbacks>,
    );

    fn update_indirect_execution_set_pipeline(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT],
    );

    fn update_indirect_execution_set_shader(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT],
    );
}

impl DeviceGeneratedCommandsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGeneratedCommandsMemoryRequirementsEXT.html>
    #[inline]
    fn get_generated_commands_memory_requirements(
        &self,
        info: &GeneratedCommandsMemoryRequirementsInfoEXT,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_generated_commands_memory_requirements_ext;

        unsafe {
            (call)(self.handle, info, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectCommandsLayoutEXT.html>
    #[inline]
    fn create_indirect_commands_layout(
        &self,
        create_info: &IndirectCommandsLayoutCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<IndirectCommandsLayoutEXT> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_indirect_commands_layout_ext;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectCommandsLayoutEXT.html>
    #[inline]
    fn destroy_indirect_commands_layout(
        &self,
        indirect_commands_layout: IndirectCommandsLayoutEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_indirect_commands_layout_ext;

        unsafe {
            (call)(
                self.handle,
                indirect_commands_layout,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateIndirectExecutionSetEXT.html>
    #[inline]
    fn create_indirect_execution_set(
        &self,
        create_info: &IndirectExecutionSetCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<IndirectExecutionSetEXT> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_indirect_execution_set_ext;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyIndirectExecutionSetEXT.html>
    #[inline]
    fn destroy_indirect_execution_set(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_indirect_execution_set_ext;

        unsafe {
            (call)(
                self.handle,
                indirect_execution_set,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetPipelineEXT.html>
    #[inline]
    fn update_indirect_execution_set_pipeline(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetPipelineEXT],
    ) {
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .update_indirect_execution_set_pipeline_ext;

        unsafe {
            (call)(
                self.handle,
                indirect_execution_set,
                execution_set_writes.len() as u32,
                execution_set_writes.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateIndirectExecutionSetShaderEXT.html>
    #[inline]
    fn update_indirect_execution_set_shader(
        &self,
        indirect_execution_set: IndirectExecutionSetEXT,
        execution_set_writes: &[WriteIndirectExecutionSetShaderEXT],
    ) {
        let call = self
            .fns()
            .ext_device_generated_commands
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .update_indirect_execution_set_shader_ext;

        unsafe {
            (call)(
                self.handle,
                indirect_execution_set,
                execution_set_writes.len() as u32,
                execution_set_writes.as_ptr(),
            )
        };
    }
}
