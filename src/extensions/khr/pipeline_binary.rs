// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_KHR_pipeline_binary";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelineBinaryDevice {
    fn create_pipeline_binaries(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PipelineBinaryHandlesInfoKHR<'_>, Error>;

    fn destroy_pipeline_binary(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
    ) -> Result<PipelineBinaryKeyKHR<'_>, Error>;

    fn get_pipeline_binary_data(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        pipeline_binary_data: &mut [c_void],
    ) -> Result<(), Error>;

    fn release_captured_pipeline_data(
        &self,
        info: &ReleaseCapturedPipelineDataInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<(), Error>;
}

impl PipelineBinaryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>
    #[inline]
    fn create_pipeline_binaries(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PipelineBinaryHandlesInfoKHR<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_pipeline_binary
                .as_ref()
                .unwrap()
                .create_pipeline_binaries_khr)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>
    #[inline]
    fn destroy_pipeline_binary(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .khr_pipeline_binary
                .as_ref()
                .unwrap()
                .destroy_pipeline_binary_khr)(
                self.handle,
                pipeline_binary,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineKeyKHR.html>
    #[inline]
    fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
    ) -> Result<PipelineBinaryKeyKHR<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_pipeline_binary
                .as_ref()
                .unwrap()
                .get_pipeline_key_khr)(
                self.handle,
                pipeline_create_info.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html>
    #[inline]
    fn get_pipeline_binary_data(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        pipeline_binary_data: &mut [c_void],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_pipeline_binary
                .as_ref()
                .unwrap()
                .get_pipeline_binary_data_khr)(
                self.handle,
                info,
                pipeline_binary_key,
                pipeline_binary_data.len() as *mut usize,
                pipeline_binary_data.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleaseCapturedPipelineDataKHR.html>
    #[inline]
    fn release_captured_pipeline_data(
        &self,
        info: &ReleaseCapturedPipelineDataInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .khr_pipeline_binary
                .as_ref()
                .unwrap()
                .release_captured_pipeline_data_khr)(
                self.handle,
                info,
                allocator.map_or(null(), from_ref),
            )
        }
        .result()
    }
}
