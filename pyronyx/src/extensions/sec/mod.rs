pub mod amigo_profiling {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_SEC_amigo_profiling";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_cache_incremental_mode {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_SEC_pipeline_cache_incremental_mode";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod ubm_surface;
pub mod throttle_hint {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_SEC_throttle_hint";
    pub const SPEC_VERSION: u32 = 1;
}
