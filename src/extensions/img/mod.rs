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
    pub const NAME: &CStr = c"VK_IMG_relaxed_line_rasterization";
    pub const SPEC_VERSION: u32 = 1;
}
