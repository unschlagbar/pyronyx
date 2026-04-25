// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_video_encode_queue";
pub const SPEC_VERSION: u32 = 12;

pub trait VideoEncodeQueuePhysicalDevice {
    fn get_video_encode_quality_level_properties(
        &self,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
    ) -> Result<VideoEncodeQualityLevelPropertiesKHR<'_>>;
}

impl VideoEncodeQueuePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    #[inline]
    fn get_video_encode_quality_level_properties(
        &self,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
    ) -> Result<VideoEncodeQualityLevelPropertiesKHR<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_video_encode_queue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_video_encode_quality_level_properties_khr;

        unsafe { (call)(self.handle, quality_level_info, out.as_mut_ptr()) }.init_on_success(out)
    }
}

pub trait VideoEncodeQueueDevice {
    fn get_encoded_video_session_parameters(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
        data: &mut [u8],
    ) -> Result<()>;
}

impl VideoEncodeQueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html>
    #[inline]
    fn get_encoded_video_session_parameters(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_video_encode_queue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_encoded_video_session_parameters_khr;

        unsafe {
            (call)(
                self.handle,
                video_session_parameters_info,
                feedback_info,
                data.len() as *mut usize,
                data.as_mut_ptr().cast(),
            )
        }
        .result()
    }
}

pub trait VideoEncodeQueueCommandBuffer {
    fn encode_video(&self, encode_info: &VideoEncodeInfoKHR);
}

impl VideoEncodeQueueCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEncodeVideoKHR.html>
    ///
    /// Queues types: `VideoEncodeKHR`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn encode_video(&self, encode_info: &VideoEncodeInfoKHR) {
        let call = self
            .fns()
            .khr_video_encode_queue
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .encode_video_khr;

        unsafe { (call)(self.handle, encode_info) };
    }
}
