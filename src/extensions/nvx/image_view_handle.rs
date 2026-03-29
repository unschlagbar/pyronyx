// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

pub const NAME: &CStr = c"VK_NVX_image_view_handle";
pub const SPEC_VERSION: u32 = 4;

pub trait ImageViewHandleDevice {
    fn get_image_view_handle(&self, info: &ImageViewHandleInfoNVX) -> u32;

    fn get_image_view_handle64(&self, info: &ImageViewHandleInfoNVX) -> u64;

    fn get_image_view_address(
        &self,
        image_view: ImageView,
    ) -> Result<ImageViewAddressPropertiesNVX<'_>, vkResult>;

    fn get_device_combined_image_sampler_index(
        &self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64;
}

impl ImageViewHandleDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandleNVX.html>
    #[inline]
    fn get_image_view_handle(&self, info: &ImageViewHandleInfoNVX) -> u32 {
        unsafe {
            (self
                .fns()
                .nvx_image_view_handle
                .as_ref()
                .unwrap()
                .get_image_view_handle_nvx)(self.handle, info)
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewHandle64NVX.html>
    #[inline]
    fn get_image_view_handle64(&self, info: &ImageViewHandleInfoNVX) -> u64 {
        unsafe {
            (self
                .fns()
                .nvx_image_view_handle
                .as_ref()
                .unwrap()
                .get_image_view_handle64_nvx)(self.handle, info)
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageViewAddressNVX.html>
    #[inline]
    fn get_image_view_address(
        &self,
        image_view: ImageView,
    ) -> Result<ImageViewAddressPropertiesNVX<'_>, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nvx_image_view_handle
                .as_ref()
                .unwrap()
                .get_image_view_address_nvx)(self.handle, image_view, out.as_mut_ptr())
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceCombinedImageSamplerIndexNVX.html>
    #[inline]
    fn get_device_combined_image_sampler_index(
        &self,
        image_view_index: u64,
        sampler_index: u64,
    ) -> u64 {
        unsafe {
            (self
                .fns()
                .nvx_image_view_handle
                .as_ref()
                .unwrap()
                .get_device_combined_image_sampler_index_nvx)(
                self.handle,
                image_view_index,
                sampler_index,
            )
        }
    }
}
