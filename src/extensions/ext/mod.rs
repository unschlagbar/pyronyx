pub mod debug_report;
pub mod depth_range_unrestricted {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_range_unrestricted";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod debug_marker;
pub mod transform_feedback;
#[deprecated = "This extension is deprecated. Use `VK_EXT_layer_settings` instead."]
pub mod validation_flags {
    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_validation_flags";
    pub const SPEC_VERSION: u32 = 3;
}
#[deprecated = "This extension is deprecated. Use `VK_VERSION_1_2` instead."]
pub mod shader_subgroup_ballot {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_subgroup_ballot";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_VERSION_1_1` instead."]
pub mod shader_subgroup_vote {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_subgroup_vote";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod texture_compression_astc_hdr {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_texture_compression_astc_hdr";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod astc_decode_mode {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_astc_decode_mode";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_robustness {
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
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_conservative_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clip_enable {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_clip_enable";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod swapchain_colorspace {
    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_swapchain_colorspace";
    pub const SPEC_VERSION: u32 = 5;
}
pub mod hdr_metadata;
pub mod external_memory_dma_buf {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_external_memory_dma_buf";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod queue_family_foreign {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_queue_family_foreign";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod debug_utils;
pub mod sampler_filter_minmax {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_sampler_filter_minmax";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod descriptor_heap;
pub mod inline_uniform_block {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_inline_uniform_block";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_stencil_export {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_stencil_export";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod sample_locations;
pub mod blend_operation_advanced {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_blend_operation_advanced";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod post_depth_coverage {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_post_depth_coverage";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_drm_format_modifier;
pub mod validation_cache;
pub mod descriptor_indexing {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_descriptor_indexing";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_viewport_index_layer {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_viewport_index_layer";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod filter_cubic {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_filter_cubic";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod global_priority {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_global_priority";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod external_memory_host;
pub mod calibrated_timestamps {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_calibrated_timestamps";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod vertex_attribute_divisor {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_vertex_attribute_divisor";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod pipeline_creation_feedback {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_creation_feedback";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_timing;
pub mod pci_bus_info {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pci_bus_info";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod metal_surface;
pub mod fragment_density_map {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_density_map";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod scalar_block_layout {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_scalar_block_layout";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod subgroup_size_control {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_subgroup_size_control";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_image_atomic_int64 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_image_atomic_int64";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod memory_budget {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_memory_budget";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod memory_priority {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_memory_priority";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_KHR_buffer_device_address` instead."]
pub mod buffer_device_address {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_buffer_device_address";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod tooling_info {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_tooling_info";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod separate_stencil_usage {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_separate_stencil_usage";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_EXT_layer_settings` instead."]
pub mod validation_features {
    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_validation_features";
    pub const SPEC_VERSION: u32 = 6;
}
pub mod fragment_shader_interlock {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_shader_interlock";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod ycbcr_image_arrays {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_ycbcr_image_arrays";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod provoking_vertex {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_provoking_vertex";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod full_screen_exclusive;
pub mod headless_surface;
pub mod line_rasterization {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_line_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_atomic_float {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_atomic_float";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod host_query_reset {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_host_query_reset";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod index_type_uint8 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_index_type_uint8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod extended_dynamic_state {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_extended_dynamic_state";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod host_image_copy {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_host_image_copy";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod map_memory_placed {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_map_memory_placed";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_atomic_float2 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_atomic_float2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod surface_maintenance1 {
    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_surface_maintenance1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod swapchain_maintenance1 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_swapchain_maintenance1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_demote_to_helper_invocation {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_demote_to_helper_invocation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod texel_buffer_alignment {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_texel_buffer_alignment";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_bias_control;
pub mod device_memory_report {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_device_memory_report";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod acquire_drm_display;
pub mod robustness2 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_robustness2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod custom_border_color {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_custom_border_color";
    pub const SPEC_VERSION: u32 = 12;
}
pub mod texture_compression_astc_3d {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_texture_compression_astc_3d";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod private_data {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_private_data";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_creation_cache_control {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_creation_cache_control";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod descriptor_buffer;
pub mod metal_objects;
pub mod graphics_pipeline_library {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_graphics_pipeline_library";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod mesh_shader;
pub mod ycbcr_2plane_444_formats {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_ycbcr_2plane_444_formats";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_density_map2 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_density_map2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_robustness {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_robustness";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_compression_control {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_compression_control";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod attachment_feedback_loop_layout {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_attachment_feedback_loop_layout";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod formats_4444 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_4444_formats";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_fault;
pub mod rgba10x6_formats {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_rgba10x6_formats";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod directfb_surface;
pub mod vertex_input_dynamic_state {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_vertex_input_dynamic_state";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod physical_device_drm {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_physical_device_drm";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_address_binding_report {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_device_address_binding_report";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clip_control {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_clip_control";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod primitive_topology_list_restart {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_primitive_topology_list_restart";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_mode_fifo_latest_ready {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_present_mode_fifo_latest_ready";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_properties;
pub mod frame_boundary {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_frame_boundary";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod multisampled_render_to_single_sampled {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_multisampled_render_to_single_sampled";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod extended_dynamic_state2 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_extended_dynamic_state2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod color_write_enable;
pub mod primitives_generated_query {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_primitives_generated_query";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod global_priority_query {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_global_priority_query";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_view_min_lod {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_view_min_lod";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod multi_draw;
pub mod image_2d_view_of_3d {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_2d_view_of_3d";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_tile_image {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_tile_image";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod opacity_micromap;
pub mod load_store_op_none {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_load_store_op_none";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod border_color_swizzle {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_border_color_swizzle";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pageable_device_local_memory;
pub mod image_sliced_view_of_3d {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_sliced_view_of_3d";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clamp_zero_one {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_depth_clamp_zero_one";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod non_seamless_cube_map {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_non_seamless_cube_map";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod application_parameters {
    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_application_parameters";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_compression_control_swapchain {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_image_compression_control_swapchain";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod nested_command_buffer {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_nested_command_buffer";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_memory_acquire_unmodified {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_external_memory_acquire_unmodified";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod extended_dynamic_state3 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_extended_dynamic_state3";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod subpass_merge_feedback {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_subpass_merge_feedback";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_module_identifier;
pub mod rasterization_order_attachment_access {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_rasterization_order_attachment_access";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod legacy_dithering {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_legacy_dithering";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod pipeline_protected_access {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_protected_access";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_object;
pub mod mutable_descriptor_type {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_mutable_descriptor_type";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod legacy_vertex_attributes {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_legacy_vertex_attributes";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod layer_settings {
    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_EXT_layer_settings";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod pipeline_library_group_handles {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_pipeline_library_group_handles";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod dynamic_rendering_unused_attachments {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_dynamic_rendering_unused_attachments";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod attachment_feedback_loop_dynamic_state;
pub mod memory_decompression;
pub mod shader_replicated_composites {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_replicated_composites";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_float8 {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_float8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_generated_commands;
pub mod ray_tracing_invocation_reorder {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_ray_tracing_invocation_reorder";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clamp_control;
pub mod external_memory_metal;
pub mod vertex_attribute_robustness {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_vertex_attribute_robustness";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_density_map_offset {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_fragment_density_map_offset";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod zero_initialize_device_memory {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_zero_initialize_device_memory";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_64bit_indexing {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_64bit_indexing";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod custom_resolve;
pub mod shader_long_vector {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_long_vector";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_uniform_buffer_unsized_array {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_uniform_buffer_unsized_array";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_subgroup_partitioned {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_EXT_shader_subgroup_partitioned";
    pub const SPEC_VERSION: u32 = 1;
}
