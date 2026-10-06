pub mod rasterization_order_attachment_access {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Promoted to [`VK_EXT_rasterization_order_attachment_access`](crate::ext::rasterization_order_attachment_access)
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_ARM_rasterization_order_attachment_access";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod shader_core_properties {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: Vulkan 1.1
    pub const NAME: &CStr = c"VK_ARM_shader_core_properties";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod scheduling_controls;
pub mod render_pass_striped {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: (([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_synchronization2`](crate::khr::synchronization2)) or Vulkan 1.3
    pub const NAME: &CStr = c"VK_ARM_render_pass_striped";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod tensors;
pub mod shader_core_builtins {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_ARM_shader_core_builtins";
    pub const SPEC_VERSION: u32 = 2;
}
pub mod data_graph;
pub mod data_graph_instruction_set_tosa {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_ARM_data_graph`](crate::arm::data_graph)
    pub const NAME: &CStr = c"VK_ARM_data_graph_instruction_set_tosa";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod tensor_controls {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_ARM_tensors`](crate::arm::tensors)
    pub const NAME: &CStr = c"VK_ARM_tensor_controls";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_opacity_micromap {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_EXT_opacity_micromap`](crate::ext::opacity_micromap)
    pub const NAME: &CStr = c"VK_ARM_pipeline_opacity_micromap";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod performance_counters_by_region;
pub mod shader_instrumentation;
pub mod format_pack {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_ARM_format_pack";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod data_graph_optical_flow;
pub mod data_graph_neural_accelerator_statistics {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_ARM_data_graph_neural_accelerator_statistics";
    pub const SPEC_VERSION: u32 = 1;
}
