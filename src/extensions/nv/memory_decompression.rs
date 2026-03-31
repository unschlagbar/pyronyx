// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
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
    #[inline]
    fn decompress_memory(&self, decompress_memory_regions: &[DecompressMemoryRegionNV]) {
        unsafe {
            (self
                .fns()
                .nv_memory_decompression
                .as_ref()
                .unwrap()
                .decompress_memory_nv)(
                self.handle,
                decompress_memory_regions.len() as u32,
                decompress_memory_regions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecompressMemoryIndirectCountNV.html>
    #[inline]
    fn decompress_memory_indirect_count(
        &self,
        indirect_commands_address: DeviceAddress,
        indirect_commands_count_address: DeviceAddress,
        stride: u32,
    ) {
        unsafe {
            (self
                .fns()
                .nv_memory_decompression
                .as_ref()
                .unwrap()
                .decompress_memory_indirect_count_nv)(
                self.handle,
                indirect_commands_address,
                indirect_commands_count_address,
                stride,
            )
        };
    }
}
