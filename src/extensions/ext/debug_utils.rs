// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_EXT_debug_utils";
pub const SPEC_VERSION: u32 = 2;

pub trait DebugUtilsDevice {
    fn set_debug_utils_object_name(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> Result<(), Error>;

    fn set_debug_utils_object_tag(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> Result<(), Error>;
}

impl DebugUtilsDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectNameEXT.html>
    #[inline]
    fn set_debug_utils_object_name(
        &self,
        name_info: &DebugUtilsObjectNameInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_debug_utils_object_name_ext;

        unsafe { (call)(self.handle, name_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetDebugUtilsObjectTagEXT.html>
    #[inline]
    fn set_debug_utils_object_tag(
        &self,
        tag_info: &DebugUtilsObjectTagInfoEXT,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_debug_utils_object_tag_ext;

        unsafe { (call)(self.handle, tag_info) }.result()
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
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .queue_begin_debug_utils_label_ext;

        unsafe { (call)(self.handle, label_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueEndDebugUtilsLabelEXT.html>
    #[inline]
    fn end_debug_utils_label(&self) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .queue_end_debug_utils_label_ext;

        unsafe { (call)(self.handle) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueInsertDebugUtilsLabelEXT.html>
    #[inline]
    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .queue_insert_debug_utils_label_ext;

        unsafe { (call)(self.handle, label_info) };
    }
}

pub trait DebugUtilsCommandBuffer {
    fn begin_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT);

    fn end_debug_utils_label(&self);

    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT);
}

impl DebugUtilsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginDebugUtilsLabelEXT.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`, `VideoDecodeKHR`, `VideoEncodeKHR`, `OpticalFlowNV`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .begin_debug_utils_label_ext;

        unsafe { (call)(self.handle, label_info) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndDebugUtilsLabelEXT.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`, `VideoDecodeKHR`, `VideoEncodeKHR`, `OpticalFlowNV`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_debug_utils_label(&self) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .end_debug_utils_label_ext;

        unsafe { (call)(self.handle) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdInsertDebugUtilsLabelEXT.html>
    ///
    /// Queues types: `Transfer`, `Graphics`, `Compute`, `VideoDecodeKHR`, `VideoEncodeKHR`, `OpticalFlowNV`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn insert_debug_utils_label(&self, label_info: &DebugUtilsLabelEXT) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .insert_debug_utils_label_ext;

        unsafe { (call)(self.handle, label_info) };
    }
}

pub trait DebugUtilsInstance {
    fn create_debug_utils_messenger(
        &self,
        create_info: &DebugUtilsMessengerCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DebugUtilsMessengerEXT, Error>;

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
    ) -> Result<DebugUtilsMessengerEXT, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_debug_utils_messenger_ext;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugUtilsMessengerEXT.html>
    #[inline]
    fn destroy_debug_utils_messenger(
        &self,
        messenger: DebugUtilsMessengerEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_debug_utils_messenger_ext;

        unsafe { (call)(self.handle, messenger, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSubmitDebugUtilsMessageEXT.html>
    #[inline]
    fn submit_debug_utils_message(
        &self,
        message_severity: DebugUtilsMessageSeverityFlagsEXT,
        message_types: DebugUtilsMessageTypeFlagsEXT,
        callback_data: &DebugUtilsMessengerCallbackDataEXT,
    ) {
        let call = self
            .fns()
            .ext_debug_utils
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .submit_debug_utils_message_ext;

        unsafe { (call)(self.handle, message_severity, message_types, callback_data) };
    }
}
