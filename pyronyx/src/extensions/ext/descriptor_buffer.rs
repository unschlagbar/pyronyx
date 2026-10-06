// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_EXT_descriptor_heap` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
///
/// Requires: (((([`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1) + [`VK_KHR_buffer_device_address`](crate::khr::buffer_device_address) + [`VK_EXT_descriptor_indexing`](crate::ext::descriptor_indexing)) or Vulkan 1.2) + [`VK_KHR_synchronization2`](crate::khr::synchronization2)) or Vulkan 1.3
pub const NAME: &CStr = c"VK_EXT_descriptor_buffer";
pub const SPEC_VERSION: u32 = 1;

pub trait DescriptorBufferDevice {
    fn get_descriptor_set_layout_size(&self, layout: DescriptorSetLayout) -> DeviceSize;

    fn get_descriptor_set_layout_binding_offset(
        &self,
        layout: DescriptorSetLayout,
        binding: u32,
    ) -> DeviceSize;

    fn get_descriptor(&self, descriptor_info: &DescriptorGetInfoEXT, descriptor: &mut [u8]);

    fn get_buffer_opaque_capture_descriptor_data(
        &self,
        info: &BufferCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()>;

    fn get_image_opaque_capture_descriptor_data(
        &self,
        info: &ImageCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()>;

    fn get_image_view_opaque_capture_descriptor_data(
        &self,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()>;

    fn get_sampler_opaque_capture_descriptor_data(
        &self,
        info: &SamplerCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()>;

    fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()>;
}

impl DescriptorBufferDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html>
    #[inline]
    fn get_descriptor_set_layout_size(&self, layout: DescriptorSetLayout) -> DeviceSize {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_descriptor_set_layout_size_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(self.handle, layout, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutBindingOffsetEXT.html>
    #[inline]
    fn get_descriptor_set_layout_binding_offset(
        &self,
        layout: DescriptorSetLayout,
        binding: u32,
    ) -> DeviceSize {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_descriptor_set_layout_binding_offset_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(self.handle, layout, binding, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html>
    #[inline]
    fn get_descriptor(&self, descriptor_info: &DescriptorGetInfoEXT, descriptor: &mut [u8]) {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_descriptor_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                descriptor_info,
                descriptor.len(),
                descriptor.as_mut_ptr().cast(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_buffer_opaque_capture_descriptor_data(
        &self,
        info: &BufferCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_buffer_opaque_capture_descriptor_data_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_image_opaque_capture_descriptor_data(
        &self,
        info: &ImageCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_image_opaque_capture_descriptor_data_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_image_view_opaque_capture_descriptor_data(
        &self,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_image_view_opaque_capture_descriptor_data_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_sampler_opaque_capture_descriptor_data(
        &self,
        info: &SamplerCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_sampler_opaque_capture_descriptor_data_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .get_acceleration_structure_opaque_capture_descriptor_data_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }
}

pub trait DescriptorBufferCommandBuffer {
    fn bind_descriptor_buffers(&self, binding_infos: &[DescriptorBufferBindingInfoEXT]);

    fn set_descriptor_buffer_offsets(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        buffer_indices: &[u32],
        offsets: &[DeviceSize],
    );

    fn bind_descriptor_buffer_embedded_samplers(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    );
}

impl DescriptorBufferCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBuffersEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `DataGraphARM`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_descriptor_buffers(&self, binding_infos: &[DescriptorBufferBindingInfoEXT]) {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .bind_descriptor_buffers_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                binding_infos.len() as u32,
                binding_infos.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDescriptorBufferOffsetsEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `DataGraphARM`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_descriptor_buffer_offsets(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        first_set: u32,
        buffer_indices: &[u32],
        offsets: &[DeviceSize],
    ) {
        assert_eq!(buffer_indices.len(), offsets.len());
        let call = self
            .fns()
            .ext_descriptor_buffer
            .set_descriptor_buffer_offsets_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                pipeline_bind_point,
                layout,
                first_set,
                buffer_indices.len() as u32,
                buffer_indices.as_ptr(),
                offsets.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindDescriptorBufferEmbeddedSamplersEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_descriptor_buffer_embedded_samplers(
        &self,
        pipeline_bind_point: PipelineBindPoint,
        layout: PipelineLayout,
        set: u32,
    ) {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .bind_descriptor_buffer_embedded_samplers_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, pipeline_bind_point, layout, set) };
    }
}
