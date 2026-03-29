mod extensions;
pub mod impl_command_buffer;
pub mod impl_device;
pub mod impl_instance;
pub mod impl_physical_device;
pub mod impl_queue;
pub mod macros;
#[cfg(feature = "rwh_06")]
pub mod raw_window_handle;
pub mod utils;
pub mod video;
pub mod vk;
pub mod vtables;

pub use extensions::*;

unsafe extern "system" {
    pub unsafe fn vkGetInstanceProcAddr(
        instance: vk::vkInstance,
        name: *const core::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;
}
