// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_FUCHSIA_buffer_collection";
pub const SPEC_VERSION: u32 = 2;

pub trait BufferCollectionDevice {
    fn create_buffer_collection(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<BufferCollectionFUCHSIA, Error>;

    fn set_buffer_collection_buffer_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        buffer_constraints_info: &BufferConstraintsInfoFUCHSIA,
    ) -> Result<(), Error>;

    fn set_buffer_collection_image_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> Result<(), Error>;

    fn destroy_buffer_collection(
        &self,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_buffer_collection_properties(
        &self,
        collection: BufferCollectionFUCHSIA,
    ) -> Result<BufferCollectionPropertiesFUCHSIA<'_>, Error>;
}

impl BufferCollectionDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferCollectionFUCHSIA.html>
    #[inline]
    fn create_buffer_collection(
        &self,
        create_info: &BufferCollectionCreateInfoFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<BufferCollectionFUCHSIA, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .fuchsia_buffer_collection
                .as_ref()
                .unwrap()
                .create_buffer_collection_fuchsia)(
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
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .fuchsia_buffer_collection
                .as_ref()
                .unwrap()
                .set_buffer_collection_buffer_constraints_fuchsia)(
                self.handle,
                collection,
                buffer_constraints_info,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetBufferCollectionImageConstraintsFUCHSIA.html>
    #[inline]
    fn set_buffer_collection_image_constraints(
        &self,
        collection: BufferCollectionFUCHSIA,
        image_constraints_info: &ImageConstraintsInfoFUCHSIA,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .fuchsia_buffer_collection
                .as_ref()
                .unwrap()
                .set_buffer_collection_image_constraints_fuchsia)(
                self.handle,
                collection,
                image_constraints_info,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferCollectionFUCHSIA.html>
    #[inline]
    fn destroy_buffer_collection(
        &self,
        collection: BufferCollectionFUCHSIA,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .fuchsia_buffer_collection
                .as_ref()
                .unwrap()
                .destroy_buffer_collection_fuchsia)(
                self.handle,
                collection,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferCollectionPropertiesFUCHSIA.html>
    #[inline]
    fn get_buffer_collection_properties(
        &self,
        collection: BufferCollectionFUCHSIA,
    ) -> Result<BufferCollectionPropertiesFUCHSIA<'_>, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .fuchsia_buffer_collection
                .as_ref()
                .unwrap()
                .get_buffer_collection_properties_fuchsia)(
                self.handle,
                collection,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}
