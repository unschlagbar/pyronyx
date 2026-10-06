pub mod cluster_culling_shader;
pub mod invocation_mask;
pub mod subpass_shading;
pub mod hdr_vivid {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: ([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_swapchain`](crate::khr::swapchain) + [`VK_EXT_hdr_metadata`](crate::ext::hdr_metadata)
    pub const NAME: &CStr = c"VK_HUAWEI_hdr_vivid";
    pub const SPEC_VERSION: u32 = 1;
}
