// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_ARM_tensors";
pub const SPEC_VERSION: u32 = 1;

pub trait TensorsDevice {
    fn create_tensor(
        &self,
        create_info: &TensorCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorARM, Error>;

    fn destroy_tensor(&self, tensor: TensorARM, allocator: Option<&AllocationCallbacks>);

    fn create_tensor_view(
        &self,
        create_info: &TensorViewCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorViewARM, Error>;

    fn destroy_tensor_view(
        &self,
        tensor_view: TensorViewARM,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_tensor_memory_requirements(
        &self,
        info: &TensorMemoryRequirementsInfoARM,
    ) -> MemoryRequirements2<'_>;

    fn bind_tensor_memory(&self, bind_infos: &[BindTensorMemoryInfoARM]) -> Result<(), Error>;

    fn get_device_tensor_memory_requirements(
        &self,
        info: &DeviceTensorMemoryRequirementsARM,
    ) -> MemoryRequirements2<'_>;

    fn get_tensor_opaque_capture_descriptor_data(
        &self,
        info: &TensorCaptureDescriptorDataInfoARM,
    ) -> Result<c_void, Error>;

    fn get_tensor_view_opaque_capture_descriptor_data(
        &self,
        info: &TensorViewCaptureDescriptorDataInfoARM,
    ) -> Result<c_void, Error>;
}

impl TensorsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorARM.html>
    #[inline]
    fn create_tensor(
        &self,
        create_info: &TensorCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorARM, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().arm_tensors.as_ref().unwrap().create_tensor_arm)(
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
        unsafe {
            (self.fns().arm_tensors.as_ref().unwrap().destroy_tensor_arm)(
                self.handle,
                tensor,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateTensorViewARM.html>
    #[inline]
    fn create_tensor_view(
        &self,
        create_info: &TensorViewCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<TensorViewARM, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .create_tensor_view_arm)(
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
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .destroy_tensor_view_arm)(
                self.handle,
                tensor_view,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorMemoryRequirementsARM.html>
    #[inline]
    fn get_tensor_memory_requirements(
        &self,
        info: &TensorMemoryRequirementsInfoARM,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .get_tensor_memory_requirements_arm)(self.handle, info, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindTensorMemoryARM.html>
    #[inline]
    fn bind_tensor_memory(&self, bind_infos: &[BindTensorMemoryInfoARM]) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .bind_tensor_memory_arm)(
                self.handle, bind_infos.len() as u32, bind_infos.as_ptr()
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceTensorMemoryRequirementsARM.html>
    #[inline]
    fn get_device_tensor_memory_requirements(
        &self,
        info: &DeviceTensorMemoryRequirementsARM,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .get_device_tensor_memory_requirements_arm)(
                self.handle, info, out.as_mut_ptr()
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorOpaqueCaptureDescriptorDataARM.html>
    #[inline]
    fn get_tensor_opaque_capture_descriptor_data(
        &self,
        info: &TensorCaptureDescriptorDataInfoARM,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .get_tensor_opaque_capture_descriptor_data_arm)(
                self.handle, info, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetTensorViewOpaqueCaptureDescriptorDataARM.html>
    #[inline]
    fn get_tensor_view_opaque_capture_descriptor_data(
        &self,
        info: &TensorViewCaptureDescriptorDataInfoARM,
    ) -> Result<c_void, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .get_tensor_view_opaque_capture_descriptor_data_arm)(
                self.handle,
                info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
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
        unsafe {
            (self.fns().arm_tensors.as_ref().unwrap().copy_tensor_arm)(
                self.handle,
                copy_tensor_info,
            )
        };
    }
}

pub trait TensorsPhysicalDevice {
    fn get_external_tensor_properties(
        &self,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
    ) -> ExternalTensorPropertiesARM<'_>;
}

impl TensorsPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalTensorPropertiesARM.html>
    #[inline]
    fn get_external_tensor_properties(
        &self,
        external_tensor_info: &PhysicalDeviceExternalTensorInfoARM,
    ) -> ExternalTensorPropertiesARM<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_tensors
                .as_ref()
                .unwrap()
                .get_physical_device_external_tensor_properties_arm)(
                self.handle,
                external_tensor_info,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }
}
