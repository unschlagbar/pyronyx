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
pub const NAME: &CStr = c"VK_AMD_gpa_interface";
pub const SPEC_VERSION: u32 = 1;

pub trait GpaInterfaceDevice {
    fn create_gpa_session(
        &self,
        create_info: &GpaSessionCreateInfoAMD,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<GpaSessionAMD>;

    fn destroy_gpa_session(
        &self,
        gpa_session: GpaSessionAMD,
        allocator: Option<&AllocationCallbacks>,
    );

    fn set_gpa_clock_mode(&self) -> Result<GpaDeviceClockModeInfoAMD<'_>>;

    fn get_gpa_clock_info(&self) -> Result<GpaDeviceGetClockInfoAMD<'_>>;

    fn get_gpa_session_status(&self, gpa_session: GpaSessionAMD) -> Result<()>;

    fn get_gpa_session_results(
        &self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        data: &mut [u8],
    ) -> Result<()>;
    fn get_gpa_session_results_len(
        &self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
    ) -> Result<usize>;

    fn reset_gpa_session(&self, gpa_session: GpaSessionAMD) -> Result<()>;
}

impl GpaInterfaceDevice for Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGpaSessionAMD.html>
    #[inline]
    fn create_gpa_session(
        &self,
        create_info: &GpaSessionCreateInfoAMD,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<GpaSessionAMD> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .create_gpa_session_amd;

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

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyGpaSessionAMD.html>
    #[inline]
    fn destroy_gpa_session(
        &self,
        gpa_session: GpaSessionAMD,
        allocator: Option<&AllocationCallbacks>,
    ) {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .destroy_gpa_session_amd;

        unsafe { (call)(self.handle, gpa_session, allocator.map_or(null(), from_ref)) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetGpaDeviceClockModeAMD.html>
    #[inline]
    fn set_gpa_clock_mode(&self) -> Result<GpaDeviceClockModeInfoAMD<'_>> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .set_gpa_device_clock_mode_amd;

        unsafe { (call)(self.handle, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaDeviceClockInfoAMD.html>
    #[inline]
    fn get_gpa_clock_info(&self) -> Result<GpaDeviceGetClockInfoAMD<'_>> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_gpa_device_clock_info_amd;

        unsafe { (call)(self.handle, out.as_mut_ptr()) }.init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionStatusAMD.html>
    #[inline]
    fn get_gpa_session_status(&self, gpa_session: GpaSessionAMD) -> Result<()> {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_gpa_session_status_amd;

        unsafe { (call)(self.handle, gpa_session) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetGpaSessionResultsAMD.html>
    ///
    /// Call [`get_gpa_session_results_len()`][`Self::get_gpa_session_results_len()`] to query the number of elements to pass to `out`.
    #[inline]
    fn get_gpa_session_results(
        &self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
        data: &mut [u8],
    ) -> Result<()> {
        let mut size_in_bytes = data.len();
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .get_gpa_session_results_amd;

        unsafe {
            (call)(
                self.handle,
                gpa_session,
                sample_id,
                &mut size_in_bytes,
                data.as_mut_ptr().cast(),
            )
        }
        .result()
    }

    /// Returns the required slice length for Call [`get_gpa_session_results`][`Self::get_gpa_session_results`].
    #[inline]
    fn get_gpa_session_results_len(
        &self,
        gpa_session: GpaSessionAMD,
        sample_id: u32,
    ) -> Result<usize> {
        let mut out: MaybeUninit<usize> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .amd_gpa_interface
                .as_ref()
                .expect(Self::EXT_LOAD_ERROR)
                .get_gpa_session_results_amd)(
                self.handle,
                gpa_session,
                sample_id,
                out.as_mut_ptr(),
                ptr::null_mut(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetGpaSessionAMD.html>
    #[inline]
    fn reset_gpa_session(&self, gpa_session: GpaSessionAMD) -> Result<()> {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .reset_gpa_session_amd;

        unsafe { (call)(self.handle, gpa_session) }.result()
    }
}

pub trait GpaInterfaceCommandBuffer {
    fn begin_gpa_session(&self, gpa_session: GpaSessionAMD) -> Result<()>;

    fn end_gpa_session(&self, gpa_session: GpaSessionAMD) -> Result<()>;

    fn begin_gpa_sample(
        &self,
        gpa_session: GpaSessionAMD,
        gpa_sample_begin_info: &GpaSampleBeginInfoAMD,
    ) -> Result<u32>;

    fn end_gpa_sample(&self, gpa_session: GpaSessionAMD, sample_id: u32);

    fn copy_gpa_session_results(&self, gpa_session: GpaSessionAMD);
}

impl GpaInterfaceCommandBuffer for CommandBuffer {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSessionAMD.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_gpa_session(&self, gpa_session: GpaSessionAMD) -> Result<()> {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .begin_gpa_session_amd;

        unsafe { (call)(self.handle, gpa_session) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSessionAMD.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_gpa_session(&self, gpa_session: GpaSessionAMD) -> Result<()> {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .end_gpa_session_amd;

        unsafe { (call)(self.handle, gpa_session) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdBeginGpaSampleAMD.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn begin_gpa_sample(
        &self,
        gpa_session: GpaSessionAMD,
        gpa_sample_begin_info: &GpaSampleBeginInfoAMD,
    ) -> Result<u32> {
        let mut out = MaybeUninit::new(Default::default());
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .begin_gpa_sample_amd;

        unsafe {
            (call)(
                self.handle,
                gpa_session,
                gpa_sample_begin_info,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdEndGpaSampleAMD.html>
    ///
    /// Queues types: `Graphics`, `Compute`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn end_gpa_sample(&self, gpa_session: GpaSessionAMD, sample_id: u32) {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .end_gpa_sample_amd;

        unsafe { (call)(self.handle, gpa_session, sample_id) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCmdCopyGpaSessionResultsAMD.html>
    ///
    /// Queues types: `Graphics`, `Compute`, `Transfer`.
    /// Task: `Executes GPU work`, `Vulkan state access`.
    /// Use inside and outside `RenderPass`.
    /// Command buffer level: `primary`, `secondary`.
    #[inline]
    fn copy_gpa_session_results(&self, gpa_session: GpaSessionAMD) {
        let call = self
            .fns()
            .amd_gpa_interface
            .as_ref()
            .expect(Self::EXT_LOAD_ERROR)
            .copy_gpa_session_results_amd;

        unsafe { (call)(self.handle, gpa_session) };
    }
}
