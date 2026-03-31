// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::CStr;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_NV_cuda_kernel_launch";
pub const SPEC_VERSION: u32 = 2;

pub trait CudaKernelLaunchDevice {
    fn create_cuda_module(
        &self,
        create_info: &CudaModuleCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CudaModuleNV, Error>;

    fn get_cuda_module_cache(&self, module: CudaModuleNV) -> Result<Vec<c_void>, Error>;

    fn create_cuda_function(
        &self,
        create_info: &CudaFunctionCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CudaFunctionNV, Error>;

    fn destroy_cuda_module(&self, module: CudaModuleNV, allocator: Option<&AllocationCallbacks>);

    fn destroy_cuda_function(
        &self,
        function: CudaFunctionNV,
        allocator: Option<&AllocationCallbacks>,
    );
}

impl CudaKernelLaunchDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaModuleNV.html>
    #[inline]
    fn create_cuda_module(
        &self,
        create_info: &CudaModuleCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CudaModuleNV, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_cuda_kernel_launch
                .as_ref()
                .unwrap()
                .create_cuda_module_nv)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCudaModuleCacheNV.html>
    #[inline]
    fn get_cuda_module_cache(&self, module: CudaModuleNV) -> Result<Vec<c_void>, Error> {
        read_into_vec_result(|count, data| unsafe {
            (self
                .fns()
                .nv_cuda_kernel_launch
                .as_ref()
                .unwrap()
                .get_cuda_module_cache_nv)(self.handle, module, count, data)
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCudaFunctionNV.html>
    #[inline]
    fn create_cuda_function(
        &self,
        create_info: &CudaFunctionCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CudaFunctionNV, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_cuda_kernel_launch
                .as_ref()
                .unwrap()
                .create_cuda_function_nv)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaModuleNV.html>
    #[inline]
    fn destroy_cuda_module(&self, module: CudaModuleNV, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self
                .fns()
                .nv_cuda_kernel_launch
                .as_ref()
                .unwrap()
                .destroy_cuda_module_nv)(
                self.handle, module, allocator.map_or(null(), from_ref)
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCudaFunctionNV.html>
    #[inline]
    fn destroy_cuda_function(
        &self,
        function: CudaFunctionNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .nv_cuda_kernel_launch
                .as_ref()
                .unwrap()
                .destroy_cuda_function_nv)(
                self.handle,
                function,
                allocator.map_or(null(), from_ref),
            )
        };
    }
}

pub trait CudaKernelLaunchCommandBuffer {
    fn cuda_launch_kernel(&self, launch_info: &CudaLaunchInfoNV);
}

impl CudaKernelLaunchCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCudaLaunchKernelNV.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn cuda_launch_kernel(&self, launch_info: &CudaLaunchInfoNV) {
        unsafe {
            (self
                .fns()
                .nv_cuda_kernel_launch
                .as_ref()
                .unwrap()
                .cuda_launch_kernel_nv)(self.handle, launch_info)
        };
    }
}
