// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_display_control";
pub const SPEC_VERSION: u32 = 1;

pub trait DisplayControlDevice {
    fn display_power_control(
        &self,
        display: DisplayKHR,
        display_power_info: &DisplayPowerInfoEXT,
    ) -> Result<(), Error>;

    fn register_event(
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
        let call = self
            .fns()
            .ext_display_control
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .display_power_control_ext;

        unsafe { (call)(self.handle, display, display_power_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterDeviceEventEXT.html>
    #[inline]
    fn register_event(
        &self,
        device_event_info: &DeviceEventInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_display_control
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .register_device_event_ext;

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
    ) -> Result<Fence, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_display_control
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .register_display_event_ext;

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
    ) -> Result<u64, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_display_control
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_swapchain_counter_ext;

        unsafe { (call)(self.handle, swapchain, counter, out.as_mut_ptr()) }.init_on_success(out)
    }
}
