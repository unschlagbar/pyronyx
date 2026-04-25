// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_pipeline_binary";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelineBinaryDevice {
    fn create_pipeline_binaries(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PipelineBinaryHandlesInfoKHR<'_>>;

    fn destroy_pipeline_binary(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
    ) -> Result<PipelineBinaryKeyKHR<'_>>;

    fn get_pipeline_binary_data(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        pipeline_binary_data: &mut [u8],
    ) -> Result<()>;

    fn release_captured_pipeline_data(
        &self,
        info: &ReleaseCapturedPipelineDataInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<()>;
}

impl PipelineBinaryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineBinariesKHR.html>
    #[inline]
    fn create_pipeline_binaries(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PipelineBinaryHandlesInfoKHR<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_pipeline_binary
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_pipeline_binaries_khr;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineBinaryKHR.html>
    #[inline]
    fn destroy_pipeline_binary(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .khr_pipeline_binary
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_pipeline_binary_khr;

        unsafe {
            (call)(
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
    ) -> Result<PipelineBinaryKeyKHR<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_pipeline_binary
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_pipeline_key_khr;

        unsafe {
            (call)(
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
        pipeline_binary_data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_pipeline_binary
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_pipeline_binary_data_khr;

        unsafe {
            (call)(
                self.handle,
                info,
                pipeline_binary_key,
                pipeline_binary_data.len() as *mut usize,
                pipeline_binary_data.as_mut_ptr().cast(),
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
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_pipeline_binary
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .release_captured_pipeline_data_khr;

        unsafe { (call)(self.handle, info, allocator.map_or(null(), from_ref)) }.result()
    }
}
