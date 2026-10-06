// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_EXT_sample_locations";
pub const SPEC_VERSION: u32 = 1;

pub trait SampleLocationsCommandBuffer {
    fn set_sample_locations(&self, sample_locations_info: &SampleLocationsInfoEXT);
}

impl SampleLocationsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_sample_locations(&self, sample_locations_info: &SampleLocationsInfoEXT) {
        let call = self
            .fns()
            .ext_sample_locations
            .set_sample_locations_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, sample_locations_info) };
    }
}

pub trait SampleLocationsPhysicalDevice {
    fn get_multisample_properties(
        &self,
        samples: SampleCountFlags,
        multisample_properties: &mut MultisamplePropertiesEXT<'_>,
    );
}

impl SampleLocationsPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[inline]
    fn get_multisample_properties(
        &self,
        samples: SampleCountFlags,
        multisample_properties: &mut MultisamplePropertiesEXT<'_>,
    ) {
        let call = self
            .fns()
            .ext_sample_locations
            .get_physical_device_multisample_properties_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, samples, multisample_properties) };
    }
}
