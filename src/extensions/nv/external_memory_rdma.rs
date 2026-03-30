// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_NV_external_memory_rdma";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryRdmaDevice {
    fn get_memory_remote_address(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> Result<RemoteAddressNV, Error>;
}

impl ExternalMemoryRdmaDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryRemoteAddressNV.html>
    #[inline]
    fn get_memory_remote_address(
        &self,
        memory_get_remote_address_info: &MemoryGetRemoteAddressInfoNV,
    ) -> Result<RemoteAddressNV, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_external_memory_rdma
                .as_ref()
                .unwrap()
                .get_memory_remote_address_nv)(
                self.handle,
                memory_get_remote_address_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
