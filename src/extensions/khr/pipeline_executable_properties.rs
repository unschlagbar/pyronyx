// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_KHR_pipeline_executable_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelineExecutablePropertiesDevice {
    fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        properties: &mut [PipelineExecutablePropertiesKHR],
    ) -> Result<(), vkResult>;

    fn get_pipeline_executable_statistics(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: &mut [PipelineExecutableStatisticKHR],
    ) -> Result<(), vkResult>;

    fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: &mut [PipelineExecutableInternalRepresentationKHR],
    ) -> Result<(), vkResult>;
}

impl PipelineExecutablePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutablePropertiesKHR.html>
    #[inline]
    fn get_pipeline_executable_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        properties: &mut [PipelineExecutablePropertiesKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_pipeline_executable_properties
                .as_ref()
                .unwrap()
                .get_pipeline_executable_properties_khr)(
                self.handle,
                pipeline_info,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableStatisticsKHR.html>
    #[inline]
    fn get_pipeline_executable_statistics(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        statistics: &mut [PipelineExecutableStatisticKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_pipeline_executable_properties
                .as_ref()
                .unwrap()
                .get_pipeline_executable_statistics_khr)(
                self.handle,
                executable_info,
                statistics.len() as *mut u32,
                statistics.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineExecutableInternalRepresentationsKHR.html>
    #[inline]
    fn get_pipeline_executable_internal_representations(
        &self,
        executable_info: &PipelineExecutableInfoKHR,
        internal_representations: &mut [PipelineExecutableInternalRepresentationKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_pipeline_executable_properties
                .as_ref()
                .unwrap()
                .get_pipeline_executable_internal_representations_khr)(
                self.handle,
                executable_info,
                internal_representations.len() as *mut u32,
                internal_representations.as_mut_ptr(),
            )
        }
        .result()
    }
}
