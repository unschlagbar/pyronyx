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
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .initialize_performance_api_intel;

        unsafe { (call)(self.handle, initialize_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUninitializePerformanceApiINTEL.html>
    #[inline]
    fn uninitialize_performance_api(&self) {
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .uninitialize_performance_api_intel;

        unsafe { (call)(self.handle) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAcquirePerformanceConfigurationINTEL.html>
    #[inline]
    fn acquire_performance_configuration(
        &self,
        acquire_info: &PerformanceConfigurationAcquireInfoINTEL,
    ) -> Result<PerformanceConfigurationINTEL, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .acquire_performance_configuration_intel;

        unsafe { (call)(self.handle, acquire_info, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkReleasePerformanceConfigurationINTEL.html>
    #[inline]
    fn release_performance_configuration(
        &self,
        configuration: PerformanceConfigurationINTEL,
    ) -> Result<(), Error> {
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .release_performance_configuration_intel;

        unsafe { (call)(self.handle, configuration) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPerformanceParameterINTEL.html>
    #[inline]
    fn get_performance_parameter(
        &self,
        parameter: PerformanceParameterTypeINTEL,
    ) -> Result<PerformanceValueINTEL, Error> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_performance_parameter_intel;

        unsafe { (call)(self.handle, parameter, out.as_mut_ptr()) }.init_on_success(out)
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
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_performance_marker_intel;

        unsafe { (call)(self.handle, marker_info) }.result()
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
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_performance_stream_marker_intel;

        unsafe { (call)(self.handle, marker_info) }.result()
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
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_performance_override_intel;

        unsafe { (call)(self.handle, override_info) }.result()
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
        let call = self
            .fns()
            .intel_performance_query
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .queue_set_performance_configuration_intel;

        unsafe { (call)(self.handle, configuration) }.result()
    }
}
