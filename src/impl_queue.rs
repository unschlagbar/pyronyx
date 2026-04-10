// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated impls
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::vk::*;

impl Queue {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit.html>
    #[inline]
    pub fn submit(&self, submits: &[SubmitInfo], fence: Fence) -> Result<(), Error> {
        let call = self.fns().v1_0.queue_submit.expect(Self::CORE_LOAD_ERROR);

        unsafe { (call)(self.handle, submits.len() as u32, submits.as_ptr(), fence) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueWaitIdle.html>
    #[inline]
    pub fn wait_idle(&self) -> Result<(), Error> {
        let call = self
            .fns()
            .v1_0
            .queue_wait_idle
            .expect(Self::CORE_LOAD_ERROR);

        unsafe { (call)(self.handle) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueBindSparse.html>
    #[inline]
    pub fn bind_sparse(&self, bind_info: &[BindSparseInfo], fence: Fence) -> Result<(), Error> {
        let call = self
            .fns()
            .v1_0
            .queue_bind_sparse
            .expect(Self::CORE_LOAD_ERROR);

        unsafe {
            (call)(
                self.handle,
                bind_info.len() as u32,
                bind_info.as_ptr(),
                fence,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkQueueSubmit2.html>
    #[inline]
    pub fn submit2(&self, submits: &[SubmitInfo2], fence: Fence) -> Result<(), Error> {
        let call = self.fns().v1_3.queue_submit2.expect(Self::CORE_LOAD_ERROR);

        unsafe { (call)(self.handle, submits.len() as u32, submits.as_ptr(), fence) }.result()
    }
}
