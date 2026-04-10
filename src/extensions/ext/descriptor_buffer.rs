// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_descriptor_buffer";
pub const SPEC_VERSION: u32 = 1;

pub trait DescriptorBufferDevice {
    fn get_descriptor_set_layout_size(&self, layout: DescriptorSetLayout) -> DeviceSize;

    fn get_descriptor_set_layout_binding_offset(
        &self,
        layout: DescriptorSetLayout,
        binding: u32,
    ) -> DeviceSize;

    fn get_descriptor(&self, descriptor_info: &DescriptorGetInfoEXT, descriptor: &mut [c_void]);

    fn get_buffer_opaque_capture_descriptor_data(
        &self,
        info: &BufferCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error>;

    fn get_image_opaque_capture_descriptor_data(
        &self,
        info: &ImageCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error>;

    fn get_image_view_opaque_capture_descriptor_data(
        &self,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error>;

    fn get_sampler_opaque_capture_descriptor_data(
        &self,
        info: &SamplerCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error>;

    fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error>;
}

impl DescriptorBufferDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSizeEXT.html>
    #[inline]
    fn get_descriptor_set_layout_size(&self, layout: DescriptorSetLayout) -> DeviceSize {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_descriptor_set_layout_size_ext;

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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_descriptor_set_layout_binding_offset_ext;

        unsafe {
            (call)(self.handle, layout, binding, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorEXT.html>
    #[inline]
    fn get_descriptor(&self, descriptor_info: &DescriptorGetInfoEXT, descriptor: &mut [c_void]) {
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_descriptor_ext;

        unsafe {
            (call)(
                self.handle,
                descriptor_info,
                descriptor.len() as usize,
                descriptor.as_mut_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_buffer_opaque_capture_descriptor_data(
        &self,
        info: &BufferCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_buffer_opaque_capture_descriptor_data_ext;

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_image_opaque_capture_descriptor_data(
        &self,
        info: &ImageCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_image_opaque_capture_descriptor_data_ext;

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_image_view_opaque_capture_descriptor_data(
        &self,
        info: &ImageViewCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_image_view_opaque_capture_descriptor_data_ext;

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSamplerOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_sampler_opaque_capture_descriptor_data(
        &self,
        info: &SamplerCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_sampler_opaque_capture_descriptor_data_ext;

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT.html>
    #[inline]
    fn get_acceleration_structure_opaque_capture_descriptor_data(
        &self,
        info: &AccelerationStructureCaptureDescriptorDataInfoEXT,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_descriptor_buffer
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_acceleration_structure_opaque_capture_descriptor_data_ext;

        unsafe { (call)(self.handle, info, out.as_mut_ptr()) }.init_on_success(out)
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .bind_descriptor_buffers_ext;

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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_descriptor_buffer_offsets_ext;

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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .bind_descriptor_buffer_embedded_samplers_ext;

        unsafe { (call)(self.handle, pipeline_bind_point, layout, set) };
    }
}
