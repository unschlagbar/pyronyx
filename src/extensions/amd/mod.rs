pub mod rasterization_order {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_rasterization_order";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_trinary_minmax {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_trinary_minmax";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_explicit_vertex_parameter {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_explicit_vertex_parameter";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod gcn_shader {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_gcn_shader";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod draw_indirect_count {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_draw_indirect_count";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod negative_viewport_height {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_negative_viewport_height";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_KHR_shader_float16_int8` instead."]
pub mod gpu_shader_half_float {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_gpu_shader_half_float";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod shader_ballot {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_ballot";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod texture_gather_bias_lod {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_texture_gather_bias_lod";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_info;
pub mod shader_image_load_store_lod {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_image_load_store_lod";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `VK_KHR_shader_float16_int8` instead."]
pub mod gpu_shader_int16 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_gpu_shader_int16";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod mixed_attachment_samples {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_mixed_attachment_samples";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_fragment_mask {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_fragment_mask";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod buffer_marker;
pub mod pipeline_compiler_control {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_pipeline_compiler_control";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_core_properties {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_core_properties";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod memory_overallocation_behavior {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_memory_overallocation_behavior";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod display_native_hdr;
pub mod shader_core_properties2 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_core_properties2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod device_coherent_memory {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_device_coherent_memory";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_early_and_late_fragment_tests {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMD_shader_early_and_late_fragment_tests";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod anti_lag;
