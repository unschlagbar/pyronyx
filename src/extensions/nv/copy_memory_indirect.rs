// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_copy_memory_indirect";
pub const SPEC_VERSION: u32 = 1;

pub trait CopyMemoryIndirectCommandBuffer {
    fn copy_memory_indirect(
        &self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    );

    fn copy_memory_to_image_indirect(
        &self,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    );
}

impl CopyMemoryIndirectCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectNV.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_indirect(
        &self,
        copy_buffer_address: DeviceAddress,
        copy_count: u32,
        stride: u32,
    ) {
        unsafe {
            (self
                .fns()
                .nv_copy_memory_indirect
                .as_ref()
                .unwrap()
                .copy_memory_indirect_nv)(
                self.handle, copy_buffer_address, copy_count, stride
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectNV.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_to_image_indirect(
        &self,
        copy_buffer_address: DeviceAddress,
        stride: u32,
        dst_image: Image,
        dst_image_layout: ImageLayout,
        image_subresources: &[ImageSubresourceLayers],
    ) {
        unsafe {
            (self
                .fns()
                .nv_copy_memory_indirect
                .as_ref()
                .unwrap()
                .copy_memory_to_image_indirect_nv)(
                self.handle,
                copy_buffer_address,
                image_subresources.len() as u32,
                stride,
                dst_image,
                dst_image_layout,
                image_subresources.as_ptr(),
            )
        };
    }
}
