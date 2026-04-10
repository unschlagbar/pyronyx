// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_memory_decompression";
pub const SPEC_VERSION: u32 = 1;

pub trait MemoryDecompressionCommandBuffer {
    fn decompress_memory(&self, decompress_memory_info_ext: &DecompressMemoryInfoEXT);

    fn decompress_memory_indirect_count(
        &self,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    );
}

impl MemoryDecompressionCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn decompress_memory(&self, decompress_memory_info_ext: &DecompressMemoryInfoEXT) {
        let call = self
            .fns()
            .ext_memory_decompression
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .decompress_memory_ext;

        unsafe { (call)(self.handle, decompress_memory_info_ext) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn decompress_memory_indirect_count(
        &self,
        decompression_method: MemoryDecompressionMethodFlagsEXT,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        max_decompression_count: u32,
        stride: u32,
    ) {
        let call = self
            .fns()
            .ext_memory_decompression
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .decompress_memory_indirect_count_ext;

        unsafe {
            (call)(
                self.handle,
                decompression_method,
                indirect_commands_address,
                indirect_commands_count_address,
                max_decompression_count,
                stride,
            )
        };
    }
}
