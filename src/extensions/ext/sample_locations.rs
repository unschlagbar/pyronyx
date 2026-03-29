// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_EXT_sample_locations";
pub const SPEC_VERSION: u32 = 1;

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
        unsafe {
            (self
                .fns()
                .ext_sample_locations
                .as_ref()
                .unwrap()
                .get_physical_device_multisample_properties_ext)(
                self.handle,
                samples,
                out.as_mut_ptr(),
            )
        };
        unsafe { out.assume_init() }
    }
}

pub trait SampleLocationsCommandBuffer {
    fn set_sample_locations(&self, sample_locations_info: &SampleLocationsInfoEXT);
}

impl SampleLocationsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetSampleLocationsEXT.html>
    #[inline]
    fn set_sample_locations(&self, sample_locations_info: &SampleLocationsInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_sample_locations
                .as_ref()
                .unwrap()
                .set_sample_locations_ext)(self.handle, sample_locations_info)
        };
    }
}
