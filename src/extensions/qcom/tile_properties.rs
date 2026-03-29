// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_QCOM_tile_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait TilePropertiesDevice {
    fn get_framebuffer_tile_properties(
        &self,
        framebuffer: Framebuffer,
        properties: &mut [TilePropertiesQCOM],
    ) -> Result<(), vkResult>;

    fn get_dynamic_rendering_tile_properties(
        &self,
        rendering_info: &RenderingInfo,
    ) -> Result<TilePropertiesQCOM<'_>, vkResult>;
}

impl TilePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>
    #[inline]
    fn get_framebuffer_tile_properties(
        &self,
        framebuffer: Framebuffer,
        properties: &mut [TilePropertiesQCOM],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .qcom_tile_properties
                .as_ref()
                .unwrap()
                .get_framebuffer_tile_properties_qcom)(
                self.handle,
                framebuffer,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html>
    #[inline]
    fn get_dynamic_rendering_tile_properties(
        &self,
        rendering_info: &RenderingInfo,
    ) -> Result<TilePropertiesQCOM<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .qcom_tile_properties
                .as_ref()
                .unwrap()
                .get_dynamic_rendering_tile_properties_qcom)(
                self.handle,
                rendering_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
