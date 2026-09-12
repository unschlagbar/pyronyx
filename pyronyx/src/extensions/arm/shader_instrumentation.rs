// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::{from_ref, null};

/// Type: `Device`
pub const NAME: &CStr = c"VK_ARM_shader_instrumentation";
pub const SPEC_VERSION: u32 = 1;

pub trait ShaderInstrumentationPhysicalDevice {
    fn enumerate_shader_instrumentation_metrics(
        &self,
        descriptions: &mut [ShaderInstrumentationMetricDescriptionARM],
    ) -> Result<()>;
    fn enumerate_shader_instrumentation_metrics_len(&self) -> Result<usize>;
}

impl ShaderInstrumentationPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM.html>
    ///
    /// Call [`enumerate_shader_instrumentation_metrics_len()`][`Self::enumerate_shader_instrumentation_metrics_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn enumerate_shader_instrumentation_metrics(
        &self,
        descriptions: &mut [ShaderInstrumentationMetricDescriptionARM],
    ) -> Result<()> {
        let mut description_count = descriptions.len() as u32;
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .enumerate_physical_device_shader_instrumentation_metrics_arm;

        unsafe {
            (call)(
                self.handle,
                &mut description_count,
                descriptions.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`enumerate_shader_instrumentation_metrics`][`Self::enumerate_shader_instrumentation_metrics`].
    #[inline]
    fn enumerate_shader_instrumentation_metrics_len(&self) -> Result<usize> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .arm_shader_instrumentation
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .enumerate_physical_device_shader_instrumentation_metrics_arm)(
                self.handle,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|v| v as usize)
    }
}

pub trait ShaderInstrumentationDevice {
    fn create_shader_instrumentation(
        &self,
        create_info: &ShaderInstrumentationCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ShaderInstrumentationARM>;

    fn destroy_shader_instrumentation(
        &self,
        instrumentation: ShaderInstrumentationARM,
        allocator: Option<&AllocationCallbacks>,
    );

    fn get_shader_instrumentation_values(
        &self,
        instrumentation: ShaderInstrumentationARM,
        metric_block_count: *mut u32,
        metric_values: &mut [u8],
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> Result<()>;

    fn clear_shader_instrumentation_metrics(&self, instrumentation: ShaderInstrumentationARM);
}

impl ShaderInstrumentationDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderInstrumentationARM.html>
    #[inline]
    fn create_shader_instrumentation(
        &self,
        create_info: &ShaderInstrumentationCreateInfoARM,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ShaderInstrumentationARM> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_shader_instrumentation_arm;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderInstrumentationARM.html>
    #[inline]
    fn destroy_shader_instrumentation(
        &self,
        instrumentation: ShaderInstrumentationARM,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_shader_instrumentation_arm;

        unsafe {
            (call)(
                self.handle,
                instrumentation,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetShaderInstrumentationValuesARM.html>
    #[inline]
    fn get_shader_instrumentation_values(
        &self,
        instrumentation: ShaderInstrumentationARM,
        metric_block_count: *mut u32,
        metric_values: &mut [u8],
        flags: ShaderInstrumentationValuesFlagsARM,
    ) -> Result<()> {
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_shader_instrumentation_values_arm;

        unsafe {
            (call)(
                self.handle,
                instrumentation,
                metric_block_count,
                metric_values.as_mut_ptr().cast(),
                flags,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkClearShaderInstrumentationMetricsARM.html>
    #[inline]
    fn clear_shader_instrumentation_metrics(&self, instrumentation: ShaderInstrumentationARM) {
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .clear_shader_instrumentation_metrics_arm;

        unsafe { (call)(self.handle, instrumentation) };
    }
}

pub trait ShaderInstrumentationCommandBuffer {
    fn begin_shader_instrumentation(&self, instrumentation: ShaderInstrumentationARM);

    fn end_shader_instrumentation(&self);
}

impl ShaderInstrumentationCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginShaderInstrumentationARM.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `DataGraphARM`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_shader_instrumentation(&self, instrumentation: ShaderInstrumentationARM) {
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .begin_shader_instrumentation_arm;

        unsafe { (call)(self.handle, instrumentation) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndShaderInstrumentationARM.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `DataGraphARM`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_shader_instrumentation(&self) {
        let call = self
            .fns()
            .arm_shader_instrumentation
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .end_shader_instrumentation_arm;

        unsafe { (call)(self.handle) };
    }
}
