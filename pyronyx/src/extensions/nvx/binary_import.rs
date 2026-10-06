// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_NVX_binary_import";
pub const SPEC_VERSION: u32 = 2;

pub trait BinaryImportDevice {
    fn create_cu_module(
        &self,
        create_info: &CuModuleCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CuModuleNVX>;

    fn create_cu_function(
        &self,
        create_info: &CuFunctionCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CuFunctionNVX>;

    fn destroy_cu_module(&self, module: CuModuleNVX, allocator: Option<&AllocationCallbacks>);

    fn destroy_cu_function(&self, function: CuFunctionNVX, allocator: Option<&AllocationCallbacks>);
}

impl BinaryImportDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuModuleNVX.html>
    #[inline]
    fn create_cu_module(
        &self,
        create_info: &CuModuleCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CuModuleNVX> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nvx_binary_import
            .create_cu_module_nvx
            .unwrap_or_else(|| Self::ext_load_error());

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCuFunctionNVX.html>
    #[inline]
    fn create_cu_function(
        &self,
        create_info: &CuFunctionCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CuFunctionNVX> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .nvx_binary_import
            .create_cu_function_nvx
            .unwrap_or_else(|| Self::ext_load_error());

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuModuleNVX.html>
    #[inline]
    fn destroy_cu_module(&self, module: CuModuleNVX, allocator: Option<&AllocationCallbacks>) {
        let call = self
            .fns()
            .nvx_binary_import
            .destroy_cu_module_nvx
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, module, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html>
    #[inline]
    fn destroy_cu_function(
        &self,
        function: CuFunctionNVX,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .nvx_binary_import
            .destroy_cu_function_nvx
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, function, allocator.map_or(null(), from_ref)) };
    }
}

pub trait BinaryImportCommandBuffer {
    fn cu_launch_kernel(&self, launch_info: &CuLaunchInfoNVX);
}

impl BinaryImportCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn cu_launch_kernel(&self, launch_info: &CuLaunchInfoNVX) {
        let call = self
            .fns()
            .nvx_binary_import
            .cu_launch_kernel_nvx
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, launch_info) };
    }
}
