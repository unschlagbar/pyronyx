// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_pipeline_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelinePropertiesDevice {
    fn get_pipeline_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        pipeline_properties: &mut BaseOutStructure<'_>,
    ) -> Result<()>;
}

impl PipelinePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html>
    #[inline]
    fn get_pipeline_properties(
        &self,
        pipeline_info: &PipelineInfoKHR,
        pipeline_properties: &mut BaseOutStructure<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_pipeline_properties
            .get_pipeline_properties_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, pipeline_info, pipeline_properties) }.result()
    }
}
