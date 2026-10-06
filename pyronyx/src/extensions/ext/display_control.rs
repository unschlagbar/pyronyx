// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: [`VK_EXT_display_surface_counter`](crate::ext::display_surface_counter) + [`VK_KHR_swapchain`](crate::khr::swapchain)
pub const NAME: &CStr = c"VK_EXT_display_control";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplayControlDevice {
    fn display_power_control(
        &self,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> Result<()>;

    fn register_event(
        &self,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence>;

    fn register_display_event(
        &self,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence>;

    fn get_swapchain_counter(
        &self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
    ) -> Result<u64>;
}

impl DisplayControlDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html>
    #[inline]
    fn display_power_control(
        &self,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_display_control
            .display_power_control_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, display, display_power_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>
    #[inline]
    fn register_event(
        &self,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_display_control
            .register_device_event_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                device_event_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDisplayEventEXT.html>
    #[inline]
    fn register_display_event(
        &self,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_display_control
            .register_display_event_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                display,
                display_event_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSwapchainCounterEXT.html>
    #[inline]
    fn get_swapchain_counter(
        &self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
    ) -> Result<u64> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_display_control
            .get_swapchain_counter_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, swapchain, counter, out.as_mut_ptr()) }.init_on_success(out)
    }
}
