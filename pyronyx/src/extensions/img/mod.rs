pub mod filter_cubic {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_IMG_filter_cubic";
    pub const SPEC_VERSION: u32 = 1;
}
#[deprecated = "This extension is deprecated. Use `` instead."]
pub mod format_pvrtc {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_IMG_format_pvrtc";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod relaxed_line_rasterization {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
    pub const NAME: &CStr = c"VK_IMG_relaxed_line_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod filter_linear_2d {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_format_feature_flags2`](crate::khr::format_feature_flags2) or Vulkan 1.3
    pub const NAME: &CStr = c"VK_IMG_filter_linear_2d";
    pub const SPEC_VERSION: u32 = 1;
}
