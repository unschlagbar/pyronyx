pub mod stream_descriptor_surface;
pub mod frame_token {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_GGP_frame_token";
    pub const SPEC_VERSION: u32 = 1;
}
