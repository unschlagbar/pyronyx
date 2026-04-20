// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_KHR_cooperative_matrix";
pub const SPEC_VERSION: u32 = 2;

pub trait CooperativeMatrixPhysicalDevice {
    fn get_cooperative_matrix_properties(
        &self,
        properties: &mut [CooperativeMatrixPropertiesKHR],
    ) -> Result<()>;
}

impl CooperativeMatrixPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[inline]
    fn get_cooperative_matrix_properties(
        &self,
        properties: &mut [CooperativeMatrixPropertiesKHR],
    ) -> Result<()> {
        let call = self
            .fns()
            .khr_cooperative_matrix
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_cooperative_matrix_properties_khr;

        unsafe {
            (call)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }
}
