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
/// Requires: [`VK_KHR_get_physical_device_properties2`](crate::khr::get_physical_device_properties2) or Vulkan 1.1
pub const NAME: &CStr = c"VK_KHR_cooperative_matrix";
pub const SPEC_VERSION: u32 = 2;

pub trait CooperativeMatrixPhysicalDevice {
    fn get_cooperative_matrix_properties(
        &self,
        properties: &mut [CooperativeMatrixPropertiesKHR],
    ) -> Result<()>;
    fn get_cooperative_matrix_properties_len(&self) -> Result<usize>;
}

impl CooperativeMatrixPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR.html>
    ///
    /// Call [`get_cooperative_matrix_properties_len()`][`Self::get_cooperative_matrix_properties_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_cooperative_matrix_properties(
        &self,
        properties: &mut [CooperativeMatrixPropertiesKHR],
    ) -> Result<()> {
        let mut property_count = properties.len() as u32;
        let call = self
            .fns()
            .khr_cooperative_matrix
            .get_physical_device_cooperative_matrix_properties_khr
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, &mut property_count, properties.as_mut_ptr()) }.result()
    }

    /// Returns the required slice length for Call [`get_cooperative_matrix_properties`][`Self::get_cooperative_matrix_properties`].
    #[inline]
    fn get_cooperative_matrix_properties_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .khr_cooperative_matrix
                .get_physical_device_cooperative_matrix_properties_khr
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
