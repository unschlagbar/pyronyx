// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_EXT_memory_priority`](crate::ext::memory_priority)
pub const NAME: &CStr = c"VK_EXT_pageable_device_local_memory";
pub const SPEC_VERSION: u32 = 1;

pub trait PageableDeviceLocalMemoryDevice {
    fn set_device_memory_priority(&self, memory: DeviceMemory, priority: f32);
}

impl PageableDeviceLocalMemoryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html>
    #[inline]
    fn set_device_memory_priority(&self, memory: DeviceMemory, priority: f32) {
        let call = self
            .fns()
            .ext_pageable_device_local_memory
            .set_device_memory_priority_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, memory, priority) };
    }
}
