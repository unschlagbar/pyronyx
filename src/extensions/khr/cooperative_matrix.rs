// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_KHR_cooperative_matrix";
pub const SPEC_VERSION: u32 = 2;

pub trait CooperativeMatrixPhysicalDevice {
    fn get_cooperative_matrix_properties(
        &self,
        properties: &mut [CooperativeMatrixPropertiesKHR],
    ) -> Result<(), vkResult>;
}

impl CooperativeMatrixPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    #[inline]
    fn get_cooperative_matrix_properties(
        &self,
        properties: &mut [CooperativeMatrixPropertiesKHR],
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .khr_cooperative_matrix
                .as_ref()
                .unwrap()
                .get_physical_device_cooperative_matrix_properties_khr)(
                self.handle,
                properties.len() as *mut u32,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }
}
