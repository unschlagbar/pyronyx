// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Instance`
pub const NAME: &CStr = c"VK_EXT_acquire_drm_display";
pub const SPEC_VERSION: u32 = 1;

pub trait AcquireDrmDisplayPhysicalDevice {
    fn acquire_drm_display(&self, drm_fd: i32, display: DisplayKHR) -> Result<()>;

    fn get_drm_display(&self, drm_fd: i32, connector_id: u32) -> Result<DisplayKHR>;
}

impl AcquireDrmDisplayPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquireDrmDisplayEXT.html>
    #[inline]
    fn acquire_drm_display(&self, drm_fd: i32, display: DisplayKHR) -> Result<()> {
        let call = self
            .fns()
            .ext_acquire_drm_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .acquire_drm_display_ext;

        unsafe { (call)(self.handle, drm_fd, display) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDrmDisplayEXT.html>
    #[inline]
    fn get_drm_display(&self, drm_fd: i32, connector_id: u32) -> Result<DisplayKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_acquire_drm_display
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_drm_display_ext;

        unsafe { (call)(self.handle, drm_fd, connector_id, out.as_mut_ptr()) }.init_on_success(out)
    }
}
