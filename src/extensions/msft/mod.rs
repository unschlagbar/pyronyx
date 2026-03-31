pub mod layered_driver {
    use core::ffi::CStr;

    /// Type: `Device`
    pub const NAME: &CStr = c"VK_MSFT_layered_driver";
    pub const SPEC_VERSION: u32 = 1;
}
