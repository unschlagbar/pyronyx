pub mod shader_integer_functions2 {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_INTEL_shader_integer_functions2";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod performance_query;
