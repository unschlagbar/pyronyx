// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: (([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_EXT_pipeline_creation_cache_control`](crate::ext::pipeline_creation_cache_control)) or Vulkan 1.3
pub const NAME: &CStr = c"VK_EXT_shader_module_identifier";
pub const SPEC_VERSION: u32 = 1;

pub trait ShaderModuleIdentifierDevice {
    fn get_shader_module_identifier(
        &self,
        shader_module: ShaderModule,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    );

    fn get_shader_module_create_info_identifier(
        &self,
        create_info: &ShaderModuleCreateInfo,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    );
}

impl ShaderModuleIdentifierDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleIdentifierEXT.html>
    #[inline]
    fn get_shader_module_identifier(
        &self,
        shader_module: ShaderModule,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    ) {
        let call = self
            .fns()
            .ext_shader_module_identifier
            .get_shader_module_identifier_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, shader_module, identifier) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderModuleCreateInfoIdentifierEXT.html>
    #[inline]
    fn get_shader_module_create_info_identifier(
        &self,
        create_info: &ShaderModuleCreateInfo,
        identifier: &mut ShaderModuleIdentifierEXT<'_>,
    ) {
        let call = self
            .fns()
            .ext_shader_module_identifier
            .get_shader_module_create_info_identifier_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, create_info, identifier) };
    }
}
