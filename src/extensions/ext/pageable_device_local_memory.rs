// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_EXT_pageable_device_local_memory";
pub const SPEC_VERSION: u32 = 1;

pub trait PageableDeviceLocalMemoryDevice {
    fn set_device_memory_priority(&self, memory: DeviceMemory, priority: f32);
}

impl PageableDeviceLocalMemoryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDeviceMemoryPriorityEXT.html>
    #[inline]
    fn set_device_memory_priority(&self, memory: DeviceMemory, priority: f32) {
        unsafe {
            (self
                .fns()
                .ext_pageable_device_local_memory
                .as_ref()
                .unwrap()
                .set_device_memory_priority_ext)(self.handle, memory, priority)
        };
    }
}
