// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_AMD_shader_info";
pub const SPEC_VERSION: u32 = 1;

pub trait ShaderInfoDevice {
    fn get_shader_info(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info: &mut [u8],
    ) -> Result<()>;
    fn get_shader_info_len(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
    ) -> Result<usize>;
}

impl ShaderInfoDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInfoAMD.html>
    ///
    /// Call [`get_shader_info_len()`][`Self::get_shader_info_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_shader_info(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
        info: &mut [u8],
    ) -> Result<()> {
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
                info.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_shader_info`][`Self::get_shader_info`].
    #[inline]
    fn get_shader_info_len(
        &self,
        pipeline: Pipeline,
        shader_stage: ShaderStageFlags,
        info_type: ShaderInfoTypeAMD,
    ) -> Result<usize> {
        let mut out: MaybeUninit<usize> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .amd_shader_info
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_shader_info_amd)(
                self.handle,
                pipeline,
                shader_stage,
                info_type,
                out.as_mut_ptr() as *mut usize,
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
    }
}
