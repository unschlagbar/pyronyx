// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_acceleration_structure`](crate::khr::acceleration_structure)
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
            .trace_rays_indirect2_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, indirect_device_address) };
    }
}
