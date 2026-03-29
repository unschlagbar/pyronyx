// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_NV_compute_occupancy_priority";
pub const SPEC_VERSION: u32 = 1;

pub trait ComputeOccupancyPriorityCommandBuffer {
    fn set_compute_occupancy_priority(&self, parameters: &ComputeOccupancyPriorityParametersNV);
}

impl ComputeOccupancyPriorityCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetComputeOccupancyPriorityNV.html>
    #[inline]
    fn set_compute_occupancy_priority(&self, parameters: &ComputeOccupancyPriorityParametersNV) {
        unsafe {
            (self
                .fns()
                .nv_compute_occupancy_priority
                .as_ref()
                .unwrap()
                .set_compute_occupancy_priority_nv)(self.handle, parameters)
        };
    }
}
