pub mod direct_driver_loading {
    use core::ffi::CStr;

    /// Type: `Instance`
    pub const NAME: &CStr = c"VK_LUNARG_direct_driver_loading";
    pub const SPEC_VERSION: u32 = 1;
}
