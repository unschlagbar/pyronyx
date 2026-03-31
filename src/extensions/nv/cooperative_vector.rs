// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_cooperative_vector";
pub const SPEC_VERSION: u32 = 4;

pub trait CooperativeVectorPhysicalDevice {
    fn get_cooperative_vector_properties(
        &self,
        properties: &mut [CooperativeVectorPropertiesNV],
    ) -> Result<(), Error>;
}

impl CooperativeVectorPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeVectorPropertiesNV.html>
    #[inline]
    fn get_cooperative_vector_properties(
        &self,
        properties: &mut [CooperativeVectorPropertiesNV],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_cooperative_vector
                .as_ref()
                .unwrap()
                .get_physical_device_cooperative_vector_properties_nv)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }
}

pub trait CooperativeVectorDevice {
    fn convert_cooperative_vector_matrix(
        &self,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> Result<(), Error>;
}

impl CooperativeVectorDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkConvertCooperativeVectorMatrixNV.html>
    #[inline]
    fn convert_cooperative_vector_matrix(
        &self,
        info: &ConvertCooperativeVectorMatrixInfoNV,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_cooperative_vector
                .as_ref()
                .unwrap()
                .convert_cooperative_vector_matrix_nv)(self.handle, info)
        }
        .result()
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
        unsafe {
            (self
                .fns()
                .nv_cooperative_vector
                .as_ref()
                .unwrap()
                .convert_cooperative_vector_matrix_nv)(
                self.handle,
                infos.len() as u32,
                infos.as_ptr(),
            )
        };
    }
}
