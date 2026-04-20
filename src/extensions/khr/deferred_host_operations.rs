// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_deferred_host_operations";
pub const SPEC_VERSION: u32 = 4;

pub trait DeferredHostOperationsDevice {
    fn create_deferred_operation(
        &self,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DeferredOperationKHR>;

    fn destroy_deferred_operation(
        &self,
        operation: DeferredOperationKHR,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_deferred_operation_max_concurrency(&self, operation: DeferredOperationKHR) -> u32;

    fn get_deferred_operation_result(&self, operation: DeferredOperationKHR) -> Result<()>;

    fn deferred_operation_join(&self, operation: DeferredOperationKHR) -> Result<()>;
}

impl DeferredHostOperationsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDeferredOperationKHR.html>
    #[inline]
    fn create_deferred_operation(
        &self,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DeferredOperationKHR> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .khr_deferred_host_operations
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_deferred_operation_khr;

        unsafe {
            (call)(
                self.handle,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDeferredOperationKHR.html>
    #[inline]
    fn destroy_deferred_operation(
        &self,
        operation: DeferredOperationKHR,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .khr_deferred_host_operations
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_deferred_operation_khr;

        unsafe { (call)(self.handle, operation, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationMaxConcurrencyKHR.html>
    #[inline]
    fn get_deferred_operation_max_concurrency(&self, operation: DeferredOperationKHR) -> u32 {
        let call = self
            .fns()
            .khr_deferred_host_operations
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_deferred_operation_max_concurrency_khr;

        unsafe { (call)(self.handle, operation) }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeferredOperationResultKHR.html>
    #[inline]
    fn get_deferred_operation_result(&self, operation: DeferredOperationKHR) -> Result<()> {
        let call = self
            .fns()
            .khr_deferred_host_operations
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_deferred_operation_result_khr;

        unsafe { (call)(self.handle, operation) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDeferredOperationJoinKHR.html>
    #[inline]
    fn deferred_operation_join(&self, operation: DeferredOperationKHR) -> Result<()> {
        let call = self
            .fns()
            .khr_deferred_host_operations
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .deferred_operation_join_khr;

        unsafe { (call)(self.handle, operation) }.result()
    }
}
