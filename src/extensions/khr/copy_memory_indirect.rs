// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_copy_memory_indirect";
pub const SPEC_VERSION: u32 = 1;

pub trait CopyMemoryIndirectCommandBuffer {
    fn copy_memory_indirect(&self, copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR);

    fn copy_memory_to_image_indirect(
        &self,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    );
}

impl CopyMemoryIndirectCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryIndirectKHR.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_indirect(&self, copy_memory_indirect_info: &CopyMemoryIndirectInfoKHR) {
        unsafe {
            (self
                .fns()
                .khr_copy_memory_indirect
                .as_ref()
                .unwrap()
                .copy_memory_indirect_khr)(self.handle, copy_memory_indirect_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToImageIndirectKHR.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_to_image_indirect(
        &self,
        copy_memory_to_image_indirect_info: &CopyMemoryToImageIndirectInfoKHR,
    ) {
        unsafe {
            (self
                .fns()
                .khr_copy_memory_indirect
                .as_ref()
                .unwrap()
                .copy_memory_to_image_indirect_khr)(
                self.handle, copy_memory_to_image_indirect_info
            )
        };
    }
}
