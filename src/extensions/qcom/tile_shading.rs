// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_QCOM_tile_shading";
pub const SPEC_VERSION: u32 = 2;

pub trait TileShadingCommandBuffer {
    fn dispatch_tile(&self, dispatch_tile_info: &DispatchTileInfoQCOM);

    fn begin_per_tile_execution(&self, per_tile_begin_info: &PerTileBeginInfoQCOM);

    fn end_per_tile_execution(&self, per_tile_end_info: &PerTileEndInfoQCOM);
}

impl TileShadingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDispatchTileQCOM.html>
    #[inline]
    fn dispatch_tile(&self, dispatch_tile_info: &DispatchTileInfoQCOM) {
        unsafe {
            (self
                .fns()
                .qcom_tile_shading
                .as_ref()
                .unwrap()
                .dispatch_tile_qcom)(self.handle, dispatch_tile_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginPerTileExecutionQCOM.html>
    #[inline]
    fn begin_per_tile_execution(&self, per_tile_begin_info: &PerTileBeginInfoQCOM) {
        unsafe {
            (self
                .fns()
                .qcom_tile_shading
                .as_ref()
                .unwrap()
                .begin_per_tile_execution_qcom)(self.handle, per_tile_begin_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndPerTileExecutionQCOM.html>
    #[inline]
    fn end_per_tile_execution(&self, per_tile_end_info: &PerTileEndInfoQCOM) {
        unsafe {
            (self
                .fns()
                .qcom_tile_shading
                .as_ref()
                .unwrap()
                .end_per_tile_execution_qcom)(self.handle, per_tile_end_info)
        };
    }
}
