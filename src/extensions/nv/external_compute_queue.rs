// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_NV_external_compute_queue";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalComputeQueueDevice {
    fn create_external_compute_queue(
        &self,
        create_info: &ExternalComputeQueueCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ExternalComputeQueueNV, Error>;

    fn destroy_external_compute_queue(
        &self,
        external_queue: ExternalComputeQueueNV,
        allocator: Option<&AllocationCallbacks>,
    );
}

impl ExternalComputeQueueDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateExternalComputeQueueNV.html>
    #[inline]
    fn create_external_compute_queue(
        &self,
        create_info: &ExternalComputeQueueCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ExternalComputeQueueNV, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_external_compute_queue
                .as_ref()
                .unwrap()
                .create_external_compute_queue_nv)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyExternalComputeQueueNV.html>
    #[inline]
    fn destroy_external_compute_queue(
        &self,
        external_queue: ExternalComputeQueueNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .nv_external_compute_queue
                .as_ref()
                .unwrap()
                .destroy_external_compute_queue_nv)(
                self.handle,
                external_queue,
                allocator.map_or(null(), from_ref),
            )
        };
    }
}
