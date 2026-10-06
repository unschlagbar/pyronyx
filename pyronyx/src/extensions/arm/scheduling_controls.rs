// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;

/// Type: `Device`
///
/// Requires: [`VK_ARM_shader_core_builtins`](crate::arm::shader_core_builtins)
pub const NAME: &CStr = c"VK_ARM_scheduling_controls";
pub const SPEC_VERSION: u32 = 2;

pub trait SchedulingControlsCommandBuffer {
    fn set_dispatch_parameters(&self, dispatch_parameters: &DispatchParametersARM);
}

impl SchedulingControlsCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSetDispatchParametersARM.html>
    ///
    /// Queues types: `Compute`.
    /// Task: `Vulkan state access`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn set_dispatch_parameters(&self, dispatch_parameters: &DispatchParametersARM) {
        let call = self
            .fns()
            .arm_scheduling_controls
            .set_dispatch_parameters_arm
            .unwrap_or_else(|| Self::ext_load_error());

        unsafe { (call)(self.handle, dispatch_parameters) };
    }
}
