// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: (Vulkan 1.1 + [`VK_KHR_synchronization2`](crate::khr::synchronization2)) or Vulkan 1.3
pub const NAME: &CStr = c"VK_KHR_video_queue";
pub const SPEC_VERSION: u32 = 8;

pub trait VideoQueuePhysicalDevice {
    fn get_video_capabilities(
        &self,
        video_profile: &VideoProfileInfoKHR,
        capabilities: &mut VideoCapabilitiesKHR<'_>,
    ) -> Result<()>;

    fn get_video_format_properties(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: &mut [VideoFormatPropertiesKHR],
    ) -> Result<()>;
    fn get_video_format_properties_len(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
    ) -> Result<usize>;
}

impl VideoQueuePhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoCapabilitiesKHR.html>
    #[inline]
    fn get_video_capabilities(
        &self,
        video_profile: &VideoProfileInfoKHR,
        capabilities: &mut VideoCapabilitiesKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_video_queue
            .get_physical_device_video_capabilities_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, video_profile, capabilities) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceVideoFormatPropertiesKHR.html>
    ///
    /// Call [`get_video_format_properties_len()`][`Self::get_video_format_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_video_format_properties(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
        video_format_properties: &mut [VideoFormatPropertiesKHR],
    ) -> Result<()> {
        let mut video_format_property_count = video_format_properties.len() as u32;
        let call = self
            .fns()
            .khr_video_queue
            .get_physical_device_video_format_properties_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                video_format_info,
                &mut video_format_property_count,
                video_format_properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_video_format_properties`][`Self::get_video_format_properties`].
    #[inline]
    fn get_video_format_properties_len(
        &self,
        video_format_info: &PhysicalDeviceVideoFormatInfoKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .get_physical_device_video_format_properties_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                video_format_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}

pub trait VideoQueueDevice {
    fn create_video_session(
        &self,
        create_info: &VideoSessionCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionKHR>;

    fn destroy_video_session(
        &self,
        video_session: VideoSessionKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn create_video_session_parameters(
        &self,
        create_info: &VideoSessionParametersCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionParametersKHR>;

    fn update_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        update_info: &VideoSessionParametersUpdateInfoKHR,
    ) -> Result<()>;

    fn destroy_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_video_session_memory_requirements(
        &self,
        video_session: VideoSessionKHR,
        memory_requirements: &mut [VideoSessionMemoryRequirementsKHR],
    ) -> Result<()>;
    fn get_video_session_memory_requirements_len(
        &self,
        video_session: VideoSessionKHR,
    ) -> Result<usize>;

    fn bind_video_session_memory(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> Result<()>;
}

impl VideoQueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateVideoSessionKHR.html>
    #[inline]
    fn create_video_session(
        &self,
        create_info: &VideoSessionCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<VideoSessionKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_video_queue
            .create_video_session_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
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
        let call = self
            .fns()
            .khr_video_queue
            .destroy_video_session_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
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
    ) -> Result<VideoSessionParametersKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_video_queue
            .create_video_session_parameters_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
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
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_video_queue
            .update_video_session_parameters_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, video_session_parameters, update_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyVideoSessionParametersKHR.html>
    #[inline]
    fn destroy_video_session_parameters(
        &self,
        video_session_parameters: VideoSessionParametersKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .khr_video_queue
            .destroy_video_session_parameters_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                video_session_parameters,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetVideoSessionMemoryRequirementsKHR.html>
    ///
    /// Call [`get_video_session_memory_requirements_len()`][`Self::get_video_session_memory_requirements_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_video_session_memory_requirements(
        &self,
        video_session: VideoSessionKHR,
        memory_requirements: &mut [VideoSessionMemoryRequirementsKHR],
    ) -> Result<()> {
        let mut memory_requirements_count = memory_requirements.len() as u32;
        let call = self
            .fns()
            .khr_video_queue
            .get_video_session_memory_requirements_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                video_session,
                &mut memory_requirements_count,
                memory_requirements.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_video_session_memory_requirements`][`Self::get_video_session_memory_requirements`].
    #[inline]
    fn get_video_session_memory_requirements_len(
        &self,
        video_session: VideoSessionKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_video_queue
                .get_video_session_memory_requirements_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                video_session,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindVideoSessionMemoryKHR.html>
    #[inline]
    fn bind_video_session_memory(
        &self,
        video_session: VideoSessionKHR,
        bind_session_memory_infos: &[BindVideoSessionMemoryInfoKHR],
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_video_queue
            .bind_video_session_memory_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                video_session,
                bind_session_memory_infos.len() as u32,
                bind_session_memory_infos.as_ptr(),
            )
        }
        .result()
    }
}

pub trait VideoQueueCommandBuffer {
    fn begin_video_coding(&self, begin_info: &VideoBeginCodingInfoKHR);

    fn control_video_coding(&self, coding_control_info: &VideoCodingControlInfoKHR);

    fn end_video_coding(&self, end_coding_info: &VideoEndCodingInfoKHR);
}

impl VideoQueueCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginVideoCodingKHR.html>
    ///
    /// Queues types: `VideoDecodeKHR`, `VideoEncodeKHR`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn begin_video_coding(&self, begin_info: &VideoBeginCodingInfoKHR) {
        let call = self
            .fns()
            .khr_video_queue
            .begin_video_coding_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, begin_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdControlVideoCodingKHR.html>
    ///
    /// Queues types: `VideoDecodeKHR`, `VideoEncodeKHR`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn control_video_coding(&self, coding_control_info: &VideoCodingControlInfoKHR) {
        let call = self
            .fns()
            .khr_video_queue
            .control_video_coding_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, coding_control_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndVideoCodingKHR.html>
    ///
    /// Queues types: `VideoDecodeKHR`, `VideoEncodeKHR`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`.
    #[inline]
    fn end_video_coding(&self, end_coding_info: &VideoEndCodingInfoKHR) {
        let call = self
            .fns()
            .khr_video_queue
            .end_video_coding_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, end_coding_info) };
    }
}
