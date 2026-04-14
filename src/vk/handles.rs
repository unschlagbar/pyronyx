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
//!   `Result<_, Error>`, etc.).
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
    utils::{raw_option, read_into_vec_result, to_option},
    vk::{
        self, AllocationCallbacks, CommandBufferAllocateInfo, DeviceCreateInfo, Error,
        InstanceCreateInfo, get_instance_proc_addr, vkCommandBuffer, vkCreateInstance, vkDevice,
        vkGetDeviceProcAddr, vkInstance, vkPhysicalDevice, vkQueue, vkResult,
    },
    vtables::{
        CommandBufferFn, DeviceFn, DeviceVTable, InstanceFn, InstanceVTable, PhysicalDeviceFn,
        QueueFn,
    },
};
use core::{
    ffi::{CStr, c_char},
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
    pub(crate) const CORE_LOAD_ERROR: &str = "Instance core function is not loaded or not aviable. You may want to increase you `vk::InstanceCreateInfo.application_info.api_version`";
    pub(crate) const EXT_LOAD_ERROR: &str = "Instance ext function is not loaded or not aviable. You may use a extension that is not present in `vk::InstanceCreateInfo.enabled_extension_names`";

    /// Returns the raw `VkInstance` handle.
    pub const fn handle(&self) -> vkInstance {
        self.handle
    }

    /// Returns a reference to the internal v-table.
    pub const fn v_table(&self) -> &InstanceVTable {
        &self.v_table
    }

    /// Returns the vulkan version this [`Instance`] was created for
    pub const fn api_version(&self) -> u32 {
        self.api_version
    }

    /// Returns a reference to the function table for this [`Instance`].
    pub const fn fns(&self) -> &InstanceFn {
        &self.v_table.instance
    }

    /// Creates a new Vulkan instance.
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateInstance.html>
    ///
    /// # Safety
    /// Calling **any** fn on [`Instance`] with invalid non [`null()`](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    ///
    /// # Example
    ///
    /// ```no_run
    /// use pyronyx::vk;
    ///
    /// let app_info = vk::ApplicationInfo {
    ///     application_name: c"My App".as_ptr(),
    ///     application_version: vk::make_api_version(0, 1, 0, 0),
    ///     engine_name: c"No Engine".as_ptr(),
    ///     engine_version: vk::make_api_version(0, 1, 0, 0),
    ///     api_version: vk::API_VERSION_1_0,
    ///     ..Default::default()
    /// };
    ///
    /// let create_info = vk::InstanceCreateInfo {
    ///     application_info: &app_info,
    ///     ..Default::default()
    /// };
    ///
    /// let instance = unsafe {
    ///     vk::Instance::create(&create_info, None).expect("failed to create instance")
    /// };
    /// ```
    ///
    /// # Extended Example (with validation layers)
    ///
    /// ```no_run
    /// use pyronyx::vk;
    /// use pyronyx::ext;
    /// use core::ffi::{CStr, c_void};
    ///
    /// const VALIDATION_LAYERS: &[&CStr] = &[c"VK_LAYER_KHRONOS_validation"];
    ///
    /// // The debug callback is only compiled in debug builds.
    /// // It prints the severity, type and message of each validation message.
    /// #[cfg(debug_assertions)]
    /// unsafe extern "system" fn debug_callback(
    ///     message_severity: vk::DebugUtilsMessageSeverityFlagsEXT,
    ///     message_type: vk::DebugUtilsMessageTypeFlagsEXT,
    ///     callback_data: *const vk::DebugUtilsMessengerCallbackDataEXT,
    ///     _user_data: *mut c_void,
    /// ) -> u32 {
    ///     let message = unsafe { CStr::from_ptr((*callback_data).message) };
    ///     println!("[Debug][{}][{}] {:?}", message_severity, message_type, message);
    ///     0
    /// }
    ///
    /// let app_info = vk::ApplicationInfo {
    ///     application_name: c"My App".as_ptr(),
    ///     application_version: vk::make_api_version(0, 1, 0, 0),
    ///     engine_name: c"No Engine".as_ptr(),
    ///     engine_version: vk::make_api_version(0, 1, 0, 0),
    ///     api_version: vk::API_VERSION_1_0,
    ///     ..Default::default()
    /// };
    ///
    /// // In debug builds we add the debug utils extension so that
    /// // the validation layer can report errors through the debug messenger.
    /// #[cfg(debug_assertions)]
    /// let extensions = [ext::debug_utils::NAME.as_ptr()];
    /// #[cfg(not(debug_assertions))]
    /// let extensions: [*const i8; 0] = [];
    ///
    /// // Validation layers are only enabled in debug builds.
    /// // In release builds no layers are loaded for maximum performance.
    /// #[cfg(debug_assertions)]
    /// let layers: &[&CStr] = VALIDATION_LAYERS;
    /// #[cfg(not(debug_assertions))]
    /// let layers: &[&CStr] = &[];
    ///
    /// let create_info = vk::InstanceCreateInfo {
    ///     application_info: &app_info,
    ///     enabled_extension_count: extensions.len() as u32,
    ///     enabled_extension_names: extensions.as_ptr(),
    ///     enabled_layer_count: layers.len() as u32,
    ///     // Safety: *const &CStr and *const *const i8 have the same layout
    ///     enabled_layer_names: layers.as_ptr().cast(),
    ///     ..Default::default()
    /// };
    ///
    /// // Chain a DebugUtilsMessengerCreateInfo into the instance create info so that
    /// // errors during instance creation and destruction are also captured,
    /// // since the debug messenger itself doesn't exist yet at that point.
    /// #[cfg(debug_assertions)]
    /// let mut debug_create_info = vk::DebugUtilsMessengerCreateInfoEXT {
    ///     message_severity: vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
    ///         | vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
    ///     message_type: vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION
    ///         | vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE,
    ///     pfn_user_callback: Some(debug_callback),
    ///     ..Default::default()
    /// };
    /// #[cfg(debug_assertions)]
    /// let create_info = create_info.next(&mut debug_create_info);
    ///
    /// let instance = unsafe {
    ///     vk::Instance::create(&create_info, None).expect("failed to create instance")
    /// };
    /// ```
    pub unsafe fn create(
        create_info: &InstanceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Instance, Error> {
        let pfn: vkCreateInstance = to_option(unsafe {
            transmute(get_instance_proc_addr(
                vkInstance::null(),
                c"vkCreateInstance".as_ptr(),
            ))
        })
        .ok_or(vk::Error::IncompatibleDriver)?;

        let mut instance = vk::vkInstance::null();
        let result = unsafe { (pfn)(create_info, raw_option(allocator), &mut instance) };

        if !matches!(result, vkResult::Success) {
            return Err(result.into());
        }

        let loader =
            |name: &CStr| unsafe { transmute(get_instance_proc_addr(instance, name.as_ptr())) };

        let extensions: &[*const c_char] = if create_info.enabled_extension_names.is_null()
            || create_info.enabled_extension_count == 0
        {
            &[]
        } else {
            unsafe {
                slice::from_raw_parts(
                    create_info.enabled_extension_names,
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
    /// Calling **any** fn on [`PhysicalDevice`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    #[inline]
    pub unsafe fn enumerate_physical_devices(&self) -> Result<Vec<PhysicalDevice>, Error> {
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
    pub fn enumerate_physical_devices_raw(&self) -> Result<Vec<vkPhysicalDevice>, Error> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .v1_0
                .enumerate_physical_devices
                .expect(Self::CORE_LOAD_ERROR))(self.handle(), count, data)
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
    pub(crate) const CORE_LOAD_ERROR: &str = "PhysicalDevice Instance core function is not loaded or not aviable. You may want to increase you `vk::InstanceCreateInfo.application_info.api_version`";
    pub(crate) const EXT_LOAD_ERROR: &str = "PhysicalDevice Instance ext function is not loaded or not aviable. You may use a extension that is not present in `vk::InstanceCreateInfo.enabled_extension_names`";

    /// Returns the raw `VkPhysicalDevice` handle.
    pub const fn handle(&self) -> vkPhysicalDevice {
        self.handle
    }

    /// Returns the function table for this [`PhysicalDevice`].
    pub const fn fns(&self) -> &PhysicalDeviceFn {
        self.v_table
    }

    /// Creates a logical device from this physical device.
    ///
    /// # Vulkan documentation
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDevice.html>
    ///
    /// # Safety
    /// Calling **any** fn on [`Device`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    pub unsafe fn create_device(
        &self,
        create_info: &DeviceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
        instance: &Instance,
    ) -> Result<Device, Error> {
        let mut handle = MaybeUninit::uninit();
        let result = unsafe {
            (self
                .v_table
                .v1_0
                .create_device
                .expect(Self::CORE_LOAD_ERROR))(
                self.handle,
                create_info,
                raw_option(allocator),
                handle.as_mut_ptr(),
            )
        };

        let handle = if !matches!(result, vkResult::Success) {
            return Err(result.into());
        } else {
            unsafe { handle.assume_init() }
        };

        let device_loader = unsafe {
            get_instance_proc_addr(instance.handle(), c"vkGetDeviceProcAddr".as_ptr())
                .ok_or(vk::Error::IncompatibleDriver)?
        };
        let device_loader: vkGetDeviceProcAddr = unsafe { transmute(device_loader) };

        let loader = |name: &CStr| unsafe { transmute((device_loader)(handle, name.as_ptr())) };

        let extensions: &[*const c_char] = if create_info.enabled_extension_names.is_null()
            || create_info.enabled_extension_count == 0
        {
            &[]
        } else {
            unsafe {
                slice::from_raw_parts(
                    create_info.enabled_extension_names,
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
    pub(crate) const CORE_LOAD_ERROR: &str = "Device core function is not loaded or not aviable. You may want to increase you `vk::InstanceCreateInfo.application_info.api_version`";
    pub(crate) const EXT_LOAD_ERROR: &str = "Device ext function is not loaded or not aviable. You may use a extension that is not present in `vk::InstanceCreateInfo.enabled_extension_names`";

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
    /// Calling **any** fn on [`Queue`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
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
            (self
                .fns()
                .v1_0
                .get_device_queue
                .expect(Self::CORE_LOAD_ERROR))(
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
    /// Calling **any** fn on [`Queue`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
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
                .expect(Self::CORE_LOAD_ERROR))(
                self.handle(), queue_info, out.as_mut_ptr()
            )
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
    /// Calling **any** fn on [`CommandBuffer`] with invalid non [null()](core::ptr::null()) pointer in the function parameter
    /// or in a parameter struct will result in undefined behavior!
    ///
    /// To catch these bugs use `VK_LAYER_KHRONOS_validation` layer in [`InstanceCreateInfo`]
    /// together with the Vulkan SDK!
    pub unsafe fn allocate_command_buffers(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
        command_buffers: &mut [CommandBuffer],
    ) -> Result<(), Error> {
        assert_eq!(
            allocate_info.command_buffer_count as usize,
            command_buffers.len()
        );

        self.allocate_command_buffers_raw(allocate_info).map(|c| {
            c.into_iter()
                .zip(command_buffers.iter_mut())
                .for_each(|(raw, c)| *c = unsafe { raw.to_command_buffer(self) })
        })
    }

    /// Allocates command buffers without v_tables.
    /// to obtain the v_tables call [`vkCommandBuffer::to_command_buffer`] or directly [`Device::allocate_command_buffers`]
    ///
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateCommandBuffers.html>
    pub fn allocate_command_buffers_raw(
        &self,
        allocate_info: &CommandBufferAllocateInfo,
    ) -> Result<Vec<vkCommandBuffer>, Error> {
        let mut buffers = Vec::with_capacity(allocate_info.command_buffer_count as usize);
        unsafe {
            (self
                .fns()
                .v1_0
                .allocate_command_buffers
                .expect(Self::CORE_LOAD_ERROR))(
                self.handle(), allocate_info, buffers.as_mut_ptr()
            )
            .set_len_on_success(buffers, allocate_info.command_buffer_count as usize)
        }
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
    pub(crate) const CORE_LOAD_ERROR: &str = "Queue Device core function is not loaded or not aviable. You may want to increase you `vk::InstanceCreateInfo.application_info.api_version`";
    pub(crate) const EXT_LOAD_ERROR: &str = "Queue Device ext function is not loaded or not aviable. You may use a extension that is not present in `vk::InstanceCreateInfo.enabled_extension_names`";

    /// Returns the raw `VkQueue` handle.
    pub const fn handle(&self) -> vkQueue {
        self.handle
    }

    /// Returns the function table for this [`Queue`].
    pub const fn fns(&self) -> &QueueFn {
        self.v_table
    }
}

/// A Vulkan command buffer.
///
/// Obtained via [`Device::allocate_command_buffers`]. Holds a static reference
/// to the function table that lives inside the parent `Device`.
#[derive(Clone, Copy, Default)]
pub struct CommandBuffer {
    pub(crate) handle: vkCommandBuffer,
    pub(crate) v_table: Option<&'static CommandBufferFn>,
}

impl CommandBuffer {
    pub(crate) const CORE_LOAD_ERROR: &str = "CommandBuffer Device core function is not loaded or not aviable. You may want to increase you `vk::InstanceCreateInfo.application_info.api_version`";
    pub(crate) const EXT_LOAD_ERROR: &str = "CommandBuffer Device ext function is not loaded or not aviable. You may use a extension that is not present in `vk::InstanceCreateInfo.enabled_extension_names`";

    /// Returns the raw `VkCommandBuffer` handle.
    pub const fn handle(&self) -> vkCommandBuffer {
        self.handle
    }

    pub const fn null() -> Self {
        Self {
            handle: vk::vkCommandBuffer::null(),
            v_table: None,
        }
    }

    /// Returns the function table for this [`CommandBuffer`].
    pub const fn fns(&self) -> &CommandBufferFn {
        self.v_table
            .expect("No v-table use `self.handle().to_command_buffer(device)`")
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
