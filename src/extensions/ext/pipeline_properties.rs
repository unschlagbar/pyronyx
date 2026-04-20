// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_pipeline_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait PipelinePropertiesDevice {
    fn get_pipeline_properties(
        &self,
        pipeline_info: &PipelineInfoEXT,
    ) -> Result<BaseOutStructure<'_>>;
}

impl PipelinePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelinePropertiesEXT.html>
    #[inline]
    fn get_pipeline_properties(
        &self,
        pipeline_info: &PipelineInfoEXT,
    ) -> Result<BaseOutStructure<'_>> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_pipeline_properties
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_pipeline_properties_ext;

        unsafe { (call)(self.handle, pipeline_info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
