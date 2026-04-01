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
pub const NAME: &CStr = c"VK_NV_optical_flow";
pub const SPEC_VERSION: u32 = 1;

pub trait OpticalFlowPhysicalDevice {
    fn get_optical_flow_image_formats(
        &self,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
        image_format_properties: &mut [OpticalFlowImageFormatPropertiesNV],
    ) -> Result<(), Error>;
    fn get_optical_flow_image_formats_len(
        &self,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
    ) -> Result<usize, Error>;
}

impl OpticalFlowPhysicalDevice for PhysicalDevice {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPhysicalDeviceOpticalFlowImageFormatsNV.html>
    ///
    /// Call [`get_optical_flow_image_formats_len()`][`Self::get_optical_flow_image_formats_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_optical_flow_image_formats(
        &self,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
        image_format_properties: &mut [OpticalFlowImageFormatPropertiesNV],
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_optical_flow
                .as_ref()
                .unwrap()
                .get_physical_device_optical_flow_image_formats_nv)(
                self.handle,
                optical_flow_image_format_info,
                image_format_properties.len() as *mut u32,
                image_format_properties.as_mut_ptr(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_optical_flow_image_formats`][`Self::get_optical_flow_image_formats`].
    #[inline]
    fn get_optical_flow_image_formats_len(
        &self,
        optical_flow_image_format_info: &OpticalFlowImageFormatInfoNV,
    ) -> Result<usize, Error> {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_optical_flow
                .as_ref()
                .unwrap()
                .get_physical_device_optical_flow_image_formats_nv)(
                self.handle,
                optical_flow_image_format_info,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
        .map(|o| o as usize)
    }
}

pub trait OpticalFlowDevice {
    fn create_optical_flow_session(
        &self,
        create_info: &OpticalFlowSessionCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<OpticalFlowSessionNV, Error>;

    fn destroy_optical_flow_session(
        &self,
        session: OpticalFlowSessionNV,
        allocator: Option<&AllocationCallbacks>,
    );

    fn bind_optical_flow_session_image(
        &self,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> Result<(), Error>;
}

impl OpticalFlowDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateOpticalFlowSessionNV.html>
    #[inline]
    fn create_optical_flow_session(
        &self,
        create_info: &OpticalFlowSessionCreateInfoNV,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<OpticalFlowSessionNV, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .nv_optical_flow
                .as_ref()
                .unwrap()
                .create_optical_flow_session_nv)(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyOpticalFlowSessionNV.html>
    #[inline]
    fn destroy_optical_flow_session(
        &self,
        session: OpticalFlowSessionNV,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self
                .fns()
                .nv_optical_flow
                .as_ref()
                .unwrap()
                .destroy_optical_flow_session_nv)(
                self.handle,
                session,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindOpticalFlowSessionImageNV.html>
    #[inline]
    fn bind_optical_flow_session_image(
        &self,
        session: OpticalFlowSessionNV,
        binding_point: OpticalFlowSessionBindingPointNV,
        view: ImageView,
        layout: ImageLayout,
    ) -> Result<(), Error> {
        unsafe {
            (self
                .fns()
                .nv_optical_flow
                .as_ref()
                .unwrap()
                .bind_optical_flow_session_image_nv)(
                self.handle,
                session,
                binding_point,
                view,
                layout,
            )
        }
        .result()
    }
}

pub trait OpticalFlowCommandBuffer {
    fn optical_flow_execute(
        &self,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV,
    );
}

impl OpticalFlowCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdOpticalFlowExecuteNV.html>
    ///
    /// Queues types: `OpticalFlowNV`.
    /// Task: `Executes GPU work`.
    /// Use outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn optical_flow_execute(
        &self,
        session: OpticalFlowSessionNV,
        execute_info: &OpticalFlowExecuteInfoNV,
    ) {
        unsafe {
            (self
                .fns()
                .nv_optical_flow
                .as_ref()
                .unwrap()
                .optical_flow_execute_nv)(self.handle, session, execute_info)
        };
    }
}
