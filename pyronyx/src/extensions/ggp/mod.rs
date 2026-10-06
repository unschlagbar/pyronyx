pub mod stream_descriptor_surface;
pub mod frame_token {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_swapchain`](crate::khr::swapchain) + [`VK_GGP_stream_descriptor_surface`](crate::ggp::stream_descriptor_surface)
    pub const NAME: &CStr = c"VK_GGP_frame_token";
    pub const SPEC_VERSION: u32 = 1;
}
