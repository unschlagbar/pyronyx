// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: ([`VK_KHR_get_memory_requirements2`](crate::khr::get_memory_requirements2) + [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2)) or Vulkan 1.1
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
            .bind_tile_memory_qcom
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, tile_memory_bind_info.map_or(null(), from_ref)) };
    }
}
