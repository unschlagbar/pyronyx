// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;

/// Type: `Device`
pub const NAME: &CStr = c"VK_AMD_shader_info";
pub const SPEC_VERSION: u32 = 1;

pub trait ShaderInfoDevice {
    fn get_shader_info(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info: &mut [c_void],
    ) -> Result<(), Error>;
}

impl ShaderInfoDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html>
    #[inline]
    fn get_shader_info(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info: &mut [c_void],
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .amd_shader_info
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_shader_info_amd;

        unsafe {
            (call)(
                self.handle,
                pipeline,
                shader_stage,
                info_type,
                info.len() as *mut usize,
                info.as_mut_ptr(),
            )
        }
        .result()
    }
}
