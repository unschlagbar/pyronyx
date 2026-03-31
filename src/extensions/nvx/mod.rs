pub mod binary_import;
pub mod image_view_handle;
pub mod multiview_per_view_attributes {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_NVX_multiview_per_view_attributes";
    pub const SPEC_VERSION: u32 = 1;
}
