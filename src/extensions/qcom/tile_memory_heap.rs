// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_QCOM_tile_memory_heap";
pub const SPEC_VERSION: u32 = 1;

pub trait TileMemoryHeapCommandBuffer {
    fn bind_tile_memory(&self, tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM>);
}

impl TileMemoryHeapCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindTileMemoryQCOM.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_tile_memory(&self, tile_memory_bind_info: Option<&TileMemoryBindInfoQCOM>) {
        let call = self
            .fns()
            .qcom_tile_memory_heap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .bind_tile_memory_qcom;

        unsafe { (call)(self.handle, tile_memory_bind_info.map_or(null(), from_ref)) };
    }
}
