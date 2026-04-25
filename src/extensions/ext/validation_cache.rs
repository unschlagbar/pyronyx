// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_validation_cache_ext;

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
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_validation_cache_ext;

        unsafe {
            (call)(
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
        data: &mut [u8],
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_validation_cache
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_validation_cache_data_ext;

        unsafe {
            (call)(
                self.handle,
                validation_cache,
                data.len() as *mut usize,
                data.as_mut_ptr().cast(),
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
    ) -> Result<()> {
        let call = self
            .fns()
            .ext_validation_cache
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .merge_validation_caches_ext;

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
