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
    fn get_device_subpass_shading_max_workgroup_size(
        &self,
        renderpass: RenderPass,
    ) -> Result<Extent2D, Error>;
}

impl SubpassShadingDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI.html>
    #[inline]
    fn get_device_subpass_shading_max_workgroup_size(
        &self,
        renderpass: RenderPass,
    ) -> Result<Extent2D, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .huawei_subpass_shading
                .as_ref()
                .unwrap()
                .get_device_subpass_shading_max_workgroup_size_huawei)(
                self.handle,
                renderpass,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }
}

pub trait SubpassShadingCommandBuffer {
    fn subpass_shading(&self);
}

impl SubpassShadingCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdSubpassShadingHUAWEI.html>
    #[inline]
    fn subpass_shading(&self) {
        unsafe {
            (self
                .fns()
                .huawei_subpass_shading
                .as_ref()
                .unwrap()
                .subpass_shading_huawei)(self.handle)
        };
    }
}
