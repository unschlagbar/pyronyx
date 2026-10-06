pub mod shader_enqueue;
pub mod dense_geometry_format {
    use core::ffi::CStr;

    /// Type: `Device`
    ///
    /// Requires: [`VK_KHR_acceleration_structure`](crate::khr::acceleration_structure) + (Vulkan 1.4 or [`VK_KHR_extended_flags`](crate::khr::extended_flags) or [`VK_KHR_maintenance5`](crate::khr::maintenance5))
    pub const NAME: &CStr = c"VK_AMDX_dense_geometry_format";
    pub const SPEC_VERSION: u32 = 1;
}
