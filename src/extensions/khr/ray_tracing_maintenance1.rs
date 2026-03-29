// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_KHR_ray_tracing_maintenance1";
pub const SPEC_VERSION: u32 = 1;

pub trait RayTracingMaintenance1CommandBuffer {
    fn trace_rays_indirect2(&self, indirect_device_address: DeviceAddress);
}

impl RayTracingMaintenance1CommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html>
    #[inline]
    fn trace_rays_indirect2(&self, indirect_device_address: DeviceAddress) {
        unsafe {
            (self
                .fns()
                .khr_ray_tracing_maintenance1
                .as_ref()
                .unwrap()
                .trace_rays_indirect2_khr)(self.handle, indirect_device_address)
        };
    }
}
