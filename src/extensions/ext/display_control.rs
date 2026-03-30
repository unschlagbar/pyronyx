// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_display_control";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplayControlDevice {
    fn display_power_control(
        &self,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> Result<(), Error>;

    fn register_device_event(
        &self,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence, Error>;

    fn register_display_event(
        &self,
        display: DisplayKHR,
        display_event_info: &DisplayEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence, Error>;

    fn get_swapchain_counter(
        &self,
        swapchain: SwapchainKHR,
        counter: SurfaceCounterFlagsEXT,
    ) -> Result<u64, Error>;
}

impl DisplayControlDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDisplayPowerControlEXT.html>
    #[inline]
    fn display_power_control(
        &self,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .ext_display_control
                .as_ref()
                .unwrap()
                .display_power_control_ext)(self.handle, display, display_power_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>
    #[inline]
    fn register_device_event(
        &self,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_display_control
                .as_ref()
                .unwrap()
                .register_device_event_ext)(
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
    ) -> Result<Fence, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_display_control
                .as_ref()
                .unwrap()
                .register_display_event_ext)(
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
    ) -> Result<u64, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_display_control
                .as_ref()
                .unwrap()
                .get_swapchain_counter_ext)(
                self.handle, swapchain, counter, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}
