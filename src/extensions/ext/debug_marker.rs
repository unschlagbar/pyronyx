// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
pub const NAME: &CStr = c"VK_EXT_debug_marker";
pub const SPEC_VERSION: u32 = 4;

pub trait DebugMarkerDevice {
    fn debug_marker_set_object_name(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> Result<(), Error>;

    fn debug_marker_set_object_tag(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> Result<(), Error>;
}

impl DebugMarkerDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html>
    #[inline]
    fn debug_marker_set_object_name(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_debug_marker
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .debug_marker_set_object_name_ext;

        unsafe { (call)(self.handle, name_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html>
    #[inline]
    fn debug_marker_set_object_tag(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_debug_marker
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .debug_marker_set_object_tag_ext;

        unsafe { (call)(self.handle, tag_info) }.result()
    }
}

pub trait DebugMarkerCommandBuffer {
    fn debug_marker_begin(&self, marker_info: &DebugMarkerMarkerInfoEXT);

    fn debug_marker_end(&self);

    fn debug_marker_insert(&self, marker_info: &DebugMarkerMarkerInfoEXT);
}

impl DebugMarkerCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`, `VideoDecodeKHR`, `VideoEncodeKHR`, `OpticalFlowNV`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn debug_marker_begin(&self, marker_info: &DebugMarkerMarkerInfoEXT) {
        let call = self
            .fns()
            .ext_debug_marker
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .debug_marker_begin_ext;

        unsafe { (call)(self.handle, marker_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`, `VideoDecodeKHR`, `VideoEncodeKHR`, `OpticalFlowNV`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn debug_marker_end(&self) {
        let call = self
            .fns()
            .ext_debug_marker
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .debug_marker_end_ext;

        unsafe { (call)(self.handle) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`, `VideoDecodeKHR`, `VideoEncodeKHR`, `OpticalFlowNV`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn debug_marker_insert(&self, marker_info: &DebugMarkerMarkerInfoEXT) {
        let call = self
            .fns()
            .ext_debug_marker
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .debug_marker_insert_ext;

        unsafe { (call)(self.handle, marker_info) };
    }
}
