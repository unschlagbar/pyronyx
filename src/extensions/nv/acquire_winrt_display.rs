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
    fn acquire_winrt_display(&self, display: DisplayKHR) -> Result<()>;

    fn get_winrt_display(&self, device_relative_id: u32) -> Result<DisplayKHR>;
}

impl AcquireWinrtDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireWinrtDisplayNV.html>
    #[inline]
    fn acquire_winrt_display(&self, display: DisplayKHR) -> Result<()> {
        let call = self
            .fns()
            .nv_acquire_winrt_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .acquire_winrt_display_nv;

        unsafe { (call)(self.handle, display) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetWinrtDisplayNV.html>
    #[inline]
    fn get_winrt_display(&self, device_relative_id: u32) -> Result<DisplayKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_acquire_winrt_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_winrt_display_nv;

        unsafe { (call)(self.handle, device_relative_id, out.as_mut_ptr()) }.init_on_success(out)
    }
}
