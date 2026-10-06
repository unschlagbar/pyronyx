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
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_KHR_pipeline_executable_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelineExecutablePropertiesDevice {
    fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        properties: &mut [PipelineExecutablePropertiesKHR],
    ) -> Result<()>;
    fn get_pipeline_executable_properties_len(
        &self,
        pipeline_info: &PipelineInfoKHR,
    ) -> Result<usize>;

    fn get_pipeline_executable_statistics(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: &mut [PipelineExecutableStatisticKHR],
    ) -> Result<()>;
    fn get_pipeline_executable_statistics_len(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
    ) -> Result<usize>;

    fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: &mut [PipelineExecutableInternalRepresentationKHR],
    ) -> Result<()>;
    fn get_pipeline_executable_internal_representations_len(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
    ) -> Result<usize>;
}

impl PipelineExecutablePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html>
    ///
    /// Call [`get_pipeline_executable_properties_len()`][`Self::get_pipeline_executable_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        properties: &mut [PipelineExecutablePropertiesKHR],
    ) -> Result<()> {
        let mut executable_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_pipeline_executable_properties
            .get_pipeline_executable_properties_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                pipeline_info,
                &mut executable_count,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_pipeline_executable_properties`][`Self::get_pipeline_executable_properties`].
    #[inline]
    fn get_pipeline_executable_properties_len(
        &self,
        pipeline_info: &PipelineInfoKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_pipeline_executable_properties
                .get_pipeline_executable_properties_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                pipeline_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html>
    ///
    /// Call [`get_pipeline_executable_statistics_len()`][`Self::get_pipeline_executable_statistics_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_pipeline_executable_statistics(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: &mut [PipelineExecutableStatisticKHR],
    ) -> Result<()> {
        let mut statistic_count = statistics.len() as u32;
        let call = self
            .fns()
            .khr_pipeline_executable_properties
            .get_pipeline_executable_statistics_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                executable_info,
                &mut statistic_count,
                statistics.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_pipeline_executable_statistics`][`Self::get_pipeline_executable_statistics`].
    #[inline]
    fn get_pipeline_executable_statistics_len(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_pipeline_executable_properties
                .get_pipeline_executable_statistics_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                executable_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    ///
    /// Call [`get_pipeline_executable_internal_representations_len()`][`Self::get_pipeline_executable_internal_representations_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: &mut [PipelineExecutableInternalRepresentationKHR],
    ) -> Result<()> {
        let mut internal_representation_count = internal_representations.len() as u32;
        let call = self
            .fns()
            .khr_pipeline_executable_properties
            .get_pipeline_executable_internal_representations_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                executable_info,
                &mut internal_representation_count,
                internal_representations.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_pipeline_executable_internal_representations`][`Self::get_pipeline_executable_internal_representations`].
    #[inline]
    fn get_pipeline_executable_internal_representations_len(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_pipeline_executable_properties
                .get_pipeline_executable_internal_representations_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                executable_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
