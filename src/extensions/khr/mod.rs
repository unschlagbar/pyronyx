pub mod android_surface;
pub mod display;
pub mod display_swapchain;
pub mod surface;
pub mod swapchain;
pub mod wayland_surface;
pub mod win32_surface;
pub mod xcb_surface;
pub mod xlib_surface;
pub mod sampler_mirror_clamp_to_edge {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_sampler_mirror_clamp_to_edge";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod video_decode_queue;
pub mod video_queue;
pub mod video_encode_h264 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_encode_h264";
    pub const SPEC_VERSION: u32 = 14;
}
pub mod video_encode_h265 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_encode_h265";
    pub const SPEC_VERSION: u32 = 14;
}
pub mod video_decode_h264 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_decode_h264";
    pub const SPEC_VERSION: u32 = 9;
}
pub mod dynamic_rendering {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_dynamic_rendering";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod multiview {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_multiview";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod get_physical_device_properties2 {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_get_physical_device_properties2";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod device_group;
pub mod shader_draw_parameters {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_draw_parameters";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance1 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance1";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod device_group_creation {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_device_group_creation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_memory_capabilities {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_external_memory_capabilities";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_memory {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_external_memory";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_memory_fd;
pub mod external_memory_win32;
pub mod win32_keyed_mutex {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_win32_keyed_mutex";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_semaphore_capabilities {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_external_semaphore_capabilities";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_semaphore {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_external_semaphore";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_semaphore_fd;
pub mod external_semaphore_win32;
pub mod push_descriptor {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_push_descriptor";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_float16_int8 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_float16_int8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod bit_storage_16 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_16bit_storage";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod incremental_present {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_incremental_present";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod descriptor_update_template {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_descriptor_update_template";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod imageless_framebuffer {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_imageless_framebuffer";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod create_renderpass2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_create_renderpass2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shared_presentable_image;
pub mod external_fence_capabilities {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_external_fence_capabilities";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_fence {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_external_fence";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_fence_fd;
pub mod external_fence_win32;
pub mod performance_query;
pub mod maintenance2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod get_surface_capabilities2;
pub mod variable_pointers {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_variable_pointers";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod get_display_properties2;
pub mod dedicated_allocation {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_dedicated_allocation";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod storage_buffer_storage_class {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_storage_buffer_storage_class";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_bfloat16 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_bfloat16";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod relaxed_block_layout {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_relaxed_block_layout";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod get_memory_requirements2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_get_memory_requirements2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_format_list {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_image_format_list";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod acceleration_structure;
pub mod ray_tracing_pipeline;
pub mod ray_query {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_ray_query";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod sampler_ycbcr_conversion {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_sampler_ycbcr_conversion";
    pub const SPEC_VERSION: u32 = 14;
}
pub mod bind_memory2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_bind_memory2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod portability_subset {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_portability_subset";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance3 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance3";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod draw_indirect_count {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_draw_indirect_count";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_subgroup_extended_types {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_subgroup_extended_types";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod bit_storage_8 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_8bit_storage";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_atomic_int64 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_atomic_int64";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_clock {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_clock";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_decode_h265 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_decode_h265";
    pub const SPEC_VERSION: u32 = 8;
}
pub mod global_priority {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_global_priority";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod driver_properties {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_driver_properties";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_float_controls {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_float_controls";
    pub const SPEC_VERSION: u32 = 4;
}
pub mod depth_stencil_resolve {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_depth_stencil_resolve";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod swapchain_mutable_format {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_swapchain_mutable_format";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod timeline_semaphore {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_timeline_semaphore";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod vulkan_memory_model {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_vulkan_memory_model";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod shader_terminate_invocation {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_terminate_invocation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_shading_rate;
pub mod dynamic_rendering_local_read {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_dynamic_rendering_local_read";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_quad_control {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_quad_control";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod spirv_1_4 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_spirv_1_4";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod surface_protected_capabilities {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_surface_protected_capabilities";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod separate_depth_stencil_layouts {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_separate_depth_stencil_layouts";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_wait;
pub mod uniform_buffer_standard_layout {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_uniform_buffer_standard_layout";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod buffer_device_address {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_buffer_device_address";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod deferred_host_operations;
pub mod pipeline_executable_properties;
pub mod map_memory2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_map_memory2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_integer_dot_product {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_integer_dot_product";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_library {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_pipeline_library";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_non_semantic_info {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_non_semantic_info";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_id {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_present_id";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod object_refresh;
pub mod video_encode_queue;
pub mod synchronization2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_synchronization2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_shader_barycentric {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_fragment_shader_barycentric";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_subgroup_uniform_control_flow {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_subgroup_uniform_control_flow";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod zero_initialize_workgroup_memory {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_zero_initialize_workgroup_memory";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod workgroup_memory_explicit_layout {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_workgroup_memory_explicit_layout";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod copy_commands2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_copy_commands2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod format_feature_flags2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_format_feature_flags2";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod ray_tracing_maintenance1;
pub mod shader_untyped_pointers {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_untyped_pointers";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod portability_enumeration {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_portability_enumeration";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance4 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance4";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_subgroup_rotate {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_subgroup_rotate";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_maximal_reconvergence {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_maximal_reconvergence";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance5 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance5";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_id2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_present_id2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_wait2;
pub mod ray_tracing_position_fetch {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_ray_tracing_position_fetch";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_binary;
pub mod surface_maintenance1 {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_KHR_surface_maintenance1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod swapchain_maintenance1;
pub mod internally_synchronized_queues {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_internally_synchronized_queues";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod cooperative_matrix;
pub mod compute_shader_derivatives {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_compute_shader_derivatives";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_decode_av1 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_decode_av1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_encode_av1 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_encode_av1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_decode_vp9 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_decode_vp9";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_maintenance1 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_maintenance1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod vertex_attribute_divisor {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_vertex_attribute_divisor";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod load_store_op_none {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_load_store_op_none";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod unified_image_layouts {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_unified_image_layouts";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_float_controls2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_float_controls2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod index_type_uint8 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_index_type_uint8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod line_rasterization {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_line_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod calibrated_timestamps;
pub mod shader_expect_assume {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_expect_assume";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod copy_memory_indirect;
pub mod maintenance6;
pub mod video_encode_intra_refresh {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_encode_intra_refresh";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_encode_quantization_map {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_encode_quantization_map";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_relaxed_extended_instruction {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_relaxed_extended_instruction";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance7 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance7";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance8 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance8";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_fma {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_shader_fma";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance9 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_maintenance9";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_maintenance2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_video_maintenance2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod depth_clamp_zero_one {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_depth_clamp_zero_one";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod robustness2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_robustness2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_mode_fifo_latest_ready {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_KHR_present_mode_fifo_latest_ready";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod maintenance10;
