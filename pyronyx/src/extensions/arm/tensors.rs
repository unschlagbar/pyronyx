// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: Vulkan 1.3
pub const NAME: &CStr = c"VK_ARM_tensors";
pub const SPEC_VERSION: u32 = 2;

pub trait TensorsDevice {
    fn create_tensor(
        &self,
        create_info: &TensorCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorARM>;

    fn destroy_tensor(&self, tensor: TensorARM, allocator: Option<&AllocationCallbacks>);

    fn create_tensor_view(
        &self,
        create_info: &TensorViewCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorViewARM>;

    fn destroy_tensor_view(
        &self,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_tensor_memory_requirements(
        &self,
        info: &TensorMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2<'_>,
    );

    fn bind_tensor_memory(&self, bind_infos: &[BindTensorMemoryInfoARM]) -> Result<()>;

    fn get_device_tensor_memory_requirements(
        &self,
        info: &DeviceTensorMemoryRequirementsARM,
        memory_requirements: &mut MemoryRequirements2<'_>,
    );

    fn get_tensor_opaque_capture_descriptor_data(
        &self,
        info: &TensorCaptureDescriptorDataInfoARM,
        data: &mut [u8],
    ) -> Result<()>;

    fn get_tensor_view_opaque_capture_descriptor_data(
        &self,
        info: &TensorViewCaptureDescriptorDataInfoARM,
        data: &mut [u8],
    ) -> Result<()>;
}

impl TensorsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html>
    #[inline]
    fn create_tensor(
        &self,
        create_info: &TensorCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorARM> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .arm_tensors
            .create_tensor_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorARM.html>
    #[inline]
    fn destroy_tensor(&self, tensor: TensorARM, allocator: Option<&AllocationCallbacks>) {
        let call = self
            .fns()
            .arm_tensors
            .destroy_tensor_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, tensor, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html>
    #[inline]
    fn create_tensor_view(
        &self,
        create_info: &TensorViewCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorViewARM> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .arm_tensors
            .create_tensor_view_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyTensorViewARM.html>
    #[inline]
    fn destroy_tensor_view(
        &self,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .arm_tensors
            .destroy_tensor_view_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, tensor_view, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html>
    #[inline]
    fn get_tensor_memory_requirements(
        &self,
        info: &TensorMemoryRequirementsInfoARM,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        let call = self
            .fns()
            .arm_tensors
            .get_tensor_memory_requirements_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, memory_requirements) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html>
    #[inline]
    fn bind_tensor_memory(&self, bind_infos: &[BindTensorMemoryInfoARM]) -> Result<()> {
        let call = self
            .fns()
            .arm_tensors
            .bind_tensor_memory_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, bind_infos.len() as u32, bind_infos.as_ptr()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html>
    #[inline]
    fn get_device_tensor_memory_requirements(
        &self,
        info: &DeviceTensorMemoryRequirementsARM,
        memory_requirements: &mut MemoryRequirements2<'_>,
    ) {
        let call = self
            .fns()
            .arm_tensors
            .get_device_tensor_memory_requirements_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, memory_requirements) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html>
    #[inline]
    fn get_tensor_opaque_capture_descriptor_data(
        &self,
        info: &TensorCaptureDescriptorDataInfoARM,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .arm_tensors
            .get_tensor_opaque_capture_descriptor_data_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>
    #[inline]
    fn get_tensor_view_opaque_capture_descriptor_data(
        &self,
        info: &TensorViewCaptureDescriptorDataInfoARM,
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .arm_tensors
            .get_tensor_view_opaque_capture_descriptor_data_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, info, data.as_mut_ptr().cast()) }.result()
    }
}

pub trait TensorsCommandBuffer {
    fn copy_tensor(&self, copy_tensor_info: &CopyTensorInfoARM);
}

impl TensorsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyTensorARM.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_tensor(&self, copy_tensor_info: &CopyTensorInfoARM) {
        let call = self
            .fns()
            .arm_tensors
            .copy_tensor_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, copy_tensor_info) };
    }
}

pub trait TensorsPhysicalDevice {
    fn get_external_tensor_properties(
        &self,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
        external_tensor_properties: &mut ExternalTensorPropertiesARM<'_>,
    );
}

impl TensorsPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>
    #[inline]
    fn get_external_tensor_properties(
        &self,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
        external_tensor_properties: &mut ExternalTensorPropertiesARM<'_>,
    ) {
        let call = self
            .fns()
            .arm_tensors
            .get_physical_device_external_tensor_properties_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                external_tensor_info,
                external_tensor_properties,
            )
        };
    }
}
