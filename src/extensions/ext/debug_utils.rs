// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_debug_utils";
pub const SPEC_VERSION: u32 = 2;

pub trait DebugUtilsDevice {
    fn set_debug_utils_object_name(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> Result<(), vkResult>;

    fn set_debug_utils_object_tag(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> Result<(), vkResult>;
}

impl DebugUtilsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html>
    #[inline]
    fn set_debug_utils_object_name(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .set_debug_utils_object_name_ext)(self.handle, name_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html>
    #[inline]
    fn set_debug_utils_object_tag(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> Result<(), vkResult> {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .set_debug_utils_object_tag_ext)(self.handle, tag_info)
        }
        .result()
    }
}

pub trait DebugUtilsInstance {
    fn create_debug_utils_messenger(
        &self,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DebugUtilsMessengerEXT, vkResult>;

    fn destroy_debug_utils_messenger(
        &self,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&AllocationCallbacks>,
    );

    fn submit_debug_utils_message(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    );
}

impl DebugUtilsInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugUtilsMessengerEXT.html>
    #[inline]
    fn create_debug_utils_messenger(
        &self,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DebugUtilsMessengerEXT, vkResult> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .create_debug_utils_messenger_ext)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html>
    #[inline]
    fn destroy_debug_utils_messenger(
        &self,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .destroy_debug_utils_messenger_ext)(
                self.handle,
                messenger,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html>
    #[inline]
    fn submit_debug_utils_message(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .submit_debug_utils_message_ext)(
                self.handle,
                message_severity,
                message_types,
                callback_data,
            )
        };
    }
}

pub trait DebugUtilsCommandBuffer {
    fn begin_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT);

    fn end_debug_utils_label(&self);

    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT);
}

impl DebugUtilsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html>
    #[inline]
    fn begin_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .begin_debug_utils_label_ext)(self.handle, label_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html>
    #[inline]
    fn end_debug_utils_label(&self) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .end_debug_utils_label_ext)(self.handle)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html>
    #[inline]
    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .insert_debug_utils_label_ext)(self.handle, label_info)
        };
    }
}

pub trait DebugUtilsQueue {
    fn begin_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT);

    fn end_debug_utils_label(&self);

    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT);
}

impl DebugUtilsQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBeginDebugUtilsLabelEXT.html>
    #[inline]
    fn begin_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .queue_begin_debug_utils_label_ext)(self.handle, label_info)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html>
    #[inline]
    fn end_debug_utils_label(&self) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .queue_end_debug_utils_label_ext)(self.handle)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html>
    #[inline]
    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        unsafe {
            (self
                .fns()
                .ext_debug_utils
                .as_ref()
                .unwrap()
                .queue_insert_debug_utils_label_ext)(self.handle, label_info)
        };
    }
}
