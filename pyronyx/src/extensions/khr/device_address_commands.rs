// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: (((([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_buffer_device_address`](crate::khr::buffer_device_address)) or Vulkan 1.2) + [`VK_KHR_synchronization2`](crate::khr::synchronization2) + [`VK_EXT_extended_dynamic_state`](crate::ext::extended_dynamic_state)) or Vulkan 1.3
pub const NAME: &CStr = c"VK_KHR_device_address_commands";
pub const SPEC_VERSION: u32 = 1;

pub trait DeviceAddressCommandsCommandBuffer {
    fn copy_memory(&self, copy_memory_info: Option<&CopyDeviceMemoryInfoKHR>);

    fn copy_memory_to_image(&self, copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR>);

    fn copy_image_to_memory(&self, copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR>);

    fn update_memory(
        &self,
        dst_range: &DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: &[u8],
    );

    fn fill_memory(
        &self,
        dst_range: &DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    );

    fn copy_query_pool_results_to_memory(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_range: &StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    );

    fn begin_conditional_rendering2(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfo2EXT,
    );

    fn bind_transform_feedback_buffers2(
        &self,
        first_binding: u32,
        binding_infos: &[BindTransformFeedbackBuffer2InfoEXT],
    );

    fn begin_transform_feedback2(
        &self,
        first_counter_range: u32,
        counter_infos: &[BindTransformFeedbackBuffer2InfoEXT],
    );

    fn end_transform_feedback2(
        &self,
        first_counter_range: u32,
        counter_infos: &[BindTransformFeedbackBuffer2InfoEXT],
    );

    fn draw_indirect_byte_count2(
        &self,
        instance_count: u32,
        first_instance: u32,
        counter_info: &BindTransformFeedbackBuffer2InfoEXT,
        counter_offset: u32,
        vertex_stride: u32,
    );

    fn write_marker_to_memory(&self, info: &MemoryMarkerInfoAMD);

    fn bind_index_buffer3(&self, info: &BindIndexBuffer3InfoKHR);

    fn bind_vertex_buffers3(&self, first_binding: u32, binding_infos: &[BindVertexBuffer3InfoKHR]);

    fn draw_indirect2(&self, info: &DrawIndirect2InfoKHR);

    fn draw_indexed_indirect2(&self, info: &DrawIndirect2InfoKHR);

    fn draw_indirect_count2(&self, info: &DrawIndirectCount2InfoKHR);

    fn draw_indexed_indirect_count2(&self, info: &DrawIndirectCount2InfoKHR);

    fn draw_mesh_tasks_indirect2(&self, info: &DrawIndirect2InfoKHR);

    fn draw_mesh_tasks_indirect_count2(&self, info: &DrawIndirectCount2InfoKHR);

    fn dispatch_indirect2(&self, info: &DispatchIndirect2InfoKHR);
}

impl DeviceAddressCommandsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryKHR.html>
    ///
    /// Queues types: `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory(&self, copy_memory_info: Option<&CopyDeviceMemoryInfoKHR>) {
        let call = self
            .fns()
            .khr_device_address_commands
            .copy_memory_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, copy_memory_info.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageKHR.html>
    ///
    /// Queues types: `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_to_image(&self, copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR>) {
        let call = self
            .fns()
            .khr_device_address_commands
            .copy_memory_to_image_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, copy_memory_info.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToMemoryKHR.html>
    ///
    /// Queues types: `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_image_to_memory(&self, copy_memory_info: Option<&CopyDeviceMemoryImageInfoKHR>) {
        let call = self
            .fns()
            .khr_device_address_commands
            .copy_image_to_memory_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, copy_memory_info.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateMemoryKHR.html>
    ///
    /// Queues types: `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn update_memory(
        &self,
        dst_range: &DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: &[u8],
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .update_memory_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                dst_range,
                dst_flags,
                data.len() as DeviceSize,
                data.as_ptr().cast(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillMemoryKHR.html>
    ///
    /// Queues types: `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn fill_memory(
        &self,
        dst_range: &DeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        data: u32,
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .fill_memory_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, dst_range, dst_flags, data) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResultsToMemoryKHR.html>
    ///
    /// Queues types: `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_query_pool_results_to_memory(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_range: &StridedDeviceAddressRangeKHR,
        dst_flags: AddressCommandFlagsKHR,
        query_result_flags: QueryResultFlags,
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .copy_query_pool_results_to_memory_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                query_pool,
                first_query,
                query_count,
                dst_range,
                dst_flags,
                query_result_flags,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginConditionalRendering2EXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_conditional_rendering2(
        &self,
        conditional_rendering_begin: &ConditionalRenderingBeginInfo2EXT,
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .begin_conditional_rendering2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, conditional_rendering_begin) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffers2EXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_transform_feedback_buffers2(
        &self,
        first_binding: u32,
        binding_infos: &[BindTransformFeedbackBuffer2InfoEXT],
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .bind_transform_feedback_buffers2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                first_binding,
                binding_infos.len() as u32,
                binding_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedback2EXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_transform_feedback2(
        &self,
        first_counter_range: u32,
        counter_infos: &[BindTransformFeedbackBuffer2InfoEXT],
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .begin_transform_feedback2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                first_counter_range,
                counter_infos.len() as u32,
                counter_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedback2EXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_transform_feedback2(
        &self,
        first_counter_range: u32,
        counter_infos: &[BindTransformFeedbackBuffer2InfoEXT],
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .end_transform_feedback2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                first_counter_range,
                counter_infos.len() as u32,
                counter_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCount2EXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_indirect_byte_count2(
        &self,
        instance_count: u32,
        first_instance: u32,
        counter_info: &BindTransformFeedbackBuffer2InfoEXT,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_indirect_byte_count2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                instance_count,
                first_instance,
                counter_info,
                counter_offset,
                vertex_stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMarkerToMemoryAMD.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn write_marker_to_memory(&self, info: &MemoryMarkerInfoAMD) {
        let call = self
            .fns()
            .khr_device_address_commands
            .write_marker_to_memory_amd
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer3KHR.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_index_buffer3(&self, info: &BindIndexBuffer3InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .bind_index_buffer3_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers3KHR.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_vertex_buffers3(&self, first_binding: u32, binding_infos: &[BindVertexBuffer3InfoKHR]) {
        let call = self
            .fns()
            .khr_device_address_commands
            .bind_vertex_buffers3_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                first_binding,
                binding_infos.len() as u32,
                binding_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect2KHR.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_indirect2(&self, info: &DrawIndirect2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_indirect2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect2KHR.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_indexed_indirect2(&self, info: &DrawIndirect2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_indexed_indirect2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount2KHR.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_indirect_count2(&self, info: &DrawIndirectCount2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_indirect_count2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount2KHR.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_indexed_indirect_count2(&self, info: &DrawIndirectCount2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_indexed_indirect_count2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirect2EXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_mesh_tasks_indirect2(&self, info: &DrawIndirect2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_mesh_tasks_indirect2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMeshTasksIndirectCount2EXT.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn draw_mesh_tasks_indirect_count2(&self, info: &DrawIndirectCount2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .draw_mesh_tasks_indirect_count2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect2KHR.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn dispatch_indirect2(&self, info: &DispatchIndirect2InfoKHR) {
        let call = self
            .fns()
            .khr_device_address_commands
            .dispatch_indirect2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) };
    }
}

pub trait DeviceAddressCommandsDevice {
    fn create_acceleration_structure2(
        &self,
        create_info: &AccelerationStructureCreateInfo2KHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureKHR>;
}

impl DeviceAddressCommandsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateAccelerationStructure2KHR.html>
    #[inline]
    fn create_acceleration_structure2(
        &self,
        create_info: &AccelerationStructureCreateInfo2KHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<AccelerationStructureKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_device_address_commands
            .create_acceleration_structure2_khr
            .unwrap_or_else(|| Self::ext_load_error());

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
}
