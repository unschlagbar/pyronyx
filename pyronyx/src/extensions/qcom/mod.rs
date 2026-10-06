pub mod render_pass_shader_resolve {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Promoted to [`VK_EXT_custom_resolve`](crate::ext::custom_resolve)
    pub const NAME: &CStr = c"VK_QCOM_render_pass_shader_resolve";
    pub const SPEC_VERSION: u32 = 4;
}
pub mod cooperative_matrix_conversion {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_cooperative_matrix`](crate::khr::cooperative_matrix)
    pub const NAME: &CStr = c"VK_QCOM_cooperative_matrix_conversion";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod elapsed_timer_query {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_QCOM_elapsed_timer_query";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod render_pass_transform {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_QCOM_render_pass_transform";
    pub const SPEC_VERSION: u32 = 5;
}
pub mod render_pass_store_ops {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_QCOM_render_pass_store_ops";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod queue_perf_hint;
pub mod image_processing3 {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_QCOM_image_processing3";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_multiple_wait_queues {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_QCOM_shader_multiple_wait_queues";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod tile_shading;
pub mod rotated_copy_commands {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_copy_commands2`](crate::khr::copy_commands2) or Vulkan 1.3
    pub const NAME: &CStr = c"VK_QCOM_rotated_copy_commands";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod fragment_density_map_offset {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Promoted to [`VK_EXT_fragment_density_map_offset`](crate::ext::fragment_density_map_offset)
    ///
    /// Requires: ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_EXT_fragment_density_map`](crate::ext::fragment_density_map)
    pub const NAME: &CStr = c"VK_QCOM_fragment_density_map_offset";
    pub const SPEC_VERSION: u32 = 3;
}
pub mod image_processing {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_format_feature_flags2`](crate::khr::format_feature_flags2) or Vulkan 1.3
    pub const NAME: &CStr = c"VK_QCOM_image_processing";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod tile_properties;
pub mod multiview_per_view_viewports {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_QCOM_multiview_per_view_viewports";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod multiview_per_view_render_areas {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_QCOM_multiview_per_view_render_areas";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod image_processing2 {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_QCOM_image_processing`](crate::qcom::image_processing)
    pub const NAME: &CStr = c"VK_QCOM_image_processing2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod filter_cubic_weights {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_EXT_filter_cubic`](crate::ext::filter_cubic)
    pub const NAME: &CStr = c"VK_QCOM_filter_cubic_weights";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod ycbcr_degamma {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_QCOM_ycbcr_degamma";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod filter_cubic_clamp {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: ([`VK_EXT_filter_cubic`](crate::ext::filter_cubic)) + (Vulkan 1.2 or [`VK_EXT_sampler_filter_minmax`](crate::ext::sampler_filter_minmax))
    pub const NAME: &CStr = c"VK_QCOM_filter_cubic_clamp";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod tile_memory_heap;
pub mod data_graph_model {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_ARM_data_graph`](crate::arm::data_graph)
    pub const NAME: &CStr = c"VK_QCOM_data_graph_model";
    pub const SPEC_VERSION: u32 = 1;
}
