// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_INTEL_performance_query";
pub const SPEC_VERSION: u32 = 2;

pub trait PerformanceQueryDevice {
    fn initialize_performance_api(
        &self,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> Result<(), Error>;

    fn uninitialize_performance_api(&self);

    fn acquire_performance_configuration(
        &self,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> Result<PerformanceConfigurationINTEL, Error>;

    fn release_performance_configuration(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result<(), Error>;

    fn get_performance_parameter(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> Result<PerformanceValueINTEL, Error>;
}

impl PerformanceQueryDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkInitializePerformanceApiINTEL.html>
    #[inline]
    fn initialize_performance_api(
        &self,
        initialize_info: &InitializePerformanceApiInfoINTEL,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .initialize_performance_api_intel)(self.handle, initialize_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html>
    #[inline]
    fn uninitialize_performance_api(&self) {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .uninitialize_performance_api_intel)(self.handle)
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html>
    #[inline]
    fn acquire_performance_configuration(
        &self,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> Result<PerformanceConfigurationINTEL, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .acquire_performance_configuration_intel)(
                self.handle,
                acquire_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html>
    #[inline]
    fn release_performance_configuration(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .release_performance_configuration_intel)(self.handle, configuration)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html>
    #[inline]
    fn get_performance_parameter(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> Result<PerformanceValueINTEL, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .get_performance_parameter_intel)(
                self.handle, parameter, out.as_mut_ptr()
            )
        }
        .init_on_success(out)
    }
}

pub trait PerformanceQueryCommandBuffer {
    fn set_performance_marker(&self, marker_info: &PerformanceMarkerInfoINTEL)
    -> Result<(), Error>;

    fn set_performance_stream_marker(
        &self,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> Result<(), Error>;

    fn set_performance_override(
        &self,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> Result<(), Error>;
}

impl PerformanceQueryCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceMarkerINTEL.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_performance_marker(
        &self,
        marker_info: &PerformanceMarkerInfoINTEL,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .set_performance_marker_intel)(self.handle, marker_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceStreamMarkerINTEL.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_performance_stream_marker(
        &self,
        marker_info: &PerformanceStreamMarkerInfoINTEL,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .set_performance_stream_marker_intel)(self.handle, marker_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetPerformanceOverrideINTEL.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_performance_override(
        &self,
        override_info: &PerformanceOverrideInfoINTEL,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .set_performance_override_intel)(self.handle, override_info)
        }
        .result()
    }
}

pub trait PerformanceQueryQueue {
    fn set_performance_configuration(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result<(), Error>;
}

impl PerformanceQueryQueue for Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSetPerformanceConfigurationINTEL.html>
    #[inline]
    fn set_performance_configuration(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .intel_performance_query
                .as_ref()
                .unwrap()
                .queue_set_performance_configuration_intel)(self.handle, configuration)
        }
        .result()
    }
}
