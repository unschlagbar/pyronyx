// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_descriptor_heap";
pub const SPEC_VERSION: u32 = 1;

pub trait DescriptorHeapDevice {
    fn write_sampler_descriptors(
        &self,
        samplers: &[SamplerCreateInfo],
        descriptors: &[HostAddressRangeEXT],
    ) -> Result<(), Error>;

    fn write_resource_descriptors(
        &self,
        resources: &[ResourceDescriptorInfoEXT],
        descriptors: &[HostAddressRangeEXT],
    ) -> Result<(), Error>;

    fn register_custom_border_color(
        &self,
        border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: bool,
    ) -> Result<u32, Error>;

    fn unregister_custom_border_color(&self, index: u32);

    fn get_image_opaque_capture_data(
        &self,
        images: &[Image],
        datas: &mut [HostAddressRangeEXT],
    ) -> Result<(), Error>;

    fn get_tensor_opaque_capture_data(
        &self,
        tensors: &[TensorARM],
        datas: &mut [HostAddressRangeEXT],
    ) -> Result<(), Error>;
}

impl DescriptorHeapDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteSamplerDescriptorsEXT.html>
    #[inline]
    fn write_sampler_descriptors(
        &self,
        samplers: &[SamplerCreateInfo],
        descriptors: &[HostAddressRangeEXT],
    ) -> Result<(), Error> {
        assert_eq!(samplers.len(), descriptors.len());
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .write_sampler_descriptors_ext)(
                self.handle,
                samplers.len() as u32,
                samplers.as_ptr(),
                descriptors.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteResourceDescriptorsEXT.html>
    #[inline]
    fn write_resource_descriptors(
        &self,
        resources: &[ResourceDescriptorInfoEXT],
        descriptors: &[HostAddressRangeEXT],
    ) -> Result<(), Error> {
        assert_eq!(resources.len(), descriptors.len());
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .write_resource_descriptors_ext)(
                self.handle,
                resources.len() as u32,
                resources.as_ptr(),
                descriptors.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkRegisterCustomBorderColorEXT.html>
    #[inline]
    fn register_custom_border_color(
        &self,
        border_color: &SamplerCustomBorderColorCreateInfoEXT,
        request_index: bool,
    ) -> Result<u32, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .register_custom_border_color_ext)(
                self.handle,
                border_color,
                request_index as _,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnregisterCustomBorderColorEXT.html>
    #[inline]
    fn unregister_custom_border_color(&self, index: u32) {
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .unregister_custom_border_color_ext)(self.handle, index)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageOpaqueCaptureDataEXT.html>
    #[inline]
    fn get_image_opaque_capture_data(
        &self,
        images: &[Image],
        datas: &mut [HostAddressRangeEXT],
    ) -> Result<(), Error> {
        assert_eq!(images.len(), datas.len());
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .get_image_opaque_capture_data_ext)(
                self.handle,
                images.len() as u32,
                images.as_ptr(),
                datas.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDataARM.html>
    #[inline]
    fn get_tensor_opaque_capture_data(
        &self,
        tensors: &[TensorARM],
        datas: &mut [HostAddressRangeEXT],
    ) -> Result<(), Error> {
        assert_eq!(tensors.len(), datas.len());
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .get_tensor_opaque_capture_data_arm)(
                self.handle,
                tensors.len() as u32,
                tensors.as_ptr(),
                datas.as_mut_ptr(),
            )
        }
        .result()
    }
}

pub trait DescriptorHeapCommandBuffer {
    fn bind_sampler_heap(&self, bind_info: &BindHeapInfoEXT);

    fn bind_resource_heap(&self, bind_info: &BindHeapInfoEXT);

    fn push_data(&self, push_data_info: &PushDataInfoEXT);
}

impl DescriptorHeapCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindSamplerHeapEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_sampler_heap(&self, bind_info: &BindHeapInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .bind_sampler_heap_ext)(self.handle, bind_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBindResourceHeapEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn bind_resource_heap(&self, bind_info: &BindHeapInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .bind_resource_heap_ext)(self.handle, bind_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdPushDataEXT.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn push_data(&self, push_data_info: &PushDataInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .push_data_ext)(self.handle, push_data_info)
        };
    }
}

pub trait DescriptorHeapPhysicalDevice {
    fn get_descriptor_size(&self, descriptor_type: DescriptorType) -> DeviceSize;
}

impl DescriptorHeapPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceDescriptorSizeEXT.html>
    #[inline]
    fn get_descriptor_size(&self, descriptor_type: DescriptorType) -> DeviceSize {
        unsafe {
            (self
                .fns()
                .ext_descriptor_heap
                .as_ref()
                .unwrap()
                .get_physical_device_descriptor_size_ext)(self.handle, descriptor_type)
        }
    }
}
