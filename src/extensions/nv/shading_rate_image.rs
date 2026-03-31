// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_shading_rate_image";
pub const SPEC_VERSION: u32 = 3;

pub trait ShadingRateImageCommandBuffer {
    fn bind_shading_rate_image(&self, image_view: ImageView, image_layout: ImageLayout);

    fn set_viewport_shading_rate_palette(
        &self,
        first_viewport: u32,
        shading_rate_palettes: &[ShadingRatePaletteNV],
    );

    fn set_coarse_sample_order(
        &self,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV],
    );
}

impl ShadingRateImageCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadingRateImageNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_shading_rate_image(&self, image_view: ImageView, image_layout: ImageLayout) {
        unsafe {
            (self
                .fns()
                .nv_shading_rate_image
                .as_ref()
                .unwrap()
                .bind_shading_rate_image_nv)(self.handle, image_view, image_layout)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportShadingRatePaletteNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_viewport_shading_rate_palette(
        &self,
        first_viewport: u32,
        shading_rate_palettes: &[ShadingRatePaletteNV],
    ) {
        unsafe {
            (self
                .fns()
                .nv_shading_rate_image
                .as_ref()
                .unwrap()
                .set_viewport_shading_rate_palette_nv)(
                self.handle,
                first_viewport,
                shading_rate_palettes.len() as u32,
                shading_rate_palettes.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoarseSampleOrderNV.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_coarse_sample_order(
        &self,
        sample_order_type: CoarseSampleOrderTypeNV,
        custom_sample_orders: &[CoarseSampleOrderCustomNV],
    ) {
        unsafe {
            (self
                .fns()
                .nv_shading_rate_image
                .as_ref()
                .unwrap()
                .set_coarse_sample_order_nv)(
                self.handle,
                sample_order_type,
                custom_sample_orders.len() as u32,
                custom_sample_orders.as_ptr(),
            )
        };
    }
}
