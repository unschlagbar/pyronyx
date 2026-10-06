// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_NV_compute_occupancy_priority";
pub const SPEC_VERSION: u32 = 1;

pub trait ComputeOccupancyPriorityCommandBuffer {
    fn set_compute_occupancy_priority(&self, parameters: &ComputeOccupancyPriorityParametersNV);
}

impl ComputeOccupancyPriorityCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_compute_occupancy_priority(&self, parameters: &ComputeOccupancyPriorityParametersNV) {
        let call = self
            .fns()
            .nv_compute_occupancy_priority
            .set_compute_occupancy_priority_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, parameters) };
    }
}
