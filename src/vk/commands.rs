// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/commands.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![allow(non_camel_case_types)]
#![allow(unused)]
use super::bitflags::*;
use super::enums::*;
use super::platform_types::*;
use super::types::*;
use core::ffi::{c_char, c_int, c_void};

// ── fn-pointer Typen ──────────────────────────────────────────────
pub type vkCreateInstance = unsafe extern "system" fn(
    create_info: *const InstanceCreateInfo,
    allocator: *const AllocationCallbacks,
    instance: *mut vkInstance,
) -> vkResult;

pub type vkDestroyInstance =
    unsafe extern "system" fn(instance: vkInstance, allocator: *const AllocationCallbacks);

pub type vkEnumeratePhysicalDevices = unsafe extern "system" fn(
    instance: vkInstance,
    physical_device_count: *mut u32,
    physical_devices: *mut vkPhysicalDevice,
) -> vkResult;

pub type vkGetDeviceProcAddr =
    unsafe extern "system" fn(device: vkDevice, name: *const c_char) -> PFN_vkVoidFunction;

pub type vkGetInstanceProcAddr =
    unsafe extern "system" fn(instance: vkInstance, name: *const c_char) -> PFN_vkVoidFunction;

pub type vkGetPhysicalDeviceProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    properties: *mut PhysicalDeviceProperties,
);

pub type vkGetPhysicalDeviceQueueFamilyProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties,
);

pub type vkGetPhysicalDeviceMemoryProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    memory_properties: *mut PhysicalDeviceMemoryProperties,
);

pub type vkGetPhysicalDeviceFeatures = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    features: *mut PhysicalDeviceFeatures,
);

pub type vkGetPhysicalDeviceFormatProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format: Format,
    format_properties: *mut FormatProperties,
);

pub type vkGetPhysicalDeviceImageFormatProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    image_format_properties: *mut ImageFormatProperties,
) -> vkResult;

pub type vkCreateDevice = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    create_info: *const DeviceCreateInfo,
    allocator: *const AllocationCallbacks,
    device: *mut vkDevice,
) -> vkResult;

pub type vkDestroyDevice =
    unsafe extern "system" fn(device: vkDevice, allocator: *const AllocationCallbacks);

pub type vkEnumerateInstanceVersion = unsafe extern "system" fn(api_version: *mut u32) -> vkResult;

pub type vkEnumerateInstanceLayerProperties = unsafe extern "system" fn(
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> vkResult;

pub type vkEnumerateInstanceExtensionProperties = unsafe extern "system" fn(
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> vkResult;

pub type vkEnumerateDeviceLayerProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut LayerProperties,
) -> vkResult;

pub type vkEnumerateDeviceExtensionProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    layer_name: *const c_char,
    property_count: *mut u32,
    properties: *mut ExtensionProperties,
) -> vkResult;

pub type vkGetDeviceQueue = unsafe extern "system" fn(
    device: vkDevice,
    queue_family_index: u32,
    queue_index: u32,
    queue: *mut vkQueue,
);

pub type vkQueueSubmit = unsafe extern "system" fn(
    queue: vkQueue,
    submit_count: u32,
    submits: *const SubmitInfo,
    fence: Fence,
) -> vkResult;

pub type vkQueueWaitIdle = unsafe extern "system" fn(queue: vkQueue) -> vkResult;

pub type vkDeviceWaitIdle = unsafe extern "system" fn(device: vkDevice) -> vkResult;

pub type vkAllocateMemory = unsafe extern "system" fn(
    device: vkDevice,
    allocate_info: *const MemoryAllocateInfo,
    allocator: *const AllocationCallbacks,
    memory: *mut DeviceMemory,
) -> vkResult;

pub type vkFreeMemory = unsafe extern "system" fn(
    device: vkDevice,
    memory: DeviceMemory,
    allocator: *const AllocationCallbacks,
);

pub type vkMapMemory = unsafe extern "system" fn(
    device: vkDevice,
    memory: DeviceMemory,
    offset: DeviceSize,
    size: DeviceSize,
    flags: MemoryMapFlags,
    pp_data: *mut *mut c_void,
) -> vkResult;

pub type vkUnmapMemory = unsafe extern "system" fn(device: vkDevice, memory: DeviceMemory);

pub type vkFlushMappedMemoryRanges = unsafe extern "system" fn(
    device: vkDevice,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> vkResult;

pub type vkInvalidateMappedMemoryRanges = unsafe extern "system" fn(
    device: vkDevice,
    memory_range_count: u32,
    memory_ranges: *const MappedMemoryRange,
) -> vkResult;

pub type vkGetDeviceMemoryCommitment = unsafe extern "system" fn(
    device: vkDevice,
    memory: DeviceMemory,
    committed_memory_in_bytes: *mut DeviceSize,
);

pub type vkGetBufferMemoryRequirements = unsafe extern "system" fn(
    device: vkDevice,
    buffer: Buffer,
    memory_requirements: *mut MemoryRequirements,
);

pub type vkBindBufferMemory = unsafe extern "system" fn(
    device: vkDevice,
    buffer: Buffer,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> vkResult;

pub type vkGetImageMemoryRequirements = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    memory_requirements: *mut MemoryRequirements,
);

pub type vkBindImageMemory = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    memory: DeviceMemory,
    memory_offset: DeviceSize,
) -> vkResult;

pub type vkGetImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements,
);

pub type vkGetPhysicalDeviceSparseImageFormatProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format: Format,
    ty: ImageType,
    samples: SampleCountFlags,
    usage: ImageUsageFlags,
    tiling: ImageTiling,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties,
);

pub type vkQueueBindSparse = unsafe extern "system" fn(
    queue: vkQueue,
    bind_info_count: u32,
    bind_info: *const BindSparseInfo,
    fence: Fence,
) -> vkResult;

pub type vkCreateFence = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const FenceCreateInfo,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> vkResult;

pub type vkDestroyFence = unsafe extern "system" fn(
    device: vkDevice,
    fence: Fence,
    allocator: *const AllocationCallbacks,
);

pub type vkResetFences =
    unsafe extern "system" fn(device: vkDevice, fence_count: u32, fences: *const Fence) -> vkResult;

pub type vkGetFenceStatus = unsafe extern "system" fn(device: vkDevice, fence: Fence) -> vkResult;

pub type vkWaitForFences = unsafe extern "system" fn(
    device: vkDevice,
    fence_count: u32,
    fences: *const Fence,
    wait_all: Bool32,
    timeout: u64,
) -> vkResult;

pub type vkCreateSemaphore = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const SemaphoreCreateInfo,
    allocator: *const AllocationCallbacks,
    semaphore: *mut Semaphore,
) -> vkResult;

pub type vkDestroySemaphore = unsafe extern "system" fn(
    device: vkDevice,
    semaphore: Semaphore,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateEvent = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const EventCreateInfo,
    allocator: *const AllocationCallbacks,
    event: *mut Event,
) -> vkResult;

pub type vkDestroyEvent = unsafe extern "system" fn(
    device: vkDevice,
    event: Event,
    allocator: *const AllocationCallbacks,
);

pub type vkGetEventStatus = unsafe extern "system" fn(device: vkDevice, event: Event) -> vkResult;

pub type vkSetEvent = unsafe extern "system" fn(device: vkDevice, event: Event) -> vkResult;

pub type vkResetEvent = unsafe extern "system" fn(device: vkDevice, event: Event) -> vkResult;

pub type vkCreateQueryPool = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const QueryPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    query_pool: *mut QueryPool,
) -> vkResult;

pub type vkDestroyQueryPool = unsafe extern "system" fn(
    device: vkDevice,
    query_pool: QueryPool,
    allocator: *const AllocationCallbacks,
);

pub type vkGetQueryPoolResults = unsafe extern "system" fn(
    device: vkDevice,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    data_size: usize,
    data: *mut c_void,
    stride: DeviceSize,
    flags: QueryResultFlags,
) -> vkResult;

pub type vkResetQueryPool = unsafe extern "system" fn(
    device: vkDevice,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);

pub type vkResetQueryPoolEXT = unsafe extern "system" fn(
    device: vkDevice,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);

pub type vkCreateBuffer = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const BufferCreateInfo,
    allocator: *const AllocationCallbacks,
    buffer: *mut Buffer,
) -> vkResult;

pub type vkDestroyBuffer = unsafe extern "system" fn(
    device: vkDevice,
    buffer: Buffer,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateBufferView = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const BufferViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut BufferView,
) -> vkResult;

pub type vkDestroyBufferView = unsafe extern "system" fn(
    device: vkDevice,
    buffer_view: BufferView,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateImage = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ImageCreateInfo,
    allocator: *const AllocationCallbacks,
    image: *mut Image,
) -> vkResult;

pub type vkDestroyImage = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    allocator: *const AllocationCallbacks,
);

pub type vkGetImageSubresourceLayout = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    subresource: *const ImageSubresource,
    layout: *mut SubresourceLayout,
);

pub type vkCreateImageView = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ImageViewCreateInfo,
    allocator: *const AllocationCallbacks,
    view: *mut ImageView,
) -> vkResult;

pub type vkDestroyImageView = unsafe extern "system" fn(
    device: vkDevice,
    image_view: ImageView,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateShaderModule = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ShaderModuleCreateInfo,
    allocator: *const AllocationCallbacks,
    shader_module: *mut ShaderModule,
) -> vkResult;

pub type vkDestroyShaderModule = unsafe extern "system" fn(
    device: vkDevice,
    shader_module: ShaderModule,
    allocator: *const AllocationCallbacks,
);

pub type vkCreatePipelineCache = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const PipelineCacheCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_cache: *mut PipelineCache,
) -> vkResult;

pub type vkDestroyPipelineCache = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_cache: PipelineCache,
    allocator: *const AllocationCallbacks,
);

pub type vkGetPipelineCacheData = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_cache: PipelineCache,
    data_size: *mut usize,
    data: *mut c_void,
) -> vkResult;

pub type vkMergePipelineCaches = unsafe extern "system" fn(
    device: vkDevice,
    dst_cache: PipelineCache,
    src_cache_count: u32,
    src_caches: *const PipelineCache,
) -> vkResult;

pub type vkCreatePipelineBinariesKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const PipelineBinaryCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    binaries: *mut PipelineBinaryHandlesInfoKHR,
) -> vkResult;

pub type vkDestroyPipelineBinaryKHR = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_binary: PipelineBinaryKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkGetPipelineKeyKHR = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_create_info: *const PipelineCreateInfoKHR,
    pipeline_key: *mut PipelineBinaryKeyKHR,
) -> vkResult;

pub type vkGetPipelineBinaryDataKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const PipelineBinaryDataInfoKHR,
    pipeline_binary_key: *mut PipelineBinaryKeyKHR,
    pipeline_binary_data_size: *mut usize,
    pipeline_binary_data: *mut c_void,
) -> vkResult;

pub type vkReleaseCapturedPipelineDataKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ReleaseCapturedPipelineDataInfoKHR,
    allocator: *const AllocationCallbacks,
) -> vkResult;

pub type vkCreateGraphicsPipelines = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const GraphicsPipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> vkResult;

pub type vkCreateComputePipelines = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const ComputePipelineCreateInfo,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> vkResult;

pub type vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI = unsafe extern "system" fn(
    device: vkDevice,
    renderpass: RenderPass,
    max_workgroup_size: *mut Extent2D,
) -> vkResult;

pub type vkDestroyPipeline = unsafe extern "system" fn(
    device: vkDevice,
    pipeline: Pipeline,
    allocator: *const AllocationCallbacks,
);

pub type vkCreatePipelineLayout = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const PipelineLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    pipeline_layout: *mut PipelineLayout,
) -> vkResult;

pub type vkDestroyPipelineLayout = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_layout: PipelineLayout,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateSampler = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const SamplerCreateInfo,
    allocator: *const AllocationCallbacks,
    sampler: *mut Sampler,
) -> vkResult;

pub type vkDestroySampler = unsafe extern "system" fn(
    device: vkDevice,
    sampler: Sampler,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateDescriptorSetLayout = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DescriptorSetLayoutCreateInfo,
    allocator: *const AllocationCallbacks,
    set_layout: *mut DescriptorSetLayout,
) -> vkResult;

pub type vkDestroyDescriptorSetLayout = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_set_layout: DescriptorSetLayout,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateDescriptorPool = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DescriptorPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_pool: *mut DescriptorPool,
) -> vkResult;

pub type vkDestroyDescriptorPool = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_pool: DescriptorPool,
    allocator: *const AllocationCallbacks,
);

pub type vkResetDescriptorPool = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_pool: DescriptorPool,
    flags: DescriptorPoolResetFlags,
) -> vkResult;

pub type vkAllocateDescriptorSets = unsafe extern "system" fn(
    device: vkDevice,
    allocate_info: *const DescriptorSetAllocateInfo,
    descriptor_sets: *mut DescriptorSet,
) -> vkResult;

pub type vkFreeDescriptorSets = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_pool: DescriptorPool,
    descriptor_set_count: u32,
    descriptor_sets: *const DescriptorSet,
) -> vkResult;

pub type vkUpdateDescriptorSets = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
    descriptor_copy_count: u32,
    descriptor_copies: *const CopyDescriptorSet,
);

pub type vkCreateFramebuffer = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const FramebufferCreateInfo,
    allocator: *const AllocationCallbacks,
    framebuffer: *mut Framebuffer,
) -> vkResult;

pub type vkDestroyFramebuffer = unsafe extern "system" fn(
    device: vkDevice,
    framebuffer: Framebuffer,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateRenderPass = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const RenderPassCreateInfo,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> vkResult;

pub type vkDestroyRenderPass = unsafe extern "system" fn(
    device: vkDevice,
    render_pass: RenderPass,
    allocator: *const AllocationCallbacks,
);

pub type vkGetRenderAreaGranularity = unsafe extern "system" fn(
    device: vkDevice,
    render_pass: RenderPass,
    granularity: *mut Extent2D,
);

pub type vkGetRenderingAreaGranularity = unsafe extern "system" fn(
    device: vkDevice,
    rendering_area_info: *const RenderingAreaInfo,
    granularity: *mut Extent2D,
);

pub type vkGetRenderingAreaGranularityKHR = unsafe extern "system" fn(
    device: vkDevice,
    rendering_area_info: *const RenderingAreaInfo,
    granularity: *mut Extent2D,
);

pub type vkCreateCommandPool = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const CommandPoolCreateInfo,
    allocator: *const AllocationCallbacks,
    command_pool: *mut CommandPool,
) -> vkResult;

pub type vkDestroyCommandPool = unsafe extern "system" fn(
    device: vkDevice,
    command_pool: CommandPool,
    allocator: *const AllocationCallbacks,
);

pub type vkResetCommandPool = unsafe extern "system" fn(
    device: vkDevice,
    command_pool: CommandPool,
    flags: CommandPoolResetFlags,
) -> vkResult;

pub type vkAllocateCommandBuffers = unsafe extern "system" fn(
    device: vkDevice,
    allocate_info: *const CommandBufferAllocateInfo,
    command_buffers: *mut vkCommandBuffer,
) -> vkResult;

pub type vkFreeCommandBuffers = unsafe extern "system" fn(
    device: vkDevice,
    command_pool: CommandPool,
    command_buffer_count: u32,
    command_buffers: *const vkCommandBuffer,
);

pub type vkBeginCommandBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    begin_info: *const CommandBufferBeginInfo,
) -> vkResult;

pub type vkEndCommandBuffer =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer) -> vkResult;

pub type vkResetCommandBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    flags: CommandBufferResetFlags,
) -> vkResult;

pub type vkCmdBindPipeline = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);

pub type vkCmdSetAttachmentFeedbackLoopEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, aspect_mask: ImageAspectFlags);

pub type vkCmdSetViewport = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewports: *const Viewport,
);

pub type vkCmdSetScissor = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_scissor: u32,
    scissor_count: u32,
    scissors: *const Rect2D,
);

pub type vkCmdSetLineWidth =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, line_width: f32);

pub type vkCmdSetDepthBias = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    depth_bias_constant_factor: f32,
    depth_bias_clamp: f32,
    depth_bias_slope_factor: f32,
);

pub type vkCmdSetBlendConstants =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, blend_constants: f32);

pub type vkCmdSetDepthBounds = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    min_depth_bounds: f32,
    max_depth_bounds: f32,
);

pub type vkCmdSetStencilCompareMask = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    face_mask: StencilFaceFlags,
    compare_mask: u32,
);

pub type vkCmdSetStencilWriteMask = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    face_mask: StencilFaceFlags,
    write_mask: u32,
);

pub type vkCmdSetStencilReference = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    face_mask: StencilFaceFlags,
    reference: u32,
);

pub type vkCmdBindDescriptorSets = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    descriptor_set_count: u32,
    descriptor_sets: *const DescriptorSet,
    dynamic_offset_count: u32,
    dynamic_offsets: *const u32,
);

pub type vkCmdBindIndexBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    index_type: IndexType,
);

pub type vkCmdBindVertexBuffers = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
);

pub type vkCmdDraw = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    vertex_count: u32,
    instance_count: u32,
    first_vertex: u32,
    first_instance: u32,
);

pub type vkCmdDrawIndexed = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    index_count: u32,
    instance_count: u32,
    first_index: u32,
    vertex_offset: i32,
    first_instance: u32,
);

pub type vkCmdDrawMultiEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    draw_count: u32,
    vertex_info: *const MultiDrawInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
);

pub type vkCmdDrawMultiIndexedEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    draw_count: u32,
    index_info: *const MultiDrawIndexedInfoEXT,
    instance_count: u32,
    first_instance: u32,
    stride: u32,
    vertex_offset: *const i32,
);

pub type vkCmdDrawIndirect = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawIndexedIndirect = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type vkCmdDispatch = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type vkCmdDispatchIndirect =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, buffer: Buffer, offset: DeviceSize);

pub type vkCmdSubpassShadingHUAWEI = unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkCmdDrawClusterHUAWEI = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type vkCmdDrawClusterIndirectHUAWEI =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, buffer: Buffer, offset: DeviceSize);

pub type vkCmdUpdatePipelineIndirectBufferNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
);

pub type vkCmdCopyBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_buffer: Buffer,
    dst_buffer: Buffer,
    region_count: u32,
    regions: *const BufferCopy,
);

pub type vkCmdCopyImage = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageCopy,
);

pub type vkCmdBlitImage = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageBlit,
    filter: Filter,
);

pub type vkCmdCopyBufferToImage = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_buffer: Buffer,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const BufferImageCopy,
);

pub type vkCmdCopyImageToBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_buffer: Buffer,
    region_count: u32,
    regions: *const BufferImageCopy,
);

pub type vkCmdCopyMemoryIndirectNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
);

pub type vkCmdCopyMemoryIndirectKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_memory_indirect_info: *const CopyMemoryIndirectInfoKHR,
);

pub type vkCmdCopyMemoryToImageIndirectNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_buffer_address: DeviceAddress,
    copy_count: u32,
    stride: u32,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    image_subresources: *const ImageSubresourceLayers,
);

pub type vkCmdCopyMemoryToImageIndirectKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_memory_to_image_indirect_info: *const CopyMemoryToImageIndirectInfoKHR,
);

pub type vkCmdUpdateBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    data_size: DeviceSize,
    data: *const c_void,
);

pub type vkCmdFillBuffer = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    size: DeviceSize,
    data: u32,
);

pub type vkCmdClearColorImage = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    color: *const ClearColorValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
);

pub type vkCmdClearDepthStencilImage = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    image: Image,
    image_layout: ImageLayout,
    depth_stencil: *const ClearDepthStencilValue,
    range_count: u32,
    ranges: *const ImageSubresourceRange,
);

pub type vkCmdClearAttachments = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    attachment_count: u32,
    attachments: *const ClearAttachment,
    rect_count: u32,
    rects: *const ClearRect,
);

pub type vkCmdResolveImage = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_image: Image,
    src_image_layout: ImageLayout,
    dst_image: Image,
    dst_image_layout: ImageLayout,
    region_count: u32,
    regions: *const ImageResolve,
);

pub type vkCmdSetEvent = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);

pub type vkCmdResetEvent = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags,
);

pub type vkCmdWaitEvents = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event_count: u32,
    events: *const Event,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    memory_barrier_count: u32,
    memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    image_memory_barriers: *const ImageMemoryBarrier,
);

pub type vkCmdPipelineBarrier = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    src_stage_mask: PipelineStageFlags,
    dst_stage_mask: PipelineStageFlags,
    dependency_flags: DependencyFlags,
    memory_barrier_count: u32,
    memory_barriers: *const MemoryBarrier,
    buffer_memory_barrier_count: u32,
    buffer_memory_barriers: *const BufferMemoryBarrier,
    image_memory_barrier_count: u32,
    image_memory_barriers: *const ImageMemoryBarrier,
);

pub type vkCmdBeginQuery = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
);

pub type vkCmdEndQuery =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, query_pool: QueryPool, query: u32);

pub type vkCmdBeginConditionalRenderingEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    conditional_rendering_begin: *const ConditionalRenderingBeginInfoEXT,
);

pub type vkCmdEndConditionalRenderingEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkCmdBeginCustomResolveEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    begin_custom_resolve_info: *const BeginCustomResolveInfoEXT,
);

pub type vkCmdResetQueryPool = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
);

pub type vkCmdWriteTimestamp = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_stage: PipelineStageFlags,
    query_pool: QueryPool,
    query: u32,
);

pub type vkCmdCopyQueryPoolResults = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    query_pool: QueryPool,
    first_query: u32,
    query_count: u32,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    stride: DeviceSize,
    flags: QueryResultFlags,
);

pub type vkCmdPushConstants = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    layout: PipelineLayout,
    stage_flags: ShaderStageFlags,
    offset: u32,
    size: u32,
    values: *const c_void,
);

pub type vkCmdBeginRenderPass = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    render_pass_begin: *const RenderPassBeginInfo,
    contents: SubpassContents,
);

pub type vkCmdNextSubpass =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, contents: SubpassContents);

pub type vkCmdEndRenderPass = unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkCmdExecuteCommands = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    command_buffer_count: u32,
    command_buffers: *const vkCommandBuffer,
);

pub type vkCreateAndroidSurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const AndroidSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateSurfaceOHOS = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const SurfaceCreateInfoOHOS,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceDisplayPropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayPropertiesKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceDisplayPlanePropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayPlanePropertiesKHR,
) -> vkResult;

pub type vkGetDisplayPlaneSupportedDisplaysKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    plane_index: u32,
    display_count: *mut u32,
    displays: *mut DisplayKHR,
) -> vkResult;

pub type vkGetDisplayModePropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    display: DisplayKHR,
    property_count: *mut u32,
    properties: *mut DisplayModePropertiesKHR,
) -> vkResult;

pub type vkCreateDisplayModeKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    display: DisplayKHR,
    create_info: *const DisplayModeCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    mode: *mut DisplayModeKHR,
) -> vkResult;

pub type vkGetDisplayPlaneCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    mode: DisplayModeKHR,
    plane_index: u32,
    capabilities: *mut DisplayPlaneCapabilitiesKHR,
) -> vkResult;

pub type vkCreateDisplayPlaneSurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const DisplaySurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateSharedSwapchainsKHR = unsafe extern "system" fn(
    device: vkDevice,
    swapchain_count: u32,
    create_infos: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchains: *mut SwapchainKHR,
) -> vkResult;

pub type vkDestroySurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    surface: SurfaceKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkGetPhysicalDeviceSurfaceSupportKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_index: u32,
    surface: SurfaceKHR,
    supported: *mut Bool32,
) -> vkResult;

pub type vkGetPhysicalDeviceSurfaceCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface: SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilitiesKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceSurfaceFormatsKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface: SurfaceKHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormatKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceSurfacePresentModesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface: SurfaceKHR,
    present_mode_count: *mut u32,
    present_modes: *mut PresentModeKHR,
) -> vkResult;

pub type vkCreateSwapchainKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const SwapchainCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    swapchain: *mut SwapchainKHR,
) -> vkResult;

pub type vkDestroySwapchainKHR = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkGetSwapchainImagesKHR = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    swapchain_image_count: *mut u32,
    swapchain_images: *mut Image,
) -> vkResult;

pub type vkAcquireNextImageKHR = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    timeout: u64,
    semaphore: Semaphore,
    fence: Fence,
    image_index: *mut u32,
) -> vkResult;

pub type vkQueuePresentKHR =
    unsafe extern "system" fn(queue: vkQueue, present_info: *const PresentInfoKHR) -> vkResult;

pub type vkCreateViSurfaceNN = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const ViSurfaceCreateInfoNN,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateWaylandSurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const WaylandSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceWaylandPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_index: u32,
    display: *mut wl_display,
) -> Bool32;

pub type vkCreateWin32SurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const Win32SurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceWin32PresentationSupportKHR =
    unsafe extern "system" fn(physical_device: vkPhysicalDevice, queue_family_index: u32) -> Bool32;

pub type vkCreateXlibSurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const XlibSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceXlibPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_index: u32,
    dpy: *mut Display,
    visual_id: VisualID,
) -> Bool32;

pub type vkCreateXcbSurfaceKHR = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const XcbSurfaceCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceXcbPresentationSupportKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_index: u32,
    connection: *mut xcb_connection_t,
    visual_id: xcb_visualid_t,
) -> Bool32;

pub type vkCreateDirectFBSurfaceEXT = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const DirectFBSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceDirectFBPresentationSupportEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_index: u32,
    dfb: *mut IDirectFB,
) -> Bool32;

pub type vkCreateImagePipeSurfaceFUCHSIA = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const ImagePipeSurfaceCreateInfoFUCHSIA,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateStreamDescriptorSurfaceGGP = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const StreamDescriptorSurfaceCreateInfoGGP,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateScreenSurfaceQNX = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const ScreenSurfaceCreateInfoQNX,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceScreenPresentationSupportQNX = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_index: u32,
    window: *mut _screen_window,
) -> Bool32;

pub type vkCreateDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const DebugReportCallbackCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    callback: *mut DebugReportCallbackEXT,
) -> vkResult;

pub type vkDestroyDebugReportCallbackEXT = unsafe extern "system" fn(
    instance: vkInstance,
    callback: DebugReportCallbackEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkDebugReportMessageEXT = unsafe extern "system" fn(
    instance: vkInstance,
    flags: DebugReportFlagsEXT,
    object_type: DebugReportObjectTypeEXT,
    object: u64,
    location: usize,
    message_code: i32,
    layer_prefix: *const c_char,
    message: *const c_char,
);

pub type vkDebugMarkerSetObjectNameEXT = unsafe extern "system" fn(
    device: vkDevice,
    name_info: *const DebugMarkerObjectNameInfoEXT,
) -> vkResult;

pub type vkDebugMarkerSetObjectTagEXT = unsafe extern "system" fn(
    device: vkDevice,
    tag_info: *const DebugMarkerObjectTagInfoEXT,
) -> vkResult;

pub type vkCmdDebugMarkerBeginEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    marker_info: *const DebugMarkerMarkerInfoEXT,
);

pub type vkCmdDebugMarkerEndEXT = unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkCmdDebugMarkerInsertEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    marker_info: *const DebugMarkerMarkerInfoEXT,
);

pub type vkGetPhysicalDeviceExternalImageFormatPropertiesNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format: Format,
    ty: ImageType,
    tiling: ImageTiling,
    usage: ImageUsageFlags,
    flags: ImageCreateFlags,
    external_handle_type: ExternalMemoryHandleTypeFlagsNV,
    external_image_format_properties: *mut ExternalImageFormatPropertiesNV,
)
    -> vkResult;

pub type vkGetMemoryWin32HandleNV = unsafe extern "system" fn(
    device: vkDevice,
    memory: DeviceMemory,
    handle_type: ExternalMemoryHandleTypeFlagsNV,
    handle: *mut HANDLE,
) -> vkResult;

pub type vkCmdExecuteGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    is_preprocessed: Bool32,
    generated_commands_info: *const GeneratedCommandsInfoNV,
);

pub type vkCmdPreprocessGeneratedCommandsNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    generated_commands_info: *const GeneratedCommandsInfoNV,
);

pub type vkCmdBindPipelineShaderGroupNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    pipeline: Pipeline,
    group_index: u32,
);

pub type vkGetGeneratedCommandsMemoryRequirementsNV = unsafe extern "system" fn(
    device: vkDevice,
    info: *const GeneratedCommandsMemoryRequirementsInfoNV,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkCreateIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const IndirectCommandsLayoutCreateInfoNV,
    allocator: *const AllocationCallbacks,
    indirect_commands_layout: *mut IndirectCommandsLayoutNV,
) -> vkResult;

pub type vkDestroyIndirectCommandsLayoutNV = unsafe extern "system" fn(
    device: vkDevice,
    indirect_commands_layout: IndirectCommandsLayoutNV,
    allocator: *const AllocationCallbacks,
);

pub type vkCmdExecuteGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    is_preprocessed: Bool32,
    generated_commands_info: *const GeneratedCommandsInfoEXT,
);

pub type vkCmdPreprocessGeneratedCommandsEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    generated_commands_info: *const GeneratedCommandsInfoEXT,
    state_command_buffer: vkCommandBuffer,
);

pub type vkGetGeneratedCommandsMemoryRequirementsEXT = unsafe extern "system" fn(
    device: vkDevice,
    info: *const GeneratedCommandsMemoryRequirementsInfoEXT,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkCreateIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const IndirectCommandsLayoutCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    indirect_commands_layout: *mut IndirectCommandsLayoutEXT,
) -> vkResult;

pub type vkDestroyIndirectCommandsLayoutEXT = unsafe extern "system" fn(
    device: vkDevice,
    indirect_commands_layout: IndirectCommandsLayoutEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const IndirectExecutionSetCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    indirect_execution_set: *mut IndirectExecutionSetEXT,
) -> vkResult;

pub type vkDestroyIndirectExecutionSetEXT = unsafe extern "system" fn(
    device: vkDevice,
    indirect_execution_set: IndirectExecutionSetEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkUpdateIndirectExecutionSetPipelineEXT = unsafe extern "system" fn(
    device: vkDevice,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    execution_set_writes: *const WriteIndirectExecutionSetPipelineEXT,
);

pub type vkUpdateIndirectExecutionSetShaderEXT = unsafe extern "system" fn(
    device: vkDevice,
    indirect_execution_set: IndirectExecutionSetEXT,
    execution_set_write_count: u32,
    execution_set_writes: *const WriteIndirectExecutionSetShaderEXT,
);

pub type vkGetPhysicalDeviceFeatures2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    features: *mut PhysicalDeviceFeatures2,
);

pub type vkGetPhysicalDeviceFeatures2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    features: *mut PhysicalDeviceFeatures2,
);

pub type vkGetPhysicalDeviceProperties2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    properties: *mut PhysicalDeviceProperties2,
);

pub type vkGetPhysicalDeviceProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    properties: *mut PhysicalDeviceProperties2,
);

pub type vkGetPhysicalDeviceFormatProperties2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format: Format,
    format_properties: *mut FormatProperties2,
);

pub type vkGetPhysicalDeviceFormatProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format: Format,
    format_properties: *mut FormatProperties2,
);

pub type vkGetPhysicalDeviceImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    image_format_info: *const PhysicalDeviceImageFormatInfo2,
    image_format_properties: *mut ImageFormatProperties2,
) -> vkResult;

pub type vkGetPhysicalDeviceImageFormatProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    image_format_info: *const PhysicalDeviceImageFormatInfo2,
    image_format_properties: *mut ImageFormatProperties2,
) -> vkResult;

pub type vkGetPhysicalDeviceQueueFamilyProperties2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties2,
);

pub type vkGetPhysicalDeviceQueueFamilyProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    queue_family_property_count: *mut u32,
    queue_family_properties: *mut QueueFamilyProperties2,
);

pub type vkGetPhysicalDeviceMemoryProperties2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    memory_properties: *mut PhysicalDeviceMemoryProperties2,
);

pub type vkGetPhysicalDeviceMemoryProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    memory_properties: *mut PhysicalDeviceMemoryProperties2,
);

pub type vkGetPhysicalDeviceSparseImageFormatProperties2 = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties2,
);

pub type vkGetPhysicalDeviceSparseImageFormatProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    format_info: *const PhysicalDeviceSparseImageFormatInfo2,
    property_count: *mut u32,
    properties: *mut SparseImageFormatProperties2,
);

pub type vkCmdPushDescriptorSet = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
);

pub type vkCmdPushDescriptorSetKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
    descriptor_write_count: u32,
    descriptor_writes: *const WriteDescriptorSet,
);

pub type vkTrimCommandPool = unsafe extern "system" fn(
    device: vkDevice,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);

pub type vkTrimCommandPoolKHR = unsafe extern "system" fn(
    device: vkDevice,
    command_pool: CommandPool,
    flags: CommandPoolTrimFlags,
);

pub type vkGetPhysicalDeviceExternalBufferProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    external_buffer_properties: *mut ExternalBufferProperties,
);

pub type vkGetPhysicalDeviceExternalBufferPropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_buffer_info: *const PhysicalDeviceExternalBufferInfo,
    external_buffer_properties: *mut ExternalBufferProperties,
);

pub type vkGetMemoryWin32HandleKHR = unsafe extern "system" fn(
    device: vkDevice,
    get_win32_handle_info: *const MemoryGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> vkResult;

pub type vkGetMemoryWin32HandlePropertiesKHR = unsafe extern "system" fn(
    device: vkDevice,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: HANDLE,
    memory_win32_handle_properties: *mut MemoryWin32HandlePropertiesKHR,
) -> vkResult;

pub type vkGetMemoryFdKHR = unsafe extern "system" fn(
    device: vkDevice,
    get_fd_info: *const MemoryGetFdInfoKHR,
    fd: *mut c_int,
) -> vkResult;

pub type vkGetMemoryFdPropertiesKHR = unsafe extern "system" fn(
    device: vkDevice,
    handle_type: ExternalMemoryHandleTypeFlags,
    fd: c_int,
    memory_fd_properties: *mut MemoryFdPropertiesKHR,
) -> vkResult;

pub type vkGetMemoryZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    get_zircon_handle_info: *const MemoryGetZirconHandleInfoFUCHSIA,
    zircon_handle: *mut zx_handle_t,
) -> vkResult;

pub type vkGetMemoryZirconHandlePropertiesFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    handle_type: ExternalMemoryHandleTypeFlags,
    zircon_handle: zx_handle_t,
    memory_zircon_handle_properties: *mut MemoryZirconHandlePropertiesFUCHSIA,
) -> vkResult;

pub type vkGetMemoryRemoteAddressNV = unsafe extern "system" fn(
    device: vkDevice,
    memory_get_remote_address_info: *const MemoryGetRemoteAddressInfoNV,
    address: *mut RemoteAddressNV,
) -> vkResult;

pub type vkGetMemorySciBufNV = unsafe extern "system" fn(
    device: vkDevice,
    get_sci_buf_info: *const MemoryGetSciBufInfoNV,
    handle: *mut NvSciBufObj,
) -> vkResult;

pub type vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        handle_type: ExternalMemoryHandleTypeFlags,
        handle: NvSciBufObj,
        memory_sci_buf_properties: *mut MemorySciBufPropertiesNV,
    ) -> vkResult;

pub type vkGetPhysicalDeviceSciBufAttributesNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    attributes: NvSciBufAttrList,
) -> vkResult;

pub type vkGetPhysicalDeviceExternalSemaphoreProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    external_semaphore_properties: *mut ExternalSemaphoreProperties,
);

pub type vkGetPhysicalDeviceExternalSemaphorePropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_semaphore_info: *const PhysicalDeviceExternalSemaphoreInfo,
    external_semaphore_properties: *mut ExternalSemaphoreProperties,
);

pub type vkGetSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: vkDevice,
    get_win32_handle_info: *const SemaphoreGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> vkResult;

pub type vkImportSemaphoreWin32HandleKHR = unsafe extern "system" fn(
    device: vkDevice,
    import_semaphore_win32_handle_info: *const ImportSemaphoreWin32HandleInfoKHR,
) -> vkResult;

pub type vkGetSemaphoreFdKHR = unsafe extern "system" fn(
    device: vkDevice,
    get_fd_info: *const SemaphoreGetFdInfoKHR,
    fd: *mut c_int,
) -> vkResult;

pub type vkImportSemaphoreFdKHR = unsafe extern "system" fn(
    device: vkDevice,
    import_semaphore_fd_info: *const ImportSemaphoreFdInfoKHR,
) -> vkResult;

pub type vkGetSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    get_zircon_handle_info: *const SemaphoreGetZirconHandleInfoFUCHSIA,
    zircon_handle: *mut zx_handle_t,
) -> vkResult;

pub type vkImportSemaphoreZirconHandleFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    import_semaphore_zircon_handle_info: *const ImportSemaphoreZirconHandleInfoFUCHSIA,
) -> vkResult;

pub type vkGetPhysicalDeviceExternalFenceProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    external_fence_properties: *mut ExternalFenceProperties,
);

pub type vkGetPhysicalDeviceExternalFencePropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_fence_info: *const PhysicalDeviceExternalFenceInfo,
    external_fence_properties: *mut ExternalFenceProperties,
);

pub type vkGetFenceWin32HandleKHR = unsafe extern "system" fn(
    device: vkDevice,
    get_win32_handle_info: *const FenceGetWin32HandleInfoKHR,
    handle: *mut HANDLE,
) -> vkResult;

pub type vkImportFenceWin32HandleKHR = unsafe extern "system" fn(
    device: vkDevice,
    import_fence_win32_handle_info: *const ImportFenceWin32HandleInfoKHR,
) -> vkResult;

pub type vkGetFenceFdKHR = unsafe extern "system" fn(
    device: vkDevice,
    get_fd_info: *const FenceGetFdInfoKHR,
    fd: *mut c_int,
) -> vkResult;

pub type vkImportFenceFdKHR = unsafe extern "system" fn(
    device: vkDevice,
    import_fence_fd_info: *const ImportFenceFdInfoKHR,
) -> vkResult;

pub type vkGetFenceSciSyncFenceNV = unsafe extern "system" fn(
    device: vkDevice,
    get_sci_sync_handle_info: *const FenceGetSciSyncInfoNV,
    handle: *mut c_void,
) -> vkResult;

pub type vkGetFenceSciSyncObjNV = unsafe extern "system" fn(
    device: vkDevice,
    get_sci_sync_handle_info: *const FenceGetSciSyncInfoNV,
    handle: *mut c_void,
) -> vkResult;

pub type vkImportFenceSciSyncFenceNV = unsafe extern "system" fn(
    device: vkDevice,
    import_fence_sci_sync_info: *const ImportFenceSciSyncInfoNV,
) -> vkResult;

pub type vkImportFenceSciSyncObjNV = unsafe extern "system" fn(
    device: vkDevice,
    import_fence_sci_sync_info: *const ImportFenceSciSyncInfoNV,
) -> vkResult;

pub type vkGetSemaphoreSciSyncObjNV = unsafe extern "system" fn(
    device: vkDevice,
    get_sci_sync_info: *const SemaphoreGetSciSyncInfoNV,
    handle: *mut c_void,
) -> vkResult;

pub type vkImportSemaphoreSciSyncObjNV = unsafe extern "system" fn(
    device: vkDevice,
    import_semaphore_sci_sync_info: *const ImportSemaphoreSciSyncInfoNV,
) -> vkResult;

pub type vkGetPhysicalDeviceSciSyncAttributesNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    sci_sync_attributes_info: *const SciSyncAttributesInfoNV,
    attributes: NvSciSyncAttrList,
) -> vkResult;

pub type vkCreateSemaphoreSciSyncPoolNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const SemaphoreSciSyncPoolCreateInfoNV,
    allocator: *const AllocationCallbacks,
    semaphore_pool: *mut SemaphoreSciSyncPoolNV,
) -> vkResult;

pub type vkDestroySemaphoreSciSyncPoolNV = unsafe extern "system" fn(
    device: vkDevice,
    semaphore_pool: SemaphoreSciSyncPoolNV,
    allocator: *const AllocationCallbacks,
);

pub type vkReleaseDisplayEXT =
    unsafe extern "system" fn(physical_device: vkPhysicalDevice, display: DisplayKHR) -> vkResult;

pub type vkAcquireXlibDisplayEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    dpy: *mut Display,
    display: DisplayKHR,
) -> vkResult;

pub type vkGetRandROutputDisplayEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    dpy: *mut Display,
    rr_output: RROutput,
    display: *mut DisplayKHR,
) -> vkResult;

pub type vkAcquireWinrtDisplayNV =
    unsafe extern "system" fn(physical_device: vkPhysicalDevice, display: DisplayKHR) -> vkResult;

pub type vkGetWinrtDisplayNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    device_relative_id: u32,
    display: *mut DisplayKHR,
) -> vkResult;

pub type vkDisplayPowerControlEXT = unsafe extern "system" fn(
    device: vkDevice,
    display: DisplayKHR,
    display_power_info: *const DisplayPowerInfoEXT,
) -> vkResult;

pub type vkRegisterDeviceEventEXT = unsafe extern "system" fn(
    device: vkDevice,
    device_event_info: *const DeviceEventInfoEXT,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> vkResult;

pub type vkRegisterDisplayEventEXT = unsafe extern "system" fn(
    device: vkDevice,
    display: DisplayKHR,
    display_event_info: *const DisplayEventInfoEXT,
    allocator: *const AllocationCallbacks,
    fence: *mut Fence,
) -> vkResult;

pub type vkGetSwapchainCounterEXT = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    counter: SurfaceCounterFlagsEXT,
    counter_value: *mut u64,
) -> vkResult;

pub type vkGetPhysicalDeviceSurfaceCapabilities2EXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface: SurfaceKHR,
    surface_capabilities: *mut SurfaceCapabilities2EXT,
) -> vkResult;

pub type vkEnumeratePhysicalDeviceGroups = unsafe extern "system" fn(
    instance: vkInstance,
    physical_device_group_count: *mut u32,
    physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> vkResult;

pub type vkEnumeratePhysicalDeviceGroupsKHR = unsafe extern "system" fn(
    instance: vkInstance,
    physical_device_group_count: *mut u32,
    physical_device_group_properties: *mut PhysicalDeviceGroupProperties,
) -> vkResult;

pub type vkGetDeviceGroupPeerMemoryFeatures = unsafe extern "system" fn(
    device: vkDevice,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    peer_memory_features: *mut PeerMemoryFeatureFlags,
);

pub type vkGetDeviceGroupPeerMemoryFeaturesKHR = unsafe extern "system" fn(
    device: vkDevice,
    heap_index: u32,
    local_device_index: u32,
    remote_device_index: u32,
    peer_memory_features: *mut PeerMemoryFeatureFlags,
);

pub type vkBindBufferMemory2 = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindBufferMemoryInfo,
) -> vkResult;

pub type vkBindBufferMemory2KHR = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindBufferMemoryInfo,
) -> vkResult;

pub type vkBindImageMemory2 = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindImageMemoryInfo,
) -> vkResult;

pub type vkBindImageMemory2KHR = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindImageMemoryInfo,
) -> vkResult;

pub type vkCmdSetDeviceMask =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, device_mask: u32);

pub type vkCmdSetDeviceMaskKHR =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, device_mask: u32);

pub type vkGetDeviceGroupPresentCapabilitiesKHR = unsafe extern "system" fn(
    device: vkDevice,
    device_group_present_capabilities: *mut DeviceGroupPresentCapabilitiesKHR,
) -> vkResult;

pub type vkGetDeviceGroupSurfacePresentModesKHR = unsafe extern "system" fn(
    device: vkDevice,
    surface: SurfaceKHR,
    modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> vkResult;

pub type vkAcquireNextImage2KHR = unsafe extern "system" fn(
    device: vkDevice,
    acquire_info: *const AcquireNextImageInfoKHR,
    image_index: *mut u32,
) -> vkResult;

pub type vkCmdDispatchBase = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type vkCmdDispatchBaseKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    base_group_x: u32,
    base_group_y: u32,
    base_group_z: u32,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type vkGetPhysicalDevicePresentRectanglesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface: SurfaceKHR,
    rect_count: *mut u32,
    rects: *mut Rect2D,
) -> vkResult;

pub type vkCreateDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DescriptorUpdateTemplateCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> vkResult;

pub type vkCreateDescriptorUpdateTemplateKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DescriptorUpdateTemplateCreateInfo,
    allocator: *const AllocationCallbacks,
    descriptor_update_template: *mut DescriptorUpdateTemplate,
) -> vkResult;

pub type vkDestroyDescriptorUpdateTemplate = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_update_template: DescriptorUpdateTemplate,
    allocator: *const AllocationCallbacks,
);

pub type vkDestroyDescriptorUpdateTemplateKHR = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_update_template: DescriptorUpdateTemplate,
    allocator: *const AllocationCallbacks,
);

pub type vkUpdateDescriptorSetWithTemplate = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    data: *const c_void,
);

pub type vkUpdateDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_set: DescriptorSet,
    descriptor_update_template: DescriptorUpdateTemplate,
    data: *const c_void,
);

pub type vkCmdPushDescriptorSetWithTemplate = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    data: *const c_void,
);

pub type vkCmdPushDescriptorSetWithTemplateKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    descriptor_update_template: DescriptorUpdateTemplate,
    layout: PipelineLayout,
    set: u32,
    data: *const c_void,
);

pub type vkSetHdrMetadataEXT = unsafe extern "system" fn(
    device: vkDevice,
    swapchain_count: u32,
    swapchains: *const SwapchainKHR,
    metadata: *const HdrMetadataEXT,
);

pub type vkGetSwapchainStatusKHR =
    unsafe extern "system" fn(device: vkDevice, swapchain: SwapchainKHR) -> vkResult;

pub type vkGetRefreshCycleDurationGOOGLE = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    display_timing_properties: *mut RefreshCycleDurationGOOGLE,
) -> vkResult;

pub type vkGetPastPresentationTimingGOOGLE = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    presentation_timing_count: *mut u32,
    presentation_timings: *mut PastPresentationTimingGOOGLE,
) -> vkResult;

pub type vkCreateIOSSurfaceMVK = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const IOSSurfaceCreateInfoMVK,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateMacOSSurfaceMVK = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const MacOSSurfaceCreateInfoMVK,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCreateMetalSurfaceEXT = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const MetalSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkCmdSetViewportWScalingNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewport_w_scalings: *const ViewportWScalingNV,
);

pub type vkCmdSetDiscardRectangleEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_discard_rectangle: u32,
    discard_rectangle_count: u32,
    discard_rectangles: *const Rect2D,
);

pub type vkCmdSetDiscardRectangleEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, discard_rectangle_enable: Bool32);

pub type vkCmdSetDiscardRectangleModeEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    discard_rectangle_mode: DiscardRectangleModeEXT,
);

pub type vkCmdSetSampleLocationsEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    sample_locations_info: *const SampleLocationsInfoEXT,
);

pub type vkGetPhysicalDeviceMultisamplePropertiesEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    samples: SampleCountFlags,
    multisample_properties: *mut MultisamplePropertiesEXT,
);

pub type vkGetPhysicalDeviceSurfaceCapabilities2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    surface_capabilities: *mut SurfaceCapabilities2KHR,
) -> vkResult;

pub type vkGetPhysicalDeviceSurfaceFormats2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    surface_format_count: *mut u32,
    surface_formats: *mut SurfaceFormat2KHR,
) -> vkResult;

pub type vkGetPhysicalDeviceDisplayProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayProperties2KHR,
) -> vkResult;

pub type vkGetPhysicalDeviceDisplayPlaneProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut DisplayPlaneProperties2KHR,
) -> vkResult;

pub type vkGetDisplayModeProperties2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    display: DisplayKHR,
    property_count: *mut u32,
    properties: *mut DisplayModeProperties2KHR,
) -> vkResult;

pub type vkGetDisplayPlaneCapabilities2KHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    display_plane_info: *const DisplayPlaneInfo2KHR,
    capabilities: *mut DisplayPlaneCapabilities2KHR,
) -> vkResult;

pub type vkGetBufferMemoryRequirements2 = unsafe extern "system" fn(
    device: vkDevice,
    info: *const BufferMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetBufferMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const BufferMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetImageMemoryRequirements2 = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ImageMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetImageMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ImageMemoryRequirementsInfo2,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetImageSparseMemoryRequirements2 = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ImageSparseMemoryRequirementsInfo2,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);

pub type vkGetImageSparseMemoryRequirements2KHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ImageSparseMemoryRequirementsInfo2,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);

pub type vkGetDeviceBufferMemoryRequirements = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceBufferMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetDeviceBufferMemoryRequirementsKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceBufferMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetDeviceImageMemoryRequirements = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceImageMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetDeviceImageMemoryRequirementsKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceImageMemoryRequirements,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetDeviceImageSparseMemoryRequirements = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceImageMemoryRequirements,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);

pub type vkGetDeviceImageSparseMemoryRequirementsKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceImageMemoryRequirements,
    sparse_memory_requirement_count: *mut u32,
    sparse_memory_requirements: *mut SparseImageMemoryRequirements2,
);

pub type vkCreateSamplerYcbcrConversion = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const SamplerYcbcrConversionCreateInfo,
    allocator: *const AllocationCallbacks,
    ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> vkResult;

pub type vkCreateSamplerYcbcrConversionKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const SamplerYcbcrConversionCreateInfo,
    allocator: *const AllocationCallbacks,
    ycbcr_conversion: *mut SamplerYcbcrConversion,
) -> vkResult;

pub type vkDestroySamplerYcbcrConversion = unsafe extern "system" fn(
    device: vkDevice,
    ycbcr_conversion: SamplerYcbcrConversion,
    allocator: *const AllocationCallbacks,
);

pub type vkDestroySamplerYcbcrConversionKHR = unsafe extern "system" fn(
    device: vkDevice,
    ycbcr_conversion: SamplerYcbcrConversion,
    allocator: *const AllocationCallbacks,
);

pub type vkGetDeviceQueue2 = unsafe extern "system" fn(
    device: vkDevice,
    queue_info: *const DeviceQueueInfo2,
    queue: *mut vkQueue,
);

pub type vkCreateValidationCacheEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ValidationCacheCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    validation_cache: *mut ValidationCacheEXT,
) -> vkResult;

pub type vkDestroyValidationCacheEXT = unsafe extern "system" fn(
    device: vkDevice,
    validation_cache: ValidationCacheEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkGetValidationCacheDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    validation_cache: ValidationCacheEXT,
    data_size: *mut usize,
    data: *mut c_void,
) -> vkResult;

pub type vkMergeValidationCachesEXT = unsafe extern "system" fn(
    device: vkDevice,
    dst_cache: ValidationCacheEXT,
    src_cache_count: u32,
    src_caches: *const ValidationCacheEXT,
) -> vkResult;

pub type vkGetDescriptorSetLayoutSupport = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DescriptorSetLayoutCreateInfo,
    support: *mut DescriptorSetLayoutSupport,
);

pub type vkGetDescriptorSetLayoutSupportKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DescriptorSetLayoutCreateInfo,
    support: *mut DescriptorSetLayoutSupport,
);

pub type vkGetShaderInfoAMD = unsafe extern "system" fn(
    device: vkDevice,
    pipeline: Pipeline,
    shader_stage: ShaderStageFlags,
    info_type: ShaderInfoTypeAMD,
    info_size: *mut usize,
    info: *mut c_void,
) -> vkResult;

pub type vkSetLocalDimmingAMD = unsafe extern "system" fn(
    device: vkDevice,
    swap_chain: SwapchainKHR,
    local_dimming_enable: Bool32,
);

pub type vkGetPhysicalDeviceCalibrateableTimeDomainsKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    time_domain_count: *mut u32,
    time_domains: *mut TimeDomainKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceCalibrateableTimeDomainsEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    time_domain_count: *mut u32,
    time_domains: *mut TimeDomainKHR,
) -> vkResult;

pub type vkGetCalibratedTimestampsKHR = unsafe extern "system" fn(
    device: vkDevice,
    timestamp_count: u32,
    timestamp_infos: *const CalibratedTimestampInfoKHR,
    timestamps: *mut u64,
    max_deviation: *mut u64,
) -> vkResult;

pub type vkGetCalibratedTimestampsEXT = unsafe extern "system" fn(
    device: vkDevice,
    timestamp_count: u32,
    timestamp_infos: *const CalibratedTimestampInfoKHR,
    timestamps: *mut u64,
    max_deviation: *mut u64,
) -> vkResult;

pub type vkSetDebugUtilsObjectNameEXT = unsafe extern "system" fn(
    device: vkDevice,
    name_info: *const DebugUtilsObjectNameInfoEXT,
) -> vkResult;

pub type vkSetDebugUtilsObjectTagEXT = unsafe extern "system" fn(
    device: vkDevice,
    tag_info: *const DebugUtilsObjectTagInfoEXT,
) -> vkResult;

pub type vkQueueBeginDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: vkQueue, label_info: *const DebugUtilsLabelEXT);

pub type vkQueueEndDebugUtilsLabelEXT = unsafe extern "system" fn(queue: vkQueue);

pub type vkQueueInsertDebugUtilsLabelEXT =
    unsafe extern "system" fn(queue: vkQueue, label_info: *const DebugUtilsLabelEXT);

pub type vkCmdBeginDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    label_info: *const DebugUtilsLabelEXT,
);

pub type vkCmdEndDebugUtilsLabelEXT = unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkCmdInsertDebugUtilsLabelEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    label_info: *const DebugUtilsLabelEXT,
);

pub type vkCreateDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const DebugUtilsMessengerCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    messenger: *mut DebugUtilsMessengerEXT,
) -> vkResult;

pub type vkDestroyDebugUtilsMessengerEXT = unsafe extern "system" fn(
    instance: vkInstance,
    messenger: DebugUtilsMessengerEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkSubmitDebugUtilsMessageEXT = unsafe extern "system" fn(
    instance: vkInstance,
    message_severity: DebugUtilsMessageSeverityFlagsEXT,
    message_types: DebugUtilsMessageTypeFlagsEXT,
    callback_data: *const DebugUtilsMessengerCallbackDataEXT,
);

pub type vkGetMemoryHostPointerPropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    handle_type: ExternalMemoryHandleTypeFlags,
    host_pointer: *const c_void,
    memory_host_pointer_properties: *mut MemoryHostPointerPropertiesEXT,
) -> vkResult;

pub type vkCmdWriteBufferMarkerAMD = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_stage: PipelineStageFlags,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);

pub type vkCreateRenderPass2 = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const RenderPassCreateInfo2,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> vkResult;

pub type vkCreateRenderPass2KHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const RenderPassCreateInfo2,
    allocator: *const AllocationCallbacks,
    render_pass: *mut RenderPass,
) -> vkResult;

pub type vkCmdBeginRenderPass2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    render_pass_begin: *const RenderPassBeginInfo,
    subpass_begin_info: *const SubpassBeginInfo,
);

pub type vkCmdBeginRenderPass2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    render_pass_begin: *const RenderPassBeginInfo,
    subpass_begin_info: *const SubpassBeginInfo,
);

pub type vkCmdNextSubpass2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    subpass_begin_info: *const SubpassBeginInfo,
    subpass_end_info: *const SubpassEndInfo,
);

pub type vkCmdNextSubpass2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    subpass_begin_info: *const SubpassBeginInfo,
    subpass_end_info: *const SubpassEndInfo,
);

pub type vkCmdEndRenderPass2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    subpass_end_info: *const SubpassEndInfo,
);

pub type vkCmdEndRenderPass2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    subpass_end_info: *const SubpassEndInfo,
);

pub type vkGetSemaphoreCounterValue =
    unsafe extern "system" fn(device: vkDevice, semaphore: Semaphore, value: *mut u64) -> vkResult;

pub type vkGetSemaphoreCounterValueKHR =
    unsafe extern "system" fn(device: vkDevice, semaphore: Semaphore, value: *mut u64) -> vkResult;

pub type vkWaitSemaphores = unsafe extern "system" fn(
    device: vkDevice,
    wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> vkResult;

pub type vkWaitSemaphoresKHR = unsafe extern "system" fn(
    device: vkDevice,
    wait_info: *const SemaphoreWaitInfo,
    timeout: u64,
) -> vkResult;

pub type vkSignalSemaphore = unsafe extern "system" fn(
    device: vkDevice,
    signal_info: *const SemaphoreSignalInfo,
) -> vkResult;

pub type vkSignalSemaphoreKHR = unsafe extern "system" fn(
    device: vkDevice,
    signal_info: *const SemaphoreSignalInfo,
) -> vkResult;

pub type vkGetAndroidHardwareBufferPropertiesANDROID = unsafe extern "system" fn(
    device: vkDevice,
    buffer: *const AHardwareBuffer,
    properties: *mut AndroidHardwareBufferPropertiesANDROID,
) -> vkResult;

pub type vkGetMemoryAndroidHardwareBufferANDROID = unsafe extern "system" fn(
    device: vkDevice,
    info: *const MemoryGetAndroidHardwareBufferInfoANDROID,
    buffer: *mut *mut AHardwareBuffer,
) -> vkResult;

pub type vkCmdDrawIndirectCount = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawIndirectCountKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawIndirectCountAMD = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawIndexedIndirectCount = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawIndexedIndirectCountKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawIndexedIndirectCountAMD = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdSetCheckpointNV =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, checkpoint_marker: *const c_void);

pub type vkGetQueueCheckpointDataNV = unsafe extern "system" fn(
    queue: vkQueue,
    checkpoint_data_count: *mut u32,
    checkpoint_data: *mut CheckpointDataNV,
);

pub type vkCmdBindTransformFeedbackBuffersEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
);

pub type vkCmdBeginTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    counter_buffers: *const Buffer,
    counter_buffer_offsets: *const DeviceSize,
);

pub type vkCmdEndTransformFeedbackEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_counter_buffer: u32,
    counter_buffer_count: u32,
    counter_buffers: *const Buffer,
    counter_buffer_offsets: *const DeviceSize,
);

pub type vkCmdBeginQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    query_pool: QueryPool,
    query: u32,
    flags: QueryControlFlags,
    index: u32,
);

pub type vkCmdEndQueryIndexedEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    query_pool: QueryPool,
    query: u32,
    index: u32,
);

pub type vkCmdDrawIndirectByteCountEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    instance_count: u32,
    first_instance: u32,
    counter_buffer: Buffer,
    counter_buffer_offset: DeviceSize,
    counter_offset: u32,
    vertex_stride: u32,
);

pub type vkCmdSetExclusiveScissorNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    exclusive_scissors: *const Rect2D,
);

pub type vkCmdSetExclusiveScissorEnableNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_exclusive_scissor: u32,
    exclusive_scissor_count: u32,
    exclusive_scissor_enables: *const Bool32,
);

pub type vkCmdBindShadingRateImageNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);

pub type vkCmdSetViewportShadingRatePaletteNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    shading_rate_palettes: *const ShadingRatePaletteNV,
);

pub type vkCmdSetCoarseSampleOrderNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    sample_order_type: CoarseSampleOrderTypeNV,
    custom_sample_order_count: u32,
    custom_sample_orders: *const CoarseSampleOrderCustomNV,
);

pub type vkCmdDrawMeshTasksNV =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, task_count: u32, first_task: u32);

pub type vkCmdDrawMeshTasksIndirectNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawMeshTasksIndirectCountNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawMeshTasksEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    group_count_x: u32,
    group_count_y: u32,
    group_count_z: u32,
);

pub type vkCmdDrawMeshTasksIndirectEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    draw_count: u32,
    stride: u32,
);

pub type vkCmdDrawMeshTasksIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    count_buffer: Buffer,
    count_buffer_offset: DeviceSize,
    max_draw_count: u32,
    stride: u32,
);

pub type vkCompileDeferredNV =
    unsafe extern "system" fn(device: vkDevice, pipeline: Pipeline, shader: u32) -> vkResult;

pub type vkCreateAccelerationStructureNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const AccelerationStructureCreateInfoNV,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureNV,
) -> vkResult;

pub type vkCmdBindInvocationMaskHUAWEI = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    image_view: ImageView,
    image_layout: ImageLayout,
);

pub type vkDestroyAccelerationStructureKHR = unsafe extern "system" fn(
    device: vkDevice,
    acceleration_structure: AccelerationStructureKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkDestroyAccelerationStructureNV = unsafe extern "system" fn(
    device: vkDevice,
    acceleration_structure: AccelerationStructureNV,
    allocator: *const AllocationCallbacks,
);

pub type vkGetAccelerationStructureMemoryRequirementsNV = unsafe extern "system" fn(
    device: vkDevice,
    info: *const AccelerationStructureMemoryRequirementsInfoNV,
    memory_requirements: *mut MemoryRequirements2KHR,
);

pub type vkBindAccelerationStructureMemoryNV = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindAccelerationStructureMemoryInfoNV,
) -> vkResult;

pub type vkCmdCopyAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    mode: CopyAccelerationStructureModeKHR,
);

pub type vkCmdCopyAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info: *const CopyAccelerationStructureInfoKHR,
);

pub type vkCopyAccelerationStructureKHR = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyAccelerationStructureInfoKHR,
) -> vkResult;

pub type vkCmdCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info: *const CopyAccelerationStructureToMemoryInfoKHR,
);

pub type vkCopyAccelerationStructureToMemoryKHR = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyAccelerationStructureToMemoryInfoKHR,
) -> vkResult;

pub type vkCmdCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info: *const CopyMemoryToAccelerationStructureInfoKHR,
);

pub type vkCopyMemoryToAccelerationStructureKHR = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMemoryToAccelerationStructureInfoKHR,
) -> vkResult;

pub type vkCmdWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);

pub type vkCmdWriteAccelerationStructuresPropertiesNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureNV,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);

pub type vkCmdBuildAccelerationStructureNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info: *const AccelerationStructureInfoNV,
    instance_data: Buffer,
    instance_offset: DeviceSize,
    update: Bool32,
    dst: AccelerationStructureNV,
    src: AccelerationStructureNV,
    scratch: Buffer,
    scratch_offset: DeviceSize,
);

pub type vkWriteAccelerationStructuresPropertiesKHR = unsafe extern "system" fn(
    device: vkDevice,
    acceleration_structure_count: u32,
    acceleration_structures: *const AccelerationStructureKHR,
    query_type: QueryType,
    data_size: usize,
    data: *mut c_void,
    stride: usize,
) -> vkResult;

pub type vkCmdTraceRaysKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    width: u32,
    height: u32,
    depth: u32,
);

pub type vkCmdTraceRaysNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    raygen_shader_binding_table_buffer: Buffer,
    raygen_shader_binding_offset: DeviceSize,
    miss_shader_binding_table_buffer: Buffer,
    miss_shader_binding_offset: DeviceSize,
    miss_shader_binding_stride: DeviceSize,
    hit_shader_binding_table_buffer: Buffer,
    hit_shader_binding_offset: DeviceSize,
    hit_shader_binding_stride: DeviceSize,
    callable_shader_binding_table_buffer: Buffer,
    callable_shader_binding_offset: DeviceSize,
    callable_shader_binding_stride: DeviceSize,
    width: u32,
    height: u32,
    depth: u32,
);

pub type vkGetRayTracingShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: vkDevice,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut c_void,
) -> vkResult;

pub type vkGetRayTracingShaderGroupHandlesNV = unsafe extern "system" fn(
    device: vkDevice,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut c_void,
) -> vkResult;

pub type vkGetRayTracingCaptureReplayShaderGroupHandlesKHR = unsafe extern "system" fn(
    device: vkDevice,
    pipeline: Pipeline,
    first_group: u32,
    group_count: u32,
    data_size: usize,
    data: *mut c_void,
) -> vkResult;

pub type vkGetAccelerationStructureHandleNV = unsafe extern "system" fn(
    device: vkDevice,
    acceleration_structure: AccelerationStructureNV,
    data_size: usize,
    data: *mut c_void,
) -> vkResult;

pub type vkCreateRayTracingPipelinesNV = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const RayTracingPipelineCreateInfoNV,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> vkResult;

pub type vkCreateRayTracingPipelinesKHR = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const RayTracingPipelineCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> vkResult;

pub type vkGetPhysicalDeviceCooperativeMatrixPropertiesNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixPropertiesNV,
) -> vkResult;

pub type vkCmdTraceRaysIndirectKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    raygen_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    miss_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    hit_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    callable_shader_binding_table: *const StridedDeviceAddressRegionKHR,
    indirect_device_address: DeviceAddress,
);

pub type vkCmdTraceRaysIndirect2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    indirect_device_address: DeviceAddress,
);

pub type vkGetClusterAccelerationStructureBuildSizesNV = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ClusterAccelerationStructureInputInfoNV,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);

pub type vkCmdBuildClusterAccelerationStructureIndirectNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    command_infos: *const ClusterAccelerationStructureCommandsInfoNV,
);

pub type vkGetDeviceAccelerationStructureCompatibilityKHR = unsafe extern "system" fn(
    device: vkDevice,
    version_info: *const AccelerationStructureVersionInfoKHR,
    compatibility: *mut AccelerationStructureCompatibilityKHR,
);

pub type vkGetRayTracingShaderGroupStackSizeKHR = unsafe extern "system" fn(
    device: vkDevice,
    pipeline: Pipeline,
    group: u32,
    group_shader: ShaderGroupShaderKHR,
) -> DeviceSize;

pub type vkCmdSetRayTracingPipelineStackSizeKHR =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, pipeline_stack_size: u32);

pub type vkGetImageViewHandleNVX =
    unsafe extern "system" fn(device: vkDevice, info: *const ImageViewHandleInfoNVX) -> u32;

pub type vkGetImageViewHandle64NVX =
    unsafe extern "system" fn(device: vkDevice, info: *const ImageViewHandleInfoNVX) -> u64;

pub type vkGetImageViewAddressNVX = unsafe extern "system" fn(
    device: vkDevice,
    image_view: ImageView,
    properties: *mut ImageViewAddressPropertiesNVX,
) -> vkResult;

pub type vkGetDeviceCombinedImageSamplerIndexNVX =
    unsafe extern "system" fn(device: vkDevice, image_view_index: u64, sampler_index: u64) -> u64;

pub type vkGetPhysicalDeviceSurfacePresentModes2EXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    present_mode_count: *mut u32,
    present_modes: *mut PresentModeKHR,
) -> vkResult;

pub type vkGetDeviceGroupSurfacePresentModes2EXT = unsafe extern "system" fn(
    device: vkDevice,
    surface_info: *const PhysicalDeviceSurfaceInfo2KHR,
    modes: *mut DeviceGroupPresentModeFlagsKHR,
) -> vkResult;

pub type vkAcquireFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: vkDevice, swapchain: SwapchainKHR) -> vkResult;

pub type vkReleaseFullScreenExclusiveModeEXT =
    unsafe extern "system" fn(device: vkDevice, swapchain: SwapchainKHR) -> vkResult;

pub type vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        queue_family_index: u32,
        counter_count: *mut u32,
        counters: *mut PerformanceCounterKHR,
        counter_descriptions: *mut PerformanceCounterDescriptionKHR,
    ) -> vkResult;

pub type vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    performance_query_create_info: *const QueryPoolPerformanceCreateInfoKHR,
    num_passes: *mut u32,
);

pub type vkAcquireProfilingLockKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const AcquireProfilingLockInfoKHR,
) -> vkResult;

pub type vkReleaseProfilingLockKHR = unsafe extern "system" fn(device: vkDevice);

pub type vkGetImageDrmFormatModifierPropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    properties: *mut ImageDrmFormatModifierPropertiesEXT,
) -> vkResult;

pub type vkGetBufferOpaqueCaptureAddress =
    unsafe extern "system" fn(device: vkDevice, info: *const BufferDeviceAddressInfo) -> u64;

pub type vkGetBufferOpaqueCaptureAddressKHR =
    unsafe extern "system" fn(device: vkDevice, info: *const BufferDeviceAddressInfo) -> u64;

pub type vkGetBufferDeviceAddress = unsafe extern "system" fn(
    device: vkDevice,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress;

pub type vkGetBufferDeviceAddressKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress;

pub type vkGetBufferDeviceAddressEXT = unsafe extern "system" fn(
    device: vkDevice,
    info: *const BufferDeviceAddressInfo,
) -> DeviceAddress;

pub type vkCreateHeadlessSurfaceEXT = unsafe extern "system" fn(
    instance: vkInstance,
    create_info: *const HeadlessSurfaceCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    surface: *mut SurfaceKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        combination_count: *mut u32,
        combinations: *mut FramebufferMixedSamplesCombinationNV,
    ) -> vkResult;

pub type vkInitializePerformanceApiINTEL = unsafe extern "system" fn(
    device: vkDevice,
    initialize_info: *const InitializePerformanceApiInfoINTEL,
) -> vkResult;

pub type vkUninitializePerformanceApiINTEL = unsafe extern "system" fn(device: vkDevice);

pub type vkCmdSetPerformanceMarkerINTEL = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    marker_info: *const PerformanceMarkerInfoINTEL,
) -> vkResult;

pub type vkCmdSetPerformanceStreamMarkerINTEL = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    marker_info: *const PerformanceStreamMarkerInfoINTEL,
) -> vkResult;

pub type vkCmdSetPerformanceOverrideINTEL = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    override_info: *const PerformanceOverrideInfoINTEL,
) -> vkResult;

pub type vkAcquirePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: vkDevice,
    acquire_info: *const PerformanceConfigurationAcquireInfoINTEL,
    configuration: *mut PerformanceConfigurationINTEL,
) -> vkResult;

pub type vkReleasePerformanceConfigurationINTEL = unsafe extern "system" fn(
    device: vkDevice,
    configuration: PerformanceConfigurationINTEL,
) -> vkResult;

pub type vkQueueSetPerformanceConfigurationINTEL = unsafe extern "system" fn(
    queue: vkQueue,
    configuration: PerformanceConfigurationINTEL,
) -> vkResult;

pub type vkGetPerformanceParameterINTEL = unsafe extern "system" fn(
    device: vkDevice,
    parameter: PerformanceParameterTypeINTEL,
    value: *mut PerformanceValueINTEL,
) -> vkResult;

pub type vkGetDeviceMemoryOpaqueCaptureAddress = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;

pub type vkGetDeviceMemoryOpaqueCaptureAddressKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceMemoryOpaqueCaptureAddressInfo,
) -> u64;

pub type vkGetPipelineExecutablePropertiesKHR = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_info: *const PipelineInfoKHR,
    executable_count: *mut u32,
    properties: *mut PipelineExecutablePropertiesKHR,
) -> vkResult;

pub type vkGetPipelineExecutableStatisticsKHR = unsafe extern "system" fn(
    device: vkDevice,
    executable_info: *const PipelineExecutableInfoKHR,
    statistic_count: *mut u32,
    statistics: *mut PipelineExecutableStatisticKHR,
) -> vkResult;

pub type vkGetPipelineExecutableInternalRepresentationsKHR = unsafe extern "system" fn(
    device: vkDevice,
    executable_info: *const PipelineExecutableInfoKHR,
    internal_representation_count: *mut u32,
    internal_representations: *mut PipelineExecutableInternalRepresentationKHR,
) -> vkResult;

pub type vkCmdSetLineStipple = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);

pub type vkCmdSetLineStippleKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);

pub type vkCmdSetLineStippleEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    line_stipple_factor: u32,
    line_stipple_pattern: u16,
);

pub type vkGetFaultData = unsafe extern "system" fn(
    device: vkDevice,
    fault_query_behavior: FaultQueryBehavior,
    unrecorded_faults: *mut Bool32,
    fault_count: *mut u32,
    faults: *mut FaultData,
) -> vkResult;

pub type vkGetPhysicalDeviceToolProperties = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    tool_count: *mut u32,
    tool_properties: *mut PhysicalDeviceToolProperties,
) -> vkResult;

pub type vkGetPhysicalDeviceToolPropertiesEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    tool_count: *mut u32,
    tool_properties: *mut PhysicalDeviceToolProperties,
) -> vkResult;

pub type vkCreateAccelerationStructureKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const AccelerationStructureCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    acceleration_structure: *mut AccelerationStructureKHR,
) -> vkResult;

pub type vkCmdBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
);

pub type vkCmdBuildAccelerationStructuresIndirectKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    indirect_device_addresses: *const DeviceAddress,
    indirect_strides: *const u32,
    pp_max_primitive_counts: *const *const u32,
);

pub type vkBuildAccelerationStructuresKHR = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    infos: *const AccelerationStructureBuildGeometryInfoKHR,
    pp_build_range_infos: *const *const AccelerationStructureBuildRangeInfoKHR,
) -> vkResult;

pub type vkGetAccelerationStructureDeviceAddressKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const AccelerationStructureDeviceAddressInfoKHR,
) -> DeviceAddress;

pub type vkCreateDeferredOperationKHR = unsafe extern "system" fn(
    device: vkDevice,
    allocator: *const AllocationCallbacks,
    deferred_operation: *mut DeferredOperationKHR,
) -> vkResult;

pub type vkDestroyDeferredOperationKHR = unsafe extern "system" fn(
    device: vkDevice,
    operation: DeferredOperationKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkGetDeferredOperationMaxConcurrencyKHR =
    unsafe extern "system" fn(device: vkDevice, operation: DeferredOperationKHR) -> u32;

pub type vkGetDeferredOperationResultKHR =
    unsafe extern "system" fn(device: vkDevice, operation: DeferredOperationKHR) -> vkResult;

pub type vkDeferredOperationJoinKHR =
    unsafe extern "system" fn(device: vkDevice, operation: DeferredOperationKHR) -> vkResult;

pub type vkGetPipelineIndirectMemoryRequirementsNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ComputePipelineCreateInfo,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkGetPipelineIndirectDeviceAddressNV = unsafe extern "system" fn(
    device: vkDevice,
    info: *const PipelineIndirectDeviceAddressInfoNV,
) -> DeviceAddress;

pub type vkAntiLagUpdateAMD =
    unsafe extern "system" fn(device: vkDevice, data: *const AntiLagDataAMD);

pub type vkCmdSetCullMode =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, cull_mode: CullModeFlags);

pub type vkCmdSetCullModeEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, cull_mode: CullModeFlags);

pub type vkCmdSetFrontFace =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, front_face: FrontFace);

pub type vkCmdSetFrontFaceEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, front_face: FrontFace);

pub type vkCmdSetPrimitiveTopology = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    primitive_topology: PrimitiveTopology,
);

pub type vkCmdSetPrimitiveTopologyEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    primitive_topology: PrimitiveTopology,
);

pub type vkCmdSetViewportWithCount = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    viewport_count: u32,
    viewports: *const Viewport,
);

pub type vkCmdSetViewportWithCountEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    viewport_count: u32,
    viewports: *const Viewport,
);

pub type vkCmdSetScissorWithCount = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    scissor_count: u32,
    scissors: *const Rect2D,
);

pub type vkCmdSetScissorWithCountEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    scissor_count: u32,
    scissors: *const Rect2D,
);

pub type vkCmdBindIndexBuffer2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
);

pub type vkCmdBindIndexBuffer2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer: Buffer,
    offset: DeviceSize,
    size: DeviceSize,
    index_type: IndexType,
);

pub type vkCmdBindVertexBuffers2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
    strides: *const DeviceSize,
);

pub type vkCmdBindVertexBuffers2EXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_binding: u32,
    binding_count: u32,
    buffers: *const Buffer,
    offsets: *const DeviceSize,
    sizes: *const DeviceSize,
    strides: *const DeviceSize,
);

pub type vkCmdSetDepthTestEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_test_enable: Bool32);

pub type vkCmdSetDepthTestEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_test_enable: Bool32);

pub type vkCmdSetDepthWriteEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_write_enable: Bool32);

pub type vkCmdSetDepthWriteEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_write_enable: Bool32);

pub type vkCmdSetDepthCompareOp =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_compare_op: CompareOp);

pub type vkCmdSetDepthCompareOpEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_compare_op: CompareOp);

pub type vkCmdSetDepthBoundsTestEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_bounds_test_enable: Bool32);

pub type vkCmdSetDepthBoundsTestEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_bounds_test_enable: Bool32);

pub type vkCmdSetStencilTestEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, stencil_test_enable: Bool32);

pub type vkCmdSetStencilTestEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, stencil_test_enable: Bool32);

pub type vkCmdSetStencilOp = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);

pub type vkCmdSetStencilOpEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    face_mask: StencilFaceFlags,
    fail_op: StencilOp,
    pass_op: StencilOp,
    depth_fail_op: StencilOp,
    compare_op: CompareOp,
);

pub type vkCmdSetPatchControlPointsEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, patch_control_points: u32);

pub type vkCmdSetRasterizerDiscardEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, rasterizer_discard_enable: Bool32);

pub type vkCmdSetRasterizerDiscardEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, rasterizer_discard_enable: Bool32);

pub type vkCmdSetDepthBiasEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_bias_enable: Bool32);

pub type vkCmdSetDepthBiasEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_bias_enable: Bool32);

pub type vkCmdSetLogicOpEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, logic_op: LogicOp);

pub type vkCmdSetPrimitiveRestartEnable =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, primitive_restart_enable: Bool32);

pub type vkCmdSetPrimitiveRestartEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, primitive_restart_enable: Bool32);

pub type vkCmdSetTessellationDomainOriginEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    domain_origin: TessellationDomainOrigin,
);

pub type vkCmdSetDepthClampEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_clamp_enable: Bool32);

pub type vkCmdSetPolygonModeEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, polygon_mode: PolygonMode);

pub type vkCmdSetRasterizationSamplesEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    rasterization_samples: SampleCountFlags,
);

pub type vkCmdSetSampleMaskEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    samples: SampleCountFlags,
    sample_mask: *const SampleMask,
);

pub type vkCmdSetAlphaToCoverageEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, alpha_to_coverage_enable: Bool32);

pub type vkCmdSetAlphaToOneEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, alpha_to_one_enable: Bool32);

pub type vkCmdSetLogicOpEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, logic_op_enable: Bool32);

pub type vkCmdSetColorBlendEnableEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_enables: *const Bool32,
);

pub type vkCmdSetColorBlendEquationEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_equations: *const ColorBlendEquationEXT,
);

pub type vkCmdSetColorWriteMaskEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_write_masks: *const ColorComponentFlags,
);

pub type vkCmdSetRasterizationStreamEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, rasterization_stream: u32);

pub type vkCmdSetConservativeRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    conservative_rasterization_mode: ConservativeRasterizationModeEXT,
);

pub type vkCmdSetExtraPrimitiveOverestimationSizeEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    extra_primitive_overestimation_size: f32,
);

pub type vkCmdSetDepthClipEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, depth_clip_enable: Bool32);

pub type vkCmdSetSampleLocationsEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, sample_locations_enable: Bool32);

pub type vkCmdSetColorBlendAdvancedEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_attachment: u32,
    attachment_count: u32,
    color_blend_advanced: *const ColorBlendAdvancedEXT,
);

pub type vkCmdSetProvokingVertexModeEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    provoking_vertex_mode: ProvokingVertexModeEXT,
);

pub type vkCmdSetLineRasterizationModeEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    line_rasterization_mode: LineRasterizationModeEXT,
);

pub type vkCmdSetLineStippleEnableEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, stippled_line_enable: Bool32);

pub type vkCmdSetDepthClipNegativeOneToOneEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, negative_one_to_one: Bool32);

pub type vkCmdSetViewportWScalingEnableNV =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, viewport_w_scaling_enable: Bool32);

pub type vkCmdSetViewportSwizzleNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    first_viewport: u32,
    viewport_count: u32,
    viewport_swizzles: *const ViewportSwizzleNV,
);

pub type vkCmdSetCoverageToColorEnableNV =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, coverage_to_color_enable: Bool32);

pub type vkCmdSetCoverageToColorLocationNV =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, coverage_to_color_location: u32);

pub type vkCmdSetCoverageModulationModeNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    coverage_modulation_mode: CoverageModulationModeNV,
);

pub type vkCmdSetCoverageModulationTableEnableNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    coverage_modulation_table_enable: Bool32,
);

pub type vkCmdSetCoverageModulationTableNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    coverage_modulation_table_count: u32,
    coverage_modulation_table: *const f32,
);

pub type vkCmdSetShadingRateImageEnableNV =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, shading_rate_image_enable: Bool32);

pub type vkCmdSetCoverageReductionModeNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    coverage_reduction_mode: CoverageReductionModeNV,
);

pub type vkCmdSetRepresentativeFragmentTestEnableNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    representative_fragment_test_enable: Bool32,
);

pub type vkCreatePrivateDataSlot = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const PrivateDataSlotCreateInfo,
    allocator: *const AllocationCallbacks,
    private_data_slot: *mut PrivateDataSlot,
) -> vkResult;

pub type vkCreatePrivateDataSlotEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const PrivateDataSlotCreateInfo,
    allocator: *const AllocationCallbacks,
    private_data_slot: *mut PrivateDataSlot,
) -> vkResult;

pub type vkDestroyPrivateDataSlot = unsafe extern "system" fn(
    device: vkDevice,
    private_data_slot: PrivateDataSlot,
    allocator: *const AllocationCallbacks,
);

pub type vkDestroyPrivateDataSlotEXT = unsafe extern "system" fn(
    device: vkDevice,
    private_data_slot: PrivateDataSlot,
    allocator: *const AllocationCallbacks,
);

pub type vkSetPrivateData = unsafe extern "system" fn(
    device: vkDevice,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> vkResult;

pub type vkSetPrivateDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: u64,
) -> vkResult;

pub type vkGetPrivateData = unsafe extern "system" fn(
    device: vkDevice,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: *mut u64,
);

pub type vkGetPrivateDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    object_type: ObjectType,
    object_handle: u64,
    private_data_slot: PrivateDataSlot,
    data: *mut u64,
);

pub type vkCmdCopyBuffer2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_buffer_info: *const CopyBufferInfo2,
);

pub type vkCmdCopyBuffer2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_buffer_info: *const CopyBufferInfo2,
);

pub type vkCmdCopyImage2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_image_info: *const CopyImageInfo2,
);

pub type vkCmdCopyImage2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_image_info: *const CopyImageInfo2,
);

pub type vkCmdBlitImage2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    blit_image_info: *const BlitImageInfo2,
);

pub type vkCmdBlitImage2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    blit_image_info: *const BlitImageInfo2,
);

pub type vkCmdCopyBufferToImage2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);

pub type vkCmdCopyBufferToImage2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_buffer_to_image_info: *const CopyBufferToImageInfo2,
);

pub type vkCmdCopyImageToBuffer2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);

pub type vkCmdCopyImageToBuffer2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_image_to_buffer_info: *const CopyImageToBufferInfo2,
);

pub type vkCmdResolveImage2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    resolve_image_info: *const ResolveImageInfo2,
);

pub type vkCmdResolveImage2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    resolve_image_info: *const ResolveImageInfo2,
);

pub type vkCmdRefreshObjectsKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    refresh_objects: *const RefreshObjectListKHR,
);

pub type vkGetPhysicalDeviceRefreshableObjectTypesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    refreshable_object_type_count: *mut u32,
    refreshable_object_types: *mut ObjectType,
) -> vkResult;

pub type vkCmdSetFragmentShadingRateKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    fragment_size: *const Extent2D,
    combiner_ops: FragmentShadingRateCombinerOpKHR,
);

pub type vkGetPhysicalDeviceFragmentShadingRatesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    fragment_shading_rate_count: *mut u32,
    fragment_shading_rates: *mut PhysicalDeviceFragmentShadingRateKHR,
) -> vkResult;

pub type vkCmdSetFragmentShadingRateEnumNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    shading_rate: FragmentShadingRateNV,
    combiner_ops: FragmentShadingRateCombinerOpKHR,
);

pub type vkGetAccelerationStructureBuildSizesKHR = unsafe extern "system" fn(
    device: vkDevice,
    build_type: AccelerationStructureBuildTypeKHR,
    build_info: *const AccelerationStructureBuildGeometryInfoKHR,
    max_primitive_counts: *const u32,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);

pub type vkCmdSetVertexInputEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    vertex_binding_description_count: u32,
    vertex_binding_descriptions: *const VertexInputBindingDescription2EXT,
    vertex_attribute_description_count: u32,
    vertex_attribute_descriptions: *const VertexInputAttributeDescription2EXT,
);

pub type vkCmdSetColorWriteEnableEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    attachment_count: u32,
    color_write_enables: *const Bool32,
);

pub type vkCmdSetEvent2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event: Event,
    dependency_info: *const DependencyInfo,
);

pub type vkCmdSetEvent2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event: Event,
    dependency_info: *const DependencyInfo,
);

pub type vkCmdResetEvent2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags2,
);

pub type vkCmdResetEvent2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event: Event,
    stage_mask: PipelineStageFlags2,
);

pub type vkCmdWaitEvents2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event_count: u32,
    events: *const Event,
    dependency_infos: *const DependencyInfo,
);

pub type vkCmdWaitEvents2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    event_count: u32,
    events: *const Event,
    dependency_infos: *const DependencyInfo,
);

pub type vkCmdPipelineBarrier2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    dependency_info: *const DependencyInfo,
);

pub type vkCmdPipelineBarrier2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    dependency_info: *const DependencyInfo,
);

pub type vkQueueSubmit2 = unsafe extern "system" fn(
    queue: vkQueue,
    submit_count: u32,
    submits: *const SubmitInfo2,
    fence: Fence,
) -> vkResult;

pub type vkQueueSubmit2KHR = unsafe extern "system" fn(
    queue: vkQueue,
    submit_count: u32,
    submits: *const SubmitInfo2,
    fence: Fence,
) -> vkResult;

pub type vkCmdWriteTimestamp2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);

pub type vkCmdWriteTimestamp2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    stage: PipelineStageFlags2,
    query_pool: QueryPool,
    query: u32,
);

pub type vkCmdWriteBufferMarker2AMD = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    stage: PipelineStageFlags2,
    dst_buffer: Buffer,
    dst_offset: DeviceSize,
    marker: u32,
);

pub type vkGetQueueCheckpointData2NV = unsafe extern "system" fn(
    queue: vkQueue,
    checkpoint_data_count: *mut u32,
    checkpoint_data: *mut CheckpointData2NV,
);

pub type vkCopyMemoryToImage = unsafe extern "system" fn(
    device: vkDevice,
    copy_memory_to_image_info: *const CopyMemoryToImageInfo,
) -> vkResult;

pub type vkCopyMemoryToImageEXT = unsafe extern "system" fn(
    device: vkDevice,
    copy_memory_to_image_info: *const CopyMemoryToImageInfo,
) -> vkResult;

pub type vkCopyImageToMemory = unsafe extern "system" fn(
    device: vkDevice,
    copy_image_to_memory_info: *const CopyImageToMemoryInfo,
) -> vkResult;

pub type vkCopyImageToMemoryEXT = unsafe extern "system" fn(
    device: vkDevice,
    copy_image_to_memory_info: *const CopyImageToMemoryInfo,
) -> vkResult;

pub type vkCopyImageToImage = unsafe extern "system" fn(
    device: vkDevice,
    copy_image_to_image_info: *const CopyImageToImageInfo,
) -> vkResult;

pub type vkCopyImageToImageEXT = unsafe extern "system" fn(
    device: vkDevice,
    copy_image_to_image_info: *const CopyImageToImageInfo,
) -> vkResult;

pub type vkTransitionImageLayout = unsafe extern "system" fn(
    device: vkDevice,
    transition_count: u32,
    transitions: *const HostImageLayoutTransitionInfo,
) -> vkResult;

pub type vkTransitionImageLayoutEXT = unsafe extern "system" fn(
    device: vkDevice,
    transition_count: u32,
    transitions: *const HostImageLayoutTransitionInfo,
) -> vkResult;

pub type vkGetCommandPoolMemoryConsumption = unsafe extern "system" fn(
    device: vkDevice,
    command_pool: CommandPool,
    command_buffer: vkCommandBuffer,
    consumption: *mut CommandPoolMemoryConsumption,
);

pub type vkGetPhysicalDeviceVideoCapabilitiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    video_profile: *const VideoProfileInfoKHR,
    capabilities: *mut VideoCapabilitiesKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceVideoFormatPropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    video_format_info: *const PhysicalDeviceVideoFormatInfoKHR,
    video_format_property_count: *mut u32,
    video_format_properties: *mut VideoFormatPropertiesKHR,
) -> vkResult;

pub type vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        quality_level_info: *const PhysicalDeviceVideoEncodeQualityLevelInfoKHR,
        quality_level_properties: *mut VideoEncodeQualityLevelPropertiesKHR,
    ) -> vkResult;

pub type vkCreateVideoSessionKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const VideoSessionCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    video_session: *mut VideoSessionKHR,
) -> vkResult;

pub type vkDestroyVideoSessionKHR = unsafe extern "system" fn(
    device: vkDevice,
    video_session: VideoSessionKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const VideoSessionParametersCreateInfoKHR,
    allocator: *const AllocationCallbacks,
    video_session_parameters: *mut VideoSessionParametersKHR,
) -> vkResult;

pub type vkUpdateVideoSessionParametersKHR = unsafe extern "system" fn(
    device: vkDevice,
    video_session_parameters: VideoSessionParametersKHR,
    update_info: *const VideoSessionParametersUpdateInfoKHR,
) -> vkResult;

pub type vkGetEncodedVideoSessionParametersKHR = unsafe extern "system" fn(
    device: vkDevice,
    video_session_parameters_info: *const VideoEncodeSessionParametersGetInfoKHR,
    feedback_info: *mut VideoEncodeSessionParametersFeedbackInfoKHR,
    data_size: *mut usize,
    data: *mut c_void,
) -> vkResult;

pub type vkDestroyVideoSessionParametersKHR = unsafe extern "system" fn(
    device: vkDevice,
    video_session_parameters: VideoSessionParametersKHR,
    allocator: *const AllocationCallbacks,
);

pub type vkGetVideoSessionMemoryRequirementsKHR = unsafe extern "system" fn(
    device: vkDevice,
    video_session: VideoSessionKHR,
    memory_requirements_count: *mut u32,
    memory_requirements: *mut VideoSessionMemoryRequirementsKHR,
) -> vkResult;

pub type vkBindVideoSessionMemoryKHR = unsafe extern "system" fn(
    device: vkDevice,
    video_session: VideoSessionKHR,
    bind_session_memory_info_count: u32,
    bind_session_memory_infos: *const BindVideoSessionMemoryInfoKHR,
) -> vkResult;

pub type vkCmdDecodeVideoKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    decode_info: *const VideoDecodeInfoKHR,
);

pub type vkCmdBeginVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    begin_info: *const VideoBeginCodingInfoKHR,
);

pub type vkCmdControlVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    coding_control_info: *const VideoCodingControlInfoKHR,
);

pub type vkCmdEndVideoCodingKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    end_coding_info: *const VideoEndCodingInfoKHR,
);

pub type vkCmdEncodeVideoKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    encode_info: *const VideoEncodeInfoKHR,
);

pub type vkCmdDecompressMemoryNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    decompress_region_count: u32,
    decompress_memory_regions: *const DecompressMemoryRegionNV,
);

pub type vkCmdDecompressMemoryIndirectCountNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    stride: u32,
);

pub type vkGetPartitionedAccelerationStructuresBuildSizesNV = unsafe extern "system" fn(
    device: vkDevice,
    info: *const PartitionedAccelerationStructureInstancesInputNV,
    size_info: *mut AccelerationStructureBuildSizesInfoKHR,
);

pub type vkCmdBuildPartitionedAccelerationStructuresNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    build_info: *const BuildPartitionedAccelerationStructureInfoNV,
);

pub type vkCmdDecompressMemoryEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    decompress_memory_info_ext: *const DecompressMemoryInfoEXT,
);

pub type vkCmdDecompressMemoryIndirectCountEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    decompression_method: MemoryDecompressionMethodFlagsEXT,
    indirect_commands_address: DeviceAddress,
    indirect_commands_count_address: DeviceAddress,
    max_decompression_count: u32,
    stride: u32,
);

pub type vkCreateCuModuleNVX = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const CuModuleCreateInfoNVX,
    allocator: *const AllocationCallbacks,
    module: *mut CuModuleNVX,
) -> vkResult;

pub type vkCreateCuFunctionNVX = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const CuFunctionCreateInfoNVX,
    allocator: *const AllocationCallbacks,
    function: *mut CuFunctionNVX,
) -> vkResult;

pub type vkDestroyCuModuleNVX = unsafe extern "system" fn(
    device: vkDevice,
    module: CuModuleNVX,
    allocator: *const AllocationCallbacks,
);

pub type vkDestroyCuFunctionNVX = unsafe extern "system" fn(
    device: vkDevice,
    function: CuFunctionNVX,
    allocator: *const AllocationCallbacks,
);

pub type vkCmdCuLaunchKernelNVX =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, launch_info: *const CuLaunchInfoNVX);

pub type vkGetDescriptorSetLayoutSizeEXT = unsafe extern "system" fn(
    device: vkDevice,
    layout: DescriptorSetLayout,
    layout_size_in_bytes: *mut DeviceSize,
);

pub type vkGetDescriptorSetLayoutBindingOffsetEXT = unsafe extern "system" fn(
    device: vkDevice,
    layout: DescriptorSetLayout,
    binding: u32,
    offset: *mut DeviceSize,
);

pub type vkGetDescriptorEXT = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_info: *const DescriptorGetInfoEXT,
    data_size: usize,
    descriptor: *mut c_void,
);

pub type vkCmdBindDescriptorBuffersEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    buffer_count: u32,
    binding_infos: *const DescriptorBufferBindingInfoEXT,
);

pub type vkCmdSetDescriptorBufferOffsetsEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    first_set: u32,
    set_count: u32,
    buffer_indices: *const u32,
    offsets: *const DeviceSize,
);

pub type vkCmdBindDescriptorBufferEmbeddedSamplersEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    pipeline_bind_point: PipelineBindPoint,
    layout: PipelineLayout,
    set: u32,
);

pub type vkGetBufferOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    info: *const BufferCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> vkResult;

pub type vkGetImageOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ImageCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> vkResult;

pub type vkGetImageViewOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ImageViewCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> vkResult;

pub type vkGetSamplerOpaqueCaptureDescriptorDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    info: *const SamplerCaptureDescriptorDataInfoEXT,
    data: *mut c_void,
) -> vkResult;

pub type vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT =
    unsafe extern "system" fn(
        device: vkDevice,
        info: *const AccelerationStructureCaptureDescriptorDataInfoEXT,
        data: *mut c_void,
    ) -> vkResult;

pub type vkSetDeviceMemoryPriorityEXT =
    unsafe extern "system" fn(device: vkDevice, memory: DeviceMemory, priority: f32);

pub type vkAcquireDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    drm_fd: i32,
    display: DisplayKHR,
) -> vkResult;

pub type vkGetDrmDisplayEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    drm_fd: i32,
    connector_id: u32,
    display: *mut DisplayKHR,
) -> vkResult;

pub type vkWaitForPresent2KHR = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    present_wait2_info: *const PresentWait2InfoKHR,
) -> vkResult;

pub type vkWaitForPresentKHR = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    present_id: u64,
    timeout: u64,
) -> vkResult;

pub type vkCreateBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const BufferCollectionCreateInfoFUCHSIA,
    allocator: *const AllocationCallbacks,
    collection: *mut BufferCollectionFUCHSIA,
) -> vkResult;

pub type vkSetBufferCollectionBufferConstraintsFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    collection: BufferCollectionFUCHSIA,
    buffer_constraints_info: *const BufferConstraintsInfoFUCHSIA,
) -> vkResult;

pub type vkSetBufferCollectionImageConstraintsFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    collection: BufferCollectionFUCHSIA,
    image_constraints_info: *const ImageConstraintsInfoFUCHSIA,
) -> vkResult;

pub type vkDestroyBufferCollectionFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    collection: BufferCollectionFUCHSIA,
    allocator: *const AllocationCallbacks,
);

pub type vkGetBufferCollectionPropertiesFUCHSIA = unsafe extern "system" fn(
    device: vkDevice,
    collection: BufferCollectionFUCHSIA,
    properties: *mut BufferCollectionPropertiesFUCHSIA,
) -> vkResult;

pub type vkCreateCudaModuleNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const CudaModuleCreateInfoNV,
    allocator: *const AllocationCallbacks,
    module: *mut CudaModuleNV,
) -> vkResult;

pub type vkGetCudaModuleCacheNV = unsafe extern "system" fn(
    device: vkDevice,
    module: CudaModuleNV,
    cache_size: *mut usize,
    cache_data: *mut c_void,
) -> vkResult;

pub type vkCreateCudaFunctionNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const CudaFunctionCreateInfoNV,
    allocator: *const AllocationCallbacks,
    function: *mut CudaFunctionNV,
) -> vkResult;

pub type vkDestroyCudaModuleNV = unsafe extern "system" fn(
    device: vkDevice,
    module: CudaModuleNV,
    allocator: *const AllocationCallbacks,
);

pub type vkDestroyCudaFunctionNV = unsafe extern "system" fn(
    device: vkDevice,
    function: CudaFunctionNV,
    allocator: *const AllocationCallbacks,
);

pub type vkCmdCudaLaunchKernelNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    launch_info: *const CudaLaunchInfoNV,
);

pub type vkCmdBeginRendering = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    rendering_info: *const RenderingInfo,
);

pub type vkCmdBeginRenderingKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    rendering_info: *const RenderingInfo,
);

pub type vkCmdEndRendering = unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkCmdEndRendering2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    rendering_end_info: *const RenderingEndInfoKHR,
);

pub type vkCmdEndRendering2EXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    rendering_end_info: *const RenderingEndInfoKHR,
);

pub type vkCmdEndRenderingKHR = unsafe extern "system" fn(command_buffer: vkCommandBuffer);

pub type vkGetDescriptorSetLayoutHostMappingInfoVALVE = unsafe extern "system" fn(
    device: vkDevice,
    binding_reference: *const DescriptorSetBindingReferenceVALVE,
    host_mapping: *mut DescriptorSetLayoutHostMappingInfoVALVE,
);

pub type vkGetDescriptorSetHostMappingVALVE = unsafe extern "system" fn(
    device: vkDevice,
    descriptor_set: DescriptorSet,
    pp_data: *mut *mut c_void,
);

pub type vkCreateMicromapEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const MicromapCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    micromap: *mut MicromapEXT,
) -> vkResult;

pub type vkCmdBuildMicromapsEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info_count: u32,
    infos: *const MicromapBuildInfoEXT,
);

pub type vkBuildMicromapsEXT = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info_count: u32,
    infos: *const MicromapBuildInfoEXT,
) -> vkResult;

pub type vkDestroyMicromapEXT = unsafe extern "system" fn(
    device: vkDevice,
    micromap: MicromapEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkCmdCopyMicromapEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, info: *const CopyMicromapInfoEXT);

pub type vkCopyMicromapEXT = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMicromapInfoEXT,
) -> vkResult;

pub type vkCmdCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info: *const CopyMicromapToMemoryInfoEXT,
);

pub type vkCopyMicromapToMemoryEXT = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMicromapToMemoryInfoEXT,
) -> vkResult;

pub type vkCmdCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info: *const CopyMemoryToMicromapInfoEXT,
);

pub type vkCopyMemoryToMicromapEXT = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    info: *const CopyMemoryToMicromapInfoEXT,
) -> vkResult;

pub type vkCmdWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    micromap_count: u32,
    micromaps: *const MicromapEXT,
    query_type: QueryType,
    query_pool: QueryPool,
    first_query: u32,
);

pub type vkWriteMicromapsPropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    micromap_count: u32,
    micromaps: *const MicromapEXT,
    query_type: QueryType,
    data_size: usize,
    data: *mut c_void,
    stride: usize,
) -> vkResult;

pub type vkGetDeviceMicromapCompatibilityEXT = unsafe extern "system" fn(
    device: vkDevice,
    version_info: *const MicromapVersionInfoEXT,
    compatibility: *mut AccelerationStructureCompatibilityKHR,
);

pub type vkGetMicromapBuildSizesEXT = unsafe extern "system" fn(
    device: vkDevice,
    build_type: AccelerationStructureBuildTypeKHR,
    build_info: *const MicromapBuildInfoEXT,
    size_info: *mut MicromapBuildSizesInfoEXT,
);

pub type vkGetShaderModuleIdentifierEXT = unsafe extern "system" fn(
    device: vkDevice,
    shader_module: ShaderModule,
    identifier: *mut ShaderModuleIdentifierEXT,
);

pub type vkGetShaderModuleCreateInfoIdentifierEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ShaderModuleCreateInfo,
    identifier: *mut ShaderModuleIdentifierEXT,
);

pub type vkGetImageSubresourceLayout2 = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
);

pub type vkGetImageSubresourceLayout2KHR = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
);

pub type vkGetImageSubresourceLayout2EXT = unsafe extern "system" fn(
    device: vkDevice,
    image: Image,
    subresource: *const ImageSubresource2,
    layout: *mut SubresourceLayout2,
);

pub type vkGetPipelinePropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_info: *const PipelineInfoEXT,
    pipeline_properties: *mut BaseOutStructure,
) -> vkResult;

pub type vkExportMetalObjectsEXT =
    unsafe extern "system" fn(device: vkDevice, metal_objects_info: *mut ExportMetalObjectsInfoEXT);

pub type vkCmdBindTileMemoryQCOM = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    tile_memory_bind_info: *const TileMemoryBindInfoQCOM,
);

pub type vkGetFramebufferTilePropertiesQCOM = unsafe extern "system" fn(
    device: vkDevice,
    framebuffer: Framebuffer,
    properties_count: *mut u32,
    properties: *mut TilePropertiesQCOM,
) -> vkResult;

pub type vkGetDynamicRenderingTilePropertiesQCOM = unsafe extern "system" fn(
    device: vkDevice,
    rendering_info: *const RenderingInfo,
    properties: *mut TilePropertiesQCOM,
) -> vkResult;

pub type vkGetPhysicalDeviceOpticalFlowImageFormatsNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    optical_flow_image_format_info: *const OpticalFlowImageFormatInfoNV,
    format_count: *mut u32,
    image_format_properties: *mut OpticalFlowImageFormatPropertiesNV,
) -> vkResult;

pub type vkCreateOpticalFlowSessionNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const OpticalFlowSessionCreateInfoNV,
    allocator: *const AllocationCallbacks,
    session: *mut OpticalFlowSessionNV,
) -> vkResult;

pub type vkDestroyOpticalFlowSessionNV = unsafe extern "system" fn(
    device: vkDevice,
    session: OpticalFlowSessionNV,
    allocator: *const AllocationCallbacks,
);

pub type vkBindOpticalFlowSessionImageNV = unsafe extern "system" fn(
    device: vkDevice,
    session: OpticalFlowSessionNV,
    binding_point: OpticalFlowSessionBindingPointNV,
    view: ImageView,
    layout: ImageLayout,
) -> vkResult;

pub type vkCmdOpticalFlowExecuteNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    session: OpticalFlowSessionNV,
    execute_info: *const OpticalFlowExecuteInfoNV,
);

pub type vkGetDeviceFaultInfoEXT = unsafe extern "system" fn(
    device: vkDevice,
    fault_counts: *mut DeviceFaultCountsEXT,
    fault_info: *mut DeviceFaultInfoEXT,
) -> vkResult;

pub type vkCmdSetDepthBias2EXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    depth_bias_info: *const DepthBiasInfoEXT,
);

pub type vkReleaseSwapchainImagesKHR = unsafe extern "system" fn(
    device: vkDevice,
    release_info: *const ReleaseSwapchainImagesInfoKHR,
) -> vkResult;

pub type vkReleaseSwapchainImagesEXT = unsafe extern "system" fn(
    device: vkDevice,
    release_info: *const ReleaseSwapchainImagesInfoKHR,
) -> vkResult;

pub type vkGetDeviceImageSubresourceLayout = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceImageSubresourceInfo,
    layout: *mut SubresourceLayout2,
);

pub type vkGetDeviceImageSubresourceLayoutKHR = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceImageSubresourceInfo,
    layout: *mut SubresourceLayout2,
);

pub type vkMapMemory2 = unsafe extern "system" fn(
    device: vkDevice,
    memory_map_info: *const MemoryMapInfo,
    pp_data: *mut *mut c_void,
) -> vkResult;

pub type vkMapMemory2KHR = unsafe extern "system" fn(
    device: vkDevice,
    memory_map_info: *const MemoryMapInfo,
    pp_data: *mut *mut c_void,
) -> vkResult;

pub type vkUnmapMemory2 = unsafe extern "system" fn(
    device: vkDevice,
    memory_unmap_info: *const MemoryUnmapInfo,
) -> vkResult;

pub type vkUnmapMemory2KHR = unsafe extern "system" fn(
    device: vkDevice,
    memory_unmap_info: *const MemoryUnmapInfo,
) -> vkResult;

pub type vkCreateShadersEXT = unsafe extern "system" fn(
    device: vkDevice,
    create_info_count: u32,
    create_infos: *const ShaderCreateInfoEXT,
    allocator: *const AllocationCallbacks,
    shaders: *mut ShaderEXT,
) -> vkResult;

pub type vkDestroyShaderEXT = unsafe extern "system" fn(
    device: vkDevice,
    shader: ShaderEXT,
    allocator: *const AllocationCallbacks,
);

pub type vkGetShaderBinaryDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    shader: ShaderEXT,
    data_size: *mut usize,
    data: *mut c_void,
) -> vkResult;

pub type vkCmdBindShadersEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    stage_count: u32,
    stages: *const ShaderStageFlags,
    shaders: *const ShaderEXT,
);

pub type vkSetSwapchainPresentTimingQueueSizeEXT =
    unsafe extern "system" fn(device: vkDevice, swapchain: SwapchainKHR, size: u32) -> vkResult;

pub type vkGetSwapchainTimingPropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    swapchain_timing_properties: *mut SwapchainTimingPropertiesEXT,
    swapchain_timing_properties_counter: *mut u64,
) -> vkResult;

pub type vkGetSwapchainTimeDomainPropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    swapchain_time_domain_properties: *mut SwapchainTimeDomainPropertiesEXT,
    time_domains_counter: *mut u64,
) -> vkResult;

pub type vkGetPastPresentationTimingEXT = unsafe extern "system" fn(
    device: vkDevice,
    past_presentation_timing_info: *const PastPresentationTimingInfoEXT,
    past_presentation_timing_properties: *mut PastPresentationTimingPropertiesEXT,
) -> vkResult;

pub type vkGetScreenBufferPropertiesQNX = unsafe extern "system" fn(
    device: vkDevice,
    buffer: *const _screen_buffer,
    properties: *mut ScreenBufferPropertiesQNX,
) -> vkResult;

pub type vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut CooperativeMatrixPropertiesKHR,
) -> vkResult;

pub type vkGetExecutionGraphPipelineScratchSizeAMDX = unsafe extern "system" fn(
    device: vkDevice,
    execution_graph: Pipeline,
    size_info: *mut ExecutionGraphPipelineScratchSizeAMDX,
) -> vkResult;

pub type vkGetExecutionGraphPipelineNodeIndexAMDX = unsafe extern "system" fn(
    device: vkDevice,
    execution_graph: Pipeline,
    node_info: *const PipelineShaderStageNodeCreateInfoAMDX,
    node_index: *mut u32,
) -> vkResult;

pub type vkCreateExecutionGraphPipelinesAMDX = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const ExecutionGraphPipelineCreateInfoAMDX,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> vkResult;

pub type vkCmdInitializeGraphScratchMemoryAMDX = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    execution_graph: Pipeline,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
);

pub type vkCmdDispatchGraphAMDX = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: *const DispatchGraphCountInfoAMDX,
);

pub type vkCmdDispatchGraphIndirectAMDX = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: *const DispatchGraphCountInfoAMDX,
);

pub type vkCmdDispatchGraphIndirectCountAMDX = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    scratch: DeviceAddress,
    scratch_size: DeviceSize,
    count_info: DeviceAddress,
);

pub type vkCmdBindDescriptorSets2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
);

pub type vkCmdBindDescriptorSets2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    bind_descriptor_sets_info: *const BindDescriptorSetsInfo,
);

pub type vkCmdPushConstants2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_constants_info: *const PushConstantsInfo,
);

pub type vkCmdPushConstants2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_constants_info: *const PushConstantsInfo,
);

pub type vkCmdPushDescriptorSet2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_descriptor_set_info: *const PushDescriptorSetInfo,
);

pub type vkCmdPushDescriptorSet2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_descriptor_set_info: *const PushDescriptorSetInfo,
);

pub type vkCmdPushDescriptorSetWithTemplate2 = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
);

pub type vkCmdPushDescriptorSetWithTemplate2KHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_descriptor_set_with_template_info: *const PushDescriptorSetWithTemplateInfo,
);

pub type vkCmdSetDescriptorBufferOffsets2EXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    set_descriptor_buffer_offsets_info: *const SetDescriptorBufferOffsetsInfoEXT,
);

pub type vkCmdBindDescriptorBufferEmbeddedSamplers2EXT = unsafe extern "system" fn(command_buffer: vkCommandBuffer, bind_descriptor_buffer_embedded_samplers_info: *const BindDescriptorBufferEmbeddedSamplersInfoEXT);

pub type vkSetLatencySleepModeNV = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    sleep_mode_info: *const LatencySleepModeInfoNV,
) -> vkResult;

pub type vkLatencySleepNV = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    sleep_info: *const LatencySleepInfoNV,
) -> vkResult;

pub type vkSetLatencyMarkerNV = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    latency_marker_info: *const SetLatencyMarkerInfoNV,
);

pub type vkGetLatencyTimingsNV = unsafe extern "system" fn(
    device: vkDevice,
    swapchain: SwapchainKHR,
    latency_marker_info: *mut GetLatencyMarkerInfoNV,
);

pub type vkQueueNotifyOutOfBandNV =
    unsafe extern "system" fn(queue: vkQueue, queue_type_info: *const OutOfBandQueueTypeInfoNV);

pub type vkCmdSetRenderingAttachmentLocations = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    location_info: *const RenderingAttachmentLocationInfo,
);

pub type vkCmdSetRenderingAttachmentLocationsKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    location_info: *const RenderingAttachmentLocationInfo,
);

pub type vkCmdSetRenderingInputAttachmentIndices = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
);

pub type vkCmdSetRenderingInputAttachmentIndicesKHR = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    input_attachment_index_info: *const RenderingInputAttachmentIndexInfo,
);

pub type vkCmdSetDepthClampRangeEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    depth_clamp_mode: DepthClampModeEXT,
    depth_clamp_range: *const DepthClampRangeEXT,
);

pub type vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        property_count: *mut u32,
        properties: *mut CooperativeMatrixFlexibleDimensionsPropertiesNV,
    ) -> vkResult;

pub type vkGetMemoryMetalHandleEXT = unsafe extern "system" fn(
    device: vkDevice,
    get_metal_handle_info: *const MemoryGetMetalHandleInfoEXT,
    handle: *mut *mut c_void,
) -> vkResult;

pub type vkGetMemoryMetalHandlePropertiesEXT = unsafe extern "system" fn(
    device: vkDevice,
    handle_type: ExternalMemoryHandleTypeFlags,
    handle: *const c_void,
    memory_metal_handle_properties: *mut MemoryMetalHandlePropertiesEXT,
) -> vkResult;

pub type vkGetPhysicalDeviceCooperativeVectorPropertiesNV = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    property_count: *mut u32,
    properties: *mut CooperativeVectorPropertiesNV,
) -> vkResult;

pub type vkConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    device: vkDevice,
    info: *const ConvertCooperativeVectorMatrixInfoNV,
) -> vkResult;

pub type vkCmdConvertCooperativeVectorMatrixNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    info_count: u32,
    infos: *const ConvertCooperativeVectorMatrixInfoNV,
);

pub type vkCmdDispatchTileQCOM = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    dispatch_tile_info: *const DispatchTileInfoQCOM,
);

pub type vkCmdBeginPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    per_tile_begin_info: *const PerTileBeginInfoQCOM,
);

pub type vkCmdEndPerTileExecutionQCOM = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    per_tile_end_info: *const PerTileEndInfoQCOM,
);

pub type vkCreateExternalComputeQueueNV = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const ExternalComputeQueueCreateInfoNV,
    allocator: *const AllocationCallbacks,
    external_queue: *mut ExternalComputeQueueNV,
) -> vkResult;

pub type vkDestroyExternalComputeQueueNV = unsafe extern "system" fn(
    device: vkDevice,
    external_queue: ExternalComputeQueueNV,
    allocator: *const AllocationCallbacks,
);

pub type vkGetExternalComputeQueueDataNV = unsafe extern "system" fn(
    external_queue: ExternalComputeQueueNV,
    params: *mut ExternalComputeQueueDataParamsNV,
    data: *mut c_void,
);

pub type vkCreateTensorARM = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const TensorCreateInfoARM,
    allocator: *const AllocationCallbacks,
    tensor: *mut TensorARM,
) -> vkResult;

pub type vkDestroyTensorARM = unsafe extern "system" fn(
    device: vkDevice,
    tensor: TensorARM,
    allocator: *const AllocationCallbacks,
);

pub type vkCreateTensorViewARM = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const TensorViewCreateInfoARM,
    allocator: *const AllocationCallbacks,
    view: *mut TensorViewARM,
) -> vkResult;

pub type vkDestroyTensorViewARM = unsafe extern "system" fn(
    device: vkDevice,
    tensor_view: TensorViewARM,
    allocator: *const AllocationCallbacks,
);

pub type vkGetTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: vkDevice,
    info: *const TensorMemoryRequirementsInfoARM,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkBindTensorMemoryARM = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindTensorMemoryInfoARM,
) -> vkResult;

pub type vkGetDeviceTensorMemoryRequirementsARM = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DeviceTensorMemoryRequirementsARM,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkCmdCopyTensorARM = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    copy_tensor_info: *const CopyTensorInfoARM,
);

pub type vkGetTensorOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: vkDevice,
    info: *const TensorCaptureDescriptorDataInfoARM,
    data: *mut c_void,
) -> vkResult;

pub type vkGetTensorViewOpaqueCaptureDescriptorDataARM = unsafe extern "system" fn(
    device: vkDevice,
    info: *const TensorViewCaptureDescriptorDataInfoARM,
    data: *mut c_void,
) -> vkResult;

pub type vkGetPhysicalDeviceExternalTensorPropertiesARM = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    external_tensor_info: *const PhysicalDeviceExternalTensorInfoARM,
    external_tensor_properties: *mut ExternalTensorPropertiesARM,
);

pub type vkCreateDataGraphPipelinesARM = unsafe extern "system" fn(
    device: vkDevice,
    deferred_operation: DeferredOperationKHR,
    pipeline_cache: PipelineCache,
    create_info_count: u32,
    create_infos: *const DataGraphPipelineCreateInfoARM,
    allocator: *const AllocationCallbacks,
    pipelines: *mut Pipeline,
) -> vkResult;

pub type vkCreateDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: vkDevice,
    create_info: *const DataGraphPipelineSessionCreateInfoARM,
    allocator: *const AllocationCallbacks,
    session: *mut DataGraphPipelineSessionARM,
) -> vkResult;

pub type vkGetDataGraphPipelineSessionBindPointRequirementsARM =
    unsafe extern "system" fn(
        device: vkDevice,
        info: *const DataGraphPipelineSessionBindPointRequirementsInfoARM,
        bind_point_requirement_count: *mut u32,
        bind_point_requirements: *mut DataGraphPipelineSessionBindPointRequirementARM,
    ) -> vkResult;

pub type vkGetDataGraphPipelineSessionMemoryRequirementsARM = unsafe extern "system" fn(
    device: vkDevice,
    info: *const DataGraphPipelineSessionMemoryRequirementsInfoARM,
    memory_requirements: *mut MemoryRequirements2,
);

pub type vkBindDataGraphPipelineSessionMemoryARM = unsafe extern "system" fn(
    device: vkDevice,
    bind_info_count: u32,
    bind_infos: *const BindDataGraphPipelineSessionMemoryInfoARM,
) -> vkResult;

pub type vkDestroyDataGraphPipelineSessionARM = unsafe extern "system" fn(
    device: vkDevice,
    session: DataGraphPipelineSessionARM,
    allocator: *const AllocationCallbacks,
);

pub type vkCmdDispatchDataGraphARM = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    session: DataGraphPipelineSessionARM,
    info: *const DataGraphPipelineDispatchInfoARM,
);

pub type vkGetDataGraphPipelineAvailablePropertiesARM = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: *mut u32,
    properties: *mut DataGraphPipelinePropertyARM,
) -> vkResult;

pub type vkGetDataGraphPipelinePropertiesARM = unsafe extern "system" fn(
    device: vkDevice,
    pipeline_info: *const DataGraphPipelineInfoARM,
    properties_count: u32,
    properties: *mut DataGraphPipelinePropertyQueryResultARM,
) -> vkResult;

pub type vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        queue_family_index: u32,
        queue_family_data_graph_property_count: *mut u32,
        queue_family_data_graph_properties: *mut QueueFamilyDataGraphPropertiesARM,
    ) -> vkResult;

pub type vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM = unsafe extern "system" fn(physical_device: vkPhysicalDevice, queue_family_data_graph_processing_engine_info: *const PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM, queue_family_data_graph_processing_engine_properties: *mut QueueFamilyDataGraphProcessingEnginePropertiesARM);

pub type vkGetNativeBufferPropertiesOHOS = unsafe extern "system" fn(
    device: vkDevice,
    buffer: *const OH_NativeBuffer,
    properties: *mut NativeBufferPropertiesOHOS,
) -> vkResult;

pub type vkGetMemoryNativeBufferOHOS = unsafe extern "system" fn(
    device: vkDevice,
    info: *const MemoryGetNativeBufferInfoOHOS,
    buffer: *mut *mut OH_NativeBuffer,
) -> vkResult;

pub type vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM =
    unsafe extern "system" fn(
        physical_device: vkPhysicalDevice,
        queue_family_index: u32,
        counter_count: *mut u32,
        counters: *mut PerformanceCounterARM,
        counter_descriptions: *mut PerformanceCounterDescriptionARM,
    ) -> vkResult;

pub type vkCmdSetComputeOccupancyPriorityNV = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    parameters: *const ComputeOccupancyPriorityParametersNV,
);

pub type vkWriteSamplerDescriptorsEXT = unsafe extern "system" fn(
    device: vkDevice,
    sampler_count: u32,
    samplers: *const SamplerCreateInfo,
    descriptors: *const HostAddressRangeEXT,
) -> vkResult;

pub type vkWriteResourceDescriptorsEXT = unsafe extern "system" fn(
    device: vkDevice,
    resource_count: u32,
    resources: *const ResourceDescriptorInfoEXT,
    descriptors: *const HostAddressRangeEXT,
) -> vkResult;

pub type vkCmdBindSamplerHeapEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, bind_info: *const BindHeapInfoEXT);

pub type vkCmdBindResourceHeapEXT =
    unsafe extern "system" fn(command_buffer: vkCommandBuffer, bind_info: *const BindHeapInfoEXT);

pub type vkCmdPushDataEXT = unsafe extern "system" fn(
    command_buffer: vkCommandBuffer,
    push_data_info: *const PushDataInfoEXT,
);

pub type vkRegisterCustomBorderColorEXT = unsafe extern "system" fn(
    device: vkDevice,
    border_color: *const SamplerCustomBorderColorCreateInfoEXT,
    request_index: Bool32,
    index: *mut u32,
) -> vkResult;

pub type vkUnregisterCustomBorderColorEXT = unsafe extern "system" fn(device: vkDevice, index: u32);

pub type vkGetImageOpaqueCaptureDataEXT = unsafe extern "system" fn(
    device: vkDevice,
    image_count: u32,
    images: *const Image,
    datas: *mut HostAddressRangeEXT,
) -> vkResult;

pub type vkGetPhysicalDeviceDescriptorSizeEXT = unsafe extern "system" fn(
    physical_device: vkPhysicalDevice,
    descriptor_type: DescriptorType,
) -> DeviceSize;

pub type vkGetTensorOpaqueCaptureDataARM = unsafe extern "system" fn(
    device: vkDevice,
    tensor_count: u32,
    tensors: *const TensorARM,
    datas: *mut HostAddressRangeEXT,
) -> vkResult;
