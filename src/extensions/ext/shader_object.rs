// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_shader_object";
pub const SPEC_VERSION: u32 = 1;

pub trait ShaderObjectCommandBuffer {
    fn set_patch_control_points(&self, patch_control_points: u32);

    fn set_logic_op(&self, logic_op: LogicOp);

    fn set_tessellation_domain_origin(&self, domain_origin: TessellationDomainOrigin);

    fn set_depth_clamp_enable(&self, depth_clamp_enable: bool);

    fn set_polygon_mode(&self, polygon_mode: PolygonMode);

    fn set_rasterization_samples(&self, rasterization_samples: SampleCountFlags);

    fn set_sample_mask(&self, samples: SampleCountFlags, sample_mask: Option<&SampleMask>);

    fn set_alpha_to_coverage_enable(&self, alpha_to_coverage_enable: bool);

    fn set_alpha_to_one_enable(&self, alpha_to_one_enable: bool);

    fn set_logic_op_enable(&self, logic_op_enable: bool);

    fn set_color_blend_enable(&self, first_attachment: u32, color_blend_enables: &[Bool32]);

    fn set_color_blend_equation(
        &self,
        first_attachment: u32,
        color_blend_equations: &[ColorBlendEquationEXT],
    );

    fn set_color_write_mask(
        &self,
        first_attachment: u32,
        color_write_masks: &[ColorComponentFlags],
    );

    fn set_rasterization_stream(&self, rasterization_stream: u32);

    fn set_conservative_rasterization_mode(
        &self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    );

    fn set_extra_primitive_overestimation_size(&self, extra_primitive_overestimation_size: f32);

    fn set_depth_clip_enable(&self, depth_clip_enable: bool);

    fn set_sample_locations_enable(&self, sample_locations_enable: bool);

    fn set_color_blend_advanced(
        &self,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    );

    fn set_provoking_vertex_mode(&self, provoking_vertex_mode: ProvokingVertexModeEXT);

    fn set_line_rasterization_mode(&self, line_rasterization_mode: LineRasterizationModeEXT);

    fn set_line_stipple_enable(&self, stippled_line_enable: bool);

    fn set_depth_clip_negative_one_to_one(&self, negative_one_to_one: bool);

    fn set_viewport_w_scaling_enable(&self, viewport_w_scaling_enable: bool);

    fn set_viewport_swizzle(&self, first_viewport: u32, viewport_swizzles: &[ViewportSwizzleNV]);

    fn set_coverage_to_color_enable(&self, coverage_to_color_enable: bool);

    fn set_coverage_to_color_location(&self, coverage_to_color_location: u32);

    fn set_coverage_modulation_mode(&self, coverage_modulation_mode: CoverageModulationModeNV);

    fn set_coverage_modulation_table_enable(&self, coverage_modulation_table_enable: bool);

    fn set_coverage_modulation_table(&self, coverage_modulation_table: &[f32]);

    fn set_shading_rate_image_enable(&self, shading_rate_image_enable: bool);

    fn set_coverage_reduction_mode(&self, coverage_reduction_mode: CoverageReductionModeNV);

    fn set_representative_fragment_test_enable(&self, representative_fragment_test_enable: bool);

    fn set_vertex_input(
        &self,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    );

    fn bind_shaders(&self, stages: &[ShaderStageFlags], shaders: &[ShaderEXT]);
}

impl ShaderObjectCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPatchControlPointsEXT.html>
    #[inline]
    fn set_patch_control_points(&self, patch_control_points: u32) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_patch_control_points_ext)(self.handle, patch_control_points)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEXT.html>
    #[inline]
    fn set_logic_op(&self, logic_op: LogicOp) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_logic_op_ext)(self.handle, logic_op)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetTessellationDomainOriginEXT.html>
    #[inline]
    fn set_tessellation_domain_origin(&self, domain_origin: TessellationDomainOrigin) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_tessellation_domain_origin_ext)(self.handle, domain_origin)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClampEnableEXT.html>
    #[inline]
    fn set_depth_clamp_enable(&self, depth_clamp_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_depth_clamp_enable_ext)(self.handle, depth_clamp_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPolygonModeEXT.html>
    #[inline]
    fn set_polygon_mode(&self, polygon_mode: PolygonMode) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_polygon_mode_ext)(self.handle, polygon_mode)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationSamplesEXT.html>
    #[inline]
    fn set_rasterization_samples(&self, rasterization_samples: SampleCountFlags) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_rasterization_samples_ext)(self.handle, rasterization_samples)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleMaskEXT.html>
    #[inline]
    fn set_sample_mask(&self, samples: SampleCountFlags, sample_mask: Option<&SampleMask>) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_sample_mask_ext)(
                self.handle, samples, sample_mask.map_or(null(), from_ref)
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToCoverageEnableEXT.html>
    #[inline]
    fn set_alpha_to_coverage_enable(&self, alpha_to_coverage_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_alpha_to_coverage_enable_ext)(
                self.handle, alpha_to_coverage_enable as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetAlphaToOneEnableEXT.html>
    #[inline]
    fn set_alpha_to_one_enable(&self, alpha_to_one_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_alpha_to_one_enable_ext)(self.handle, alpha_to_one_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLogicOpEnableEXT.html>
    #[inline]
    fn set_logic_op_enable(&self, logic_op_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_logic_op_enable_ext)(self.handle, logic_op_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEnableEXT.html>
    #[inline]
    fn set_color_blend_enable(&self, first_attachment: u32, color_blend_enables: &[Bool32]) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_color_blend_enable_ext)(
                self.handle,
                first_attachment,
                color_blend_enables.len() as u32,
                color_blend_enables.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendEquationEXT.html>
    #[inline]
    fn set_color_blend_equation(
        &self,
        first_attachment: u32,
        color_blend_equations: &[ColorBlendEquationEXT],
    ) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_color_blend_equation_ext)(
                self.handle,
                first_attachment,
                color_blend_equations.len() as u32,
                color_blend_equations.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorWriteMaskEXT.html>
    #[inline]
    fn set_color_write_mask(
        &self,
        first_attachment: u32,
        color_write_masks: &[ColorComponentFlags],
    ) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_color_write_mask_ext)(
                self.handle,
                first_attachment,
                color_write_masks.len() as u32,
                color_write_masks.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRasterizationStreamEXT.html>
    #[inline]
    fn set_rasterization_stream(&self, rasterization_stream: u32) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_rasterization_stream_ext)(self.handle, rasterization_stream)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetConservativeRasterizationModeEXT.html>
    #[inline]
    fn set_conservative_rasterization_mode(
        &self,
        conservative_rasterization_mode: ConservativeRasterizationModeEXT,
    ) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_conservative_rasterization_mode_ext)(
                self.handle,
                conservative_rasterization_mode,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetExtraPrimitiveOverestimationSizeEXT.html>
    #[inline]
    fn set_extra_primitive_overestimation_size(&self, extra_primitive_overestimation_size: f32) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_extra_primitive_overestimation_size_ext)(
                self.handle,
                extra_primitive_overestimation_size,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipEnableEXT.html>
    #[inline]
    fn set_depth_clip_enable(&self, depth_clip_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_depth_clip_enable_ext)(self.handle, depth_clip_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEnableEXT.html>
    #[inline]
    fn set_sample_locations_enable(&self, sample_locations_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_sample_locations_enable_ext)(
                self.handle, sample_locations_enable as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetColorBlendAdvancedEXT.html>
    #[inline]
    fn set_color_blend_advanced(
        &self,
        first_attachment: u32,
        color_blend_advanced: &[ColorBlendAdvancedEXT],
    ) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_color_blend_advanced_ext)(
                self.handle,
                first_attachment,
                color_blend_advanced.len() as u32,
                color_blend_advanced.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetProvokingVertexModeEXT.html>
    #[inline]
    fn set_provoking_vertex_mode(&self, provoking_vertex_mode: ProvokingVertexModeEXT) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_provoking_vertex_mode_ext)(self.handle, provoking_vertex_mode)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineRasterizationModeEXT.html>
    #[inline]
    fn set_line_rasterization_mode(&self, line_rasterization_mode: LineRasterizationModeEXT) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_line_rasterization_mode_ext)(self.handle, line_rasterization_mode)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetLineStippleEnableEXT.html>
    #[inline]
    fn set_line_stipple_enable(&self, stippled_line_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_line_stipple_enable_ext)(self.handle, stippled_line_enable as _)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDepthClipNegativeOneToOneEXT.html>
    #[inline]
    fn set_depth_clip_negative_one_to_one(&self, negative_one_to_one: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_depth_clip_negative_one_to_one_ext)(
                self.handle, negative_one_to_one as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportWScalingEnableNV.html>
    #[inline]
    fn set_viewport_w_scaling_enable(&self, viewport_w_scaling_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_viewport_w_scaling_enable_nv)(
                self.handle, viewport_w_scaling_enable as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetViewportSwizzleNV.html>
    #[inline]
    fn set_viewport_swizzle(&self, first_viewport: u32, viewport_swizzles: &[ViewportSwizzleNV]) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_viewport_swizzle_nv)(
                self.handle,
                first_viewport,
                viewport_swizzles.len() as u32,
                viewport_swizzles.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorEnableNV.html>
    #[inline]
    fn set_coverage_to_color_enable(&self, coverage_to_color_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_coverage_to_color_enable_nv)(
                self.handle, coverage_to_color_enable as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageToColorLocationNV.html>
    #[inline]
    fn set_coverage_to_color_location(&self, coverage_to_color_location: u32) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_coverage_to_color_location_nv)(
                self.handle, coverage_to_color_location
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationModeNV.html>
    #[inline]
    fn set_coverage_modulation_mode(&self, coverage_modulation_mode: CoverageModulationModeNV) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_coverage_modulation_mode_nv)(self.handle, coverage_modulation_mode)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableEnableNV.html>
    #[inline]
    fn set_coverage_modulation_table_enable(&self, coverage_modulation_table_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_coverage_modulation_table_enable_nv)(
                self.handle,
                coverage_modulation_table_enable as _,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageModulationTableNV.html>
    #[inline]
    fn set_coverage_modulation_table(&self, coverage_modulation_table: &[f32]) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_coverage_modulation_table_nv)(
                self.handle,
                coverage_modulation_table.len() as u32,
                coverage_modulation_table.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetShadingRateImageEnableNV.html>
    #[inline]
    fn set_shading_rate_image_enable(&self, shading_rate_image_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_shading_rate_image_enable_nv)(
                self.handle, shading_rate_image_enable as _
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCoverageReductionModeNV.html>
    #[inline]
    fn set_coverage_reduction_mode(&self, coverage_reduction_mode: CoverageReductionModeNV) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_coverage_reduction_mode_nv)(self.handle, coverage_reduction_mode)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetRepresentativeFragmentTestEnableNV.html>
    #[inline]
    fn set_representative_fragment_test_enable(&self, representative_fragment_test_enable: bool) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_representative_fragment_test_enable_nv)(
                self.handle,
                representative_fragment_test_enable as _,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetVertexInputEXT.html>
    #[inline]
    fn set_vertex_input(
        &self,
        vertex_binding_descriptions: &[VertexInputBindingDescription2EXT],
        vertex_attribute_descriptions: &[VertexInputAttributeDescription2EXT],
    ) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .set_vertex_input_ext)(
                self.handle,
                vertex_binding_descriptions.len() as u32,
                vertex_binding_descriptions.as_ptr(),
                vertex_attribute_descriptions.len() as u32,
                vertex_attribute_descriptions.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindShadersEXT.html>
    #[inline]
    fn bind_shaders(&self, stages: &[ShaderStageFlags], shaders: &[ShaderEXT]) {
        assert_eq!(stages.len(), shaders.len());
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .bind_shaders_ext)(
                self.handle,
                stages.len() as u32,
                stages.as_ptr(),
                shaders.as_ptr(),
            )
        };
    }
}

pub trait ShaderObjectDevice {
    fn create_shaders(
        &self,
        create_infos: &[ShaderCreateInfoEXT],
        allocator: Option<&AllocationCallbacks>,
        shaders: &mut [ShaderEXT],
    ) -> Result<(), vkResult>;

    fn destroy_shader(&self, shader: ShaderEXT, allocator: Option<&AllocationCallbacks>);

    fn get_shader_binary_data(&self, shader: ShaderEXT) -> Result<Vec<c_void>, vkResult>;
}

impl ShaderObjectDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShadersEXT.html>
    #[inline]
    fn create_shaders(
        &self,
        create_infos: &[ShaderCreateInfoEXT],
        allocator: Option<&AllocationCallbacks>,
        shaders: &mut [ShaderEXT],
    ) -> Result<(), vkResult> {
        assert_eq!(create_infos.len(), shaders.len());
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .create_shaders_ext)(
                self.handle,
                create_infos.len() as u32,
                create_infos.as_ptr(),
                allocator.map_or(null(), from_ref),
                shaders.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderEXT.html>
    #[inline]
    fn destroy_shader(&self, shader: ShaderEXT, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .destroy_shader_ext)(
                self.handle, shader, allocator.map_or(null(), from_ref)
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderBinaryDataEXT.html>
    #[inline]
    fn get_shader_binary_data(&self, shader: ShaderEXT) -> Result<Vec<c_void>, vkResult> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .ext_shader_object
                .as_ref()
                .unwrap()
                .get_shader_binary_data_ext)(self.handle, shader, count, data)
        })
    }
}
