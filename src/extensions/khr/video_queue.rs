// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_video_queue";
pub const SPEC_VERSION: u32 = 8;

pub trait VideoQueueCommandBuffer {
    fn begin_video_coding(&self, begin_info: &VideoBeginCodingInfoKHR);

    fn control_video_coding(&self, coding_control_info: &VideoCodingControlInfoKHR);

    fn end_video_coding(&self, end_coding_info: &VideoEndCodingInfoKHR);
}

impl VideoQueueCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html>
    #[inline]
    fn begin_video_coding(&self, begin_info: &VideoBeginCodingInfoKHR) {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .begin_video_coding_khr)(self.handle, begin_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html>
    #[inline]
    fn control_video_coding(&self, coding_control_info: &VideoCodingControlInfoKHR) {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .control_video_coding_khr)(self.handle, coding_control_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html>
    #[inline]
    fn end_video_coding(&self, end_coding_info: &VideoEndCodingInfoKHR) {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .end_video_coding_khr)(self.handle, end_coding_info)
        };
    }
}

pub trait VideoQueueDevice {
    fn create_video_session(
        &self,
        create_info: &VideoSessionCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionKHR, vkResult>;

    fn destroy_video_session(
        &self,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn create_video_session_parameters(
        &self,
        create_info: &VideoSessionParametersCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionParametersKHR, vkResult>;

    fn update_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> Result<(), vkResult>;

    fn destroy_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_video_session_memory_requirements(
        &self,
        video_session: VideoSessionKHR,
        memory_requirements: &mut [VideoSessionMemoryRequirementsKHR],
    ) -> Result<(), vkResult>;

    fn bind_video_session_memory(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> Result<(), vkResult>;
}

impl VideoQueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html>
    #[inline]
    fn create_video_session(
        &self,
        create_info: &VideoSessionCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .create_video_session_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionKHR.html>
    #[inline]
    fn destroy_video_session(
        &self,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .destroy_video_session_khr)(
                self.handle,
                video_session,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionParametersKHR.html>
    #[inline]
    fn create_video_session_parameters(
        &self,
        create_info: &VideoSessionParametersCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionParametersKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .create_video_session_parameters_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateVideoSessionParametersKHR.html>
    #[inline]
    fn update_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .update_video_session_parameters_khr)(
                self.handle,
                video_session_parameters,
                update_info,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html>
    #[inline]
    fn destroy_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .destroy_video_session_parameters_khr)(
                self.handle,
                video_session_parameters,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html>
    #[inline]
    fn get_video_session_memory_requirements(
        &self,
        video_session: VideoSessionKHR,
        memory_requirements: &mut [VideoSessionMemoryRequirementsKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .get_video_session_memory_requirements_khr)(
                self.handle,
                video_session,
                memory_requirements.len() as *mut u32,
                memory_requirements.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html>
    #[inline]
    fn bind_video_session_memory(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .bind_video_session_memory_khr)(
                self.handle,
                video_session,
                bind_session_memory_infos.len() as u32,
                bind_session_memory_infos.as_ptr(),
            )
        }
        .result()
    }
}

pub trait VideoQueuePhysicalDevice {
    fn get_video_capabilities(
        &self,
        video_profile: &VideoProfileInfoKHR,
    ) -> Result<VideoCapabilitiesKHR<'_>, vkResult>;

    fn get_video_format_properties(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: &mut [VideoFormatPropertiesKHR],
    ) -> Result<(), vkResult>;
}

impl VideoQueuePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    #[inline]
    fn get_video_capabilities(
        &self,
        video_profile: &VideoProfileInfoKHR,
    ) -> Result<VideoCapabilitiesKHR<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .get_physical_device_video_capabilities_khr)(
                self.handle,
                video_profile,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    #[inline]
    fn get_video_format_properties(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: &mut [VideoFormatPropertiesKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .as_ref()
                .unwrap()
                .get_physical_device_video_format_properties_khr)(
                self.handle,
                video_format_info,
                video_format_properties.len() as *mut u32,
                video_format_properties.as_mut_ptr(),
            )
        }
        .result()
    }
}
