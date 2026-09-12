// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_QCOM_tile_properties";
pub const SPEC_VERSION: u32 = 1;

pub trait TilePropertiesDevice {
    fn get_framebuffer_tile_properties(
        &self,
        framebuffer: Framebuffer,
        properties: &mut [TilePropertiesQCOM],
    ) -> Result<()>;
    fn get_framebuffer_tile_properties_len(&self, framebuffer: Framebuffer) -> Result<usize>;

    fn get_dynamic_rendering_tile_properties(
        &self,
        rendering_info: &RenderingInfo,
    ) -> Result<TilePropertiesQCOM<'_>>;
}

impl TilePropertiesDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFramebufferTilePropertiesQCOM.html>
    ///
    /// Call [`get_framebuffer_tile_properties_len()`][`Self::get_framebuffer_tile_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_framebuffer_tile_properties(
        &self,
        framebuffer: Framebuffer,
        properties: &mut [TilePropertiesQCOM],
    ) -> Result<()> {
        let mut properties_count = properties.len() as u32;
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
                &mut properties_count,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_framebuffer_tile_properties`][`Self::get_framebuffer_tile_properties`].
    #[inline]
    fn get_framebuffer_tile_properties_len(&self, framebuffer: Framebuffer) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .qcom_tile_properties
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_framebuffer_tile_properties_qcom)(
                self.handle,
                framebuffer,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDynamicRenderingTilePropertiesQCOM.html>
    #[inline]
    fn get_dynamic_rendering_tile_properties(
        &self,
        rendering_info: &RenderingInfo,
    ) -> Result<TilePropertiesQCOM<'_>> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .qcom_tile_properties
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_dynamic_rendering_tile_properties_qcom;

        unsafe { (call)(self.handle, rendering_info, out.as_mut_ptr()) }.init_on_success(out)
    }
}
