// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_EXT_transform_feedback";
pub const SPEC_VERSION: u32 = 1;

pub trait TransformFeedbackCommandBuffer {
    fn bind_transform_feedback_buffers(
        &self,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: &[DeviceSize],
    );

    fn begin_transform_feedback(
        &self,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: &[DeviceSize],
    );

    fn end_transform_feedback(
        &self,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: &[DeviceSize],
    );

    fn begin_query_indexed(
        &self,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    );

    fn end_query_indexed(&self, query_pool: QueryPool, query: u32, index: u32);

    fn draw_indirect_byte_count(
        &self,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    );
}

impl TransformFeedbackCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTransformFeedbackBuffersEXT.html>
    #[inline]
    fn bind_transform_feedback_buffers(
        &self,
        first_binding: u32,
        buffers: &[Buffer],
        offsets: &[DeviceSize],
        sizes: &[DeviceSize],
    ) {
        assert_eq!(buffers.len(), offsets.len());
        assert_eq!(buffers.len(), sizes.len());
        unsafe {
            (self
                .fns()
                .ext_transform_feedback
                .as_ref()
                .unwrap()
                .bind_transform_feedback_buffers_ext)(
                self.handle,
                first_binding,
                buffers.len() as u32,
                buffers.as_ptr(),
                offsets.as_ptr(),
                sizes.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginTransformFeedbackEXT.html>
    #[inline]
    fn begin_transform_feedback(
        &self,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: &[DeviceSize],
    ) {
        assert_eq!(counter_buffers.len(), counter_buffer_offsets.len());
        unsafe {
            (self
                .fns()
                .ext_transform_feedback
                .as_ref()
                .unwrap()
                .begin_transform_feedback_ext)(
                self.handle,
                first_counter_buffer,
                counter_buffers.len() as u32,
                counter_buffers.as_ptr(),
                counter_buffer_offsets.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndTransformFeedbackEXT.html>
    #[inline]
    fn end_transform_feedback(
        &self,
        first_counter_buffer: u32,
        counter_buffers: &[Buffer],
        counter_buffer_offsets: &[DeviceSize],
    ) {
        assert_eq!(counter_buffers.len(), counter_buffer_offsets.len());
        unsafe {
            (self
                .fns()
                .ext_transform_feedback
                .as_ref()
                .unwrap()
                .end_transform_feedback_ext)(
                self.handle,
                first_counter_buffer,
                counter_buffers.len() as u32,
                counter_buffers.as_ptr(),
                counter_buffer_offsets.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginQueryIndexedEXT.html>
    #[inline]
    fn begin_query_indexed(
        &self,
        query_pool: QueryPool,
        query: u32,
        flags: QueryControlFlags,
        index: u32,
    ) {
        unsafe {
            (self
                .fns()
                .ext_transform_feedback
                .as_ref()
                .unwrap()
                .begin_query_indexed_ext)(self.handle, query_pool, query, flags, index)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndQueryIndexedEXT.html>
    #[inline]
    fn end_query_indexed(&self, query_pool: QueryPool, query: u32, index: u32) {
        unsafe {
            (self
                .fns()
                .ext_transform_feedback
                .as_ref()
                .unwrap()
                .end_query_indexed_ext)(self.handle, query_pool, query, index)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawIndirectByteCountEXT.html>
    #[inline]
    fn draw_indirect_byte_count(
        &self,
        instance_count: u32,
        first_instance: u32,
        counter_buffer: Buffer,
        counter_buffer_offset: DeviceSize,
        counter_offset: u32,
        vertex_stride: u32,
    ) {
        unsafe {
            (self
                .fns()
                .ext_transform_feedback
                .as_ref()
                .unwrap()
                .draw_indirect_byte_count_ext)(
                self.handle,
                instance_count,
                first_instance,
                counter_buffer,
                counter_buffer_offset,
                counter_offset,
                vertex_stride,
            )
        };
    }
}
