// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_KHR_cooperative_matrix`](crate::khr::cooperative_matrix)
pub const NAME: &CStr = c"VK_EXT_cooperative_matrix_maintenance1";
pub const SPEC_VERSION: u32 = 1;

pub trait CooperativeMatrixMaintenance1PhysicalDevice {
    fn get_cooperative_matrix_properties2(
        &self,
        cooperative_matrix_info: &PhysicalDeviceCooperativeMatrixInfo2EXT,
        properties: &mut [CooperativeMatrixProperties2EXT],
    ) -> Result<()>;
}

impl CooperativeMatrixMaintenance1PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixProperties2EXT.html>
    #[inline]
    fn get_cooperative_matrix_properties2(
        &self,
        cooperative_matrix_info: &PhysicalDeviceCooperativeMatrixInfo2EXT,
        properties: &mut [CooperativeMatrixProperties2EXT],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .ext_cooperative_matrix_maintenance1
            .get_physical_device_cooperative_matrix_properties2_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                cooperative_matrix_info,
                &mut property_count,
                properties.as_mut_ptr(),
            )
        }
        .result()
    }
}
