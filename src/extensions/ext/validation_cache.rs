// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_validation_cache";
pub const SPEC_VERSION: u32 = 1;

pub trait ValidationCacheDevice {
    fn create_validation_cache(
        &self,
        create_info: &ValidationCacheCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ValidationCacheEXT, Error>;

    fn destroy_validation_cache(
        &self,
        validation_cache: ValidationCacheEXT,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_validation_cache_data(
        &self,
        validation_cache: ValidationCacheEXT,
        data: &mut [c_void],
    ) -> Result<(), Error>;

    fn merge_validation_caches(
        &self,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> Result<(), Error>;
}

impl ValidationCacheDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateValidationCacheEXT.html>
    #[inline]
    fn create_validation_cache(
        &self,
        create_info: &ValidationCacheCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ValidationCacheEXT, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_validation_cache
                .as_ref()
                .unwrap()
                .create_validation_cache_ext)(
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
        unsafe {
            (self
                .fns()
                .ext_validation_cache
                .as_ref()
                .unwrap()
                .destroy_validation_cache_ext)(
                self.handle,
                validation_cache,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetValidationCacheDataEXT.html>
    #[inline]
    fn get_validation_cache_data(
        &self,
        validation_cache: ValidationCacheEXT,
        data: &mut [c_void],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .ext_validation_cache
                .as_ref()
                .unwrap()
                .get_validation_cache_data_ext)(
                self.handle,
                validation_cache,
                data.len() as *mut usize,
                data.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergeValidationCachesEXT.html>
    #[inline]
    fn merge_validation_caches(
        &self,
        dst_cache: ValidationCacheEXT,
        src_caches: &[ValidationCacheEXT],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .ext_validation_cache
                .as_ref()
                .unwrap()
                .merge_validation_caches_ext)(
                self.handle,
                dst_cache,
                src_caches.len() as u32,
                src_caches.as_ptr(),
            )
        }
        .result()
    }
}
