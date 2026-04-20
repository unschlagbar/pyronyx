// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::mem::MaybeUninit;

/// Type: `Device`
pub const NAME: &CStr = c"VK_HUAWEI_subpass_shading";
pub const SPEC_VERSION: u32 = 3;

pub trait SubpassShadingDevice {
    fn get_subpass_shading_max_workgroup_size(&self, renderpass: RenderPass) -> Result<Extent2D>;
}

impl SubpassShadingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>
    #[inline]
    fn get_subpass_shading_max_workgroup_size(&self, renderpass: RenderPass) -> Result<Extent2D> {
        let mut out = MaybeUninit::uninit();
        let call = self
            .fns()
            .huawei_subpass_shading
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_device_subpass_shading_max_workgroup_size_huawei;

        unsafe { (call)(self.handle, renderpass, out.as_mut_ptr()) }.init_on_success(out)
    }
}

pub trait SubpassShadingCommandBuffer {
    fn subpass_shading(&self);
}

impl SubpassShadingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html>
    ///
    /// Queues types: `Graphics`.
    /// Task: `Executes GPU work`.
    /// Use inside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn subpass_shading(&self) {
        let call = self
            .fns()
            .huawei_subpass_shading
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .subpass_shading_huawei;

        unsafe { (call)(self.handle) };
    }
}
