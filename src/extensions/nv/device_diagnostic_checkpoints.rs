// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_device_diagnostic_checkpoints";
pub const SPEC_VERSION: u32 = 2;

pub trait DeviceDiagnosticCheckpointsCommandBuffer {
    fn set_checkpoint(&self, checkpoint_marker: &c_void);
}

impl DeviceDiagnosticCheckpointsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetCheckpointNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
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

pub trait DeviceDiagnosticCheckpointsQueue {
    fn get_checkpoint_data(&self, checkpoint_data: &mut [CheckpointDataNV]);
    fn get_checkpoint_data_len(&self) -> usize;

    fn get_checkpoint_data2(&self, checkpoint_data: &mut [CheckpointData2NV]);
}

impl DeviceDiagnosticCheckpointsQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueueCheckpointDataNV.html>
    ///
    /// Call [`get_checkpoint_data_len()`][`Self::get_checkpoint_data_len()`] to query the number of elements to pass to `out`.
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

    /// Returns the required slice length for Call [`get_checkpoint_data`][`Self::get_checkpoint_data`].
    #[inline]
    fn get_checkpoint_data_len(&self) -> usize {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_device_diagnostic_checkpoints
                .as_ref()
                .unwrap()
                .get_queue_checkpoint_data_nv)(
                self.handle, out.as_mut_ptr(), ptr::null_mut()
            );
            out.assume_init() as usize
        }
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
