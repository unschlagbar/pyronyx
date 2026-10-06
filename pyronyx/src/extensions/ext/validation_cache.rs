// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_validation_cache";
pub const SPEC_VERSION: u32 = 1;

pub trait ValidationCacheDevice {
    fn create_validation_cache(
        &self,
        create_info: &ValidationCacheCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ValidationCacheEXT>;

    fn destroy_validation_cache(
        &self,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_validation_cache_data(
        &self,
        validation_cache: ValidationCacheEXT,
        data: &mut [u8],
    ) -> Result<()>;
    fn get_validation_cache_data_len(&self, validation_cache: ValidationCacheEXT) -> Result<usize>;

    fn merge_validation_caches(
        &self,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> Result<()>;
}

impl ValidationCacheDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html>
    #[inline]
    fn create_validation_cache(
        &self,
        create_info: &ValidationCacheCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ValidationCacheEXT> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_validation_cache
            .create_validation_cache_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyValidationCacheEXT.html>
    #[inline]
    fn destroy_validation_cache(
        &self,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .ext_validation_cache
            .destroy_validation_cache_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                validation_cache,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html>
    ///
    /// Call [`get_validation_cache_data_len()`][`Self::get_validation_cache_data_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_validation_cache_data(
        &self,
        validation_cache: ValidationCacheEXT,
        data: &mut [u8],
    ) -> Result<()> {
        let mut data_size = data.len();
        let call = self
            .fns()
            .ext_validation_cache
            .get_validation_cache_data_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                validation_cache,
                &mut data_size,
                data.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_validation_cache_data`][`Self::get_validation_cache_data`].
    #[inline]
    fn get_validation_cache_data_len(&self, validation_cache: ValidationCacheEXT) -> Result<usize> {
        let mut out: MaybeUninit<usize> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_validation_cache
                .get_validation_cache_data_ext
                .unwrap_or_else(|| Self::ext_load_error()))(
                self.handle,
                validation_cache,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html>
    #[inline]
    fn merge_validation_caches(
        &self,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_validation_cache
            .merge_validation_caches_ext
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe {
            (call)(
                self.handle,
                dst_cache,
                src_caches.len() as u32,
                src_caches.as_ptr(),
            )
        }
        .result()
    }
}
