mod bitflags;
mod commands;
mod constants;
mod display_bitflags;
mod enums;
mod error;
mod handles;
mod platform_types;
mod types;

use core::ffi::CStr;
use core::mem::{MaybeUninit, transmute};
use core::ptr;

pub use bitflags::*;
pub(crate) use commands::*;
pub use constants::*;
pub use enums::*;
pub use error::Error;
pub use handles::*;
pub use platform_types::*;
pub use types::*;

use crate::utils::{read_into_vec_result, to_option};

pub use crate::vkGetInstanceProcAddr as get_instance_proc_addr;

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceLayerProperties.html>
pub fn enumerate_instance_layer_properties() -> Result<Vec<LayerProperties>, Error> {
    let pfn: vkEnumerateInstanceLayerProperties = to_option(unsafe {
        transmute(get_instance_proc_addr(
            vkInstance::null(),
            c"vkEnumerateInstanceLayerProperties".as_ptr(),
        ))
    })
    .unwrap();
    read_into_vec_result(|count, data| unsafe { (pfn)(count, data) })
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceExtensionProperties.html>
pub fn enumerate_instance_extension_properties(
    layer_name: Option<&CStr>,
) -> Result<Vec<ExtensionProperties>, Error> {
    let pfn: vkEnumerateInstanceExtensionProperties = to_option(unsafe {
        transmute(get_instance_proc_addr(
            vkInstance::null(),
            c"vkEnumerateInstanceExtensionProperties".as_ptr(),
        ))
    })
    .unwrap();
    read_into_vec_result(|count, data| unsafe {
        (pfn)(
            layer_name.map_or(ptr::null(), |str| str.as_ptr()),
            count,
            data,
        )
    })
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumerateInstanceVersion.html>
///
/// Since it's a Vulkan 1.1 function,
/// it just returns [`API_VERSION_1_0`] if the fn is not available!
pub fn enumerate_instance_version() -> u32 {
    let pfn: Option<vkEnumerateInstanceVersion> = to_option(unsafe {
        transmute(get_instance_proc_addr(
            vkInstance::null(),
            c"vkEnumerateInstanceVersion".as_ptr(),
        ))
    });

    if let Some(pfn) = pfn {
        let mut version = API_VERSION_1_0;
        let _ = unsafe { (pfn)(&mut version) };
        version
    } else {
        API_VERSION_1_0
    }
}

impl vkResult {
    #[inline]
    pub fn init_on_success<T>(&self, v: MaybeUninit<T>) -> Result<T, Error> {
        match self {
            Self::Success => Ok(unsafe { v.assume_init() }),
            _ => Err((*self).into()),
        }
    }

    #[inline]
    pub fn result(&self) -> Result<(), Error> {
        match self {
            Self::Success => Ok(()),
            _ => Err((*self).into()),
        }
    }

    #[inline]
    pub unsafe fn set_len_on_success<T>(self, mut v: Vec<T>, len: usize) -> Result<Vec<T>, Error> {
        match self {
            Self::Success => {
                unsafe { v.set_len(len) };
                Ok(v)
            }
            _ => Err(self.into()),
        }
    }
}

impl vkPhysicalDevice {
    /// # Safety
    /// The returned [`PhysicalDevice`] borrows their function table from this
    /// `Instance`. Dropping the `Instance` while any [`PhysicalDevice`] is still
    /// in use is undefined behaviour.
    ///
    /// Calling **any** fn on [`PhysicalDevice`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    #[inline]
    pub unsafe fn to_physical_device(self, instance: &Instance) -> PhysicalDevice {
        let v_table = &instance.v_table().physical_device;
        let v_table = unsafe { transmute::<&_, &'static _>(v_table) };

        PhysicalDevice {
            handle: self,
            v_table,
        }
    }
}

impl vkCommandBuffer {
    /// # Safety
    /// The returned [`CommandBuffer`] borrows their function table from this
    /// `Device`. Dropping the `Device` while any [`CommandBuffer`] is still
    /// in use is undefined behaviour.
    ///
    /// Calling **any** fn on [`Queue`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    #[inline]
    pub unsafe fn to_command_buffer(self, device: &Device) -> CommandBuffer {
        let v_table = &device.v_table().command_buffer;
        let v_table = unsafe { transmute::<&_, &'static _>(v_table) };

        CommandBuffer {
            handle: self,
            v_table: Some(v_table),
        }
    }
}

impl vkQueue {
    /// # Safety
    /// The returned [`Queue`] borrows their function table from this
    /// `Device`. Dropping the `Device` while any [`Queue`] is still
    /// in use is undefined behaviour.
    ///
    /// Calling **any** fn on [`Queue`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    #[inline]
    pub unsafe fn to_queue(self, device: &Device) -> Queue {
        let v_table = &device.v_table().queue;
        let v_table = unsafe { transmute::<&_, &'static _>(v_table) };

        Queue {
            handle: self,
            v_table,
        }
    }
}

impl PhysicalDeviceMemoryProperties {
    #[inline]
    pub fn memory_types_as_slice(&self) -> &[MemoryType] {
        &self.memory_types[..self.memory_type_count as _]
    }
    #[inline]
    pub fn memory_heaps_as_slice(&self) -> &[MemoryHeap] {
        &self.memory_heaps[..self.memory_heap_count as _]
    }
}

impl ColorComponentFlags {
    /// Contraction of [`R`][Self::R] | [`G`][Self::G] | [`B`][Self::B] | [`A`][Self::A]
    pub const RGBA: Self = Self(Self::R.0 | Self::G.0 | Self::B.0 | Self::A.0);
}
