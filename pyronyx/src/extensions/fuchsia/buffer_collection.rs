// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
///
/// Requires: [`VK_FUCHSIA_external_memory`](crate::fuchsia::external_memory) + ([`VK_KHR_sampler_ycbcr_conversion`](crate::khr::sampler_ycbcr_conversion) or Vulkan 1.1)
pub const NAME: &CStr = c"VK_FUCHSIA_buffer_collection";
pub const SPEC_VERSION: u32 = 2;

pub trait BufferCollectionDevice {
    fn create_buffer_collection(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<BufferCollectionFUCHSIA>;

    fn set_buffer_collection_buffer_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> Result<()>;

    fn set_buffer_collection_image_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> Result<()>;

    fn destroy_buffer_collection(
        &self,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_buffer_collection_properties(
        &self,
        collection: BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> Result<()>;
}

impl BufferCollectionDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html>
    #[inline]
    fn create_buffer_collection(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<BufferCollectionFUCHSIA> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .fuchsia_buffer_collection
            .create_buffer_collection_fuchsia
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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionBufferConstraintsFUCHSIA.html>
    #[inline]
    fn set_buffer_collection_buffer_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> Result<()> {
        let call = self
            .fns()
            .fuchsia_buffer_collection
            .set_buffer_collection_buffer_constraints_fuchsia
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, collection, buffer_constraints_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html>
    #[inline]
    fn set_buffer_collection_image_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> Result<()> {
        let call = self
            .fns()
            .fuchsia_buffer_collection
            .set_buffer_collection_image_constraints_fuchsia
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, collection, image_constraints_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html>
    #[inline]
    fn destroy_buffer_collection(
        &self,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .fuchsia_buffer_collection
            .destroy_buffer_collection_fuchsia
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, collection, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html>
    #[inline]
    fn get_buffer_collection_properties(
        &self,
        collection: BufferCollectionFUCHSIA,
        properties: &mut BufferCollectionPropertiesFUCHSIA<'_>,
    ) -> Result<()> {
        let call = self
            .fns()
            .fuchsia_buffer_collection
            .get_buffer_collection_properties_fuchsia
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, collection, properties) }.result()
    }
}
