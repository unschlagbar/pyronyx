// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — generated impls
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use crate::utils::read_into_vec;
use crate::utils::read_into_vec_result;
use crate::vk::*;
use core::ffi::c_void;
use core::mem::MaybeUninit;
use core::ptr;
use core::ptr::{from_ref, null};

impl Device {
    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDevice.html>
    #[inline]
    pub fn destroy_device(&self, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_device.unwrap())(
                self.handle,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDeviceWaitIdle.html>
    #[inline]
    pub fn device_wait_idle(&self) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.device_wait_idle.unwrap())(self.handle) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateMemory.html>
    #[inline]
    pub fn allocate_memory(
        &self,
        allocate_info: &MemoryAllocateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DeviceMemory, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.allocate_memory.unwrap())(
                self.handle,
                allocate_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeMemory.html>
    #[inline]
    pub fn free_memory(&self, memory: DeviceMemory, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.free_memory.unwrap())(
                self.handle,
                memory,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory.html>
    #[inline]
    pub fn map_memory(
        &self,
        memory: DeviceMemory,
        offset: DeviceSize,
        size: DeviceSize,
        flags: MemoryMapFlags,
    ) -> Result<*mut c_void, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.map_memory.unwrap())(
                self.handle,
                memory,
                offset,
                size,
                flags,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory.html>
    #[inline]
    pub fn unmap_memory(&self, memory: DeviceMemory) {
        unsafe { (self.fns().v1_0.unmap_memory.unwrap())(self.handle, memory) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFlushMappedMemoryRanges.html>
    #[inline]
    pub fn flush_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange],
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.flush_mapped_memory_ranges.unwrap())(
                self.handle,
                memory_ranges.len() as u32,
                memory_ranges.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkInvalidateMappedMemoryRanges.html>
    #[inline]
    pub fn invalidate_mapped_memory_ranges(
        &self,
        memory_ranges: &[MappedMemoryRange],
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.invalidate_mapped_memory_ranges.unwrap())(
                self.handle,
                memory_ranges.len() as u32,
                memory_ranges.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryCommitment.html>
    #[inline]
    pub fn get_device_memory_commitment(&self, memory: DeviceMemory) -> DeviceSize {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_device_memory_commitment.unwrap())(
                self.handle,
                memory,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements.html>
    #[inline]
    pub fn get_buffer_memory_requirements(&self, buffer: Buffer) -> MemoryRequirements {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_buffer_memory_requirements.unwrap())(
                self.handle,
                buffer,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory.html>
    #[inline]
    pub fn bind_buffer_memory(
        &self,
        buffer: Buffer,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.bind_buffer_memory.unwrap())(
                self.handle,
                buffer,
                memory,
                memory_offset,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements.html>
    #[inline]
    pub fn get_image_memory_requirements(&self, image: Image) -> MemoryRequirements {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_image_memory_requirements.unwrap())(
                self.handle,
                image,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory.html>
    #[inline]
    pub fn bind_image_memory(
        &self,
        image: Image,
        memory: DeviceMemory,
        memory_offset: DeviceSize,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.bind_image_memory.unwrap())(self.handle, image, memory, memory_offset)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements.html>
    #[inline]
    pub fn get_image_sparse_memory_requirements(
        &self,
        image: Image,
    ) -> Vec<SparseImageMemoryRequirements> {
        read_into_vec(|count, data| unsafe {
            (self
                .fns()
                .v1_0
                .get_image_sparse_memory_requirements
                .unwrap())(self.handle, image, count, data)
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFence.html>
    #[inline]
    pub fn create_fence(
        &self,
        create_info: &FenceCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Fence, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_fence.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFence.html>
    #[inline]
    pub fn destroy_fence(&self, fence: Fence, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_fence.unwrap())(
                self.handle,
                fence,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetFences.html>
    #[inline]
    pub fn reset_fences(&self, fences: &[Fence]) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.reset_fences.unwrap())(
                self.handle,
                fences.len() as u32,
                fences.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFenceStatus.html>
    #[inline]
    pub fn get_fence_status(&self, fence: Fence) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.get_fence_status.unwrap())(self.handle, fence) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitForFences.html>
    #[inline]
    pub fn wait_for_fences(
        &self,
        fences: &[Fence],
        wait_all: bool,
        timeout: u64,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.wait_for_fences.unwrap())(
                self.handle,
                fences.len() as u32,
                fences.as_ptr(),
                wait_all as _,
                timeout,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSemaphore.html>
    #[inline]
    pub fn create_semaphore(
        &self,
        create_info: &SemaphoreCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Semaphore, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_semaphore.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySemaphore.html>
    #[inline]
    pub fn destroy_semaphore(&self, semaphore: Semaphore, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_semaphore.unwrap())(
                self.handle,
                semaphore,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateEvent.html>
    #[inline]
    pub fn create_event(
        &self,
        create_info: &EventCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Event, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_event.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyEvent.html>
    #[inline]
    pub fn destroy_event(&self, event: Event, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_event.unwrap())(
                self.handle,
                event,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetEventStatus.html>
    #[inline]
    pub fn get_event_status(&self, event: Event) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.get_event_status.unwrap())(self.handle, event) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetEvent.html>
    #[inline]
    pub fn set_event(&self, event: Event) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.set_event.unwrap())(self.handle, event) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetEvent.html>
    #[inline]
    pub fn reset_event(&self, event: Event) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.reset_event.unwrap())(self.handle, event) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateQueryPool.html>
    #[inline]
    pub fn create_query_pool(
        &self,
        create_info: &QueryPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<QueryPool, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_query_pool.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyQueryPool.html>
    #[inline]
    pub fn destroy_query_pool(
        &self,
        query_pool: QueryPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_query_pool.unwrap())(
                self.handle,
                query_pool,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetQueryPoolResults.html>
    #[inline]
    pub fn get_query_pool_results(
        &self,
        query_pool: QueryPool,
        first_query: u32,
        query_count: u32,
        data: &mut [c_void],
        stride: DeviceSize,
        flags: QueryResultFlags,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.get_query_pool_results.unwrap())(
                self.handle,
                query_pool,
                first_query,
                query_count,
                data.len() as usize,
                data.as_mut_ptr(),
                stride,
                flags,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetQueryPool.html>
    #[inline]
    pub fn reset_query_pool(&self, query_pool: QueryPool, first_query: u32, query_count: u32) {
        unsafe {
            (self.fns().v1_2.reset_query_pool.unwrap())(
                self.handle,
                query_pool,
                first_query,
                query_count,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBuffer.html>
    #[inline]
    pub fn create_buffer(
        &self,
        create_info: &BufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Buffer, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_buffer.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBuffer.html>
    #[inline]
    pub fn destroy_buffer(&self, buffer: Buffer, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_buffer.unwrap())(
                self.handle,
                buffer,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateBufferView.html>
    #[inline]
    pub fn create_buffer_view(
        &self,
        create_info: &BufferViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<BufferView, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_buffer_view.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyBufferView.html>
    #[inline]
    pub fn destroy_buffer_view(
        &self,
        buffer_view: BufferView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_buffer_view.unwrap())(
                self.handle,
                buffer_view,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImage.html>
    #[inline]
    pub fn create_image(
        &self,
        create_info: &ImageCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Image, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_image.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImage.html>
    #[inline]
    pub fn destroy_image(&self, image: Image, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_image.unwrap())(
                self.handle,
                image,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout.html>
    #[inline]
    pub fn get_image_subresource_layout(
        &self,
        image: Image,
        subresource: &ImageSubresource,
    ) -> SubresourceLayout {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_image_subresource_layout.unwrap())(
                self.handle,
                image,
                subresource,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateImageView.html>
    #[inline]
    pub fn create_image_view(
        &self,
        create_info: &ImageViewCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ImageView, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_image_view.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyImageView.html>
    #[inline]
    pub fn destroy_image_view(
        &self,
        image_view: ImageView,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_image_view.unwrap())(
                self.handle,
                image_view,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateShaderModule.html>
    #[inline]
    pub fn create_shader_module(
        &self,
        create_info: &ShaderModuleCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<ShaderModule, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_shader_module.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyShaderModule.html>
    #[inline]
    pub fn destroy_shader_module(
        &self,
        shader_module: ShaderModule,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_shader_module.unwrap())(
                self.handle,
                shader_module,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineCache.html>
    #[inline]
    pub fn create_pipeline_cache(
        &self,
        create_info: &PipelineCacheCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PipelineCache, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_pipeline_cache.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineCache.html>
    #[inline]
    pub fn destroy_pipeline_cache(
        &self,
        pipeline_cache: PipelineCache,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_pipeline_cache.unwrap())(
                self.handle,
                pipeline_cache,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPipelineCacheData.html>
    #[inline]
    pub fn get_pipeline_cache_data(
        &self,
        pipeline_cache: PipelineCache,
    ) -> Result<Vec<c_void>, Error> {
        read_into_vec_result(|count, data| unsafe {
            (self.fns().v1_0.get_pipeline_cache_data.unwrap())(
                self.handle,
                pipeline_cache,
                count,
                data,
            )
        })
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMergePipelineCaches.html>
    #[inline]
    pub fn merge_pipeline_caches(
        &self,
        dst_cache: PipelineCache,
        src_caches: &[PipelineCache],
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.merge_pipeline_caches.unwrap())(
                self.handle,
                dst_cache,
                src_caches.len() as u32,
                src_caches.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateGraphicsPipelines.html>
    #[inline]
    pub fn create_graphics_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[GraphicsPipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), Error> {
        assert_eq!(create_infos.len(), pipelines.len());
        unsafe {
            (self.fns().v1_0.create_graphics_pipelines.unwrap())(
                self.handle,
                pipeline_cache,
                create_infos.len() as u32,
                create_infos.as_ptr(),
                allocator.map_or(null(), from_ref),
                pipelines.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateComputePipelines.html>
    #[inline]
    pub fn create_compute_pipelines(
        &self,
        pipeline_cache: PipelineCache,
        create_infos: &[ComputePipelineCreateInfo],
        allocator: Option<&AllocationCallbacks>,
        pipelines: &mut [Pipeline],
    ) -> Result<(), Error> {
        assert_eq!(create_infos.len(), pipelines.len());
        unsafe {
            (self.fns().v1_0.create_compute_pipelines.unwrap())(
                self.handle,
                pipeline_cache,
                create_infos.len() as u32,
                create_infos.as_ptr(),
                allocator.map_or(null(), from_ref),
                pipelines.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipeline.html>
    #[inline]
    pub fn destroy_pipeline(&self, pipeline: Pipeline, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_pipeline.unwrap())(
                self.handle,
                pipeline,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePipelineLayout.html>
    #[inline]
    pub fn create_pipeline_layout(
        &self,
        create_info: &PipelineLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PipelineLayout, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_pipeline_layout.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPipelineLayout.html>
    #[inline]
    pub fn destroy_pipeline_layout(
        &self,
        pipeline_layout: PipelineLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_pipeline_layout.unwrap())(
                self.handle,
                pipeline_layout,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSampler.html>
    #[inline]
    pub fn create_sampler(
        &self,
        create_info: &SamplerCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Sampler, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_sampler.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySampler.html>
    #[inline]
    pub fn destroy_sampler(&self, sampler: Sampler, allocator: Option<&AllocationCallbacks>) {
        unsafe {
            (self.fns().v1_0.destroy_sampler.unwrap())(
                self.handle,
                sampler,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorSetLayout.html>
    #[inline]
    pub fn create_descriptor_set_layout(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DescriptorSetLayout, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_descriptor_set_layout.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorSetLayout.html>
    #[inline]
    pub fn destroy_descriptor_set_layout(
        &self,
        descriptor_set_layout: DescriptorSetLayout,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_descriptor_set_layout.unwrap())(
                self.handle,
                descriptor_set_layout,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorPool.html>
    #[inline]
    pub fn create_descriptor_pool(
        &self,
        create_info: &DescriptorPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DescriptorPool, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_descriptor_pool.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorPool.html>
    #[inline]
    pub fn destroy_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_descriptor_pool.unwrap())(
                self.handle,
                descriptor_pool,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetDescriptorPool.html>
    #[inline]
    pub fn reset_descriptor_pool(
        &self,
        descriptor_pool: DescriptorPool,
        flags: DescriptorPoolResetFlags,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.reset_descriptor_pool.unwrap())(self.handle, descriptor_pool, flags)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkAllocateDescriptorSets.html>
    #[inline]
    pub fn allocate_descriptor_sets(
        &self,
        allocate_info: &DescriptorSetAllocateInfo,
        descriptor_sets: &mut [DescriptorSet],
    ) -> Result<(), Error> {
        assert_eq!(
            descriptor_sets.len(),
            allocate_info.descriptor_set_count as usize
        );
        unsafe {
            (self.fns().v1_0.allocate_descriptor_sets.unwrap())(
                self.handle,
                allocate_info,
                descriptor_sets.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeDescriptorSets.html>
    #[inline]
    pub fn free_descriptor_sets(
        &self,
        descriptor_pool: DescriptorPool,
        descriptor_sets: &[DescriptorSet],
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.free_descriptor_sets.unwrap())(
                self.handle,
                descriptor_pool,
                descriptor_sets.len() as u32,
                descriptor_sets.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSets.html>
    #[inline]
    pub fn update_descriptor_sets(
        &self,
        descriptor_writes: &[WriteDescriptorSet],
        descriptor_copies: &[CopyDescriptorSet],
    ) {
        unsafe {
            (self.fns().v1_0.update_descriptor_sets.unwrap())(
                self.handle,
                descriptor_writes.len() as u32,
                descriptor_writes.as_ptr(),
                descriptor_copies.len() as u32,
                descriptor_copies.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateFramebuffer.html>
    #[inline]
    pub fn create_framebuffer(
        &self,
        create_info: &FramebufferCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<Framebuffer, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_framebuffer.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyFramebuffer.html>
    #[inline]
    pub fn destroy_framebuffer(
        &self,
        framebuffer: Framebuffer,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_framebuffer.unwrap())(
                self.handle,
                framebuffer,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass.html>
    #[inline]
    pub fn create_render_pass(
        &self,
        create_info: &RenderPassCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<RenderPass, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_render_pass.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyRenderPass.html>
    #[inline]
    pub fn destroy_render_pass(
        &self,
        render_pass: RenderPass,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_render_pass.unwrap())(
                self.handle,
                render_pass,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderAreaGranularity.html>
    #[inline]
    pub fn get_render_area_granularity(&self, render_pass: RenderPass) -> Extent2D {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_render_area_granularity.unwrap())(
                self.handle,
                render_pass,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetRenderingAreaGranularity.html>
    #[inline]
    pub fn get_rendering_area_granularity(
        &self,
        rendering_area_info: &RenderingAreaInfo,
    ) -> Extent2D {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_4.get_rendering_area_granularity.unwrap())(
                self.handle,
                rendering_area_info,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateCommandPool.html>
    #[inline]
    pub fn create_command_pool(
        &self,
        create_info: &CommandPoolCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<CommandPool, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.create_command_pool.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyCommandPool.html>
    #[inline]
    pub fn destroy_command_pool(
        &self,
        command_pool: CommandPool,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_0.destroy_command_pool.unwrap())(
                self.handle,
                command_pool,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkResetCommandPool.html>
    #[inline]
    pub fn reset_command_pool(
        &self,
        command_pool: CommandPool,
        flags: CommandPoolResetFlags,
    ) -> Result<(), Error> {
        unsafe { (self.fns().v1_0.reset_command_pool.unwrap())(self.handle, command_pool, flags) }
            .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkFreeCommandBuffers.html>
    #[inline]
    pub fn free_command_buffers(
        &self,
        command_pool: CommandPool,
        command_buffers: &[vkCommandBuffer],
    ) {
        unsafe {
            (self.fns().v1_0.free_command_buffers.unwrap())(
                self.handle,
                command_pool,
                command_buffers.len() as u32,
                command_buffers.as_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTrimCommandPool.html>
    #[inline]
    pub fn trim_command_pool(&self, command_pool: CommandPool, flags: CommandPoolTrimFlags) {
        unsafe { (self.fns().v1_1.trim_command_pool.unwrap())(self.handle, command_pool, flags) };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceGroupPeerMemoryFeatures.html>
    #[inline]
    pub fn get_device_group_peer_memory_features(
        &self,
        heap_index: u32,
        local_device_index: u32,
        remote_device_index: u32,
    ) -> PeerMemoryFeatureFlags {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_1
                .get_device_group_peer_memory_features
                .unwrap())(
                self.handle,
                heap_index,
                local_device_index,
                remote_device_index,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindBufferMemory2.html>
    #[inline]
    pub fn bind_buffer_memory2(&self, bind_infos: &[BindBufferMemoryInfo]) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_1.bind_buffer_memory2.unwrap())(
                self.handle,
                bind_infos.len() as u32,
                bind_infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkBindImageMemory2.html>
    #[inline]
    pub fn bind_image_memory2(&self, bind_infos: &[BindImageMemoryInfo]) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_1.bind_image_memory2.unwrap())(
                self.handle,
                bind_infos.len() as u32,
                bind_infos.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateDescriptorUpdateTemplate.html>
    #[inline]
    pub fn create_descriptor_update_template(
        &self,
        create_info: &DescriptorUpdateTemplateCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<DescriptorUpdateTemplate, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.create_descriptor_update_template.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyDescriptorUpdateTemplate.html>
    #[inline]
    pub fn destroy_descriptor_update_template(
        &self,
        descriptor_update_template: DescriptorUpdateTemplate,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_1.destroy_descriptor_update_template.unwrap())(
                self.handle,
                descriptor_update_template,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUpdateDescriptorSetWithTemplate.html>
    #[inline]
    pub fn update_descriptor_set_with_template(
        &self,
        descriptor_set: DescriptorSet,
        descriptor_update_template: DescriptorUpdateTemplate,
        data: &c_void,
    ) {
        unsafe {
            (self.fns().v1_1.update_descriptor_set_with_template.unwrap())(
                self.handle,
                descriptor_set,
                descriptor_update_template,
                data,
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferMemoryRequirements2.html>
    #[inline]
    pub fn get_buffer_memory_requirements2(
        &self,
        info: &BufferMemoryRequirementsInfo2,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.get_buffer_memory_requirements2.unwrap())(
                self.handle,
                info,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageMemoryRequirements2.html>
    #[inline]
    pub fn get_image_memory_requirements2(
        &self,
        info: &ImageMemoryRequirementsInfo2,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.get_image_memory_requirements2.unwrap())(
                self.handle,
                info,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSparseMemoryRequirements2.html>
    #[inline]
    pub fn get_image_sparse_memory_requirements2(
        &self,
        info: &ImageSparseMemoryRequirementsInfo2,
        sparse_memory_requirements: &mut [SparseImageMemoryRequirements2],
    ) {
        unsafe {
            (self
                .fns()
                .v1_1
                .get_image_sparse_memory_requirements2
                .unwrap())(
                self.handle,
                info,
                sparse_memory_requirements.len() as *mut u32,
                sparse_memory_requirements.as_mut_ptr(),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceBufferMemoryRequirements.html>
    #[inline]
    pub fn get_device_buffer_memory_requirements(
        &self,
        info: &DeviceBufferMemoryRequirements,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_3
                .get_device_buffer_memory_requirements
                .unwrap())(self.handle, info, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageMemoryRequirements.html>
    #[inline]
    pub fn get_device_image_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
    ) -> MemoryRequirements2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_3
                .get_device_image_memory_requirements
                .unwrap())(self.handle, info, out.as_mut_ptr());
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSparseMemoryRequirements.html>
    ///
    /// Call [`get_device_image_sparse_memory_requirements_len()`][`Self::get_device_image_sparse_memory_requirements_len()`] to query the number of elements to pass to `out`.
    #[inline]
    pub fn get_device_image_sparse_memory_requirements(
        &self,
        info: &DeviceImageMemoryRequirements,
        sparse_memory_requirements: &mut [SparseImageMemoryRequirements2],
    ) {
        unsafe {
            (self
                .fns()
                .v1_3
                .get_device_image_sparse_memory_requirements
                .unwrap())(
                self.handle,
                info,
                sparse_memory_requirements.len() as *mut u32,
                sparse_memory_requirements.as_mut_ptr(),
            )
        };
    }

    /// Returns the required slice length for Call [`get_device_image_sparse_memory_requirements`][`Self::get_device_image_sparse_memory_requirements`].
    #[inline]
    pub fn get_device_image_sparse_memory_requirements_len(
        &self,
        info: &DeviceImageMemoryRequirements,
    ) -> usize {
        let mut out: MaybeUninit<u32> = MaybeUninit::uninit();
        unsafe {
            (self
                .fns()
                .v1_3
                .get_device_image_sparse_memory_requirements
                .unwrap())(self.handle, info, out.as_mut_ptr(), ptr::null_mut());
            out.assume_init() as usize
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateSamplerYcbcrConversion.html>
    #[inline]
    pub fn create_sampler_ycbcr_conversion(
        &self,
        create_info: &SamplerYcbcrConversionCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<SamplerYcbcrConversion, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.create_sampler_ycbcr_conversion.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroySamplerYcbcrConversion.html>
    #[inline]
    pub fn destroy_sampler_ycbcr_conversion(
        &self,
        ycbcr_conversion: SamplerYcbcrConversion,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_1.destroy_sampler_ycbcr_conversion.unwrap())(
                self.handle,
                ycbcr_conversion,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDescriptorSetLayoutSupport.html>
    #[inline]
    pub fn get_descriptor_set_layout_support(
        &self,
        create_info: &DescriptorSetLayoutCreateInfo,
    ) -> DescriptorSetLayoutSupport<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_1.get_descriptor_set_layout_support.unwrap())(
                self.handle,
                create_info,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreateRenderPass2.html>
    #[inline]
    pub fn create_render_pass2(
        &self,
        create_info: &RenderPassCreateInfo2,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<RenderPass, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_2.create_render_pass2.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetSemaphoreCounterValue.html>
    #[inline]
    pub fn get_semaphore_counter_value(&self, semaphore: Semaphore) -> Result<u64, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_2.get_semaphore_counter_value.unwrap())(
                self.handle,
                semaphore,
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkWaitSemaphores.html>
    #[inline]
    pub fn wait_semaphores(
        &self,
        wait_info: &SemaphoreWaitInfo,
        timeout: u64,
    ) -> Result<(), Error> {
        unsafe { (self.fns().v1_2.wait_semaphores.unwrap())(self.handle, wait_info, timeout) }
            .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSignalSemaphore.html>
    #[inline]
    pub fn signal_semaphore(&self, signal_info: &SemaphoreSignalInfo) -> Result<(), Error> {
        unsafe { (self.fns().v1_2.signal_semaphore.unwrap())(self.handle, signal_info) }.result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferOpaqueCaptureAddress.html>
    #[inline]
    pub fn get_buffer_opaque_capture_address(&self, info: &BufferDeviceAddressInfo) -> u64 {
        unsafe { (self.fns().v1_2.get_buffer_opaque_capture_address.unwrap())(self.handle, info) }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetBufferDeviceAddress.html>
    #[inline]
    pub fn get_buffer_device_address(&self, info: &BufferDeviceAddressInfo) -> DeviceAddress {
        unsafe { (self.fns().v1_2.get_buffer_device_address.unwrap())(self.handle, info) }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceMemoryOpaqueCaptureAddress.html>
    #[inline]
    pub fn get_device_memory_opaque_capture_address(
        &self,
        info: &DeviceMemoryOpaqueCaptureAddressInfo,
    ) -> u64 {
        unsafe {
            (self
                .fns()
                .v1_2
                .get_device_memory_opaque_capture_address
                .unwrap())(self.handle, info)
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetFaultData.html>
    #[inline]
    pub fn get_fault_data(
        &self,
        fault_query_behavior: FaultQueryBehavior,
        unrecorded_faults: *mut Bool32,
        faults: &mut [FaultData],
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_0.get_fault_data.unwrap())(
                self.handle,
                fault_query_behavior,
                unrecorded_faults,
                faults.len() as *mut u32,
                faults.as_mut_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCreatePrivateDataSlot.html>
    #[inline]
    pub fn create_private_data_slot(
        &self,
        create_info: &PrivateDataSlotCreateInfo,
        allocator: Option<&AllocationCallbacks>,
    ) -> Result<PrivateDataSlot, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_3.create_private_data_slot.unwrap())(
                self.handle,
                create_info,
                allocator.map_or(null(), from_ref),
                out.as_mut_ptr(),
            )
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkDestroyPrivateDataSlot.html>
    #[inline]
    pub fn destroy_private_data_slot(
        &self,
        private_data_slot: PrivateDataSlot,
        allocator: Option<&AllocationCallbacks>,
    ) {
        unsafe {
            (self.fns().v1_3.destroy_private_data_slot.unwrap())(
                self.handle,
                private_data_slot,
                allocator.map_or(null(), from_ref),
            )
        };
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkSetPrivateData.html>
    #[inline]
    pub fn set_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
        data: u64,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_3.set_private_data.unwrap())(
                self.handle,
                object_type,
                object_handle,
                private_data_slot,
                data,
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetPrivateData.html>
    #[inline]
    pub fn get_private_data(
        &self,
        object_type: ObjectType,
        object_handle: u64,
        private_data_slot: PrivateDataSlot,
    ) -> u64 {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_3.get_private_data.unwrap())(
                self.handle,
                object_type,
                object_handle,
                private_data_slot,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyMemoryToImage.html>
    #[inline]
    pub fn copy_memory_to_image(
        &self,
        copy_memory_to_image_info: &CopyMemoryToImageInfo,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_4.copy_memory_to_image.unwrap())(self.handle, copy_memory_to_image_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToMemory.html>
    #[inline]
    pub fn copy_image_to_memory(
        &self,
        copy_image_to_memory_info: &CopyImageToMemoryInfo,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_4.copy_image_to_memory.unwrap())(self.handle, copy_image_to_memory_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkCopyImageToImage.html>
    #[inline]
    pub fn copy_image_to_image(
        &self,
        copy_image_to_image_info: &CopyImageToImageInfo,
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_4.copy_image_to_image.unwrap())(self.handle, copy_image_to_image_info)
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkTransitionImageLayout.html>
    #[inline]
    pub fn transition_image_layout(
        &self,
        transitions: &[HostImageLayoutTransitionInfo],
    ) -> Result<(), Error> {
        unsafe {
            (self.fns().v1_4.transition_image_layout.unwrap())(
                self.handle,
                transitions.len() as u32,
                transitions.as_ptr(),
            )
        }
        .result()
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetCommandPoolMemoryConsumption.html>
    #[inline]
    pub fn get_command_pool_memory_consumption(
        &self,
        command_pool: CommandPool,
        command_buffer: vkCommandBuffer,
    ) -> CommandPoolMemoryConsumption<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_0.get_command_pool_memory_consumption.unwrap())(
                self.handle,
                command_pool,
                command_buffer,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetImageSubresourceLayout2.html>
    #[inline]
    pub fn get_image_subresource_layout2(
        &self,
        image: Image,
        subresource: &ImageSubresource2,
    ) -> SubresourceLayout2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_4.get_image_subresource_layout2.unwrap())(
                self.handle,
                image,
                subresource,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetDeviceImageSubresourceLayout.html>
    #[inline]
    pub fn get_device_image_subresource_layout(
        &self,
        info: &DeviceImageSubresourceInfo,
    ) -> SubresourceLayout2<'_> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_4.get_device_image_subresource_layout.unwrap())(
                self.handle,
                info,
                out.as_mut_ptr(),
            );
            out.assume_init()
        }
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkMapMemory2.html>
    #[inline]
    pub fn map_memory2(&self, memory_map_info: &MemoryMapInfo) -> Result<*mut c_void, Error> {
        let mut out = MaybeUninit::uninit();
        unsafe {
            (self.fns().v1_4.map_memory2.unwrap())(self.handle, memory_map_info, out.as_mut_ptr())
        }
        .init_on_success(out)
    }

    /// <https://docs.vulkan.org/refpages/latest/refpages/source/vkUnmapMemory2.html>
    #[inline]
    pub fn unmap_memory2(&self, memory_unmap_info: &MemoryUnmapInfo) -> Result<(), Error> {
        unsafe { (self.fns().v1_4.unmap_memory2.unwrap())(self.handle, memory_unmap_info) }.result()
    }
}
