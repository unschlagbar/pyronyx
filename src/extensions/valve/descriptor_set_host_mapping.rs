// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_VALVE_descriptor_set_host_mapping";
pub const SPEC_VERSION: u32 = 1;

pub trait DescriptorSetHostMappingDevice {
    fn get_descriptor_set_layout_host_mapping_info(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
    ) -> DescriptorSetLayoutHostMappingInfoVALVE<'_>;

    fn get_descriptor_set_host_mapping(&self, descriptor_set: DescriptorSet) -> *mut c_void;
}

impl DescriptorSetHostMappingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutHostMappingInfoVALVE.html>
    #[inline]
    fn get_descriptor_set_layout_host_mapping_info(
        &self,
        binding_reference: &DescriptorSetBindingReferenceVALVE,
    ) -> DescriptorSetLayoutHostMappingInfoVALVE<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .valve_descriptor_set_host_mapping
                .as_ref()
                .unwrap()
                .get_descriptor_set_layout_host_mapping_info_valve)(
                self.handle,
                binding_reference,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetHostMappingVALVE.html>
    #[inline]
    fn get_descriptor_set_host_mapping(&self, descriptor_set: DescriptorSet) -> *mut c_void {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .valve_descriptor_set_host_mapping
                .as_ref()
                .unwrap()
                .get_descriptor_set_host_mapping_valve)(
                self.handle,
                descriptor_set,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }
}
