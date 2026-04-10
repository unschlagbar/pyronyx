// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_sample_locations_ext;

        unsafe { (call)(self.handle, sample_locations_info) };
    }
}

pub trait SampleLocationsPhysicalDevice {
    fn get_multisample_properties(&self, samples: SampleCountFlags)
    -> MultisamplePropertiesEXT<'_>;
}

impl SampleLocationsPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceMultisamplePropertiesEXT.html>
    #[inline]
    fn get_multisample_properties(
        &self,
        samples: SampleCountFlags,
    ) -> MultisamplePropertiesEXT<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_sample_locations
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_multisample_properties_ext;

        unsafe {
            (call)(self.handle, samples, out.as_mut_ptr());
            out.assume_init()
        }
    }
}
