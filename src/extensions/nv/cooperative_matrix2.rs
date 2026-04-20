// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_cooperative_matrix2";
pub const SPEC_VERSION: u32 = 1;

pub trait CooperativeMatrix2PhysicalDevice {
    fn get_cooperative_matrix_flexible_dimensions_properties(
        &self,
        properties: &mut [CooperativeMatrixFlexibleDimensionsPropertiesNV],
    ) -> Result<()>;
}

impl CooperativeMatrix2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    #[inline]
    fn get_cooperative_matrix_flexible_dimensions_properties(
        &self,
        properties: &mut [CooperativeMatrixFlexibleDimensionsPropertiesNV],
    ) -> Result<()> {
        let call = self
            .fns()
            .nv_cooperative_matrix2
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv;

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
