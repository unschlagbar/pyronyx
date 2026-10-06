// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: [`VK_KHR_external_memory`](crate::khr::external_memory) or Vulkan 1.1
pub const NAME: &CStr = c"VK_NV_external_memory_rdma";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryRdmaDevice {
    fn get_memory_remote_address(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> Result<RemoteAddressNV>;
}

impl ExternalMemoryRdmaDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html>
    #[inline]
    fn get_memory_remote_address(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> Result<RemoteAddressNV> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_memory_rdma
            .get_memory_remote_address_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                memory_get_remote_address_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
