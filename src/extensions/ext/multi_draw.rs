// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated extensions
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;
use core::ffi::CStr;
use core::ptr::{from_ref, null};

pub const NAME: &CStr = c"VK_EXT_multi_draw";
pub const SPEC_VERSION: u32 = 1;

pub trait MultiDrawCommandBuffer {
    fn draw_multi(
        &self,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    );

    fn draw_multi_indexed(
        &self,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    );
}

impl MultiDrawCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiEXT.html>
    #[inline]
    fn draw_multi(
        &self,
        vertex_info: &[MultiDrawInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
    ) {
        unsafe {
            (self.fns().ext_multi_draw.as_ref().unwrap().draw_multi_ext)(
                self.handle,
                vertex_info.len() as u32,
                vertex_info.as_ptr(),
                instance_count,
                first_instance,
                stride,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdDrawMultiIndexedEXT.html>
    #[inline]
    fn draw_multi_indexed(
        &self,
        index_info: &[MultiDrawIndexedInfoEXT],
        instance_count: u32,
        first_instance: u32,
        stride: u32,
        vertex_offset: Option<&i32>,
    ) {
        unsafe {
            (self
                .fns()
                .ext_multi_draw
                .as_ref()
                .unwrap()
                .draw_multi_indexed_ext)(
                self.handle,
                index_info.len() as u32,
                index_info.as_ptr(),
                instance_count,
                first_instance,
                stride,
                vertex_offset.map_or(null(), from_ref),
            )
        };
    }
}
