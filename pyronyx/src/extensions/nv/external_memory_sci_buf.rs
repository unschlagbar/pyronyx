// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: Vulkan 1.1
pub const NAME: &CStr = c"VK_NV_external_memory_sci_buf";
pub const SPEC_VERSION: u32 = 2;

pub trait ExternalMemorySciBufDevice {
    fn get_memory_sci_buf(&self, get_sci_buf_info: &MemoryGetSciBufInfoNV) -> Result<NvSciBufObj>;
}

impl ExternalMemorySciBufDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMemorySciBufNV.html>
    #[inline]
    fn get_memory_sci_buf(&self, get_sci_buf_info: &MemoryGetSciBufInfoNV) -> Result<NvSciBufObj> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nv_external_memory_sci_buf
            .get_memory_sci_buf_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, get_sci_buf_info, out.as_mut_ptr()) }.init_on_success(out)
    }
}

pub trait ExternalMemorySciBufPhysicalDevice {
    fn get_external_memory_sci_buf_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: NvSciBufObj,
        memory_sci_buf_properties: &mut MemorySciBufPropertiesNV<'_>,
    ) -> Result<()>;

    fn get_sci_buf_attributes(&self, attributes: NvSciBufAttrList) -> Result<()>;
}

impl ExternalMemorySciBufPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV.html>
    #[inline]
    fn get_external_memory_sci_buf_properties(
        &self,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: NvSciBufObj,
        memory_sci_buf_properties: &mut MemorySciBufPropertiesNV<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_external_memory_sci_buf
            .get_physical_device_external_memory_sci_buf_properties_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, handle_type, handle, memory_sci_buf_properties) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSciBufAttributesNV.html>
    #[inline]
    fn get_sci_buf_attributes(&self, attributes: NvSciBufAttrList) -> Result<()> {
        let call = self
            .fns()
            .nv_external_memory_sci_buf
            .get_physical_device_sci_buf_attributes_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, attributes) }.result()
    }
}
