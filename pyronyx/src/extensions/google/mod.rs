pub mod display_timing;
pub mod hlsl_functionality1 {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_GOOGLE_hlsl_functionality1";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod decorate_string {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_GOOGLE_decorate_string";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod user_type {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_GOOGLE_user_type";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod surfaceless_query {
    use core::ffi::CStr;

    /// Type: `Instance`
    ///
    /// Requires: [`VK_KHR_surface`](crate::khr::surface)
    pub const NAME: &CStr = c"VK_GOOGLE_surfaceless_query";
    pub const SPEC_VERSION: u32 = 2;
}
