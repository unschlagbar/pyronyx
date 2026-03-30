// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated impls
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::c_void;

impl CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBeginCommandBuffer.html>
    #[inline]
    pub fn begin(&self, begin_info: &CommandBufferBeginInfo) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.begin_command_buffer.unwrap())(self.handle, begin_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEndCommandBuffer.html>
    #[inline]
    pub fn end(&self) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.end_command_buffer.unwrap())(self.handle) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandBuffer.html>
    #[inline]
    pub fn reset(&self, flags: CommandBufferResetFlags) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.reset_command_buffer.unwrap())(self.handle, flags) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindPipeline.html>
    #[inline]
    pub fn bind_pipeline(&self, pipeline_bind_point: PipelineBindPoint, pipeline: Pipeline) {
        unsafe {
            (self.fns().v1_0.bind_pipeline.unwrap())(self.handle, pipeline_bind_point, pipeline)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewport.html>
    #[inline]
    pub fn set_viewport(&self, first_viewport: u32, viewports: &[Viewport]) {
        unsafe {
            (self.fns().v1_0.set_viewport.unwrap())(
                self.handle,
                first_viewport,
                viewports.len() as u32,
                viewports.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissor.html>
    #[inline]
    pub fn set_scissor(&self, first_scissor: u32, scissors: &[Rect2D]) {
        unsafe {
            (self.fns().v1_0.set_scissor.unwrap())(
                self.handle,
                first_scissor,
                scissors.len() as u32,
                scissors.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineWidth.html>
    #[inline]
    pub fn set_line_width(&self, line_width: f32) {
        unsafe { (self.fns().v1_0.set_line_width.unwrap())(self.handle, line_width) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBias.html>
    #[inline]
    pub fn set_depth_bias(
        &self,
        depth_bias_constant_factor: f32,
        depth_bias_clamp: f32,
        depth_bias_slope_factor: f32,
    ) {
        unsafe {
            (self.fns().v1_0.set_depth_bias.unwrap())(
                self.handle,
                depth_bias_constant_factor,
                depth_bias_clamp,
                depth_bias_slope_factor,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetBlendConstants.html>
    #[inline]
    pub fn set_blend_constants(&self, blend_constants: f32) {
        unsafe { (self.fns().v1_0.set_blend_constants.unwrap())(self.handle, blend_constants) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBounds.html>
    #[inline]
    pub fn set_depth_bounds(&self, min_depth_bounds: f32, max_depth_bounds: f32) {
        unsafe {
            (self.fns().v1_0.set_depth_bounds.unwrap())(
                self.handle,
                min_depth_bounds,
                max_depth_bounds,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilCompareMask.html>
    #[inline]
    pub fn set_stencil_compare_mask(&self, face_mask: StencilFaceFlags, compare_mask: u32) {
        unsafe {
            (self.fns().v1_0.set_stencil_compare_mask.unwrap())(
                self.handle,
                face_mask,
                compare_mask,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilWriteMask.html>
    #[inline]
    pub fn set_stencil_write_mask(&self, face_mask: StencilFaceFlags, write_mask: u32) {
        unsafe {
            (self.fns().v1_0.set_stencil_write_mask.unwrap())(self.handle, face_mask, write_mask)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilReference.html>
    #[inline]
    pub fn set_stencil_reference(&self, face_mask: StencilFaceFlags, reference: u32) {
        unsafe {
            (self.fns().v1_0.set_stencil_reference.unwrap())(self.handle, face_mask, reference)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets.html>
    #[inline]
    pub fn bind_descriptor_sets(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        descriptor_sets: &[DescriptorSet],
        dynamic_offsets: &[u32],
    ) {
        unsafe {
            (self.fns().v1_0.bind_descriptor_sets.unwrap())(
                self.handle,
                pipeline_bind_point,
                layout,
                first_set,
                descriptor_sets.len() as u32,
                descriptor_sets.as_ptr(),
                dynamic_offsets.len() as u32,
                dynamic_offsets.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer.html>
    #[inline]
    pub fn bind_index_buffer(&self, buffer: Buffer, offset: DeviceSize, index_type: IndexType) {
        unsafe {
            (self.fns().v1_0.bind_index_buffer.unwrap())(self.handle, buffer, offset, index_type)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers.html>
    #[inline]
    pub fn bind_vertex_buffers(
        &self,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
    ) {
        assert_eq!(buffers.len(), offsets.len());
        unsafe {
            (self.fns().v1_0.bind_vertex_buffers.unwrap())(
                self.handle,
                first_binding,
                buffers.len() as u32,
                buffers.as_ptr(),
                offsets.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDraw.html>
    #[inline]
    pub fn draw(
        &self,
        vertex_count: u32,
        instance_count: u32,
        first_vertex: u32,
        first_instance: u32,
    ) {
        unsafe {
            (self.fns().v1_0.draw.unwrap())(
                self.handle,
                vertex_count,
                instance_count,
                first_vertex,
                first_instance,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexed.html>
    #[inline]
    pub fn draw_indexed(
        &self,
        index_count: u32,
        instance_count: u32,
        first_index: u32,
        vertex_offset: i32,
        first_instance: u32,
    ) {
        unsafe {
            (self.fns().v1_0.draw_indexed.unwrap())(
                self.handle,
                index_count,
                instance_count,
                first_index,
                vertex_offset,
                first_instance,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirect.html>
    #[inline]
    pub fn draw_indirect(&self, buffer: Buffer, offset: DeviceSize, draw_count: u32, stride: u32) {
        unsafe {
            (self.fns().v1_0.draw_indirect.unwrap())(
                self.handle,
                buffer,
                offset,
                draw_count,
                stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirect.html>
    #[inline]
    pub fn draw_indexed_indirect(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fns().v1_0.draw_indexed_indirect.unwrap())(
                self.handle,
                buffer,
                offset,
                draw_count,
                stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatch.html>
    #[inline]
    pub fn dispatch(&self, group_count_x: u32, group_count_y: u32, group_count_z: u32) {
        unsafe {
            (self.fns().v1_0.dispatch.unwrap())(
                self.handle,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchIndirect.html>
    #[inline]
    pub fn dispatch_indirect(&self, buffer: Buffer, offset: DeviceSize) {
        unsafe { (self.fns().v1_0.dispatch_indirect.unwrap())(self.handle, buffer, offset) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer.html>
    #[inline]
    pub fn copy_buffer(&self, src_buffer: Buffer, dst_buffer: Buffer, regions: &[BufferCopy]) {
        unsafe {
            (self.fns().v1_0.copy_buffer.unwrap())(
                self.handle,
                src_buffer,
                dst_buffer,
                regions.len() as u32,
                regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage.html>
    #[inline]
    pub fn copy_image(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageCopy],
    ) {
        unsafe {
            (self.fns().v1_0.copy_image.unwrap())(
                self.handle,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage.html>
    #[inline]
    pub fn blit_image(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageBlit],
        filter: Filter,
    ) {
        unsafe {
            (self.fns().v1_0.blit_image.unwrap())(
                self.handle,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
                filter,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage.html>
    #[inline]
    pub fn copy_buffer_to_image(
        &self,
        src_buffer: Buffer,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.fns().v1_0.copy_buffer_to_image.unwrap())(
                self.handle,
                src_buffer,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer.html>
    #[inline]
    pub fn copy_image_to_buffer(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_buffer: Buffer,
        regions: &[BufferImageCopy],
    ) {
        unsafe {
            (self.fns().v1_0.copy_image_to_buffer.unwrap())(
                self.handle,
                src_image,
                src_image_layout,
                dst_buffer,
                regions.len() as u32,
                regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdUpdateBuffer.html>
    #[inline]
    pub fn update_buffer(&self, dst_buffer: Buffer, dst_offset: DeviceSize, data: &[c_void]) {
        unsafe {
            (self.fns().v1_0.update_buffer.unwrap())(
                self.handle,
                dst_buffer,
                dst_offset,
                data.len() as DeviceSize,
                data.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdFillBuffer.html>
    #[inline]
    pub fn fill_buffer(
        &self,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        size: DeviceSize,
        data: u32,
    ) {
        unsafe {
            (self.fns().v1_0.fill_buffer.unwrap())(self.handle, dst_buffer, dst_offset, size, data)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearColorImage.html>
    #[inline]
    pub fn clear_color_image(
        &self,
        image: Image,
        image_layout: ImageLayout,
        color: &ClearColorValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.fns().v1_0.clear_color_image.unwrap())(
                self.handle,
                image,
                image_layout,
                color,
                ranges.len() as u32,
                ranges.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearDepthStencilImage.html>
    #[inline]
    pub fn clear_depth_stencil_image(
        &self,
        image: Image,
        image_layout: ImageLayout,
        depth_stencil: &ClearDepthStencilValue,
        ranges: &[ImageSubresourceRange],
    ) {
        unsafe {
            (self.fns().v1_0.clear_depth_stencil_image.unwrap())(
                self.handle,
                image,
                image_layout,
                depth_stencil,
                ranges.len() as u32,
                ranges.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdClearAttachments.html>
    #[inline]
    pub fn clear_attachments(&self, attachments: &[ClearAttachment], rects: &[ClearRect]) {
        unsafe {
            (self.fns().v1_0.clear_attachments.unwrap())(
                self.handle,
                attachments.len() as u32,
                attachments.as_ptr(),
                rects.len() as u32,
                rects.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage.html>
    #[inline]
    pub fn resolve_image(
        &self,
        src_image: Image,
        src_image_layout: ImageLayout,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        regions: &[ImageResolve],
    ) {
        unsafe {
            (self.fns().v1_0.resolve_image.unwrap())(
                self.handle,
                src_image,
                src_image_layout,
                dst_image,
                dst_image_layout,
                regions.len() as u32,
                regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent.html>
    #[inline]
    pub fn set_event(&self, event: Event, stage_mask: PipelineStageFlags) {
        unsafe { (self.fns().v1_0.set_event.unwrap())(self.handle, event, stage_mask) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent.html>
    #[inline]
    pub fn reset_event(&self, event: Event, stage_mask: PipelineStageFlags) {
        unsafe { (self.fns().v1_0.reset_event.unwrap())(self.handle, event, stage_mask) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents.html>
    #[inline]
    pub fn wait_events(
        &self,
        events: &[Event],
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        unsafe {
            (self.fns().v1_0.wait_events.unwrap())(
                self.handle,
                events.len() as u32,
                events.as_ptr(),
                src_stage_mask,
                dst_stage_mask,
                memory_barriers.len() as u32,
                memory_barriers.as_ptr(),
                buffer_memory_barriers.len() as u32,
                buffer_memory_barriers.as_ptr(),
                image_memory_barriers.len() as u32,
                image_memory_barriers.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier.html>
    #[inline]
    pub fn pipeline_barrier(
        &self,
        src_stage_mask: PipelineStageFlags,
        dst_stage_mask: PipelineStageFlags,
        dependency_flags: DependencyFlags,
        memory_barriers: &[MemoryBarrier],
        buffer_memory_barriers: &[BufferMemoryBarrier],
        image_memory_barriers: &[ImageMemoryBarrier],
    ) {
        unsafe {
            (self.fns().v1_0.pipeline_barrier.unwrap())(
                self.handle,
                src_stage_mask,
                dst_stage_mask,
                dependency_flags,
                memory_barriers.len() as u32,
                memory_barriers.as_ptr(),
                buffer_memory_barriers.len() as u32,
                buffer_memory_barriers.as_ptr(),
                image_memory_barriers.len() as u32,
                image_memory_barriers.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQuery.html>
    #[inline]
    pub fn begin_query(&self, query_pool: QueryPool, query: u32, flags: QueryControlFlags) {
        unsafe { (self.fns().v1_0.begin_query.unwrap())(self.handle, query_pool, query, flags) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQuery.html>
    #[inline]
    pub fn end_query(&self, query_pool: QueryPool, query: u32) {
        unsafe { (self.fns().v1_0.end_query.unwrap())(self.handle, query_pool, query) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetQueryPool.html>
    #[inline]
    pub fn reset_query_pool(&self, query_pool: QueryPool, first_query: u32, query_count: u32) {
        unsafe {
            (self.fns().v1_0.reset_query_pool.unwrap())(
                self.handle,
                query_pool,
                first_query,
                query_count,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp.html>
    #[inline]
    pub fn write_timestamp(
        &self,
        pipeline_stage: PipelineStageFlags,
        query_pool: QueryPool,
        query: u32,
    ) {
        unsafe {
            (self.fns().v1_0.write_timestamp.unwrap())(
                self.handle,
                pipeline_stage,
                query_pool,
                query,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyQueryPoolResults.html>
    #[inline]
    pub fn copy_query_pool_results(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        dst_buffer: Buffer,
        dst_offset: DeviceSize,
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) {
        unsafe {
            (self.fns().v1_0.copy_query_pool_results.unwrap())(
                self.handle,
                query_pool,
                first_query,
                query_count,
                dst_buffer,
                dst_offset,
                stride,
                flags,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants.html>
    #[inline]
    pub fn push_constants(
        &self,
        layout: PipelineLayout,
        stage_flags: ShaderStageFlags,
        offset: u32,
        values: &[c_void],
    ) {
        unsafe {
            (self.fns().v1_0.push_constants.unwrap())(
                self.handle,
                layout,
                stage_flags,
                offset,
                values.len() as u32,
                values.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass.html>
    #[inline]
    pub fn begin_render_pass(
        &self,
        render_pass_begin: &RenderPassBeginInfo,
        contents: SubpassContents,
    ) {
        unsafe {
            (self.fns().v1_0.begin_render_pass.unwrap())(self.handle, render_pass_begin, contents)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass.html>
    #[inline]
    pub fn next_subpass(&self, contents: SubpassContents) {
        unsafe { (self.fns().v1_0.next_subpass.unwrap())(self.handle, contents) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass.html>
    #[inline]
    pub fn end_render_pass(&self) {
        unsafe { (self.fns().v1_0.end_render_pass.unwrap())(self.handle) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdExecuteCommands.html>
    #[inline]
    pub fn execute_commands(&self, command_buffers: &[vkCommandBuffer]) {
        unsafe {
            (self.fns().v1_0.execute_commands.unwrap())(
                self.handle,
                command_buffers.len() as u32,
                command_buffers.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet.html>
    #[inline]
    pub fn push_descriptor_set(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
        descriptor_writes: &[WriteDescriptorSet],
    ) {
        unsafe {
            (self.fns().v1_4.push_descriptor_set.unwrap())(
                self.handle,
                pipeline_bind_point,
                layout,
                set,
                descriptor_writes.len() as u32,
                descriptor_writes.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDeviceMask.html>
    #[inline]
    pub fn set_device_mask(&self, device_mask: u32) {
        unsafe { (self.fns().v1_1.set_device_mask.unwrap())(self.handle, device_mask) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchBase.html>
    #[inline]
    pub fn dispatch_base(
        &self,
        base_group_x: u32,
        base_group_y: u32,
        base_group_z: u32,
        group_count_x: u32,
        group_count_y: u32,
        group_count_z: u32,
    ) {
        unsafe {
            (self.fns().v1_1.dispatch_base.unwrap())(
                self.handle,
                base_group_x,
                base_group_y,
                base_group_z,
                group_count_x,
                group_count_y,
                group_count_z,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate.html>
    #[inline]
    pub fn push_descriptor_set_with_template(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        layout: PipelineLayout,
        set: u32,
        data: &c_void,
    ) {
        unsafe {
            (self.fns().v1_4.push_descriptor_set_with_template.unwrap())(
                self.handle,
                descriptor_update_template,
                layout,
                set,
                data,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRenderPass2.html>
    #[inline]
    pub fn begin_render_pass2(
        &self,
        render_pass_begin: &RenderPassBeginInfo,
        subpass_begin_info: &SubpassBeginInfo,
    ) {
        unsafe {
            (self.fns().v1_2.begin_render_pass2.unwrap())(
                self.handle,
                render_pass_begin,
                subpass_begin_info,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdNextSubpass2.html>
    #[inline]
    pub fn next_subpass2(
        &self,
        subpass_begin_info: &SubpassBeginInfo,
        subpass_end_info: &SubpassEndInfo,
    ) {
        unsafe {
            (self.fns().v1_2.next_subpass2.unwrap())(
                self.handle,
                subpass_begin_info,
                subpass_end_info,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRenderPass2.html>
    #[inline]
    pub fn end_render_pass2(&self, subpass_end_info: &SubpassEndInfo) {
        unsafe { (self.fns().v1_2.end_render_pass2.unwrap())(self.handle, subpass_end_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectCount.html>
    #[inline]
    pub fn draw_indirect_count(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fns().v1_2.draw_indirect_count.unwrap())(
                self.handle,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndexedIndirectCount.html>
    #[inline]
    pub fn draw_indexed_indirect_count(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        count_buffer: Buffer,
        count_buffer_offset: DeviceSize,
        max_draw_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fns().v1_2.draw_indexed_indirect_count.unwrap())(
                self.handle,
                buffer,
                offset,
                count_buffer,
                count_buffer_offset,
                max_draw_count,
                stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStipple.html>
    #[inline]
    pub fn set_line_stipple(&self, line_stipple_factor: u32, line_stipple_pattern: u16) {
        unsafe {
            (self.fns().v1_4.set_line_stipple.unwrap())(
                self.handle,
                line_stipple_factor,
                line_stipple_pattern,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCullMode.html>
    #[inline]
    pub fn set_cull_mode(&self, cull_mode: CullModeFlags) {
        unsafe { (self.fns().v1_3.set_cull_mode.unwrap())(self.handle, cull_mode) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetFrontFace.html>
    #[inline]
    pub fn set_front_face(&self, front_face: FrontFace) {
        unsafe { (self.fns().v1_3.set_front_face.unwrap())(self.handle, front_face) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveTopology.html>
    #[inline]
    pub fn set_primitive_topology(&self, primitive_topology: PrimitiveTopology) {
        unsafe {
            (self.fns().v1_3.set_primitive_topology.unwrap())(self.handle, primitive_topology)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWithCount.html>
    #[inline]
    pub fn set_viewport_with_count(&self, viewports: &[Viewport]) {
        unsafe {
            (self.fns().v1_3.set_viewport_with_count.unwrap())(
                self.handle,
                viewports.len() as u32,
                viewports.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetScissorWithCount.html>
    #[inline]
    pub fn set_scissor_with_count(&self, scissors: &[Rect2D]) {
        unsafe {
            (self.fns().v1_3.set_scissor_with_count.unwrap())(
                self.handle,
                scissors.len() as u32,
                scissors.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindIndexBuffer2.html>
    #[inline]
    pub fn bind_index_buffer2(
        &self,
        buffer: Buffer,
        offset: DeviceSize,
        size: DeviceSize,
        index_type: IndexType,
    ) {
        unsafe {
            (self.fns().v1_4.bind_index_buffer2.unwrap())(
                self.handle,
                buffer,
                offset,
                size,
                index_type,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindVertexBuffers2.html>
    #[inline]
    pub fn bind_vertex_buffers2(
        &self,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: &[DeviceSize],
        strides: &[DeviceSize],
    ) {
        assert_eq!(buffers.len(), offsets.len());
        assert_eq!(buffers.len(), sizes.len());
        assert_eq!(buffers.len(), strides.len());
        unsafe {
            (self.fns().v1_3.bind_vertex_buffers2.unwrap())(
                self.handle,
                first_binding,
                buffers.len() as u32,
                buffers.as_ptr(),
                offsets.as_ptr(),
                sizes.as_ptr(),
                strides.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthTestEnable.html>
    #[inline]
    pub fn set_depth_test_enable(&self, depth_test_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_depth_test_enable.unwrap())(self.handle, depth_test_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthWriteEnable.html>
    #[inline]
    pub fn set_depth_write_enable(&self, depth_write_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_depth_write_enable.unwrap())(self.handle, depth_write_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthCompareOp.html>
    #[inline]
    pub fn set_depth_compare_op(&self, depth_compare_op: CompareOp) {
        unsafe { (self.fns().v1_3.set_depth_compare_op.unwrap())(self.handle, depth_compare_op) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBoundsTestEnable.html>
    #[inline]
    pub fn set_depth_bounds_test_enable(&self, depth_bounds_test_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_depth_bounds_test_enable.unwrap())(
                self.handle,
                depth_bounds_test_enable as _,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilTestEnable.html>
    #[inline]
    pub fn set_stencil_test_enable(&self, stencil_test_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_stencil_test_enable.unwrap())(
                self.handle,
                stencil_test_enable as _,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetStencilOp.html>
    #[inline]
    pub fn set_stencil_op(
        &self,
        face_mask: StencilFaceFlags,
        fail_op: StencilOp,
        pass_op: StencilOp,
        depth_fail_op: StencilOp,
        compare_op: CompareOp,
    ) {
        unsafe {
            (self.fns().v1_3.set_stencil_op.unwrap())(
                self.handle,
                face_mask,
                fail_op,
                pass_op,
                depth_fail_op,
                compare_op,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizerDiscardEnable.html>
    #[inline]
    pub fn set_rasterizer_discard_enable(&self, rasterizer_discard_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_rasterizer_discard_enable.unwrap())(
                self.handle,
                rasterizer_discard_enable as _,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthBiasEnable.html>
    #[inline]
    pub fn set_depth_bias_enable(&self, depth_bias_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_depth_bias_enable.unwrap())(self.handle, depth_bias_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPrimitiveRestartEnable.html>
    #[inline]
    pub fn set_primitive_restart_enable(&self, primitive_restart_enable: bool) {
        unsafe {
            (self.fns().v1_3.set_primitive_restart_enable.unwrap())(
                self.handle,
                primitive_restart_enable as _,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBuffer2.html>
    #[inline]
    pub fn copy_buffer2(&self, copy_buffer_info: &CopyBufferInfo2) {
        unsafe { (self.fns().v1_3.copy_buffer2.unwrap())(self.handle, copy_buffer_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImage2.html>
    #[inline]
    pub fn copy_image2(&self, copy_image_info: &CopyImageInfo2) {
        unsafe { (self.fns().v1_3.copy_image2.unwrap())(self.handle, copy_image_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBlitImage2.html>
    #[inline]
    pub fn blit_image2(&self, blit_image_info: &BlitImageInfo2) {
        unsafe { (self.fns().v1_3.blit_image2.unwrap())(self.handle, blit_image_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyBufferToImage2.html>
    #[inline]
    pub fn copy_buffer_to_image2(&self, copy_buffer_to_image_info: &CopyBufferToImageInfo2) {
        unsafe {
            (self.fns().v1_3.copy_buffer_to_image2.unwrap())(self.handle, copy_buffer_to_image_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyImageToBuffer2.html>
    #[inline]
    pub fn copy_image_to_buffer2(&self, copy_image_to_buffer_info: &CopyImageToBufferInfo2) {
        unsafe {
            (self.fns().v1_3.copy_image_to_buffer2.unwrap())(self.handle, copy_image_to_buffer_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResolveImage2.html>
    #[inline]
    pub fn resolve_image2(&self, resolve_image_info: &ResolveImageInfo2) {
        unsafe { (self.fns().v1_3.resolve_image2.unwrap())(self.handle, resolve_image_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetEvent2.html>
    #[inline]
    pub fn set_event2(&self, event: Event, dependency_info: &DependencyInfo) {
        unsafe { (self.fns().v1_3.set_event2.unwrap())(self.handle, event, dependency_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdResetEvent2.html>
    #[inline]
    pub fn reset_event2(&self, event: Event, stage_mask: PipelineStageFlags2) {
        unsafe { (self.fns().v1_3.reset_event2.unwrap())(self.handle, event, stage_mask) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWaitEvents2.html>
    #[inline]
    pub fn wait_events2(&self, events: &[Event], dependency_infos: &[DependencyInfo]) {
        assert_eq!(events.len(), dependency_infos.len());
        unsafe {
            (self.fns().v1_3.wait_events2.unwrap())(
                self.handle,
                events.len() as u32,
                events.as_ptr(),
                dependency_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPipelineBarrier2.html>
    #[inline]
    pub fn pipeline_barrier2(&self, dependency_info: &DependencyInfo) {
        unsafe { (self.fns().v1_3.pipeline_barrier2.unwrap())(self.handle, dependency_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteTimestamp2.html>
    #[inline]
    pub fn write_timestamp2(&self, stage: PipelineStageFlags2, query_pool: QueryPool, query: u32) {
        unsafe {
            (self.fns().v1_3.write_timestamp2.unwrap())(self.handle, stage, query_pool, query)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginRendering.html>
    #[inline]
    pub fn begin_rendering(&self, rendering_info: &RenderingInfo) {
        unsafe { (self.fns().v1_3.begin_rendering.unwrap())(self.handle, rendering_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering.html>
    #[inline]
    pub fn end_rendering(&self) {
        unsafe { (self.fns().v1_3.end_rendering.unwrap())(self.handle) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorSets2.html>
    #[inline]
    pub fn bind_descriptor_sets2(&self, bind_descriptor_sets_info: &BindDescriptorSetsInfo) {
        unsafe {
            (self.fns().v1_4.bind_descriptor_sets2.unwrap())(self.handle, bind_descriptor_sets_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushConstants2.html>
    #[inline]
    pub fn push_constants2(&self, push_constants_info: &PushConstantsInfo) {
        unsafe { (self.fns().v1_4.push_constants2.unwrap())(self.handle, push_constants_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSet2.html>
    #[inline]
    pub fn push_descriptor_set2(&self, push_descriptor_set_info: &PushDescriptorSetInfo) {
        unsafe {
            (self.fns().v1_4.push_descriptor_set2.unwrap())(self.handle, push_descriptor_set_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDescriptorSetWithTemplate2.html>
    #[inline]
    pub fn push_descriptor_set_with_template2(
        &self,
        push_descriptor_set_with_template_info: &PushDescriptorSetWithTemplateInfo,
    ) {
        unsafe {
            (self.fns().v1_4.push_descriptor_set_with_template2.unwrap())(
                self.handle,
                push_descriptor_set_with_template_info,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingAttachmentLocations.html>
    #[inline]
    pub fn set_rendering_attachment_locations(
        &self,
        location_info: &RenderingAttachmentLocationInfo,
    ) {
        unsafe {
            (self.fns().v1_4.set_rendering_attachment_locations.unwrap())(
                self.handle,
                location_info,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRenderingInputAttachmentIndices.html>
    #[inline]
    pub fn set_rendering_input_attachment_indices(
        &self,
        input_attachment_index_info: &RenderingInputAttachmentIndexInfo,
    ) {
        unsafe {
            (self
                .fns()
                .v1_4
                .set_rendering_input_attachment_indices
                .unwrap())(self.handle, input_attachment_index_info)
        };
    }
}
