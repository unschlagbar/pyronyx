// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_EXT_acquire_xlib_display";
pub const SPEC_VERSION: u32 = 1;

pub trait AcquireXlibDisplayPhysicalDevice {
    fn acquire_xlib_display(&self, dpy: *mut Display, display: DisplayKHR) -> Result<(), vkResult>;

    fn get_rand_r_output_display(
        &self,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> Result<DisplayKHR, vkResult>;
}

impl AcquireXlibDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireXlibDisplayEXT.html>
    #[inline]
    fn acquire_xlib_display(&self, dpy: *mut Display, display: DisplayKHR) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .ext_acquire_xlib_display
                .as_ref()
                .unwrap()
                .acquire_xlib_display_ext)(self.handle, dpy, display)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRandROutputDisplayEXT.html>
    #[inline]
    fn get_rand_r_output_display(
        &self,
        dpy: *mut Display,
        rr_output: RROutput,
    ) -> Result<DisplayKHR, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_acquire_xlib_display
                .as_ref()
                .unwrap()
                .get_rand_r_output_display_ext)(
                self.handle, dpy, rr_output, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}
