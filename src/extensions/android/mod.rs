pub mod external_memory_android_hardware_buffer;
pub mod external_format_resolve {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_ANDROID_external_format_resolve";
    pub const SPEC_VERSION: u32 = 1;
}
