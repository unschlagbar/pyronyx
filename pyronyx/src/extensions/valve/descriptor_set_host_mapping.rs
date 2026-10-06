// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_VALVE_descriptor_set_host_mapping";
pub const SPEC_VERSION: u32 = 1;

pub trait DescriptorSetHostMappingDevice {
    fn get_descriptor_set_layout_host_mapping_info(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    );

    fn get_descriptor_set_host_mapping(&self, descriptor_set: DescriptorSet) -> *mut c_void;
}

impl DescriptorSetHostMappingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>
    #[inline]
    fn get_descriptor_set_layout_host_mapping_info(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
        host_mapping: &mut DescriptorSetLayoutHostMappingInfoVALVE<'_>,
    ) {
        let call = self
            .fns()
            .valve_descriptor_set_host_mapping
            .get_descriptor_set_layout_host_mapping_info_valve
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, binding_reference, host_mapping) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html>
    #[inline]
    fn get_descriptor_set_host_mapping(&self, descriptor_set: DescriptorSet) -> *mut c_void {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .valve_descriptor_set_host_mapping
            .get_descriptor_set_host_mapping_valve
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(self.handle, descriptor_set, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
