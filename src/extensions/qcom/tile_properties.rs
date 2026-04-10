// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_QCOM_tile_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait TilePropertiesDevice {
    fn get_framebuffer_tile_properties(
        &self,
        framebuffer: Framebuffer,
        properties: &mut [TilePropertiesQCOM],
    ) -> Result<(), Error>;

    fn get_dynamic_rendering_tile_properties(
        &self,
        rendering_info: &RenderingInfo,
    ) -> Result<TilePropertiesQCOM<'_>, Error>;
}

impl TilePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>
    #[inline]
    fn get_framebuffer_tile_properties(
        &self,
        framebuffer: Framebuffer,
        properties: &mut [TilePropertiesQCOM],
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .qcom_tile_properties
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_framebuffer_tile_properties_qcom;

        unsafe {
            (call)(
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
    ) -> Result<TilePropertiesQCOM<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .qcom_tile_properties
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_dynamic_rendering_tile_properties_qcom;

        unsafe { (call)(self.handle, rendering_info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
