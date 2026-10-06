// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_video_queue`](crate::khr::video_queue) + ([`VK_KHR_synchronization2`](crate::khr::synchronization2) or Vulkan 1.3)
pub const NAME: &CStr = c"VK_KHR_video_encode_queue";
pub const SPEC_VERSION: u32 = 12;

pub trait VideoEncodeQueuePhysicalDevice {
    fn get_video_encode_quality_level_properties(
        &self,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> Result<()>;
}

impl VideoEncodeQueuePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR.html>
    #[inline]
    fn get_video_encode_quality_level_properties(
        &self,
        quality_level_info: &PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: &mut VideoEncodeQualityLevelPropertiesKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_video_encode_queue
            .get_physical_device_video_encode_quality_level_properties_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, quality_level_info, quality_level_properties) }.result()
    }
}

pub trait VideoEncodeQueueDevice {
    fn get_encoded_video_session_parameters(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
        data: &mut [u8],
    ) -> Result<()>;
    fn get_encoded_video_session_parameters_len(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    ) -> Result<usize>;
}

impl VideoEncodeQueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEncodedVideoSessionParametersKHR.html>
    ///
    /// Call [`get_encoded_video_session_parameters_len()`][`Self::get_encoded_video_session_parameters_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_encoded_video_session_parameters(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
        data: &mut [u8],
    ) -> Result<()> {
        let mut data_size = data.len();
        let call = self
            .fns()
            .khr_video_encode_queue
            .get_encoded_video_session_parameters_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                video_session_parameters_info,
                feedback_info,
                &mut data_size,
                data.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_encoded_video_session_parameters`][`Self::get_encoded_video_session_parameters`].
    #[inline]
    fn get_encoded_video_session_parameters_len(
        &self,
        video_session_parameters_info: &VideoEncodeSessionParametersGetInfoKHR,
        feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<usize> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_video_encode_queue
                .get_encoded_video_session_parameters_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                video_session_parameters_info,
                feedback_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
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
            .encode_video_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, encode_info) };
    }
}
