// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_NVX_binary_import";
pub const SPEC_VERSION: u32 = 2;

pub trait BinaryImportDevice {
    fn create_cu_module(
        &self,
        create_info: &CuModuleCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CuModuleNVX, Error>;

    fn create_cu_function(
        &self,
        create_info: &CuFunctionCreateInfoNVX,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CuFunctionNVX, Error>;

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
    ) -> Result<CuModuleNVX, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nvx_binary_import
                .as_ref()
                .unwrap()
                .create_cu_module_nvx)(
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
    ) -> Result<CuFunctionNVX, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nvx_binary_import
                .as_ref()
                .unwrap()
                .create_cu_function_nvx)(
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
        unsafe {
            (self
                .fns()
                .nvx_binary_import
                .as_ref()
                .unwrap()
                .destroy_cu_module_nvx)(
                self.handle, module, allocator.map_or(null(), from_ref)
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCuFunctionNVX.html>
    #[inline]
    fn destroy_cu_function(
        &self,
        function: CuFunctionNVX,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .nvx_binary_import
                .as_ref()
                .unwrap()
                .destroy_cu_function_nvx)(
                self.handle, function, allocator.map_or(null(), from_ref)
            )
        };
    }
}

pub trait BinaryImportCommandBuffer {
    fn cu_launch_kernel(&self, launch_info: &CuLaunchInfoNVX);
}

impl BinaryImportCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCuLaunchKernelNVX.html>
    #[inline]
    fn cu_launch_kernel(&self, launch_info: &CuLaunchInfoNVX) {
        unsafe {
            (self
                .fns()
                .nvx_binary_import
                .as_ref()
                .unwrap()
                .cu_launch_kernel_nvx)(self.handle, launch_info)
        };
    }
}
