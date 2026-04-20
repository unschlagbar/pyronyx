// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![deprecated = "This extension is deprecated. Use `VK_EXT_debug_utils` instead."]
use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_char;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Instance`
pub const NAME: &CStr = c"VK_EXT_debug_report";
pub const SPEC_VERSION: u32 = 10;

pub trait DebugReportInstance {
    fn create_debug_report_callback(
        &self,
        create_info: &DebugReportCallbackCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DebugReportCallbackEXT>;

    fn destroy_debug_report_callback(
        &self,
        callback: DebugReportCallbackEXT,
        allocator: Option<&AllocationCallbacks>,
    );

    fn debug_report_message(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: &c_char,
        message: &c_char,
    );
}

impl DebugReportInstance for Instance {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDebugReportCallbackEXT.html>
    #[inline]
    fn create_debug_report_callback(
        &self,
        create_info: &DebugReportCallbackCreateInfoEXT,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DebugReportCallbackEXT> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .ext_debug_report
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_debug_report_callback_ext;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDebugReportCallbackEXT.html>
    #[inline]
    fn destroy_debug_report_callback(
        &self,
        callback: DebugReportCallbackEXT,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .ext_debug_report
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_debug_report_callback_ext;

        unsafe { (call)(self.handle, callback, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDebugReportMessageEXT.html>
    #[inline]
    fn debug_report_message(
        &self,
        flags: DebugReportFlagsEXT,
        object_type: DebugReportObjectTypeEXT,
        object: u64,
        location: usize,
        message_code: i32,
        layer_prefix: &c_char,
        message: &c_char,
    ) {
        let call = self
            .fns()
            .ext_debug_report
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .debug_report_message_ext;

        unsafe {
            (call)(
                self.handle,
                flags,
                object_type,
                object,
                location,
                message_code,
                layer_prefix,
                message,
            )
        };
    }
}
