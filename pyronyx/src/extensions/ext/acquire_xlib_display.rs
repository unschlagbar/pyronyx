// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Instance`
///
/// Requires: [`VK_EXT_direct_mode_display`](crate::ext::direct_mode_display)
pub const NAME: &CStr = c"VK_EXT_acquire_xlib_display";
pub const SPEC_VERSION: u32 = 1;

pub trait AcquireXlibDisplayPhysicalDevice {
    fn acquire_xlib_display(&self, dpy: *mut Display, display: DisplayKHR) -> Result<()>;

    fn get_rand_r_output_display(
        &self,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> Result<DisplayKHR>;
}

impl AcquireXlibDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html>
    #[inline]
    fn acquire_xlib_display(&self, dpy: *mut Display, display: DisplayKHR) -> Result<()> {
        let call = self
            .fns()
            .ext_acquire_xlib_display
            .acquire_xlib_display_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, dpy, display) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html>
    #[inline]
    fn get_rand_r_output_display(
        &self,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> Result<DisplayKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_acquire_xlib_display
            .get_rand_r_output_display_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, dpy, rr_output, out.as_mut_ptr()) }.init_on_success(out)
    }
}
