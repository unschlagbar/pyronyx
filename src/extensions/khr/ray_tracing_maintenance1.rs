// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_ray_tracing_maintenance1";
pub const SPEC_VERSION: u32 = 1;

pub trait RayTracingMaintenance1CommandBuffer {
    fn trace_rays_indirect2(&self, indirect_device_address: DeviceAddress);
}

impl RayTracingMaintenance1CommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdTraceRaysIndirect2KHR.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn trace_rays_indirect2(&self, indirect_device_address: DeviceAddress) {
        let call = self
            .fns()
            .khr_ray_tracing_maintenance1
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .trace_rays_indirect2_khr;

        unsafe { (call)(self.handle, indirect_device_address) };
    }
}
