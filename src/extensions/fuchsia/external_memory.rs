// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_FUCHSIA_external_memory";
pub const SPEC_VERSION: u32 = 1;

pub trait ExternalMemoryDevice {
    fn get_memory_zircon_handle(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t, Error>;

    fn get_memory_zircon_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
    ) -> Result<MemoryZirconHandlePropertiesFUCHSIA<'_>, Error>;
}

impl ExternalMemoryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandleFUCHSIA.html>
    #[inline]
    fn get_memory_zircon_handle(
        &self,
        get_zircon_handle_info: &MemoryGetZirconHandleInfoFUCHSIA,
    ) -> Result<zx_handle_t, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .fuchsia_external_memory
                .as_ref()
                .unwrap()
                .get_memory_zircon_handle_fuchsia)(
                self.handle,
                get_zircon_handle_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemoryZirconHandlePropertiesFUCHSIA.html>
    #[inline]
    fn get_memory_zircon_handle_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        zircon_handle: zx_handle_t,
    ) -> Result<MemoryZirconHandlePropertiesFUCHSIA<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .fuchsia_external_memory
                .as_ref()
                .unwrap()
                .get_memory_zircon_handle_properties_fuchsia)(
                self.handle,
                handle_type,
                zircon_handle,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
