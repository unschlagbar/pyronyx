#[deprecated = "This extension is deprecated. Use `` instead."]
pub mod glsl_shader {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_glsl_shader";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_KHR_dedicated_allocation` instead."]
pub mod dedicated_allocation {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_dedicated_allocation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod corner_sampled_image {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_corner_sampled_image";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod private_vendor_info {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_private_vendor_info";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod external_memory_capabilities;
#[deprecated = "This extension is deprecated. Use `VK_KHR_external_memory` instead."]
pub mod external_memory {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_external_memory";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_memory_win32;
pub mod win32_keyed_mutex {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_win32_keyed_mutex";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod clip_space_w_scaling;
pub mod sample_mask_override_coverage {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_sample_mask_override_coverage";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod geometry_shader_passthrough {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_geometry_shader_passthrough";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod viewport_array2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_viewport_array2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod viewport_swizzle {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_viewport_swizzle";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_coverage_to_color {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_fragment_coverage_to_color";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod framebuffer_mixed_samples {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_framebuffer_mixed_samples";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fill_rectangle {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_fill_rectangle";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_sm_builtins {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_shader_sm_builtins";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod ray_tracing;
pub mod shading_rate_image;
pub mod representative_fragment_test {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_representative_fragment_test";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_subgroup_partitioned {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_shader_subgroup_partitioned";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod compute_shader_derivatives {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_compute_shader_derivatives";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod mesh_shader;
pub mod fragment_shader_barycentric {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_fragment_shader_barycentric";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_image_footprint {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_shader_image_footprint";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod device_diagnostic_checkpoints;
pub mod scissor_exclusive;
pub mod dedicated_allocation_image_aliasing {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_dedicated_allocation_image_aliasing";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod cooperative_matrix;
pub mod coverage_reduction_mode;
pub mod device_generated_commands;
pub mod inherited_viewport_scissor {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_inherited_viewport_scissor";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod present_barrier {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_present_barrier";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_diagnostics_config {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_device_diagnostics_config";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod cuda_kernel_launch;
pub mod low_latency {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_low_latency";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod fragment_shading_rate_enums;
pub mod ray_tracing_motion_blur {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_ray_tracing_motion_blur";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod acquire_winrt_display;
pub mod external_memory_rdma;
pub mod external_memory_sci_buf;
pub mod external_sci_sync;
#[deprecated = "This extension is deprecated. Use `VK_NV_cluster_acceleration_structure` instead."]
pub mod displacement_micromap {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_displacement_micromap";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod copy_memory_indirect;
pub mod device_generated_commands_compute;
pub mod memory_decompression;
pub mod ray_tracing_linear_swept_spheres {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_ray_tracing_linear_swept_spheres";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod linear_color_attachment {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_linear_color_attachment";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_sci_sync2;
pub mod optical_flow;
pub mod ray_tracing_invocation_reorder {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_ray_tracing_invocation_reorder";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod cooperative_vector;
pub mod extended_sparse_address_space {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_extended_sparse_address_space";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod low_latency2;
pub mod per_stage_descriptor_set {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_per_stage_descriptor_set";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod descriptor_pool_overallocation {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_descriptor_pool_overallocation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod display_stereo {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_NV_display_stereo";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod raw_access_chains {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_raw_access_chains";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod external_compute_queue;
pub mod command_buffer_inheritance {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_command_buffer_inheritance";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_atomic_float16_vector {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_shader_atomic_float16_vector";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod ray_tracing_validation {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_ray_tracing_validation";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod cluster_acceleration_structure;
pub mod partitioned_acceleration_structure;
pub mod push_constant_bank {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_push_constant_bank";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod cooperative_matrix2;
pub mod present_metering {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NV_present_metering";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod compute_occupancy_priority;
