pub mod mutable_descriptor_type {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Promoted to [`VK_EXT_mutable_descriptor_type`](crate::ext::mutable_descriptor_type)
    ///
    /// Requires: [`VK_KHR_maintenance3`](crate::khr::maintenance3)
    pub const NAME: &CStr = c"VK_VALVE_mutable_descriptor_type";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod video_encode_rgb_conversion {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_video_encode_queue`](crate::khr::video_encode_queue) + ([`VK_KHR_sampler_ycbcr_conversion`](crate::khr::sampler_ycbcr_conversion) or Vulkan 1.1)
    pub const NAME: &CStr = c"VK_VALVE_video_encode_rgb_conversion";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod descriptor_set_host_mapping;
pub mod fragment_density_map_layered {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: (Vulkan 1.4 or [`VK_KHR_extended_flags`](crate::khr::extended_flags) or [`VK_KHR_maintenance5`](crate::khr::maintenance5)) + [`VK_EXT_fragment_density_map`](crate::ext::fragment_density_map)
    pub const NAME: &CStr = c"VK_VALVE_fragment_density_map_layered";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_mixed_float_dot_product {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + ([`VK_KHR_shader_float16_int8`](crate::khr::shader_float16_int8) or Vulkan 1.2)
    pub const NAME: &CStr = c"VK_VALVE_shader_mixed_float_dot_product";
    pub const SPEC_VERSION: u32 = 1;
}
