pub mod debug_report;
pub mod depth_range_unrestricted {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_range_unrestricted";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod debug_marker;
pub mod transform_feedback;
#[deprecated = "This extension is deprecated. Use `VK_EXT_layer_settings` instead."]
pub mod validation_flags {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_validation_flags";
    pub const SPEC_VERSION: u32 = 3;
}
#[deprecated = "This extension is deprecated. Use `VK_VERSION_1_2` instead."]
pub mod shader_subgroup_ballot {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_subgroup_ballot";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_VERSION_1_1` instead."]
pub mod shader_subgroup_vote {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_subgroup_vote";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod texture_compression_astc_hdr {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_texture_compression_astc_hdr";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod astc_decode_mode {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_astc_decode_mode";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_robustness {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_robustness";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod acquire_xlib_display;
pub mod conditional_rendering;
pub mod direct_mode_display;
pub mod discard_rectangles;
pub mod display_control;
pub mod display_surface_counter;
pub mod conservative_rasterization {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_conservative_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clip_enable {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_clip_enable";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod swapchain_colorspace {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_swapchain_colorspace";
    pub const SPEC_VERSION: u32 = 5;
}
pub mod hdr_metadata;
pub mod external_memory_dma_buf {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_external_memory_dma_buf";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod queue_family_foreign {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_queue_family_foreign";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod debug_utils;
pub mod sampler_filter_minmax {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_sampler_filter_minmax";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod descriptor_heap;
pub mod inline_uniform_block {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_inline_uniform_block";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_stencil_export {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_stencil_export";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod sample_locations;
pub mod blend_operation_advanced {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_blend_operation_advanced";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod post_depth_coverage {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_post_depth_coverage";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_drm_format_modifier;
pub mod validation_cache;
pub mod descriptor_indexing {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_descriptor_indexing";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_viewport_index_layer {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_viewport_index_layer";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod filter_cubic {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_filter_cubic";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod global_priority {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_global_priority";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod external_memory_host;
pub mod calibrated_timestamps {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_calibrated_timestamps";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod vertex_attribute_divisor {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_vertex_attribute_divisor";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod pipeline_creation_feedback {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_creation_feedback";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_timing;
pub mod pci_bus_info {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pci_bus_info";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod metal_surface;
pub mod fragment_density_map {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_density_map";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod scalar_block_layout {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_scalar_block_layout";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod subgroup_size_control {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_subgroup_size_control";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_image_atomic_int64 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_image_atomic_int64";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod memory_budget {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_memory_budget";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod memory_priority {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_memory_priority";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_KHR_buffer_device_address` instead."]
pub mod buffer_device_address {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_buffer_device_address";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod tooling_info {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_tooling_info";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod separate_stencil_usage {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_separate_stencil_usage";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_EXT_layer_settings` instead."]
pub mod validation_features {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_validation_features";
    pub const SPEC_VERSION: u32 = 6;
}
pub mod fragment_shader_interlock {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_shader_interlock";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod ycbcr_image_arrays {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_ycbcr_image_arrays";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod provoking_vertex {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_provoking_vertex";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod full_screen_exclusive;
pub mod headless_surface;
pub mod line_rasterization {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_line_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_atomic_float {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_atomic_float";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod host_query_reset {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_host_query_reset";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod index_type_uint8 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_index_type_uint8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod extended_dynamic_state {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_extended_dynamic_state";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod host_image_copy {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_host_image_copy";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod map_memory_placed {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_map_memory_placed";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_atomic_float2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_atomic_float2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod surface_maintenance1 {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_surface_maintenance1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod swapchain_maintenance1 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_swapchain_maintenance1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_demote_to_helper_invocation {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_demote_to_helper_invocation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod texel_buffer_alignment {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_texel_buffer_alignment";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_bias_control;
pub mod device_memory_report {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_device_memory_report";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod acquire_drm_display;
pub mod robustness2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_robustness2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod custom_border_color {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_custom_border_color";
    pub const SPEC_VERSION: u32 = 12;
}
pub mod texture_compression_astc_3d {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_texture_compression_astc_3d";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod private_data {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_private_data";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_creation_cache_control {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_creation_cache_control";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod descriptor_buffer;
pub mod metal_objects;
pub mod graphics_pipeline_library {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_graphics_pipeline_library";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod mesh_shader;
pub mod ycbcr_2plane_444_formats {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_ycbcr_2plane_444_formats";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_density_map2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_density_map2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_robustness {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_robustness";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_compression_control {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_compression_control";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod attachment_feedback_loop_layout {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_attachment_feedback_loop_layout";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod formats_4444 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_4444_formats";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_fault;
pub mod rgba10x6_formats {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_rgba10x6_formats";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod directfb_surface;
pub mod vertex_input_dynamic_state {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_vertex_input_dynamic_state";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod physical_device_drm {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_physical_device_drm";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_address_binding_report {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_device_address_binding_report";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clip_control {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_clip_control";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod primitive_topology_list_restart {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_primitive_topology_list_restart";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_mode_fifo_latest_ready {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_present_mode_fifo_latest_ready";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_properties;
pub mod frame_boundary {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_frame_boundary";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod multisampled_render_to_single_sampled {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_multisampled_render_to_single_sampled";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod extended_dynamic_state2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_extended_dynamic_state2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod color_write_enable;
pub mod primitives_generated_query {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_primitives_generated_query";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod global_priority_query {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_global_priority_query";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_view_min_lod {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_view_min_lod";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod multi_draw;
pub mod image_2d_view_of_3d {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_2d_view_of_3d";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_tile_image {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_tile_image";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod opacity_micromap;
pub mod load_store_op_none {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_load_store_op_none";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod border_color_swizzle {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_border_color_swizzle";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pageable_device_local_memory;
pub mod image_sliced_view_of_3d {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_sliced_view_of_3d";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clamp_zero_one {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_clamp_zero_one";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod non_seamless_cube_map {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_non_seamless_cube_map";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod application_parameters {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_application_parameters";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_compression_control_swapchain {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_compression_control_swapchain";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod nested_command_buffer {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_nested_command_buffer";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_memory_acquire_unmodified {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_external_memory_acquire_unmodified";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod extended_dynamic_state3 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_extended_dynamic_state3";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod subpass_merge_feedback {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_subpass_merge_feedback";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_module_identifier;
pub mod rasterization_order_attachment_access {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_rasterization_order_attachment_access";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod legacy_dithering {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_legacy_dithering";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod pipeline_protected_access {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_protected_access";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_object;
pub mod mutable_descriptor_type {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_mutable_descriptor_type";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod legacy_vertex_attributes {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_legacy_vertex_attributes";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod layer_settings {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_layer_settings";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod pipeline_library_group_handles {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_library_group_handles";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod dynamic_rendering_unused_attachments {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_dynamic_rendering_unused_attachments";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod attachment_feedback_loop_dynamic_state;
pub mod memory_decompression;
pub mod shader_replicated_composites {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_replicated_composites";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_float8 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_float8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_generated_commands;
pub mod ray_tracing_invocation_reorder {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_ray_tracing_invocation_reorder";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clamp_control;
pub mod external_memory_metal;
pub mod vertex_attribute_robustness {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_vertex_attribute_robustness";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_density_map_offset {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_density_map_offset";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod zero_initialize_device_memory {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_zero_initialize_device_memory";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_64bit_indexing {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_64bit_indexing";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod custom_resolve;
pub mod shader_long_vector {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_long_vector";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_uniform_buffer_unsized_array {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_uniform_buffer_unsized_array";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_subgroup_partitioned {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_subgroup_partitioned";
    pub const SPEC_VERSION: u32 = 1;
}
