// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Promoted to [`VK_EXT_memory_decompression`](crate::ext::memory_decompression)
///
/// Requires: (([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_buffer_device_address`](crate::khr::buffer_device_address)) or Vulkan 1.2
pub const NAME: &CStr = c"VK_NV_memory_decompression";
pub const SPEC_VERSION: u32 = 1;

pub trait MemoryDecompressionCommandBuffer {
    fn decompress_memory(&self, decompress_memory_regions: &[DecompressMemoryRegionNV]);

    fn decompress_memory_indirect_count(
        &self,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    );
}

impl MemoryDecompressionCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn decompress_memory(&self, decompress_memory_regions: &[DecompressMemoryRegionNV]) {
        let call = self
            .fns()
            .nv_memory_decompression
            .decompress_memory_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                decompress_memory_regions.len() as u32,
                decompress_memory_regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn decompress_memory_indirect_count(
        &self,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        let call = self
            .fns()
            .nv_memory_decompression
            .decompress_memory_indirect_count_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                indirect_commands_address,
                indirect_commands_count_address,
                stride,
            )
        };
    }
}
