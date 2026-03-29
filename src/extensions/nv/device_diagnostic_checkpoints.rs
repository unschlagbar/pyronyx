// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;

pub const NAME: &CStr = c"VK_NV_device_diagnostic_checkpoints";
pub const SPEC_VERSION: u32 = 2;

pub trait DeviceDiagnosticCheckpointsQueue {
    fn get_checkpoint_data(&self, checkpoint_data: &mut [CheckpointDataNV]);

    fn get_checkpoint_data2(&self, checkpoint_data: &mut [CheckpointData2NV]);
}

impl DeviceDiagnosticCheckpointsQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html>
    #[inline]
    fn get_checkpoint_data(&self, checkpoint_data: &mut [CheckpointDataNV]) {
        unsafe {
            (self
                .fns()
                .nv_device_diagnostic_checkpoints
                .as_ref()
                .unwrap()
                .get_queue_checkpoint_data_nv)(
                self.handle,
                checkpoint_data.len() as *mut u32,
                checkpoint_data.as_mut_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointData2NV.html>
    #[inline]
    fn get_checkpoint_data2(&self, checkpoint_data: &mut [CheckpointData2NV]) {
        unsafe {
            (self
                .fns()
                .nv_device_diagnostic_checkpoints
                .as_ref()
                .unwrap()
                .get_queue_checkpoint_data2_nv)(
                self.handle,
                checkpoint_data.len() as *mut u32,
                checkpoint_data.as_mut_ptr(),
            )
        };
    }
}

pub trait DeviceDiagnosticCheckpointsCommandBuffer {
    fn set_checkpoint(&self, checkpoint_marker: &c_void);
}

impl DeviceDiagnosticCheckpointsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html>
    #[inline]
    fn set_checkpoint(&self, checkpoint_marker: &c_void) {
        unsafe {
            (self
                .fns()
                .nv_device_diagnostic_checkpoints
                .as_ref()
                .unwrap()
                .set_checkpoint_nv)(self.handle, checkpoint_marker)
        };
    }
}
