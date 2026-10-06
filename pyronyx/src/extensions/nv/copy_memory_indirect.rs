// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Promoted to [`VK_KHR_copy_memory_indirect`](crate::khr::copy_memory_indirect)
///
/// Requires: (([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_buffer_device_address`](crate::khr::buffer_device_address)) or Vulkan 1.2
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
        let call = self
            .fns()
            .nv_copy_memory_indirect
            .copy_memory_indirect_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, copy_buffer_address, copy_count, stride) };
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
        let call = self
            .fns()
            .nv_copy_memory_indirect
            .copy_memory_to_image_indirect_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
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
