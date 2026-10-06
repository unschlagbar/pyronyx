// Wrappers mirror the Vulkan API: they pass caller pointers straight to the driver,
// take as many arguments as the C function, and `Default` is always generated as an impl.
#![allow(
    clippy::not_unsafe_ptr_arg_deref,
    clippy::too_many_arguments,
    clippy::derivable_impls
)]

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
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>
    pub unsafe fn vkGetInstanceProcAddr(
        instance: vk::vkInstance,
        name: *const core::ffi::c_char,
    ) -> vk::PFN_vkVoidFunction;
}
