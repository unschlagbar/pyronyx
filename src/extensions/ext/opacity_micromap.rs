// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_opacity_micromap";
pub const SPEC_VERSION: u32 = 2;

pub trait OpacityMicromapDevice {
    fn create_micromap(
        &self,
        create_info: &MicromapCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<MicromapEXT, Error>;

    fn build_micromaps(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[MicromapBuildInfoEXT],
    ) -> Result<(), Error>;

    fn destroy_micromap(&self, micromap: MicromapEXT, allocator: Option<&AllocationCallbacks>);

    fn copy_micromap(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapInfoEXT,
    ) -> Result<(), Error>;

    fn copy_micromap_to_memory(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> Result<(), Error>;

    fn copy_memory_to_micromap(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> Result<(), Error>;

    fn write_micromaps_properties(
        &self,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        data: &mut [c_void],
        stride: usize,
    ) -> Result<(), Error>;

    fn get_micromap_compatibility(
        &self,
        version_info: &MicromapVersionInfoEXT,
    ) -> AccelerationStructureCompatibilityKHR;

    fn get_micromap_build_sizes(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT,
    ) -> MicromapBuildSizesInfoEXT<'_>;
}

impl OpacityMicromapDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateMicromapEXT.html>
    #[inline]
    fn create_micromap(
        &self,
        create_info: &MicromapCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<MicromapEXT, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_micromap_ext;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBuildMicromapsEXT.html>
    #[inline]
    fn build_micromaps(
        &self,
        deferred_operation: DeferredOperationKHR,
        infos: &[MicromapBuildInfoEXT],
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_micromaps_ext;

        unsafe {
            (call)(
                self.handle,
                deferred_operation,
                infos.len() as u32,
                infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyMicromapEXT.html>
    #[inline]
    fn destroy_micromap(&self, micromap: MicromapEXT, allocator: Option<&AllocationCallbacks>) {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_micromap_ext;

        unsafe { (call)(self.handle, micromap, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapEXT.html>
    #[inline]
    fn copy_micromap(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_micromap_ext;

        unsafe { (call)(self.handle, deferred_operation, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMicromapToMemoryEXT.html>
    #[inline]
    fn copy_micromap_to_memory(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMicromapToMemoryInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_micromap_to_memory_ext;

        unsafe { (call)(self.handle, deferred_operation, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToMicromapEXT.html>
    #[inline]
    fn copy_memory_to_micromap(
        &self,
        deferred_operation: DeferredOperationKHR,
        info: &CopyMemoryToMicromapInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_memory_to_micromap_ext;

        unsafe { (call)(self.handle, deferred_operation, info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWriteMicromapsPropertiesEXT.html>
    #[inline]
    fn write_micromaps_properties(
        &self,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        data: &mut [c_void],
        stride: usize,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .write_micromaps_properties_ext;

        unsafe {
            (call)(
                self.handle,
                micromaps.len() as u32,
                micromaps.as_ptr(),
                query_type,
                data.len() as usize,
                data.as_mut_ptr(),
                stride,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMicromapCompatibilityEXT.html>
    #[inline]
    fn get_micromap_compatibility(
        &self,
        version_info: &MicromapVersionInfoEXT,
    ) -> AccelerationStructureCompatibilityKHR {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_device_micromap_compatibility_ext;

        unsafe {
            (call)(self.handle, version_info, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetMicromapBuildSizesEXT.html>
    #[inline]
    fn get_micromap_build_sizes(
        &self,
        build_type: AccelerationStructureBuildTypeKHR,
        build_info: &MicromapBuildInfoEXT,
    ) -> MicromapBuildSizesInfoEXT<'_> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_micromap_build_sizes_ext;

        unsafe {
            (call)(self.handle, build_type, build_info, out.as_mut_ptr());
            out.assume_init()
        }
    }
}

pub trait OpacityMicromapCommandBuffer {
    fn build_micromaps(&self, infos: &[MicromapBuildInfoEXT]);

    fn copy_micromap(&self, info: &CopyMicromapInfoEXT);

    fn copy_micromap_to_memory(&self, info: &CopyMicromapToMemoryInfoEXT);

    fn copy_memory_to_micromap(&self, info: &CopyMemoryToMicromapInfoEXT);

    fn write_micromaps_properties(
        &self,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    );
}

impl OpacityMicromapCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBuildMicromapsEXT.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn build_micromaps(&self, infos: &[MicromapBuildInfoEXT]) {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .build_micromaps_ext;

        unsafe { (call)(self.handle, infos.len() as u32, infos.as_ptr()) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapEXT.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_micromap(&self, info: &CopyMicromapInfoEXT) {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_micromap_ext;

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMicromapToMemoryEXT.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_micromap_to_memory(&self, info: &CopyMicromapToMemoryInfoEXT) {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_micromap_to_memory_ext;

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyMemoryToMicromapEXT.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_memory_to_micromap(&self, info: &CopyMemoryToMicromapInfoEXT) {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_memory_to_micromap_ext;

        unsafe { (call)(self.handle, info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdWriteMicromapsPropertiesEXT.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn write_micromaps_properties(
        &self,
        micromaps: &[MicromapEXT],
        query_type: QueryType,
        query_pool: QueryPool,
        first_query: u32,
    ) {
        let call = self
            .fns()
            .ext_opacity_micromap
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .write_micromaps_properties_ext;

        unsafe {
            (call)(
                self.handle,
                micromaps.len() as u32,
                micromaps.as_ptr(),
                query_type,
                query_pool,
                first_query,
            )
        };
    }
}
