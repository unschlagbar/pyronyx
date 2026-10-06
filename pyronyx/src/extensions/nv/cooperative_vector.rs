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
pub const NAME: &CStr = c"VK_NV_cooperative_vector";
pub const SPEC_VERSION: u32 = 4;

pub trait CooperativeVectorPhysicalDevice {
    fn get_cooperative_vector_properties(
        &self,
        properties: &mut [CooperativeVectorPropertiesNV],
    ) -> Result<()>;
    fn get_cooperative_vector_properties_len(&self) -> Result<usize>;
}

impl CooperativeVectorPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>
    ///
    /// Call [`get_cooperative_vector_properties_len()`][`Self::get_cooperative_vector_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_cooperative_vector_properties(
        &self,
        properties: &mut [CooperativeVectorPropertiesNV],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .nv_cooperative_vector
            .get_physical_device_cooperative_vector_properties_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_cooperative_vector_properties`][`Self::get_cooperative_vector_properties`].
    #[inline]
    fn get_cooperative_vector_properties_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_cooperative_vector
                .get_physical_device_cooperative_vector_properties_nv
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}

pub trait CooperativeVectorDevice {
    fn convert_cooperative_vector_matrix(
        &self,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> Result<()>;
}

impl CooperativeVectorDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html>
    #[inline]
    fn convert_cooperative_vector_matrix(
        &self,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_cooperative_vector
            .convert_cooperative_vector_matrix_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info) }.result()
    }
}

pub trait CooperativeVectorCommandBuffer {
    fn convert_cooperative_vector_matrix(&self, infos: &[ConvertCooperativeVectorMatrixInfoNV]);
}

impl CooperativeVectorCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdConvertCooperativeVectorMatrixNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn convert_cooperative_vector_matrix(&self, infos: &[ConvertCooperativeVectorMatrixInfoNV]) {
        let call = self
            .fns()
            .nv_cooperative_vector
            .convert_cooperative_vector_matrix_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, infos.len() as u32, infos.as_ptr()) };
    }
}
