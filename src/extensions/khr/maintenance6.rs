// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_maintenance6";
pub const SPEC_VERSION: u32 = 1;

pub trait Maintenance6CommandBuffer {
    fn set_descriptor_buffer_offsets2(
        &self,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    );

    fn bind_descriptor_buffer_embedded_samplers2(
        &self,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    );
}

impl Maintenance6CommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsets2EXT.html>
    #[inline]
    fn set_descriptor_buffer_offsets2(
        &self,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    ) {
        unsafe {
            (self
                .fns()
                .khr_maintenance6
                .as_ref()
                .unwrap()
                .set_descriptor_buffer_offsets2_ext)(
                self.handle,
                set_descriptor_buffer_offsets_info,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
    #[inline]
    fn bind_descriptor_buffer_embedded_samplers2(
        &self,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) {
        unsafe {
            (self
                .fns()
                .khr_maintenance6
                .as_ref()
                .unwrap()
                .bind_descriptor_buffer_embedded_samplers2_ext)(
                self.handle,
                bind_descriptor_buffer_embedded_samplers_info,
            )
        };
    }
}
