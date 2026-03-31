pub mod amigo_profiling {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_SEC_amigo_profiling";
    pub const SPEC_VERSION: u32 = 1;
}
pub mod pipeline_cache_incremental_mode {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_SEC_pipeline_cache_incremental_mode";
    pub const SPEC_VERSION: u32 = 1;
}
