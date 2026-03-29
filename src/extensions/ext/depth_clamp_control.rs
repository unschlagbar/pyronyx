// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_depth_clamp_control";
pub const SPEC_VERSION: u32 = 1;

pub trait DepthClampControlCommandBuffer {
    fn set_depth_clamp_range(
        &self,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    );
}

impl DepthClampControlCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampRangeEXT.html>
    #[inline]
    fn set_depth_clamp_range(
        &self,
        depth_clamp_mode: DepthClampModeEXT,
        depth_clamp_range: Option<&DepthClampRangeEXT>,
    ) {
        unsafe {
            (self
                .fns()
                .ext_depth_clamp_control
                .as_ref()
                .unwrap()
                .set_depth_clamp_range_ext)(
                self.handle,
                depth_clamp_mode,
                depth_clamp_range.map_or(null(), from_ref),
            )
        };
    }
}
