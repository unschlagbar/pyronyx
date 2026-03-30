//! Thin, zero-cost Vulkan wrapper for Rust.
//!
//! This module provides the core handle types (`Instance`, `PhysicalDevice`,
//! `Device`, `Queue`, `CommandBuffer`) together with their v-tables.
//! All function pointers are loaded **once** at creation time via the official
//! Vulkan loader (`vkGetInstanceProcAddr` / `vkGetDeviceProcAddr`).
//!
//! # Design & Why Wrappers Exist
//!
//! - **Zero overhead**: Every method is `#[inline]` and compiles down to a
//!   direct function-pointer call.
//! - **Dynamic loading**: Vulkan functions are not linked statically; they are
//!   resolved at runtime so the crate works with any Vulkan loader / ICD.
//! - **Idiomatic Rust API**: The generated `impl` blocks (see `codegen` module)
//!   turn raw C signatures into nice Rust signatures (`&mut [T]`, `Vec<T>`,
//!   `Result<_, vkResult>`, etc.).
//! - **Safety**: Most Vulkan calls are inherently unsafe. This wrapper makes
//!   the *safe* parts as ergonomic as possible while clearly marking the
//!   unsafe parts.
//!
//! # Lifetime & Safety Rules (Important!)
//!
//! - `Instance`, `Device` own their v-table (`Box<…VTable>`).
//! - `PhysicalDevice`, `Queue`, `CommandBuffer` hold a **static** reference
//!   to a v-table that lives inside the parent `Instance`/`Device`.
//! - **Dropping the parent invalidates all children**:
//!   ```text
//!   If you drop an `Instance` while you still hold `PhysicalDevice`s,
//!   or drop a `Device` while you still hold `Queue`s / `CommandBuffer`s,
//!   any further use of the children is **undefined behaviour**.
//!   ```
//!   The v-table pointer becomes dangling. This is the same rule as in the
//!   official C API – the wrapper does **not** add reference counting.

use crate::{
    utils::{raw_option, read_into_vec_result},
    vk::{
        self, AllocationCallbacks, CommandBufferAllocateInfo, DeviceCreateInfo, InstanceCreateInfo,
        get_instance_proc_addr, vkCommandBuffer, vkCreateInstance, vkDevice, vkGetDeviceProcAddr,
        vkInstance, vkPhysicalDevice, vkQueue, vkResult,
    },
    vtables::{
        CommandBufferFn, DeviceFn, DeviceVTable, InstanceFn, InstanceVTable, PhysicalDeviceFn,
        QueueFn, to_option,
    },
};
use core::{
    ffi::CStr,
    fmt,
    mem::{MaybeUninit, transmute},
    slice,
};

/// An owned Vulkan instance.
///
/// Created with [`Instance::create`]. Owns the instance handle **and** the
/// v-table that contains all function pointers for this instance.
#[derive(Clone)]
pub struct Instance {
    pub(crate) handle: vkInstance,
    pub(crate) v_table: Box<InstanceVTable>,
    pub(crate) api_version: u32,
}

impl Instance {
    /// Returns the raw `VkInstance` handle.
    pub const fn handle(&self) -> vkInstance {
        self.handle
    }

    /// Returns a reference to the internal v-table.
    pub const fn v_table(&self) -> &InstanceVTable {
        &self.v_table
    }

    /// Returns the vulkan version this fn was created for
    pub const fn api_version(&self) -> u32 {
        self.api_version
    }

    /// Returns a reference to the function table for this instance.
    ///
    /// All generated methods on `Instance` ultimately call through this.
    pub const fn fns(&self) -> &InstanceFn {
        &self.v_table.instance
    }

    /// Creates a new Vulkan instance.
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>
    pub fn create(
        create_info: &InstanceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Instance, vkResult> {
        let pfn: vkCreateInstance = to_option(unsafe {
            transmute(get_instance_proc_addr(
                vkInstance::null(),
                c"vkCreateInstance".as_ptr(),
            ))
        })
        .unwrap();

        let mut instance = vk::vkInstance::null();
        let result = unsafe { (pfn)(create_info, raw_option(allocator), &mut instance) };

        if !matches!(result, vkResult::Success) {
            return Err(result);
        }

        let loader =
            |name: &CStr| unsafe { transmute(get_instance_proc_addr(instance, name.as_ptr())) };

        let extensions: &[*const i8] = if create_info.pp_enabled_extension_names.is_null()
            || create_info.enabled_extension_count == 0
        {
            &[]
        } else {
            unsafe {
                slice::from_raw_parts(
                    create_info.pp_enabled_extension_names.cast(),
                    create_info.enabled_extension_count as usize,
                )
            }
        };

        let api_version = (unsafe { *create_info.application_info }).api_version;

        let out = Self {
            handle: instance,
            v_table: Box::new(InstanceVTable::load(loader, api_version, extensions)),
            api_version,
        };
        Ok(out)
    }

    /// Returns all [`PhysicalDevice`]s from this [`Instance`] as a wrapped with v_tables.
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>
    ///
    /// Unlike [`Instance::enumerate_physical_devices_raw`], which returns raw handles, this method returns
    /// [`PhysicalDevice`] objects that are ready to use with their associated function tables.
    ///
    /// # Safety
    /// The returned [`PhysicalDevice`] borrows its function table from this
    /// [`Instance`]. Dropping the `Device` while any [`PhysicalDevice`] is still
    /// in use is undefined behaviour.
    ///
    /// All methods called on this may result in undefined behaviour
    /// if there are invalid pointers in the parameter structs,
    /// use the `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK to catch these bugs!
    #[inline]
    pub unsafe fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, vkResult> {
        self.enumerate_physical_devices_raw().map(|p| {
            p.into_iter()
                .map(|d| unsafe { d.to_physical_device(self) })
                .collect()
        })
    }

    /// Returns al PhysicalDevices from this [`Instance`] without v_tables.
    /// to obtain the v_tables call [`vkPhysicalDevice::to_physical_device`] or directly [`Instance::enumerate_physical_devices`]
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDevices.html>
    #[inline]
    pub fn enumerate_physical_devices_raw(&self) -> Result<Vec<vkPhysicalDevice>, vkResult> {
        read_into_vec_result(|count, data| unsafe {
            (self.fns().v1_0.enumerate_physical_devices.unwrap())(self.handle(), count, data)
        })
    }
}

/// A physical device (GPU / integrated graphics).
///
/// Obtained via [`Instance::enumerate_physical_devices`].
///
/// Holds a **static** reference to the function table stored inside the parent
/// `Instance`. Therefore it must not outlive the `Instance`.
#[derive(Clone, Copy)]
pub struct PhysicalDevice {
    pub(crate) handle: vkPhysicalDevice,
    pub(crate) v_table: &'static PhysicalDeviceFn,
}

impl PhysicalDevice {
    /// Returns the raw `VkPhysicalDevice` handle.
    pub const fn handle(&self) -> vkPhysicalDevice {
        self.handle
    }

    /// Returns the function table for this physical device.
    pub const fn fns(&self) -> &PhysicalDeviceFn {
        self.v_table
    }

    /// Creates a logical device from this physical device.
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>
    pub fn create_device(
        &self,
        create_info: &DeviceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        instance: &Instance,
    ) -> Result<Device, vkResult> {
        let mut handle = MaybeUninit::uninit();
        let result = unsafe {
            (self.v_table.v1_0.create_device.unwrap())(
                self.handle,
                create_info,
                raw_option(allocator),
                handle.as_mut_ptr(),
            )
        };

        let handle = if !matches!(result, vkResult::Success) {
            return Err(result);
        } else {
            unsafe { handle.assume_init() }
        };

        let device_loader = unsafe {
            get_instance_proc_addr(instance.handle(), c"vkGetDeviceProcAddr".as_ptr()).unwrap()
        };
        let device_loader: vkGetDeviceProcAddr = unsafe { transmute(device_loader) };

        let loader = |name: &CStr| unsafe { transmute((device_loader)(handle, name.as_ptr())) };

        let extensions: &[*const i8] = if create_info.pp_enabled_extension_names.is_null()
            || create_info.enabled_extension_count == 0
        {
            &[]
        } else {
            unsafe {
                slice::from_raw_parts(
                    create_info.pp_enabled_extension_names.cast(),
                    create_info.enabled_extension_count as usize,
                )
            }
        };

        Ok(Device {
            handle,
            v_table: Box::new(DeviceVTable::load(
                loader,
                instance.api_version(),
                extensions,
            )),
        })
    }
}

/// A logical Vulkan device.
///
/// Owns the device handle **and** its v-table. All queues and command buffers
/// created from this device borrow their function pointers from this v-table.
#[derive(Clone)]
pub struct Device {
    pub(crate) handle: vkDevice,
    pub(crate) v_table: Box<DeviceVTable>,
}

impl Device {
    /// Returns the raw `VkDevice` handle.
    pub const fn handle(&self) -> vkDevice {
        self.handle
    }

    /// Returns a reference to the internal v-table.
    pub const fn v_table(&self) -> &DeviceVTable {
        &self.v_table
    }

    /// Returns the function table for this device.
    pub const fn fns(&self) -> &DeviceFn {
        &self.v_table.device
    }

    /// Returns a queue from this device as a wrapped [`Queue`] object with v_table.
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>
    ///
    /// Unlike [`Device::get_device_queue_raw`], which returns raw handles, this method returns
    /// [`Queue`] objects that are ready to use with their associated function tables.
    ///
    /// # Safety
    /// The returned [`Queue`] borrows its function table from this
    /// [`Device`]. Dropping the `Device` while any [`Queue`] is still
    /// in use is undefined behaviour.
    ///
    /// All methods called on this may result in undefined behaviour
    /// if there are invalid pointers in the parameter structs,
    /// use the `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK to catch these bugs!
    pub unsafe fn get_device_queue(&self, queue_family_index: u32, queue_index: u32) -> Queue {
        let handle = self.get_device_queue_raw(queue_family_index, queue_index);
        let v_table = &self.v_table().queue;
        let v_table = unsafe { transmute(v_table) };
        Queue { handle, v_table }
    }

    /// Returns a queue from this device without v_table.
    /// to obtain the v_table call [`vkQueue::to_queue`] or directly [`Device::get_device_queue`]
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue.html>
    pub fn get_device_queue_raw(&self, queue_family_index: u32, queue_index: u32) -> vkQueue {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_device_queue.unwrap())(
                self.handle(),
                queue_family_index,
                queue_index,
                out.as_mut_ptr(),
            )
        }
        unsafe { out.assume_init() }
    }

    /// Returns a queue from this device using `VkDeviceQueueInfo2` as a wrapped [`Queue`] with v_table.
    ///
    /// Use this instead of [`Device::get_device_queue`] when you need to query queues
    /// created with [`vk::DeviceQueueCreateFlags`] (e.g. `PROTECTED_BIT`).
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>
    ///
    /// # Safety
    /// The returned [`Queue`] borrows its function table from this
    /// [`Device`]. Dropping the `Device` while any [`Queue`] is still
    /// in use is undefined behaviour.
    ///
    /// Requires Vulkan 1.1 or `VK_KHR_get_physical_device_properties2`.
    pub unsafe fn get_device_queue2(&self, queue_info: &vk::DeviceQueueInfo2) -> Queue {
        let handle = self.get_device_queue2_raw(queue_info);
        let v_table: &'static QueueFn = unsafe { transmute(&self.v_table().queue) };
        Queue { handle, v_table }
    }

    /// Returns a queue from this device using `VkDeviceQueueInfo2` without v_table.
    ///
    /// To obtain the v_table call [`vkQueue::to_queue`] or use [`Device::get_device_queue2`] directly.
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceQueue2.html>
    ///
    /// Requires Vulkan 1.1 or `VK_KHR_get_physical_device_properties2`.
    pub fn get_device_queue2_raw(&self, queue_info: &vk::DeviceQueueInfo2) -> vkQueue {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1 // Vulkan 1.1 dispatch table
                .get_device_queue2
                .unwrap())(self.handle(), queue_info, out.as_mut_ptr())
        }
        unsafe { out.assume_init() }
    }

    /// Allocates command buffers and returns wrapped [`CommandBuffer`] objects with v_tables.
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>
    ///
    /// Unlike [`Device::allocate_command_buffers_raw`], which returns raw handles, this method returns
    /// [`CommandBuffer`] objects that are ready to use with their associated function tables.
    ///
    /// # Safety
    /// The returned [`CommandBuffer`] borrows their function table from this
    /// `Device`. Dropping the `Device` while any [`CommandBuffer`] is still
    /// in use is undefined behaviour.
    ///
    /// All methods called on this may result in undefined behaviour
    /// if there are invalid pointers in the parameter structs,
    /// use the `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK to catch these bugs!
    pub unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
    ) -> Result<Vec<CommandBuffer>, vkResult> {
        self.allocate_command_buffers_raw(allocate_info).map(|c| {
            c.into_iter()
                .map(|c| unsafe { c.to_command_buffer(self) })
                .collect()
        })
    }

    /// Allocates command buffers without v_tables.
    /// to obtain the v_tables call [`vkCommandBuffer::to_command_buffer`] or directly [`Device::allocate_command_buffers`]
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>
    pub fn allocate_command_buffers_raw(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
    ) -> Result<Vec<vkCommandBuffer>, vkResult> {
        let mut buffers = Vec::with_capacity(allocate_info.command_buffer_count as usize);
        let result = unsafe {
            (self.fns().v1_0.allocate_command_buffers.unwrap())(
                self.handle(),
                allocate_info,
                buffers.as_mut_ptr(),
            )
            .set_len_on_success(buffers, allocate_info.command_buffer_count as usize)
        };

        if let Err(e) = result {
            return Err(e);
        }

        result
    }
}

/// A Vulkan queue.
///
/// Obtained via [`Device::get_device_queue`]. Holds a static reference to the
/// function table that lives inside the parent `Device`.
#[derive(Clone, Copy)]
pub struct Queue {
    pub(crate) handle: vkQueue,
    pub(crate) v_table: &'static QueueFn,
}

impl Queue {
    /// Returns the raw `VkQueue` handle.
    pub const fn handle(&self) -> vkQueue {
        self.handle
    }

    /// Returns the function table for this queue.
    pub const fn fns(&self) -> &QueueFn {
        self.v_table
    }
}

/// A Vulkan command buffer.
///
/// Obtained via [`Device::allocate_command_buffers`]. Holds a static reference
/// to the function table that lives inside the parent `Device`.
#[derive(Clone, Copy)]
pub struct CommandBuffer {
    pub(crate) handle: vkCommandBuffer,
    pub(crate) v_table: &'static CommandBufferFn,
}

impl CommandBuffer {
    /// Returns the raw `VkCommandBuffer` handle.
    pub const fn handle(&self) -> vkCommandBuffer {
        self.handle
    }

    /// Returns the function table for this command buffer.
    pub const fn fns(&self) -> &CommandBufferFn {
        self.v_table
    }
}

// -----------------------------------------------------------------------------
// Debug impls (very minimal – these types are opaque)
impl fmt::Debug for Instance {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instance")
    }
}

impl fmt::Debug for PhysicalDevice {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PhysicalDevice")
    }
}

impl fmt::Debug for Device {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Device")
    }
}

impl fmt::Debug for Queue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Queue")
    }
}

impl fmt::Debug for CommandBuffer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CommandBuffer")
    }
}
