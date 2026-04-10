// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_shader_module_identifier";
pub const SPEC_VERSION: u32 = 1;

pub trait ShaderModuleIdentifierDevice {
    fn get_shader_module_identifier(
        &self,
        shader_module: ShaderModule,
    ) -> ShaderModuleIdentifierEXT<'_>;

    fn get_shader_module_create_info_identifier(
        &self,
        create_info: &ShaderModuleCreateInfo,
    ) -> ShaderModuleIdentifierEXT<'_>;
}

impl ShaderModuleIdentifierDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html>
    #[inline]
    fn get_shader_module_identifier(
        &self,
        shader_module: ShaderModule,
    ) -> ShaderModuleIdentifierEXT<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_shader_module_identifier
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_shader_module_identifier_ext;

        unsafe {
            (call)(self.handle, shader_module, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html>
    #[inline]
    fn get_shader_module_create_info_identifier(
        &self,
        create_info: &ShaderModuleCreateInfo,
    ) -> ShaderModuleIdentifierEXT<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_shader_module_identifier
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_shader_module_create_info_identifier_ext;

        unsafe {
            (call)(self.handle, create_info, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
