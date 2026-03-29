// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated impls
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec;
use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::c_char;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

impl PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties.html>
    #[inline]
    pub fn get_properties(&self) -> PhysicalDeviceProperties {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_physical_device_properties.unwrap())(self.handle, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties.html>
    #[inline]
    pub fn get_queue_family_properties(&self) -> Vec<QueueFamilyProperties> {
        read_into_vec(|count, data| unsafe {
            (self
                .fns()
                .v1_0
                .get_physical_device_queue_family_properties
                .unwrap())(self.handle, count, data)
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties.html>
    #[inline]
    pub fn get_memory_properties(&self) -> PhysicalDeviceMemoryProperties {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_0
                .get_physical_device_memory_properties
                .unwrap())(self.handle, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures.html>
    #[inline]
    pub fn get_features(&self) -> PhysicalDeviceFeatures {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_physical_device_features.unwrap())(self.handle, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties.html>
    #[inline]
    pub fn get_format_properties(&self, format: Format) -> FormatProperties {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_0
                .get_physical_device_format_properties
                .unwrap())(self.handle, format, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties.html>
    #[inline]
    pub fn get_image_format_properties(
        &self,
        format: Format,
        ty: ImageType,
        tiling: ImageTiling,
        usage: ImageUsageFlags,
        flags: ImageCreateFlags,
    ) -> Result<ImageFormatProperties, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_0
                .get_physical_device_image_format_properties
                .unwrap())(
                self.handle,
                format,
                ty,
                tiling,
                usage,
                flags,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceLayerProperties.html>
    #[inline]
    pub fn enumerate_device_layer_properties(&self) -> Result<Vec<LayerProperties>, vkResult> {
        read_into_vec_result(|count, data| unsafe {
            (self.fns().v1_0.enumerate_device_layer_properties.unwrap())(self.handle, count, data)
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateDeviceExtensionProperties.html>
    #[inline]
    pub fn enumerate_device_extension_properties(
        &self,
        layer_name: Option<&c_char>,
    ) -> Result<Vec<ExtensionProperties>, vkResult> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .v1_0
                .enumerate_device_extension_properties
                .unwrap())(
                self.handle,
                layer_name.map_or(null(), from_ref),
                count,
                data,
            )
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties.html>
    #[inline]
    pub fn get_sparse_image_format_properties(
        &self,
        format: Format,
        ty: ImageType,
        samples: SampleCountFlags,
        usage: ImageUsageFlags,
        tiling: ImageTiling,
    ) -> Vec<SparseImageFormatProperties> {
        read_into_vec(|count, data| unsafe {
            (self
                .fns()
                .v1_0
                .get_physical_device_sparse_image_format_properties
                .unwrap())(self.handle, format, ty, samples, usage, tiling, count, data)
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFeatures2.html>
    #[inline]
    pub fn get_features2(&self) -> PhysicalDeviceFeatures2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.get_physical_device_features2.unwrap())(self.handle, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceProperties2.html>
    #[inline]
    pub fn get_properties2(&self) -> PhysicalDeviceProperties2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.get_physical_device_properties2.unwrap())(
                self.handle,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceFormatProperties2.html>
    #[inline]
    pub fn get_format_properties2(&self, format: Format) -> FormatProperties2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_format_properties2
                .unwrap())(self.handle, format, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceImageFormatProperties2.html>
    #[inline]
    pub fn get_image_format_properties2(
        &self,
        image_format_info: &PhysicalDeviceImageFormatInfo2,
    ) -> Result<ImageFormatProperties2<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_image_format_properties2
                .unwrap())(self.handle, image_format_info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyProperties2.html>
    #[inline]
    pub fn get_queue_family_properties2(
        &self,
        queue_family_properties: &mut [QueueFamilyProperties2],
    ) {
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_queue_family_properties2
                .unwrap())(
                self.handle,
                queue_family_properties.len() as *mut u32,
                queue_family_properties.as_mut_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMemoryProperties2.html>
    #[inline]
    pub fn get_memory_properties2(&self) -> PhysicalDeviceMemoryProperties2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_memory_properties2
                .unwrap())(self.handle, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceSparseImageFormatProperties2.html>
    #[inline]
    pub fn get_sparse_image_format_properties2(
        &self,
        format_info: &PhysicalDeviceSparseImageFormatInfo2,
        properties: &mut [SparseImageFormatProperties2],
    ) {
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_sparse_image_format_properties2
                .unwrap())(
                self.handle,
                format_info,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalBufferProperties.html>
    #[inline]
    pub fn get_external_buffer_properties(
        &self,
        external_buffer_info: &PhysicalDeviceExternalBufferInfo,
    ) -> ExternalBufferProperties<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_external_buffer_properties
                .unwrap())(self.handle, external_buffer_info, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalSemaphoreProperties.html>
    #[inline]
    pub fn get_external_semaphore_properties(
        &self,
        external_semaphore_info: &PhysicalDeviceExternalSemaphoreInfo,
    ) -> ExternalSemaphoreProperties<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_external_semaphore_properties
                .unwrap())(self.handle, external_semaphore_info, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceExternalFenceProperties.html>
    #[inline]
    pub fn get_external_fence_properties(
        &self,
        external_fence_info: &PhysicalDeviceExternalFenceInfo,
    ) -> ExternalFenceProperties<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_physical_device_external_fence_properties
                .unwrap())(self.handle, external_fence_info, out.as_mut_ptr())
        };
        unsafe { out.assume_init() }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceToolProperties.html>
    #[inline]
    pub fn get_tool_properties(
        &self,
        tool_properties: &mut [PhysicalDeviceToolProperties],
    ) -> Result<(), vkResult> {
        unsafe {
            (self.fns().v1_3.get_physical_device_tool_properties.unwrap())(
                self.handle,
                tool_properties.len() as *mut u32,
                tool_properties.as_mut_ptr(),
            )
        }
        .result()
    }
}
