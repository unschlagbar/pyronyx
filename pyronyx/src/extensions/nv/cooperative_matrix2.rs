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
/// Requires: [`VK_KHR_cooperative_matrix`](crate::khr::cooperative_matrix)
pub const NAME: &CStr = c"VK_NV_cooperative_matrix2";
pub const SPEC_VERSION: u32 = 1;

pub trait CooperativeMatrix2PhysicalDevice {
    fn get_cooperative_matrix_flexible_dimensions_properties(
        &self,
        properties: &mut [CooperativeMatrixFlexibleDimensionsPropertiesNV],
    ) -> Result<()>;
    fn get_cooperative_matrix_flexible_dimensions_properties_len(&self) -> Result<usize>;
}

impl CooperativeMatrix2PhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV.html>
    ///
    /// Call [`get_cooperative_matrix_flexible_dimensions_properties_len()`][`Self::get_cooperative_matrix_flexible_dimensions_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_cooperative_matrix_flexible_dimensions_properties(
        &self,
        properties: &mut [CooperativeMatrixFlexibleDimensionsPropertiesNV],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .nv_cooperative_matrix2
            .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_cooperative_matrix_flexible_dimensions_properties`][`Self::get_cooperative_matrix_flexible_dimensions_properties`].
    #[inline]
    fn get_cooperative_matrix_flexible_dimensions_properties_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_cooperative_matrix2
                .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}
