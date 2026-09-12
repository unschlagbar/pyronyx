// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_ARM_data_graph_optical_flow";
pub const SPEC_VERSION: u32 = 1;

pub trait DataGraphOpticalFlowPhysicalDevice {
    fn get_queue_family_data_graph_engine_operation_properties(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
    ) -> Result<BaseOutStructure<'_>>;

    fn get_queue_family_data_graph_optical_flow_image_formats(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
    ) -> Result<Vec<DataGraphOpticalFlowImageFormatPropertiesARM<'_>>>;
}

impl DataGraphOpticalFlowPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM.html>
    #[inline]
    fn get_queue_family_data_graph_engine_operation_properties(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
    ) -> Result<BaseOutStructure<'_>> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .arm_data_graph_optical_flow
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_queue_family_data_graph_engine_operation_properties_arm;

        unsafe {
            (call)(
                self.handle,
                queue_family_index,
                queue_family_data_graph_properties,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM.html>
    #[inline]
    fn get_queue_family_data_graph_optical_flow_image_formats(
        &self,
        queue_family_index: u32,
        queue_family_data_graph_properties: &QueueFamilyDataGraphPropertiesARM,
        optical_flow_image_format_info: &DataGraphOpticalFlowImageFormatInfoARM,
    ) -> Result<Vec<DataGraphOpticalFlowImageFormatPropertiesARM<'_>>> {
        let call = self
            .fns()
            .arm_data_graph_optical_flow
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm;

        read_into_vec_result(|count, data| unsafe {
            (call)(
                self.handle,
                queue_family_index,
                queue_family_data_graph_properties,
                optical_flow_image_format_info,
                count,
                data,
            )
        })
    }
}
