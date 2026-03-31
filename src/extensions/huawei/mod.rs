pub mod cluster_culling_shader;
pub mod invocation_mask;
pub mod subpass_shading;
pub mod hdr_vivid {
    /// Type: `Device`
    pub const NAME: &CStr = c"VK_HUAWEI_hdr_vivid";
    pub const SPEC_VERSION: u32 = 1;
}
