// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: ([`VK_KHR_external_memory_capabilities`](crate::khr::external_memory_capabilities) + [`VK_KHR_external_memory`](crate::khr::external_memory)) or Vulkan 1.1
pub const NAME: &CStr = c"VK_FUCHSIA_external_memory";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryDevice {
    fn get_memory_zircon_handle(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t>;

    fn get_memory_zircon_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> Result<()>;
}

impl ExternalMemoryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html>
    #[inline]
    fn get_memory_zircon_handle(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .fuchsia_external_memory
            .get_memory_zircon_handle_fuchsia
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, get_zircon_handle_info, out.as_mut_ptr()) }
            .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>
    #[inline]
    fn get_memory_zircon_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
        memory_zircon_handle_properties: &mut MemoryZirconHandlePropertiesFUCHSIA<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .fuchsia_external_memory
            .get_memory_zircon_handle_properties_fuchsia
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                handle_type,
                zircon_handle,
                memory_zircon_handle_properties,
            )
        }
        .result()
    }
}
