// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;

/// Type: `Device`
///
/// Requires: [`VK_ARM_data_graph`](crate::arm::data_graph)
pub const NAME: &CStr = c"VK_ARM_data_graph_optical_flow";
pub const SPEC_VERSION: u32 = 1;

pub trait DataGraphOpticalFlowPhysicalDevice {
    fn get_queue_family_data_graph_engine_operation_properties(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        properties: &mut BaseOutStructure<'_>,
    ) -> Result<()>;

    fn get_queue_family_data_graph_optical_flow_image_formats(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
        image_format_properties: &mut [DataGraphOpticalFlowImageFormatPropertiesARM],
    ) -> Result<()>;
    fn get_queue_family_data_graph_optical_flow_image_formats_len(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
    ) -> Result<usize>;
}

impl DataGraphOpticalFlowPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    #[inline]
    fn get_queue_family_data_graph_engine_operation_properties(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        properties: &mut BaseOutStructure<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .arm_data_graph_optical_flow
            .get_physical_device_queue_family_data_graph_engine_operation_properties_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                queue_family_index,
                queue_family_data_graph_properties,
                properties,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>
    ///
    /// Call [`get_queue_family_data_graph_optical_flow_image_formats_len()`][`Self::get_queue_family_data_graph_optical_flow_image_formats_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_queue_family_data_graph_optical_flow_image_formats(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
        image_format_properties: &mut [DataGraphOpticalFlowImageFormatPropertiesARM],
    ) -> Result<()> {
        let mut format_count = image_format_properties.len() as u32;
        let call = self
            .fns()
            .arm_data_graph_optical_flow
            .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                queue_family_index,
                queue_family_data_graph_properties,
                optical_flow_image_format_info,
                &mut format_count,
                image_format_properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_queue_family_data_graph_optical_flow_image_formats`][`Self::get_queue_family_data_graph_optical_flow_image_formats`].
    #[inline]
    fn get_queue_family_data_graph_optical_flow_image_formats_len(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
    ) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_data_graph_optical_flow
                .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                queue_family_index,
                queue_family_data_graph_properties,
                optical_flow_image_format_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
