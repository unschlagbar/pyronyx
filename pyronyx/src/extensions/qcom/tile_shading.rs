// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_QCOM_tile_properties`](crate::qcom::tile_properties)
pub const NAME: &CStr = c"VK_QCOM_tile_shading";
pub const SPEC_VERSION: u32 = 2;

pub trait TileShadingCommandBuffer {
    fn dispatch_tile(&self, dispatch_tile_info: &DispatchTileInfoQCOM);

    fn begin_per_tile_execution(&self, per_tile_begin_info: &PerTileBeginInfoQCOM);

    fn end_per_tile_execution(&self, per_tile_end_info: &PerTileEndInfoQCOM);
}

impl TileShadingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html>
    ///
    /// Affected by Conditional Rendering.
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn dispatch_tile(&self, dispatch_tile_info: &DispatchTileInfoQCOM) {
        let call = self
            .fns()
            .qcom_tile_shading
            .dispatch_tile_qcom
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, dispatch_tile_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_per_tile_execution(&self, per_tile_begin_info: &PerTileBeginInfoQCOM) {
        let call = self
            .fns()
            .qcom_tile_shading
            .begin_per_tile_execution_qcom
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, per_tile_begin_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_per_tile_execution(&self, per_tile_end_info: &PerTileEndInfoQCOM) {
        let call = self
            .fns()
            .qcom_tile_shading
            .end_per_tile_execution_qcom
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, per_tile_end_info) };
    }
}
