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
/// Requires: Vulkan 1.4 or [`VK_KHR_extended_flags`](crate::khr::extended_flags) or [`VK_KHR_maintenance5`](crate::khr::maintenance5)
pub const NAME: &CStr = c"VK_KHR_pipeline_binary";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelineBinaryDevice {
    fn create_pipeline_binaries(
        &self,
        create_info: &PipelineBinaryCreateInfoKHR,
        allocator: Option<&AllocationCallbacks>,
        binaries: &mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> Result<()>;

    fn destroy_pipeline_binary(
        &self,
        pipeline_binary: PipelineBinaryKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_pipeline_key(
        &self,
        pipeline_create_info: Option<&PipelineCreateInfoKHR>,
        pipeline_key: &mut PipelineBinaryKeyKHR<'_>,
    ) -> Result<()>;

    fn get_pipeline_binary_data(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        pipeline_binary_data: &mut [u8],
    ) -> Result<()>;
    fn get_pipeline_binary_data_len(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
    ) -> Result<usize>;

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
        binaries: &mut PipelineBinaryHandlesInfoKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_pipeline_binary
            .create_pipeline_binaries_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                binaries,
            )
        }
        .result()
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
            .destroy_pipeline_binary_khr
            .unwrap_or_else(|| Self::ext_load_error());

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
        pipeline_key: &mut PipelineBinaryKeyKHR<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_pipeline_binary
            .get_pipeline_key_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                pipeline_create_info.map_or(null(), from_ref),
                pipeline_key,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineBinaryDataKHR.html>
    ///
    /// Call [`get_pipeline_binary_data_len()`][`Self::get_pipeline_binary_data_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_pipeline_binary_data(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
        pipeline_binary_data: &mut [u8],
    ) -> Result<()> {
        let mut pipeline_binary_data_size = pipeline_binary_data.len();
        let call = self
            .fns()
            .khr_pipeline_binary
            .get_pipeline_binary_data_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                info,
                pipeline_binary_key,
                &mut pipeline_binary_data_size,
                pipeline_binary_data.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_pipeline_binary_data`][`Self::get_pipeline_binary_data`].
    #[inline]
    fn get_pipeline_binary_data_len(
        &self,
        info: &PipelineBinaryDataInfoKHR,
        pipeline_binary_key: *mut PipelineBinaryKeyKHR,
    ) -> Result<usize> {
        let mut out: MaybeUninit<usize> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_pipeline_binary
                .get_pipeline_binary_data_khr
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                info,
                pipeline_binary_key,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
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
            .release_captured_pipeline_data_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, allocator.map_or(null(), from_ref)) }.result()
    }
}
