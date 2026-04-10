// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_video_decode_queue";
pub const SPEC_VERSION: u32 = 8;

pub trait VideoDecodeQueueCommandBuffer {
    fn decode_video(&self, decode_info: &VideoDecodeInfoKHR);
}

impl VideoDecodeQueueCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDecodeVideoKHR.html>
    ///
    /// Queues types: `VideoDecodeKHR`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn decode_video(&self, decode_info: &VideoDecodeInfoKHR) {
        let call = self
            .fns()
            .khr_video_decode_queue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .decode_video_khr;

        unsafe { (call)(self.handle, decode_info) };
    }
}
