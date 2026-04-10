// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_maintenance10";
pub const SPEC_VERSION: u32 = 1;

pub trait Maintenance10CommandBuffer {
    fn end_rendering2(&self, rendering_end_info: Option<&RenderingEndInfoKHR>);
}

impl Maintenance10CommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndRendering2KHR.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_rendering2(&self, rendering_end_info: Option<&RenderingEndInfoKHR>) {
        let call = self
            .fns()
            .khr_maintenance10
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .end_rendering2_khr;

        unsafe { (call)(self.handle, rendering_end_info.map_or(null(), from_ref)) };
    }
}
