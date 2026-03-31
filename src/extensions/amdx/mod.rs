pub mod shader_enqueue;
pub mod dense_geometry_format {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_AMDX_dense_geometry_format";
    pub const SPEC_VERSION: u32 = 1;
}
