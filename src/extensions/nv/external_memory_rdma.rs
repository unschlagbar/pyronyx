// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_memory_remote_address_nv;

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
