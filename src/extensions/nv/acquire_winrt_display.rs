// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_acquire_winrt_display";
pub const SPEC_VERSION: u32 = 1;

pub trait AcquireWinrtDisplayPhysicalDevice {
    fn acquire_winrt_display(&self, display: DisplayKHR) -> Result<(), Error>;

    fn get_winrt_display(&self, device_relative_id: u32) -> Result<DisplayKHR, Error>;
}

impl AcquireWinrtDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html>
    #[inline]
    fn acquire_winrt_display(&self, display: DisplayKHR) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_acquire_winrt_display
                .as_ref()
                .unwrap()
                .acquire_winrt_display_nv)(self.handle, display)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html>
    #[inline]
    fn get_winrt_display(&self, device_relative_id: u32) -> Result<DisplayKHR, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_acquire_winrt_display
                .as_ref()
                .unwrap()
                .get_winrt_display_nv)(self.handle, device_relative_id, out.as_mut_ptr())
        }
        .init_on_success(out)
    }
}
