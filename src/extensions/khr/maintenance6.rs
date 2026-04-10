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
    ///
    /// Queues types: `Graphics`, `Compute`, `DataGraphARM`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_descriptor_buffer_offsets2(
        &self,
        set_descriptor_buffer_offsets_info: &SetDescriptorBufferOffsetsInfoEXT,
    ) {
        let call = self
            .fns()
            .khr_maintenance6
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_descriptor_buffer_offsets2_ext;

        unsafe { (call)(self.handle, set_descriptor_buffer_offsets_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplers2EXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_descriptor_buffer_embedded_samplers2(
        &self,
        bind_descriptor_buffer_embedded_samplers_info: &BindDescriptorBufferEmbeddedSamplersInfoEXT,
    ) {
        let call = self
            .fns()
            .khr_maintenance6
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .bind_descriptor_buffer_embedded_samplers2_ext;

        unsafe { (call)(self.handle, bind_descriptor_buffer_embedded_samplers_info) };
    }
}
