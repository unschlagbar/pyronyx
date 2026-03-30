// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

pub const NAME: &CStr = c"VK_EXT_debug_marker";
pub const SPEC_VERSION: u32 = 4;

pub trait DebugMarkerDevice {
    fn debug_marker_set_object_name(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> Result<(), vkResult>;

    fn debug_marker_set_object_tag(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> Result<(), vkResult>;
}

impl DebugMarkerDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectNameEXT.html>
    #[inline]
    fn debug_marker_set_object_name(
        &self,
        name_info: &DebugMarkerObjectNameInfoEXT,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .ext_debug_marker
                .as_ref()
                .unwrap()
                .debug_marker_set_object_name_ext)(self.handle, name_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugMarkerSetObjectTagEXT.html>
    #[inline]
    fn debug_marker_set_object_tag(
        &self,
        tag_info: &DebugMarkerObjectTagInfoEXT,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .ext_debug_marker
                .as_ref()
                .unwrap()
                .debug_marker_set_object_tag_ext)(self.handle, tag_info)
        }
        .result()
    }
}

pub trait DebugMarkerCommandBuffer {
    fn debug_marker_begin(&self, marker_info: &DebugMarkerMarkerInfoEXT);

    fn debug_marker_end(&self);

    fn debug_marker_insert(&self, marker_info: &DebugMarkerMarkerInfoEXT);
}

impl DebugMarkerCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerBeginEXT.html>
    #[inline]
    fn debug_marker_begin(&self, marker_info: &DebugMarkerMarkerInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_debug_marker
                .as_ref()
                .unwrap()
                .debug_marker_begin_ext)(self.handle, marker_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerEndEXT.html>
    #[inline]
    fn debug_marker_end(&self) {
        unsafe {
            (self
                .fns()
                .ext_debug_marker
                .as_ref()
                .unwrap()
                .debug_marker_end_ext)(self.handle)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDebugMarkerInsertEXT.html>
    #[inline]
    fn debug_marker_insert(&self, marker_info: &DebugMarkerMarkerInfoEXT) {
        unsafe {
            (self
                .fns()
                .ext_debug_marker
                .as_ref()
                .unwrap()
                .debug_marker_insert_ext)(self.handle, marker_info)
        };
    }
}
