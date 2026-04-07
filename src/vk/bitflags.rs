// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/bitflags.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![allow(non_upper_case_globals)]
use crate::{
    vk::types::{Flags, Flags64},
    vk_bitflags_wrapped,
};

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFramebufferCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FramebufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FramebufferCreateFlags, Flags);

impl FramebufferCreateFlags {
    pub const Imageless: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct QueryPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryPoolCreateFlags, Flags);

impl QueryPoolCreateFlags {
    pub const ResetKHR: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderPassCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct RenderPassCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(RenderPassCreateFlags, Flags);

impl RenderPassCreateFlags {
    pub const TransformQCOM: Self = Self(0b10);
    pub const PerLayerFragmentDensityVALVE: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SamplerCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SamplerCreateFlags, Flags);

impl SamplerCreateFlags {
    pub const SubsampledEXT: Self = Self(0b1);
    pub const SubsampledCoarseReconstructionEXT: Self = Self(0b10);
    pub const DescriptorBufferCaptureReplayEXT: Self = Self(0b1000);
    pub const NonSeamlessCubeMapEXT: Self = Self(0b100);
    pub const ImageProcessingQCOM: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineLayoutCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineLayoutCreateFlags, Flags);

impl PipelineLayoutCreateFlags {
    pub const IndependentSetsEXT: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineCacheCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCacheCreateFlags, Flags);

impl PipelineCacheCreateFlags {
    pub const ExternallySynchronized: Self = Self(0b1);
    pub const ReadOnly: Self = Self(0b10);
    pub const UseApplicationStorage: Self = Self(0b100);
    pub const InternallySynchronizedMergeKHR: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineShaderStageCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineShaderStageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineShaderStageCreateFlags, Flags);

impl PipelineShaderStageCreateFlags {
    pub const AllowVaryingSubgroupSize: Self = Self(0b1);
    pub const RequireFullSubgroups: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorSetLayoutCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DescriptorSetLayoutCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorSetLayoutCreateFlags, Flags);

impl DescriptorSetLayoutCreateFlags {
    pub const UpdateAfterBindPool: Self = Self(0b10);
    pub const PushDescriptor: Self = Self(0b1);
    pub const DescriptorBufferEXT: Self = Self(0b1_0000);
    pub const EmbeddedImmutableSamplersEXT: Self = Self(0b10_0000);
    pub const IndirectBindableNV: Self = Self(0b1000_0000);
    pub const HostOnlyPoolEXT: Self = Self(0b100);
    pub const PerStageNV: Self = Self(0b100_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInstanceCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct InstanceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(InstanceCreateFlags, Flags);

impl InstanceCreateFlags {
    pub const EnumeratePortabilityKHR: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceQueueCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeviceQueueCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceQueueCreateFlags, Flags);

impl DeviceQueueCreateFlags {
    /// Queue is a protected-capable device queue
    pub const Protected: Self = Self(0b1);
    pub const InternallySynchronizedKHR: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct QueueFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueueFlags, Flags);

impl QueueFlags {
    /// Queue supports graphics operations
    pub const Graphics: Self = Self(0b1);
    /// Queue supports compute operations
    pub const Compute: Self = Self(0b10);
    /// Queue supports transfer operations
    pub const Transfer: Self = Self(0b100);
    /// Queue supports sparse resource memory management operations
    pub const SparseBinding: Self = Self(0b1000);
    /// Queues may support protected operations
    pub const Protected: Self = Self(0b1_0000);
    pub const VideoDecodeKHR: Self = Self(0b10_0000);
    pub const VideoEncodeKHR: Self = Self(0b100_0000);
    pub const OpticalFlowNV: Self = Self(0b1_0000_0000);
    pub const DataGraphARM: Self = Self(0b100_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryPropertyFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemoryPropertyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryPropertyFlags, Flags);

impl MemoryPropertyFlags {
    /// If otherwise stated, then allocate memory on device
    pub const DeviceLocal: Self = Self(0b1);
    /// Memory is mappable by host
    pub const HostVisible: Self = Self(0b10);
    /// Memory will have i/o coherency. If not set, application may need to use vkFlushMappedMemoryRanges and vkInvalidateMappedMemoryRanges to flush/invalidate host cache
    pub const HostCoherent: Self = Self(0b100);
    /// Memory will be cached by the host
    pub const HostCached: Self = Self(0b1000);
    /// Memory may be allocated by the driver when it is required
    pub const LazilyAllocated: Self = Self(0b1_0000);
    /// Memory is protected
    pub const Protected: Self = Self(0b10_0000);
    pub const DeviceCoherentAMD: Self = Self(0b100_0000);
    pub const DeviceUncachedAMD: Self = Self(0b1000_0000);
    pub const RdmaCapableNV: Self = Self(0b1_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryHeapFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemoryHeapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryHeapFlags, Flags);

impl MemoryHeapFlags {
    /// If set, heap represents device memory
    pub const DeviceLocal: Self = Self(0b1);
    /// If set, heap allocations allocate multiple instances by default
    pub const MultiInstance: Self = Self(0b10);
    pub const SeuSafe: Self = Self(0b100);
    pub const TileMemoryQCOM: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AccessFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AccessFlags, Flags);

impl AccessFlags {
    /// Controls coherency of indirect command reads
    pub const IndirectCommandRead: Self = Self(0b1);
    /// Controls coherency of index reads
    pub const IndexRead: Self = Self(0b10);
    /// Controls coherency of vertex attribute reads
    pub const VertexAttributeRead: Self = Self(0b100);
    /// Controls coherency of uniform buffer reads
    pub const UniformRead: Self = Self(0b1000);
    /// Controls coherency of input attachment reads
    pub const InputAttachmentRead: Self = Self(0b1_0000);
    /// Controls coherency of shader reads
    pub const ShaderRead: Self = Self(0b10_0000);
    /// Controls coherency of shader writes
    pub const ShaderWrite: Self = Self(0b100_0000);
    /// Controls coherency of color attachment reads
    pub const ColorAttachmentRead: Self = Self(0b1000_0000);
    /// Controls coherency of color attachment writes
    pub const ColorAttachmentWrite: Self = Self(0b1_0000_0000);
    /// Controls coherency of depth/stencil attachment reads
    pub const DepthStencilAttachmentRead: Self = Self(0b10_0000_0000);
    /// Controls coherency of depth/stencil attachment writes
    pub const DepthStencilAttachmentWrite: Self = Self(0b100_0000_0000);
    /// Controls coherency of transfer reads
    pub const TransferRead: Self = Self(0b1000_0000_0000);
    /// Controls coherency of transfer writes
    pub const TransferWrite: Self = Self(0b1_0000_0000_0000);
    /// Controls coherency of host reads
    pub const HostRead: Self = Self(0b10_0000_0000_0000);
    /// Controls coherency of host writes
    pub const HostWrite: Self = Self(0b100_0000_0000_0000);
    /// Controls coherency of memory reads
    pub const MemoryRead: Self = Self(0b1000_0000_0000_0000);
    /// Controls coherency of memory writes
    pub const MemoryWrite: Self = Self(0b1_0000_0000_0000_0000);
    pub const None: Self = Self(0);
    pub const TransformFeedbackWriteEXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const TransformFeedbackCounterReadEXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const TransformFeedbackCounterWriteEXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    /// read access flag for reading conditional rendering predicate
    pub const ConditionalRenderingReadEXT: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const ColorAttachmentReadNoncoherentEXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const AccelerationStructureReadKHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const AccelerationStructureWriteKHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const FragmentDensityMapReadEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const FragmentShadingRateAttachmentReadKHR: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const CommandPreprocessReadEXT: Self = Self(0b10_0000_0000_0000_0000);
    pub const CommandPreprocessWriteEXT: Self = Self(0b100_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferUsageFlags, Flags);

impl BufferUsageFlags {
    /// Can be used as a source of transfer operations
    pub const TransferSrc: Self = Self(0b1);
    /// Can be used as a destination of transfer operations
    pub const TransferDst: Self = Self(0b10);
    /// Can be used as TBO
    pub const UniformTexelBuffer: Self = Self(0b100);
    /// Can be used as IBO
    pub const StorageTexelBuffer: Self = Self(0b1000);
    /// Can be used as UBO
    pub const UniformBuffer: Self = Self(0b1_0000);
    /// Can be used as SSBO
    pub const StorageBuffer: Self = Self(0b10_0000);
    /// Can be used as source of fixed-function index fetch (index buffer)
    pub const IndexBuffer: Self = Self(0b100_0000);
    /// Can be used as source of fixed-function vertex fetch (VBO)
    pub const VertexBuffer: Self = Self(0b1000_0000);
    /// Can be the source of indirect parameters (e.g. indirect buffer, parameter buffer)
    pub const IndirectBuffer: Self = Self(0b1_0000_0000);
    pub const ShaderDeviceAddress: Self = Self(0b10_0000_0000_0000_0000);
    pub const VideoDecodeSrcKHR: Self = Self(0b10_0000_0000_0000);
    pub const VideoDecodeDstKHR: Self = Self(0b100_0000_0000_0000);
    pub const TransformFeedbackBufferEXT: Self = Self(0b1000_0000_0000);
    pub const TransformFeedbackCounterBufferEXT: Self = Self(0b1_0000_0000_0000);
    /// Specifies the buffer can be used as predicate in conditional rendering
    pub const ConditionalRenderingEXT: Self = Self(0b10_0000_0000);
    pub const ExecutionGraphScratchAMDX: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const DescriptorHeapEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
    pub const AccelerationStructureBuildInputReadOnlyKHR: Self = Self(0b1000_0000_0000_0000_0000);
    pub const AccelerationStructureStorageKHR: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const ShaderBindingTableKHR: Self = Self(0b100_0000_0000);
    pub const VideoEncodeDstKHR: Self = Self(0b1000_0000_0000_0000);
    pub const VideoEncodeSrcKHR: Self = Self(0b1_0000_0000_0000_0000);
    pub const SamplerDescriptorBufferEXT: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const ResourceDescriptorBufferEXT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const PushDescriptorsDescriptorBufferEXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const MicromapBuildInputReadOnlyEXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const MicromapStorageEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const TileMemoryQCOM: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BufferCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(BufferCreateFlags, Flags);

impl BufferCreateFlags {
    /// Buffer should support sparse backing
    pub const SparseBinding: Self = Self(0b1);
    /// Buffer should support sparse backing with partial residency
    pub const SparseResidency: Self = Self(0b10);
    /// Buffer should support constant data access to physical memory ranges mapped into multiple locations of sparse buffers
    pub const SparseAliased: Self = Self(0b100);
    /// Buffer requires protected memory
    pub const Protected: Self = Self(0b1000);
    pub const DeviceAddressCaptureReplay: Self = Self(0b1_0000);
    pub const DescriptorBufferCaptureReplayEXT: Self = Self(0b10_0000);
    pub const VideoProfileIndependentKHR: Self = Self(0b100_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderStageFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ShaderStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderStageFlags, Flags);

impl ShaderStageFlags {
    pub const Vertex: Self = Self(0b1);
    pub const TessellationControl: Self = Self(0b10);
    pub const TessellationEvaluation: Self = Self(0b100);
    pub const Geometry: Self = Self(0b1000);
    pub const Fragment: Self = Self(0b1_0000);
    pub const Compute: Self = Self(0b10_0000);
    pub const AllGraphics: Self = Self(0b1_1111);
    pub const All: Self = Self(0b111_1111_1111_1111_1111_1111_1111_1111);
    pub const RaygenKHR: Self = Self(0b1_0000_0000);
    pub const AnyHitKHR: Self = Self(0b10_0000_0000);
    pub const ClosestHitKHR: Self = Self(0b100_0000_0000);
    pub const MissKHR: Self = Self(0b1000_0000_0000);
    pub const IntersectionKHR: Self = Self(0b1_0000_0000_0000);
    pub const CallableKHR: Self = Self(0b10_0000_0000_0000);
    pub const TaskEXT: Self = Self(0b100_0000);
    pub const MeshEXT: Self = Self(0b1000_0000);
    pub const SubpassShadingHUAWEI: Self = Self(0b100_0000_0000_0000);
    pub const ClusterCullingHUAWEI: Self = Self(0b1000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageUsageFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageUsageFlags, Flags);

impl ImageUsageFlags {
    /// Can be used as a source of transfer operations
    pub const TransferSrc: Self = Self(0b1);
    /// Can be used as a destination of transfer operations
    pub const TransferDst: Self = Self(0b10);
    /// Can be sampled from (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
    pub const Sampled: Self = Self(0b100);
    /// Can be used as storage image (STORAGE_IMAGE descriptor type)
    pub const Storage: Self = Self(0b1000);
    /// Can be used as framebuffer color attachment
    pub const ColorAttachment: Self = Self(0b1_0000);
    /// Can be used as framebuffer depth/stencil attachment
    pub const DepthStencilAttachment: Self = Self(0b10_0000);
    /// Image data not needed outside of rendering
    pub const TransientAttachment: Self = Self(0b100_0000);
    /// Can be used as framebuffer input attachment
    pub const InputAttachment: Self = Self(0b1000_0000);
    pub const HostTransfer: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const VideoDecodeDstKHR: Self = Self(0b100_0000_0000);
    pub const VideoDecodeSrcKHR: Self = Self(0b1000_0000_0000);
    pub const VideoDecodeDpbKHR: Self = Self(0b1_0000_0000_0000);
    pub const FragmentDensityMapEXT: Self = Self(0b10_0000_0000);
    pub const FragmentShadingRateAttachmentKHR: Self = Self(0b1_0000_0000);
    pub const VideoEncodeDstKHR: Self = Self(0b10_0000_0000_0000);
    pub const VideoEncodeSrcKHR: Self = Self(0b100_0000_0000_0000);
    pub const VideoEncodeDpbKHR: Self = Self(0b1000_0000_0000_0000);
    pub const AttachmentFeedbackLoopEXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const InvocationMaskHUAWEI: Self = Self(0b100_0000_0000_0000_0000);
    pub const SampleWeightQCOM: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const SampleBlockMatchQCOM: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const TensorAliasingARM: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const TileMemoryQCOM: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeQuantizationDeltaMapKHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeEmphasisMapKHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCreateFlags, Flags);

impl ImageCreateFlags {
    /// Image should support sparse backing
    pub const SparseBinding: Self = Self(0b1);
    /// Image should support sparse backing with partial residency
    pub const SparseResidency: Self = Self(0b10);
    /// Image should support constant data access to physical memory ranges mapped into multiple locations of sparse images
    pub const SparseAliased: Self = Self(0b100);
    /// Allows image views to have different format than the base image
    pub const MutableFormat: Self = Self(0b1000);
    /// Allows creating image views with cube type from the created image
    pub const CubeCompatible: Self = Self(0b1_0000);
    pub const Alias: Self = Self(0b100_0000_0000);
    /// Allows using VkBindImageMemoryDeviceGroupInfo::pSplitInstanceBindRegions when binding memory to the image
    pub const SplitInstanceBindRegions: Self = Self(0b100_0000);
    /// The 3D image can be viewed as a 2D or 2D array image
    pub const Type2DArrayCompatible: Self = Self(0b10_0000);
    pub const BlockTexelViewCompatible: Self = Self(0b1000_0000);
    pub const ExtendedUsage: Self = Self(0b1_0000_0000);
    /// Image requires protected memory
    pub const Protected: Self = Self(0b1000_0000_0000);
    pub const Disjoint: Self = Self(0b10_0000_0000);
    pub const CornerSampledNV: Self = Self(0b10_0000_0000_0000);
    pub const DescriptorHeapCaptureReplayEXT: Self = Self(0b1_0000_0000_0000_0000);
    pub const SampleLocationsCompatibleDepthEXT: Self = Self(0b1_0000_0000_0000);
    pub const SubsampledEXT: Self = Self(0b100_0000_0000_0000);
    pub const MultisampledRenderToSingleSampledEXT: Self = Self(0b100_0000_0000_0000_0000);
    /// Image is created with a layout where individual slices are capable of being used as 2D images
    pub const Type2DViewCompatibleEXT: Self = Self(0b10_0000_0000_0000_0000);
    pub const VideoProfileIndependentKHR: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const FragmentDensityMapOffsetEXT: Self = Self(0b1000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageViewCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageViewCreateFlags, Flags);

impl ImageViewCreateFlags {
    pub const FragmentDensityMapDynamicEXT: Self = Self(0b1);
    pub const DescriptorBufferCaptureReplayEXT: Self = Self(0b100);
    pub const FragmentDensityMapDeferredEXT: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreateFlags, Flags);

impl PipelineCreateFlags {
    pub const DisableOptimization: Self = Self(0b1);
    pub const AllowDerivatives: Self = Self(0b10);
    pub const Derivative: Self = Self(0b100);
    pub const DispatchBase: Self = Self(0b1_0000);
    pub const ViewIndexFromDeviceIndex: Self = Self(0b1000);
    pub const FailOnPipelineCompileRequired: Self = Self(0b1_0000_0000);
    pub const EarlyReturnOnFailure: Self = Self(0b10_0000_0000);
    pub const NoProtectedAccess: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const ProtectedAccessOnly: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
    pub const RayTracingNoNullAnyHitShadersKHR: Self = Self(0b100_0000_0000_0000);
    pub const RayTracingNoNullClosestHitShadersKHR: Self = Self(0b1000_0000_0000_0000);
    pub const RayTracingNoNullMissShadersKHR: Self = Self(0b1_0000_0000_0000_0000);
    pub const RayTracingNoNullIntersectionShadersKHR: Self = Self(0b10_0000_0000_0000_0000);
    pub const RayTracingSkipTrianglesKHR: Self = Self(0b1_0000_0000_0000);
    pub const RayTracingSkipAabbsKHR: Self = Self(0b10_0000_0000_0000);
    pub const RayTracingShaderGroupHandleCaptureReplayKHR: Self = Self(0b1000_0000_0000_0000_0000);
    pub const DeferCompileNV: Self = Self(0b10_0000);
    pub const RenderingFragmentDensityMapAttachmentEXT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const RenderingFragmentShadingRateAttachmentKHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const CaptureStatisticsKHR: Self = Self(0b100_0000);
    pub const CaptureInternalRepresentationsKHR: Self = Self(0b1000_0000);
    pub const IndirectBindableNV: Self = Self(0b100_0000_0000_0000_0000);
    pub const LibraryKHR: Self = Self(0b1000_0000_0000);
    pub const DescriptorBufferEXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
    pub const RetainLinkTimeOptimizationInfoEXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const LinkTimeOptimizationEXT: Self = Self(0b100_0000_0000);
    pub const RayTracingAllowMotionNV: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const ColorAttachmentFeedbackLoopEXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const DepthStencilAttachmentFeedbackLoopEXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const RayTracingOpacityMicromapEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const RayTracingDisplacementMicromapNV: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkColorComponentFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ColorComponentFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ColorComponentFlags, Flags);

impl ColorComponentFlags {
    pub const R: Self = Self(0b1);
    pub const G: Self = Self(0b10);
    pub const B: Self = Self(0b100);
    pub const A: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FenceCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceCreateFlags, Flags);

impl FenceCreateFlags {
    pub const Signaled: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FormatFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FormatFeatureFlags, Flags);

impl FormatFeatureFlags {
    /// Format can be used for sampled images (SAMPLED_IMAGE and COMBINED_IMAGE_SAMPLER descriptor types)
    pub const SampledImage: Self = Self(0b1);
    /// Format can be used for storage images (STORAGE_IMAGE descriptor type)
    pub const StorageImage: Self = Self(0b10);
    /// Format supports atomic operations in case it is used for storage images
    pub const StorageImageAtomic: Self = Self(0b100);
    /// Format can be used for uniform texel buffers (TBOs)
    pub const UniformTexelBuffer: Self = Self(0b1000);
    /// Format can be used for storage texel buffers (IBOs)
    pub const StorageTexelBuffer: Self = Self(0b1_0000);
    /// Format supports atomic operations in case it is used for storage texel buffers
    pub const StorageTexelBufferAtomic: Self = Self(0b10_0000);
    /// Format can be used for vertex buffers (VBOs)
    pub const VertexBuffer: Self = Self(0b100_0000);
    /// Format can be used for color attachment images
    pub const ColorAttachment: Self = Self(0b1000_0000);
    /// Format supports blending in case it is used for color attachment images
    pub const ColorAttachmentBlend: Self = Self(0b1_0000_0000);
    /// Format can be used for depth/stencil attachment images
    pub const DepthStencilAttachment: Self = Self(0b10_0000_0000);
    /// Format can be used as the source image of blits with vkCmdBlitImage
    pub const BlitSrc: Self = Self(0b100_0000_0000);
    /// Format can be used as the destination image of blits with vkCmdBlitImage
    pub const BlitDst: Self = Self(0b1000_0000_0000);
    /// Format can be filtered with VK_FILTER_LINEAR when being sampled
    pub const SampledImageFilterLinear: Self = Self(0b1_0000_0000_0000);
    /// Format can be used as the source image of image transfer commands
    pub const TransferSrc: Self = Self(0b100_0000_0000_0000);
    /// Format can be used as the destination image of image transfer commands
    pub const TransferDst: Self = Self(0b1000_0000_0000_0000);
    /// Format can have midpoint rather than cosited chroma samples
    pub const MidpointChromaSamples: Self = Self(0b10_0000_0000_0000_0000);
    /// Format can be used with linear filtering whilst color conversion is enabled
    pub const SampledImageYcbcrConversionLinearFilter: Self = Self(0b100_0000_0000_0000_0000);
    /// Format can have different chroma, min and mag filters
    pub const SampledImageYcbcrConversionSeparateReconstructionFilter: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const SampledImageYcbcrConversionChromaReconstructionExplicit: Self =
        Self(0b1_0000_0000_0000_0000_0000);
    pub const SampledImageYcbcrConversionChromaReconstructionExplicitForceable: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    /// Format supports disjoint planes
    pub const Disjoint: Self = Self(0b100_0000_0000_0000_0000_0000);
    /// Format can have cosited rather than midpoint chroma samples
    pub const CositedChromaSamples: Self = Self(0b1000_0000_0000_0000_0000_0000);
    /// Format can be used with min/max reduction filtering
    pub const SampledImageFilterMinmax: Self = Self(0b1_0000_0000_0000_0000);
    pub const VideoDecodeOutputKHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VideoDecodeDpbKHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const AccelerationStructureVertexBufferKHR: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000);
    pub const SampledImageFilterCubicEXT: Self = Self(0b10_0000_0000_0000);
    pub const FragmentDensityMapEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const FragmentShadingRateAttachmentKHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeInputKHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeDpbKHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryControlFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct QueryControlFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryControlFlags, Flags);

impl QueryControlFlags {
    /// Require precise results to be collected by the query
    pub const Precise: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct QueryResultFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryResultFlags, Flags);

impl QueryResultFlags {
    /// Results of the queries are written to the destination buffer as 64-bit values
    pub const Type64: Self = Self(0b1);
    /// Results of the queries are waited on before proceeding with the result copy
    pub const Wait: Self = Self(0b10);
    /// Besides the results of the query, the availability of the results is also written
    pub const WithAvailability: Self = Self(0b100);
    /// Copy the partial results of the query even if the final results are not available
    pub const Partial: Self = Self(0b1000);
    pub const WithStatusKHR: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkEventCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct EventCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(EventCreateFlags, Flags);

impl EventCreateFlags {
    pub const DeviceOnly: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CommandPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolCreateFlags, Flags);

impl CommandPoolCreateFlags {
    /// Command buffers have a short lifetime
    pub const Transient: Self = Self(0b1);
    /// Command buffers may release their memory individually
    pub const ResetCommandBuffer: Self = Self(0b10);
    /// Command buffers allocated from pool are protected command buffers
    pub const Protected: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandPoolResetFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CommandPoolResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandPoolResetFlags, Flags);

impl CommandPoolResetFlags {
    /// Release resources owned by the pool
    pub const ReleaseResources: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferResetFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CommandBufferResetFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferResetFlags, Flags);

impl CommandBufferResetFlags {
    /// Release resources owned by the buffer
    pub const ReleaseResources: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferUsageFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CommandBufferUsageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CommandBufferUsageFlags, Flags);

impl CommandBufferUsageFlags {
    pub const OneTimeSubmit: Self = Self(0b1);
    pub const RenderPassContinue: Self = Self(0b10);
    /// Command buffer may be submitted/executed more than once simultaneously
    pub const SimultaneousUse: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPipelineStatisticFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct QueryPipelineStatisticFlags(pub(crate) Flags);
vk_bitflags_wrapped!(QueryPipelineStatisticFlags, Flags);

impl QueryPipelineStatisticFlags {
    /// Optional
    pub const InputAssemblyVertices: Self = Self(0b1);
    /// Optional
    pub const InputAssemblyPrimitives: Self = Self(0b10);
    /// Optional
    pub const VertexShaderInvocations: Self = Self(0b100);
    /// Optional
    pub const GeometryShaderInvocations: Self = Self(0b1000);
    /// Optional
    pub const GeometryShaderPrimitives: Self = Self(0b1_0000);
    /// Optional
    pub const ClippingInvocations: Self = Self(0b10_0000);
    /// Optional
    pub const ClippingPrimitives: Self = Self(0b100_0000);
    /// Optional
    pub const FragmentShaderInvocations: Self = Self(0b1000_0000);
    /// Optional
    pub const TessellationControlShaderPatches: Self = Self(0b1_0000_0000);
    /// Optional
    pub const TessellationEvaluationShaderInvocations: Self = Self(0b10_0000_0000);
    /// Optional
    pub const ComputeShaderInvocations: Self = Self(0b100_0000_0000);
    pub const TaskShaderInvocationsEXT: Self = Self(0b1000_0000_0000);
    pub const MeshShaderInvocationsEXT: Self = Self(0b1_0000_0000_0000);
    pub const ClusterCullingShaderInvocationsHUAWEI: Self = Self(0b10_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryMapFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemoryMapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryMapFlags, Flags);

impl MemoryMapFlags {
    pub const PlacedEXT: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryUnmapFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemoryUnmapFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryUnmapFlags, Flags);

impl MemoryUnmapFlags {
    pub const ReserveEXT: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageAspectFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageAspectFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ImageAspectFlags, Flags);

impl ImageAspectFlags {
    pub const Color: Self = Self(0b1);
    pub const Depth: Self = Self(0b10);
    pub const Stencil: Self = Self(0b100);
    pub const Metadata: Self = Self(0b1000);
    pub const Plane0: Self = Self(0b1_0000);
    pub const Plane1: Self = Self(0b10_0000);
    pub const Plane2: Self = Self(0b100_0000);
    pub const None: Self = Self(0);
    pub const MemoryPlane0EXT: Self = Self(0b1000_0000);
    pub const MemoryPlane1EXT: Self = Self(0b1_0000_0000);
    pub const MemoryPlane2EXT: Self = Self(0b10_0000_0000);
    pub const MemoryPlane3EXT: Self = Self(0b100_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseMemoryBindFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SparseMemoryBindFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseMemoryBindFlags, Flags);

impl SparseMemoryBindFlags {
    /// Operation binds resource metadata to memory
    pub const Metadata: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSparseImageFormatFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SparseImageFormatFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SparseImageFormatFlags, Flags);

impl SparseImageFormatFlags {
    /// Image uses a single mip tail region for all array layers
    pub const SingleMiptail: Self = Self(0b1);
    /// Image requires mip level dimensions to be an integer multiple of the sparse image block dimensions for non-tail mip levels.
    pub const AlignedMipSize: Self = Self(0b10);
    /// Image uses a non-standard sparse image block dimensions
    pub const NonstandardBlockSize: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassDescriptionFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SubpassDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubpassDescriptionFlags, Flags);

impl SubpassDescriptionFlags {
    pub const PerViewAttributesNVX: Self = Self(0b1);
    pub const PerViewPositionXOnlyNVX: Self = Self(0b10);
    pub const TileShadingApronQCOM: Self = Self(0b1_0000_0000);
    pub const RasterizationOrderAttachmentColorAccessEXT: Self = Self(0b1_0000);
    pub const RasterizationOrderAttachmentDepthAccessEXT: Self = Self(0b10_0000);
    pub const RasterizationOrderAttachmentStencilAccessEXT: Self = Self(0b100_0000);
    pub const EnableLegacyDitheringEXT: Self = Self(0b1000_0000);
    pub const FragmentRegionEXT: Self = Self(0b100);
    pub const CustomResolveEXT: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineStageFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineStageFlags, Flags);

impl PipelineStageFlags {
    /// Before subsequent commands are processed
    pub const TopOfPipe: Self = Self(0b1);
    /// Draw/DispatchIndirect command fetch
    pub const DrawIndirect: Self = Self(0b10);
    /// Vertex/index fetch
    pub const VertexInput: Self = Self(0b100);
    /// Vertex shading
    pub const VertexShader: Self = Self(0b1000);
    /// Tessellation control shading
    pub const TessellationControlShader: Self = Self(0b1_0000);
    /// Tessellation evaluation shading
    pub const TessellationEvaluationShader: Self = Self(0b10_0000);
    /// Geometry shading
    pub const GeometryShader: Self = Self(0b100_0000);
    /// Fragment shading
    pub const FragmentShader: Self = Self(0b1000_0000);
    /// Early fragment (depth and stencil) tests
    pub const EarlyFragmentTests: Self = Self(0b1_0000_0000);
    /// Late fragment (depth and stencil) tests
    pub const LateFragmentTests: Self = Self(0b10_0000_0000);
    /// Color attachment writes
    pub const ColorAttachmentOutput: Self = Self(0b100_0000_0000);
    /// Compute shading
    pub const ComputeShader: Self = Self(0b1000_0000_0000);
    /// Transfer/copy operations
    pub const Transfer: Self = Self(0b1_0000_0000_0000);
    /// After previous commands have completed
    pub const BottomOfPipe: Self = Self(0b10_0000_0000_0000);
    /// Indicates host (CPU) is a source/sink of the dependency
    pub const Host: Self = Self(0b100_0000_0000_0000);
    /// All stages of the graphics pipeline
    pub const AllGraphics: Self = Self(0b1000_0000_0000_0000);
    /// All stages supported on the queue
    pub const AllCommands: Self = Self(0b1_0000_0000_0000_0000);
    pub const None: Self = Self(0);
    pub const TransformFeedbackEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    /// A pipeline stage for conditional rendering predicate fetch
    pub const ConditionalRenderingEXT: Self = Self(0b100_0000_0000_0000_0000);
    pub const AccelerationStructureBuildKHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const RayTracingShaderKHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const FragmentDensityProcessEXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const FragmentShadingRateAttachmentKHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const TaskShaderEXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const MeshShaderEXT: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const CommandPreprocessEXT: Self = Self(0b10_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSampleCountFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SampleCountFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SampleCountFlags, Flags);

impl SampleCountFlags {
    /// Sample count 1 supported
    pub const Type1: Self = Self(0b1);
    /// Sample count 2 supported
    pub const Type2: Self = Self(0b10);
    /// Sample count 4 supported
    pub const Type4: Self = Self(0b100);
    /// Sample count 8 supported
    pub const Type8: Self = Self(0b1000);
    /// Sample count 16 supported
    pub const Type16: Self = Self(0b1_0000);
    /// Sample count 32 supported
    pub const Type32: Self = Self(0b10_0000);
    /// Sample count 64 supported
    pub const Type64: Self = Self(0b100_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentDescriptionFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AttachmentDescriptionFlags(pub(crate) Flags);
vk_bitflags_wrapped!(AttachmentDescriptionFlags, Flags);

impl AttachmentDescriptionFlags {
    /// The attachment may alias physical memory of another attachment in the same render pass
    pub const MayAlias: Self = Self(0b1);
    pub const ResolveSkipTransferFunctionKHR: Self = Self(0b10);
    pub const ResolveEnableTransferFunctionKHR: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilFaceFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct StencilFaceFlags(pub(crate) Flags);
vk_bitflags_wrapped!(StencilFaceFlags, Flags);

impl StencilFaceFlags {
    /// Front face
    pub const Front: Self = Self(0b1);
    /// Back face
    pub const Back: Self = Self(0b10);
    /// Front and back faces
    pub const FrontAndBack: Self = Self(0b11);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCullModeFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CullModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(CullModeFlags, Flags);

impl CullModeFlags {
    pub const None: Self = Self(0);
    pub const Front: Self = Self(0b1);
    pub const Back: Self = Self(0b10);
    pub const FrontAndBack: Self = Self(0b11);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorPoolCreateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DescriptorPoolCreateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorPoolCreateFlags, Flags);

impl DescriptorPoolCreateFlags {
    /// Descriptor sets may be freed individually
    pub const FreeDescriptorSet: Self = Self(0b1);
    pub const UpdateAfterBind: Self = Self(0b10);
    pub const HostOnlyEXT: Self = Self(0b100);
    pub const AllowOverallocationSetsNV: Self = Self(0b1000);
    pub const AllowOverallocationPoolsNV: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDependencyFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DependencyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DependencyFlags, Flags);

impl DependencyFlags {
    /// Dependency is per pixel region
    pub const ByRegion: Self = Self(0b1);
    /// Dependency is across devices
    pub const DeviceGroup: Self = Self(0b100);
    pub const ViewLocal: Self = Self(0b10);
    /// Dependency may be a feedback loop
    pub const FeedbackLoopEXT: Self = Self(0b1000);
    pub const QueueFamilyOwnershipTransferUseAllStagesKHR: Self = Self(0b10_0000);
    pub const AsymmetricEventKHR: Self = Self(0b100_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubgroupFeatureFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SubgroupFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubgroupFeatureFlags, Flags);

impl SubgroupFeatureFlags {
    /// Basic subgroup operations
    pub const Basic: Self = Self(0b1);
    /// Vote subgroup operations
    pub const Vote: Self = Self(0b10);
    /// Arithmetic subgroup operations
    pub const Arithmetic: Self = Self(0b100);
    /// Ballot subgroup operations
    pub const Ballot: Self = Self(0b1000);
    /// Shuffle subgroup operations
    pub const Shuffle: Self = Self(0b1_0000);
    /// Shuffle relative subgroup operations
    pub const ShuffleRelative: Self = Self(0b10_0000);
    /// Clustered subgroup operations
    pub const Clustered: Self = Self(0b100_0000);
    /// Quad subgroup operations
    pub const Quad: Self = Self(0b1000_0000);
    pub const Rotate: Self = Self(0b10_0000_0000);
    pub const RotateClustered: Self = Self(0b100_0000_0000);
    pub const PartitionedEXT: Self = Self(0b1_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct IndirectCommandsLayoutUsageFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsNV, Flags);

impl IndirectCommandsLayoutUsageFlagsNV {
    pub const ExplicitPreprocess: Self = Self(0b1);
    pub const IndexedSequences: Self = Self(0b10);
    pub const UnorderedSequences: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectStateFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct IndirectStateFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectStateFlagsNV, Flags);

impl IndirectStateFlagsNV {
    pub const FlagFrontface: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct GeometryFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(GeometryFlagsKHR, Flags);

impl GeometryFlagsKHR {
    pub const Opaque: Self = Self(0b1);
    pub const NoDuplicateAnyHitInvocation: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryInstanceFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct GeometryInstanceFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(GeometryInstanceFlagsKHR, Flags);

impl GeometryInstanceFlagsKHR {
    pub const TriangleFacingCullDisable: Self = Self(0b1);
    pub const TriangleFlipFacing: Self = Self(0b10);
    pub const ForceOpaque: Self = Self(0b100);
    pub const ForceNoOpaque: Self = Self(0b1000);
    pub const ForceOpacityMicromap2State: Self = Self(0b1_0000);
    pub const DisableOpacityMicromaps: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureGeometryFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClusterAccelerationStructureGeometryFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ClusterAccelerationStructureGeometryFlagsNV, Flags);

impl ClusterAccelerationStructureGeometryFlagsNV {
    pub const CullDisable: Self = Self(0b1);
    pub const NoDuplicateAnyhitInvocation: Self = Self(0b10);
    pub const Opaque: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureClusterFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClusterAccelerationStructureClusterFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ClusterAccelerationStructureClusterFlagsNV, Flags);

impl ClusterAccelerationStructureClusterFlagsNV {
    pub const AllowDisableOpacityMicromaps: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureAddressResolutionFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClusterAccelerationStructureAddressResolutionFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ClusterAccelerationStructureAddressResolutionFlagsNV, Flags);

impl ClusterAccelerationStructureAddressResolutionFlagsNV {
    pub const None: Self = Self(0);
    pub const IndirectedDstImplicitData: Self = Self(0b1);
    pub const IndirectedScratchData: Self = Self(0b10);
    pub const IndirectedDstAddressArray: Self = Self(0b100);
    pub const IndirectedDstSizesArray: Self = Self(0b1000);
    pub const IndirectedSrcInfosArray: Self = Self(0b1_0000);
    pub const IndirectedSrcInfosCount: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BuildAccelerationStructureFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(BuildAccelerationStructureFlagsKHR, Flags);

impl BuildAccelerationStructureFlagsKHR {
    pub const AllowUpdate: Self = Self(0b1);
    pub const AllowCompaction: Self = Self(0b10);
    pub const PreferFastTrace: Self = Self(0b100);
    pub const PreferFastBuild: Self = Self(0b1000);
    pub const LowMemory: Self = Self(0b1_0000);
    pub const Motion: Self = Self(0b10_0000);
    pub const AllowOpacityMicromapUpdate: Self = Self(0b100_0000);
    pub const AllowDisableOpacityMicromaps: Self = Self(0b1000_0000);
    pub const AllowOpacityMicromapDataUpdate: Self = Self(0b1_0000_0000);
    pub const AllowDisplacementMicromapUpdate: Self = Self(0b10_0000_0000);
    pub const AllowDataAccess: Self = Self(0b1000_0000_0000);
    pub const AllowClusterOpacityMicromaps: Self = Self(0b1_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCreateFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AccelerationStructureCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(AccelerationStructureCreateFlagsKHR, Flags);

impl AccelerationStructureCreateFlagsKHR {
    pub const DeviceAddressCaptureReplay: Self = Self(0b1);
    pub const DescriptorBufferCaptureReplay: Self = Self(0b1000);
    pub const Motion: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreationFeedbackFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineCreationFeedbackFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCreationFeedbackFlags, Flags);

impl PipelineCreationFeedbackFlags {
    pub const Valid: Self = Self(0b1);
    pub const ApplicationPipelineCacheHit: Self = Self(0b10);
    pub const BasePipelineAcceleration: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterDescriptionFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PerformanceCounterDescriptionFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(PerformanceCounterDescriptionFlagsKHR, Flags);

impl PerformanceCounterDescriptionFlagsKHR {
    pub const PerformanceImpacting: Self = Self(0b1);
    pub const ConcurrentlyImpacted: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAcquireProfilingLockFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AcquireProfilingLockFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(AcquireProfilingLockFlagsKHR, Flags);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreWaitFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SemaphoreWaitFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreWaitFlags, Flags);

impl SemaphoreWaitFlags {
    pub const Any: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCompilerControlFlagBitsAMD.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineCompilerControlFlagsAMD(pub(crate) Flags);
vk_bitflags_wrapped!(PipelineCompilerControlFlagsAMD, Flags);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCorePropertiesFlagBitsAMD.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ShaderCorePropertiesFlagsAMD(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderCorePropertiesFlagsAMD, Flags);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceDiagnosticsConfigFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeviceDiagnosticsConfigFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceDiagnosticsConfigFlagsNV, Flags);

impl DeviceDiagnosticsConfigFlagsNV {
    pub const EnableShaderDebugInfo: Self = Self(0b1);
    pub const EnableResourceTracking: Self = Self(0b10);
    pub const EnableAutomaticCheckpoints: Self = Self(0b100);
    pub const EnableShaderErrorReporting: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRefreshObjectFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct RefreshObjectFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(RefreshObjectFlagsKHR, Flags);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits2.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AccessFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(AccessFlags2, Flags64);

impl AccessFlags2 {
    pub const None: Self = Self(0);
    pub const IndirectCommandRead: Self = Self(0b1);
    pub const IndexRead: Self = Self(0b10);
    pub const VertexAttributeRead: Self = Self(0b100);
    pub const UniformRead: Self = Self(0b1000);
    pub const InputAttachmentRead: Self = Self(0b1_0000);
    pub const ShaderRead: Self = Self(0b10_0000);
    pub const ShaderWrite: Self = Self(0b100_0000);
    pub const ColorAttachmentRead: Self = Self(0b1000_0000);
    pub const ColorAttachmentWrite: Self = Self(0b1_0000_0000);
    pub const DepthStencilAttachmentRead: Self = Self(0b10_0000_0000);
    pub const DepthStencilAttachmentWrite: Self = Self(0b100_0000_0000);
    pub const TransferRead: Self = Self(0b1000_0000_0000);
    pub const TransferWrite: Self = Self(0b1_0000_0000_0000);
    pub const HostRead: Self = Self(0b10_0000_0000_0000);
    pub const HostWrite: Self = Self(0b100_0000_0000_0000);
    pub const MemoryRead: Self = Self(0b1000_0000_0000_0000);
    pub const MemoryWrite: Self = Self(0b1_0000_0000_0000_0000);
    pub const ShaderSampledRead: Self = Self(0);
    pub const ShaderStorageRead: Self = Self(0);
    pub const ShaderStorageWrite: Self = Self(0);
    pub const VideoDecodeReadKHR: Self = Self(0);
    pub const VideoDecodeWriteKHR: Self = Self(0);
    pub const SamplerHeapReadEXT: Self = Self(0);
    pub const ResourceHeapReadEXT: Self = Self(0);
    pub const VideoEncodeReadKHR: Self = Self(0);
    pub const VideoEncodeWriteKHR: Self = Self(0);
    pub const ShaderTileAttachmentReadQCOM: Self = Self(0);
    pub const ShaderTileAttachmentWriteQCOM: Self = Self(0);
    pub const TransformFeedbackWriteEXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const TransformFeedbackCounterReadEXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const TransformFeedbackCounterWriteEXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    /// read access flag for reading conditional rendering predicate
    pub const ConditionalRenderingReadEXT: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const CommandPreprocessReadEXT: Self = Self(0b10_0000_0000_0000_0000);
    pub const CommandPreprocessWriteEXT: Self = Self(0b100_0000_0000_0000_0000);
    pub const FragmentShadingRateAttachmentReadKHR: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const AccelerationStructureReadKHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const AccelerationStructureWriteKHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const FragmentDensityMapReadEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const ColorAttachmentReadNoncoherentEXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const DescriptorBufferReadEXT: Self = Self(0);
    pub const InvocationMaskReadHUAWEI: Self = Self(0);
    pub const ShaderBindingTableReadKHR: Self = Self(0);
    pub const MicromapReadEXT: Self = Self(0);
    pub const MicromapWriteEXT: Self = Self(0);
    pub const OpticalFlowReadNV: Self = Self(0);
    pub const OpticalFlowWriteNV: Self = Self(0);
    pub const DataGraphReadARM: Self = Self(0);
    pub const DataGraphWriteARM: Self = Self(0);
    pub const MemoryDecompressionReadEXT: Self = Self(0);
    pub const MemoryDecompressionWriteEXT: Self = Self(0);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineStageFlagBits2.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineStageFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(PipelineStageFlags2, Flags64);

impl PipelineStageFlags2 {
    pub const None: Self = Self(0);
    pub const TopOfPipe: Self = Self(0b1);
    pub const DrawIndirect: Self = Self(0b10);
    pub const VertexInput: Self = Self(0b100);
    pub const VertexShader: Self = Self(0b1000);
    pub const TessellationControlShader: Self = Self(0b1_0000);
    pub const TessellationEvaluationShader: Self = Self(0b10_0000);
    pub const GeometryShader: Self = Self(0b100_0000);
    pub const FragmentShader: Self = Self(0b1000_0000);
    pub const EarlyFragmentTests: Self = Self(0b1_0000_0000);
    pub const LateFragmentTests: Self = Self(0b10_0000_0000);
    pub const ColorAttachmentOutput: Self = Self(0b100_0000_0000);
    pub const ComputeShader: Self = Self(0b1000_0000_0000);
    pub const AllTransfer: Self = Self(0b1_0000_0000_0000);
    pub const BottomOfPipe: Self = Self(0b10_0000_0000_0000);
    pub const Host: Self = Self(0b100_0000_0000_0000);
    pub const AllGraphics: Self = Self(0b1000_0000_0000_0000);
    pub const AllCommands: Self = Self(0b1_0000_0000_0000_0000);
    pub const Copy: Self = Self(0);
    pub const Resolve: Self = Self(0);
    pub const Blit: Self = Self(0);
    pub const Clear: Self = Self(0);
    pub const IndexInput: Self = Self(0);
    pub const VertexAttributeInput: Self = Self(0);
    pub const PreRasterizationShaders: Self = Self(0);
    pub const VideoDecodeKHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeKHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const TransformFeedbackEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    /// A pipeline stage for conditional rendering predicate fetch
    pub const ConditionalRenderingEXT: Self = Self(0b100_0000_0000_0000_0000);
    pub const CommandPreprocessEXT: Self = Self(0b10_0000_0000_0000_0000);
    pub const FragmentShadingRateAttachmentKHR: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const AccelerationStructureBuildKHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const RayTracingShaderKHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const FragmentDensityProcessEXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const TaskShaderEXT: Self = Self(0b1000_0000_0000_0000_0000);
    pub const MeshShaderEXT: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const SubpassShaderHUAWEI: Self = Self(0);
    pub const InvocationMaskHUAWEI: Self = Self(0);
    pub const AccelerationStructureCopyKHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
    pub const MicromapBuildEXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
    pub const ClusterCullingShaderHUAWEI: Self = Self(0);
    pub const OpticalFlowNV: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
    pub const ConvertCooperativeVectorMatrixNV: Self = Self(0);
    pub const DataGraphARM: Self = Self(0);
    pub const CopyIndirectKHR: Self = Self(0);
    pub const MemoryDecompressionEXT: Self = Self(0);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormatFeatureFlagBits2.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FormatFeatureFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(FormatFeatureFlags2, Flags64);

impl FormatFeatureFlags2 {
    pub const SampledImage: Self = Self(0b1);
    pub const StorageImage: Self = Self(0b10);
    pub const StorageImageAtomic: Self = Self(0b100);
    pub const UniformTexelBuffer: Self = Self(0b1000);
    pub const StorageTexelBuffer: Self = Self(0b1_0000);
    pub const StorageTexelBufferAtomic: Self = Self(0b10_0000);
    pub const VertexBuffer: Self = Self(0b100_0000);
    pub const ColorAttachment: Self = Self(0b1000_0000);
    pub const ColorAttachmentBlend: Self = Self(0b1_0000_0000);
    pub const DepthStencilAttachment: Self = Self(0b10_0000_0000);
    pub const BlitSrc: Self = Self(0b100_0000_0000);
    pub const BlitDst: Self = Self(0b1000_0000_0000);
    pub const SampledImageFilterLinear: Self = Self(0b1_0000_0000_0000);
    pub const TransferSrc: Self = Self(0b100_0000_0000_0000);
    pub const TransferDst: Self = Self(0b1000_0000_0000_0000);
    pub const SampledImageFilterMinmax: Self = Self(0b1_0000_0000_0000_0000);
    pub const MidpointChromaSamples: Self = Self(0b10_0000_0000_0000_0000);
    pub const SampledImageYcbcrConversionLinearFilter: Self = Self(0b100_0000_0000_0000_0000);
    pub const SampledImageYcbcrConversionSeparateReconstructionFilter: Self =
        Self(0b1000_0000_0000_0000_0000);
    pub const SampledImageYcbcrConversionChromaReconstructionExplicit: Self =
        Self(0b1_0000_0000_0000_0000_0000);
    pub const SampledImageYcbcrConversionChromaReconstructionExplicitForceable: Self =
        Self(0b10_0000_0000_0000_0000_0000);
    pub const Disjoint: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const CositedChromaSamples: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const StorageReadWithoutFormat: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000);
    pub const StorageWriteWithoutFormat: Self = Self(0);
    pub const SampledImageDepthComparison: Self = Self(0);
    /// This is an interaction with EXT_filter_cubic, though not tagged that way
    pub const SampledImageFilterCubic: Self = Self(0b10_0000_0000_0000);
    pub const HostImageTransfer: Self = Self(0);
    pub const VideoDecodeOutputKHR: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const VideoDecodeDpbKHR: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const AccelerationStructureVertexBufferKHR: Self =
        Self(0b10_0000_0000_0000_0000_0000_0000_0000);
    pub const FragmentDensityMapEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const FragmentShadingRateAttachmentKHR: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeInputKHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const VideoEncodeDpbKHR: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
    pub const AccelerationStructureRadiusBufferNV: Self = Self(0);
    /// Format support linear image as render target, it cannot be mixed with non linear attachment
    pub const LinearColorAttachmentNV: Self = Self(0);
    pub const WeightImageQCOM: Self = Self(0);
    pub const WeightSampledImageQCOM: Self = Self(0);
    pub const BlockMatchingQCOM: Self = Self(0);
    pub const BoxFilterSampledQCOM: Self = Self(0);
    pub const TensorShaderARM: Self = Self(0);
    pub const TensorImageAliasingARM: Self = Self(0);
    pub const OpticalFlowImageNV: Self = Self(0);
    pub const OpticalFlowVectorNV: Self = Self(0);
    pub const OpticalFlowCostNV: Self = Self(0);
    pub const TensorDataGraphARM: Self = Self(0);
    pub const CopyImageIndirectDstKHR: Self = Self(0);
    pub const VideoEncodeQuantizationDeltaMapKHR: Self = Self(0);
    pub const VideoEncodeEmphasisMapKHR: Self = Self(0);
    pub const DepthCopyOnComputeQueueKHR: Self = Self(0);
    pub const DepthCopyOnTransferQueueKHR: Self = Self(0);
    pub const StencilCopyOnComputeQueueKHR: Self = Self(0);
    pub const StencilCopyOnTransferQueueKHR: Self = Self(0);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct RenderingFlags(pub(crate) Flags);
vk_bitflags_wrapped!(RenderingFlags, Flags);

impl RenderingFlags {
    pub const ContentsSecondaryCommandBuffers: Self = Self(0b1);
    pub const Suspending: Self = Self(0b10);
    pub const Resuming: Self = Self(0b100);
    pub const EnableLegacyDitheringEXT: Self = Self(0b1000);
    /// Promoted from extension 452
    pub const ContentsInlineKHR: Self = Self(0b1_0000);
    pub const PerLayerFragmentDensityVALVE: Self = Self(0b10_0000);
    pub const FragmentRegionEXT: Self = Self(0b100_0000);
    pub const CustomResolveEXT: Self = Self(0b1000_0000);
    pub const LocalReadConcurrentAccessControlKHR: Self = Self(0b1_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryDecompressionMethodFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemoryDecompressionMethodFlagsEXT(pub(crate) Flags64);
vk_bitflags_wrapped!(MemoryDecompressionMethodFlagsEXT, Flags64);

impl MemoryDecompressionMethodFlagsEXT {
    pub const Gdeflate10: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BuildMicromapFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(BuildMicromapFlagsEXT, Flags);

impl BuildMicromapFlagsEXT {
    pub const PreferFastTrace: Self = Self(0b1);
    pub const PreferFastBuild: Self = Self(0b10);
    pub const AllowCompaction: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapCreateFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MicromapCreateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(MicromapCreateFlagsEXT, Flags);

impl MicromapCreateFlagsEXT {
    pub const DeviceAddressCaptureReplay: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsLayoutUsageFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct IndirectCommandsLayoutUsageFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectCommandsLayoutUsageFlagsEXT, Flags);

impl IndirectCommandsLayoutUsageFlagsEXT {
    pub const ExplicitPreprocess: Self = Self(0b1);
    pub const UnorderedSequences: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsInputModeFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct IndirectCommandsInputModeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(IndirectCommandsInputModeFlagsEXT, Flags);

impl IndirectCommandsInputModeFlagsEXT {
    pub const VulkanIndexBuffer: Self = Self(0b1);
    pub const DxgiIndexBuffer: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCreateFlagBits2.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PipelineCreateFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(PipelineCreateFlags2, Flags64);

impl PipelineCreateFlags2 {
    pub const DisableOptimization: Self = Self(0b1);
    pub const AllowDerivatives: Self = Self(0b10);
    pub const Derivative: Self = Self(0b100);
    pub const ViewIndexFromDeviceIndex: Self = Self(0b1000);
    pub const DispatchBase: Self = Self(0b1_0000);
    pub const FailOnPipelineCompileRequired: Self = Self(0b1_0000_0000);
    pub const EarlyReturnOnFailure: Self = Self(0b10_0000_0000);
    pub const NoProtectedAccess: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const ProtectedAccessOnly: Self = Self(0b100_0000_0000_0000_0000_0000_0000_0000);
    pub const ExecutionGraphAMDX: Self = Self(0);
    pub const DescriptorHeapEXT: Self = Self(0);
    pub const RayTracingAllowSpheresAndLinearSweptSpheresNV: Self = Self(0);
    pub const EnableLegacyDitheringEXT: Self = Self(0);
    pub const DeferCompileNV: Self = Self(0b10_0000);
    pub const CaptureStatisticsKHR: Self = Self(0b100_0000);
    pub const CaptureInternalRepresentationsKHR: Self = Self(0b1000_0000);
    pub const LinkTimeOptimizationEXT: Self = Self(0b100_0000_0000);
    pub const RetainLinkTimeOptimizationInfoEXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const LibraryKHR: Self = Self(0b1000_0000_0000);
    pub const RayTracingSkipTrianglesKHR: Self = Self(0b1_0000_0000_0000);
    pub const RayTracingSkipAabbsKHR: Self = Self(0b10_0000_0000_0000);
    pub const RayTracingNoNullAnyHitShadersKHR: Self = Self(0b100_0000_0000_0000);
    pub const RayTracingNoNullClosestHitShadersKHR: Self = Self(0b1000_0000_0000_0000);
    pub const RayTracingNoNullMissShadersKHR: Self = Self(0b1_0000_0000_0000_0000);
    pub const RayTracingNoNullIntersectionShadersKHR: Self = Self(0b10_0000_0000_0000_0000);
    pub const RayTracingShaderGroupHandleCaptureReplayKHR: Self = Self(0b1000_0000_0000_0000_0000);
    pub const IndirectBindableNV: Self = Self(0b100_0000_0000_0000_0000);
    pub const RayTracingAllowMotionNV: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const RenderingFragmentShadingRateAttachmentKHR: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const RenderingFragmentDensityMapAttachmentEXT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const RayTracingOpacityMicromapEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const ColorAttachmentFeedbackLoopEXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const DepthStencilAttachmentFeedbackLoopEXT: Self =
        Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const RayTracingDisplacementMicromapNV: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
    pub const DescriptorBufferEXT: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
    pub const DisallowOpacityMicromapARM: Self = Self(0);
    pub const CaptureDataKHR: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000);
    pub const IndirectBindableEXT: Self = Self(0);
    pub const PerLayerFragmentDensityVALVE: Self = Self(0);
    pub const Type64BitIndexingEXT: Self = Self(0);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBufferUsageFlagBits2.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct BufferUsageFlags2(pub(crate) Flags64);
vk_bitflags_wrapped!(BufferUsageFlags2, Flags64);

impl BufferUsageFlags2 {
    pub const TransferSrc: Self = Self(0b1);
    pub const TransferDst: Self = Self(0b10);
    pub const UniformTexelBuffer: Self = Self(0b100);
    pub const StorageTexelBuffer: Self = Self(0b1000);
    pub const UniformBuffer: Self = Self(0b1_0000);
    pub const StorageBuffer: Self = Self(0b10_0000);
    pub const IndexBuffer: Self = Self(0b100_0000);
    pub const VertexBuffer: Self = Self(0b1000_0000);
    pub const IndirectBuffer: Self = Self(0b1_0000_0000);
    pub const ShaderDeviceAddress: Self = Self(0b10_0000_0000_0000_0000);
    pub const ExecutionGraphScratchAMDX: Self = Self(0b10_0000_0000_0000_0000_0000_0000);
    pub const DescriptorHeapEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000_0000);
    pub const ConditionalRenderingEXT: Self = Self(0b10_0000_0000);
    pub const ShaderBindingTableKHR: Self = Self(0b100_0000_0000);
    pub const TransformFeedbackBufferEXT: Self = Self(0b1000_0000_0000);
    pub const TransformFeedbackCounterBufferEXT: Self = Self(0b1_0000_0000_0000);
    pub const VideoDecodeSrcKHR: Self = Self(0b10_0000_0000_0000);
    pub const VideoDecodeDstKHR: Self = Self(0b100_0000_0000_0000);
    pub const VideoEncodeDstKHR: Self = Self(0b1000_0000_0000_0000);
    pub const VideoEncodeSrcKHR: Self = Self(0b1_0000_0000_0000_0000);
    pub const AccelerationStructureBuildInputReadOnlyKHR: Self = Self(0b1000_0000_0000_0000_0000);
    pub const AccelerationStructureStorageKHR: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const SamplerDescriptorBufferEXT: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const ResourceDescriptorBufferEXT: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const PushDescriptorsDescriptorBufferEXT: Self = Self(0b100_0000_0000_0000_0000_0000_0000);
    pub const MicromapBuildInputReadOnlyEXT: Self = Self(0b1000_0000_0000_0000_0000_0000);
    pub const MicromapStorageEXT: Self = Self(0b1_0000_0000_0000_0000_0000_0000);
    pub const CompressedDataDgf1AMDX: Self = Self(0);
    pub const DataGraphForeignDescriptorARM: Self = Self(0b10_0000_0000_0000_0000_0000_0000_0000);
    pub const TileMemoryQCOM: Self = Self(0b1000_0000_0000_0000_0000_0000_0000);
    pub const MemoryDecompressionEXT: Self = Self(0);
    pub const PreprocessBufferEXT: Self = Self(0b1000_0000_0000_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAddressCopyFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AddressCopyFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(AddressCopyFlagsKHR, Flags);

impl AddressCopyFlagsKHR {
    pub const DeviceLocal: Self = Self(0b1);
    pub const Sparse: Self = Self(0b10);
    pub const Protected: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorCreateFlagBitsARM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TensorCreateFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(TensorCreateFlagsARM, Flags64);

impl TensorCreateFlagsARM {
    pub const MutableFormat: Self = Self(0b1);
    pub const Protected: Self = Self(0b10);
    pub const DescriptorHeapCaptureReplay: Self = Self(0b1000);
    pub const DescriptorBufferCaptureReplay: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorUsageFlagBitsARM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TensorUsageFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(TensorUsageFlagsARM, Flags64);

impl TensorUsageFlagsARM {
    /// Tensor written/read through shader descriptor
    pub const Shader: Self = Self(0b10);
    /// Tensor can be src of a transfer operation
    pub const TransferSrc: Self = Self(0b100);
    /// Tensor can be dst of a transfer operation
    pub const TransferDst: Self = Self(0b1000);
    /// Tensor can be aliased with an image
    pub const ImageAliasing: Self = Self(0b1_0000);
    pub const DataGraph: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorViewCreateFlagBitsARM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TensorViewCreateFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(TensorViewCreateFlagsARM, Flags64);

impl TensorViewCreateFlagsARM {
    pub const DescriptorBufferCaptureReplay: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionCreateFlagBitsARM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DataGraphPipelineSessionCreateFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(DataGraphPipelineSessionCreateFlagsARM, Flags64);

impl DataGraphPipelineSessionCreateFlagsARM {
    pub const Protected: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineDispatchFlagBitsARM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DataGraphPipelineDispatchFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(DataGraphPipelineDispatchFlagsARM, Flags64);

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbModelConversionFlagBitsVALVE.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeRgbModelConversionFlagsVALVE(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeRgbModelConversionFlagsVALVE, Flags);

impl VideoEncodeRgbModelConversionFlagsVALVE {
    pub const RgbIdentity: Self = Self(0b1);
    pub const YcbcrIdentity: Self = Self(0b10);
    pub const Ycbcr709: Self = Self(0b100);
    pub const Ycbcr601: Self = Self(0b1000);
    pub const Ycbcr2020: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbRangeCompressionFlagBitsVALVE.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeRgbRangeCompressionFlagsVALVE(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeRgbRangeCompressionFlagsVALVE, Flags);

impl VideoEncodeRgbRangeCompressionFlagsVALVE {
    pub const FullRange: Self = Self(0b1);
    pub const NarrowRange: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRgbChromaOffsetFlagBitsVALVE.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeRgbChromaOffsetFlagsVALVE(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeRgbChromaOffsetFlagsVALVE, Flags);

impl VideoEncodeRgbChromaOffsetFlagsVALVE {
    pub const CositedEven: Self = Self(0b1);
    pub const Midpoint: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSpirvResourceTypeFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SpirvResourceTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(SpirvResourceTypeFlagsEXT, Flags);

impl SpirvResourceTypeFlagsEXT {
    pub const All: Self = Self(0b111_1111_1111_1111_1111_1111_1111_1111);
    pub const Sampler: Self = Self(0b1);
    pub const SampledImage: Self = Self(0b10);
    pub const ReadOnlyImage: Self = Self(0b100);
    pub const ReadWriteImage: Self = Self(0b1000);
    pub const CombinedSampledImage: Self = Self(0b1_0000);
    pub const UniformBuffer: Self = Self(0b10_0000);
    pub const ReadOnlyStorageBuffer: Self = Self(0b100_0000);
    pub const ReadWriteStorageBuffer: Self = Self(0b1000_0000);
    pub const AccelerationStructure: Self = Self(0b1_0000_0000);
    pub const Tensor: Self = Self(0b10_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompositeAlphaFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct CompositeAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(CompositeAlphaFlagsKHR, Flags);

impl CompositeAlphaFlagsKHR {
    pub const Opaque: Self = Self(0b1);
    pub const PreMultiplied: Self = Self(0b10);
    pub const PostMultiplied: Self = Self(0b100);
    pub const Inherit: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPlaneAlphaFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DisplayPlaneAlphaFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DisplayPlaneAlphaFlagsKHR, Flags);

impl DisplayPlaneAlphaFlagsKHR {
    pub const Opaque: Self = Self(0b1);
    pub const Global: Self = Self(0b10);
    pub const PerPixel: Self = Self(0b100);
    pub const PerPixelPremultiplied: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceTransformFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SurfaceTransformFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceTransformFlagsKHR, Flags);

impl SurfaceTransformFlagsKHR {
    pub const Identity: Self = Self(0b1);
    pub const Rotate90: Self = Self(0b10);
    pub const Rotate180: Self = Self(0b100);
    pub const Rotate270: Self = Self(0b1000);
    pub const HorizontalMirror: Self = Self(0b1_0000);
    pub const HorizontalMirrorRotate90: Self = Self(0b10_0000);
    pub const HorizontalMirrorRotate180: Self = Self(0b100_0000);
    pub const HorizontalMirrorRotate270: Self = Self(0b1000_0000);
    pub const Inherit: Self = Self(0b1_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSwapchainCreateFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SwapchainCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(SwapchainCreateFlagsKHR, Flags);

impl SwapchainCreateFlagsKHR {
    /// Allow images with VK_IMAGE_CREATE_SPLIT_INSTANCE_BIND_REGIONS_BIT
    pub const SplitInstanceBindRegions: Self = Self(0b1);
    /// Swapchain is protected
    pub const Protected: Self = Self(0b10);
    pub const MutableFormat: Self = Self(0b100);
    pub const PresentTiming: Self = Self(0b10_0000_0000);
    /// Allow use of VK_KHR_present_id2 with this swapchain
    pub const PresentId2: Self = Self(0b100_0000);
    /// Allow use of VK_KHR_present_wait2 with this swapchain
    pub const PresentWait2: Self = Self(0b1000_0000);
    pub const DeferredMemoryAllocation: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPeerMemoryFeatureFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PeerMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(PeerMemoryFeatureFlags, Flags);

impl PeerMemoryFeatureFlags {
    /// Can read with vkCmdCopy commands
    pub const CopySrc: Self = Self(0b1);
    /// Can write with vkCmdCopy commands
    pub const CopyDst: Self = Self(0b10);
    /// Can read with any access type/command
    pub const GenericSrc: Self = Self(0b100);
    /// Can write with and access type/command
    pub const GenericDst: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryAllocateFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct MemoryAllocateFlags(pub(crate) Flags);
vk_bitflags_wrapped!(MemoryAllocateFlags, Flags);

impl MemoryAllocateFlags {
    /// Force allocation on specific devices
    pub const DeviceMask: Self = Self(0b1);
    pub const DeviceAddress: Self = Self(0b10);
    pub const DeviceAddressCaptureReplay: Self = Self(0b100);
    pub const ZeroInitializeEXT: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceGroupPresentModeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeviceGroupPresentModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceGroupPresentModeFlagsKHR, Flags);

impl DeviceGroupPresentModeFlagsKHR {
    /// Present from local memory
    pub const Local: Self = Self(0b1);
    /// Present from remote memory
    pub const Remote: Self = Self(0b10);
    /// Present sum of local and/or remote memory
    pub const Sum: Self = Self(0b100);
    /// Each physical device presents from local memory
    pub const LocalMultiDevice: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DebugReportFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugReportFlagsEXT, Flags);

impl DebugReportFlagsEXT {
    pub const Information: Self = Self(0b1);
    pub const Warning: Self = Self(0b10);
    pub const PerformanceWarning: Self = Self(0b100);
    pub const Error: Self = Self(0b1000);
    pub const Debug: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalMemoryHandleTypeFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlagsNV, Flags);

impl ExternalMemoryHandleTypeFlagsNV {
    pub const OpaqueWin32: Self = Self(0b1);
    pub const OpaqueWin32Kmt: Self = Self(0b10);
    pub const D3D11Image: Self = Self(0b100);
    pub const D3D11ImageKmt: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureIndexFormatFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ClusterAccelerationStructureIndexFormatFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ClusterAccelerationStructureIndexFormatFlagsNV, Flags);

impl ClusterAccelerationStructureIndexFormatFlagsNV {
    pub const Type8Bit: Self = Self(0b1);
    pub const Type16Bit: Self = Self(0b10);
    pub const Type32Bit: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalMemoryFeatureFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlagsNV, Flags);

impl ExternalMemoryFeatureFlagsNV {
    pub const DedicatedOnly: Self = Self(0b1);
    pub const Exportable: Self = Self(0b10);
    pub const Importable: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryHandleTypeFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalMemoryHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryHandleTypeFlags, Flags);

impl ExternalMemoryHandleTypeFlags {
    pub const OpaqueFd: Self = Self(0b1);
    pub const OpaqueWin32: Self = Self(0b10);
    pub const OpaqueWin32Kmt: Self = Self(0b100);
    pub const D3D11Texture: Self = Self(0b1000);
    pub const D3D11TextureKmt: Self = Self(0b1_0000);
    pub const D3D12Heap: Self = Self(0b10_0000);
    pub const D3D12Resource: Self = Self(0b100_0000);
    pub const DmaBufEXT: Self = Self(0b10_0000_0000);
    pub const ANDROIDHardwareBufferANDROID: Self = Self(0b100_0000_0000);
    pub const HostAllocationEXT: Self = Self(0b1000_0000);
    pub const HostMappedForeignMemoryEXT: Self = Self(0b1_0000_0000);
    pub const ZirconVmoFUCHSIA: Self = Self(0b1000_0000_0000);
    pub const RdmaAddressNV: Self = Self(0b1_0000_0000_0000);
    pub const SciBufNV: Self = Self(0b10_0000_0000_0000);
    pub const OhNativeBufferOHOS: Self = Self(0b1000_0000_0000_0000);
    pub const ScreenBufferQNX: Self = Self(0b100_0000_0000_0000);
    pub const MtlbufferEXT: Self = Self(0b1_0000_0000_0000_0000);
    pub const MtltextureEXT: Self = Self(0b10_0000_0000_0000_0000);
    pub const MtlheapEXT: Self = Self(0b100_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalMemoryFeatureFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalMemoryFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalMemoryFeatureFlags, Flags);

impl ExternalMemoryFeatureFlags {
    pub const DedicatedOnly: Self = Self(0b1);
    pub const Exportable: Self = Self(0b10);
    pub const Importable: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreHandleTypeFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalSemaphoreHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreHandleTypeFlags, Flags);

impl ExternalSemaphoreHandleTypeFlags {
    pub const OpaqueFd: Self = Self(0b1);
    pub const OpaqueWin32: Self = Self(0b10);
    pub const OpaqueWin32Kmt: Self = Self(0b100);
    pub const D3D12Fence: Self = Self(0b1000);
    pub const SyncFd: Self = Self(0b1_0000);
    pub const ZirconEventFUCHSIA: Self = Self(0b1000_0000);
    pub const SciSyncObjNV: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalSemaphoreFeatureFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalSemaphoreFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalSemaphoreFeatureFlags, Flags);

impl ExternalSemaphoreFeatureFlags {
    pub const Exportable: Self = Self(0b1);
    pub const Importable: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreImportFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SemaphoreImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SemaphoreImportFlags, Flags);

impl SemaphoreImportFlags {
    pub const Temporary: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceHandleTypeFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalFenceHandleTypeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceHandleTypeFlags, Flags);

impl ExternalFenceHandleTypeFlags {
    pub const OpaqueFd: Self = Self(0b1);
    pub const OpaqueWin32: Self = Self(0b10);
    pub const OpaqueWin32Kmt: Self = Self(0b100);
    pub const SyncFd: Self = Self(0b1000);
    pub const SciSyncObjNV: Self = Self(0b1_0000);
    pub const SciSyncFenceNV: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExternalFenceFeatureFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExternalFenceFeatureFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ExternalFenceFeatureFlags, Flags);

impl ExternalFenceFeatureFlags {
    pub const Exportable: Self = Self(0b1);
    pub const Importable: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFenceImportFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FenceImportFlags(pub(crate) Flags);
vk_bitflags_wrapped!(FenceImportFlags, Flags);

impl FenceImportFlags {
    pub const Temporary: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSurfaceCounterFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SurfaceCounterFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(SurfaceCounterFlagsEXT, Flags);

impl SurfaceCounterFlagsEXT {
    pub const Vblank: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageSeverityFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DebugUtilsMessageSeverityFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageSeverityFlagsEXT, Flags);

impl DebugUtilsMessageSeverityFlagsEXT {
    pub const Verbose: Self = Self(0b1);
    pub const Info: Self = Self(0b1_0000);
    pub const Warning: Self = Self(0b1_0000_0000);
    pub const Error: Self = Self(0b1_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugUtilsMessageTypeFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DebugUtilsMessageTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DebugUtilsMessageTypeFlagsEXT, Flags);

impl DebugUtilsMessageTypeFlagsEXT {
    pub const General: Self = Self(0b1);
    pub const Validation: Self = Self(0b10);
    pub const Performance: Self = Self(0b100);
    pub const DeviceAddressBinding: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorBindingFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DescriptorBindingFlags(pub(crate) Flags);
vk_bitflags_wrapped!(DescriptorBindingFlags, Flags);

impl DescriptorBindingFlags {
    pub const UpdateAfterBind: Self = Self(0b1);
    pub const UpdateUnusedWhilePending: Self = Self(0b10);
    pub const PartiallyBound: Self = Self(0b100);
    pub const VariableDescriptorCount: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConditionalRenderingFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ConditionalRenderingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ConditionalRenderingFlagsEXT, Flags);

impl ConditionalRenderingFlagsEXT {
    pub const Inverted: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveModeFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ResolveModeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ResolveModeFlags, Flags);

impl ResolveModeFlags {
    pub const None: Self = Self(0);
    pub const SampleZero: Self = Self(0b1);
    pub const Average: Self = Self(0b10);
    pub const Min: Self = Self(0b100);
    pub const Max: Self = Self(0b1000);
    pub const ExternalFormatDownsampleANDROID: Self = Self(0b1_0000);
    pub const CustomEXT: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkToolPurposeFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ToolPurposeFlags(pub(crate) Flags);
vk_bitflags_wrapped!(ToolPurposeFlags, Flags);

impl ToolPurposeFlags {
    pub const Validation: Self = Self(0b1);
    pub const Profiling: Self = Self(0b10);
    pub const Tracing: Self = Self(0b100);
    pub const AdditionalFeatures: Self = Self(0b1000);
    pub const ModifyingFeatures: Self = Self(0b1_0000);
    pub const DebugReportingEXT: Self = Self(0b10_0000);
    pub const DebugMarkersEXT: Self = Self(0b100_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubmitFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct SubmitFlags(pub(crate) Flags);
vk_bitflags_wrapped!(SubmitFlags, Flags);

impl SubmitFlags {
    pub const Protected: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkHostImageCopyFlagBits.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct HostImageCopyFlags(pub(crate) Flags);
vk_bitflags_wrapped!(HostImageCopyFlags, Flags);

impl HostImageCopyFlags {
    pub const Memcpy: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureInstanceFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PartitionedAccelerationStructureInstanceFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(PartitionedAccelerationStructureInstanceFlagsNV, Flags);

impl PartitionedAccelerationStructureInstanceFlagsNV {
    pub const FlagTriangleFacingCullDisable: Self = Self(0b1);
    pub const FlagTriangleFlipFacing: Self = Self(0b10);
    pub const FlagForceOpaque: Self = Self(0b100);
    pub const FlagForceNoOpaque: Self = Self(0b1000);
    pub const FlagEnableExplicitBoundingBox: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageConstraintsInfoFlagBitsFUCHSIA.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageConstraintsInfoFlagsFUCHSIA(pub(crate) Flags);
vk_bitflags_wrapped!(ImageConstraintsInfoFlagsFUCHSIA, Flags);

impl ImageConstraintsInfoFlagsFUCHSIA {
    pub const CpuReadRarely: Self = Self(0b1);
    pub const CpuReadOften: Self = Self(0b10);
    pub const CpuWriteRarely: Self = Self(0b100);
    pub const CpuWriteOften: Self = Self(0b1000);
    pub const ProtectedOptional: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGraphicsPipelineLibraryFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct GraphicsPipelineLibraryFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(GraphicsPipelineLibraryFlagsEXT, Flags);

impl GraphicsPipelineLibraryFlagsEXT {
    pub const VertexInputInterface: Self = Self(0b1);
    pub const PreRasterizationShaders: Self = Self(0b10);
    pub const FragmentShader: Self = Self(0b100);
    pub const FragmentOutputInterface: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageCompressionFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCompressionFlagsEXT, Flags);

impl ImageCompressionFlagsEXT {
    pub const Default: Self = Self(0);
    pub const FixedRateDefault: Self = Self(0b1);
    pub const FixedRateExplicit: Self = Self(0b10);
    pub const Disabled: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageCompressionFixedRateFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ImageCompressionFixedRateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ImageCompressionFixedRateFlagsEXT, Flags);

impl ImageCompressionFixedRateFlagsEXT {
    pub const None: Self = Self(0);
    pub const Type1Bpc: Self = Self(0b1);
    pub const Type2Bpc: Self = Self(0b10);
    pub const Type3Bpc: Self = Self(0b100);
    pub const Type4Bpc: Self = Self(0b1000);
    pub const Type5Bpc: Self = Self(0b1_0000);
    pub const Type6Bpc: Self = Self(0b10_0000);
    pub const Type7Bpc: Self = Self(0b100_0000);
    pub const Type8Bpc: Self = Self(0b1000_0000);
    pub const Type9Bpc: Self = Self(0b1_0000_0000);
    pub const Type10Bpc: Self = Self(0b10_0000_0000);
    pub const Type11Bpc: Self = Self(0b100_0000_0000);
    pub const Type12Bpc: Self = Self(0b1000_0000_0000);
    pub const Type13Bpc: Self = Self(0b1_0000_0000_0000);
    pub const Type14Bpc: Self = Self(0b10_0000_0000_0000);
    pub const Type15Bpc: Self = Self(0b100_0000_0000_0000);
    pub const Type16Bpc: Self = Self(0b1000_0000_0000_0000);
    pub const Type17Bpc: Self = Self(0b1_0000_0000_0000_0000);
    pub const Type18Bpc: Self = Self(0b10_0000_0000_0000_0000);
    pub const Type19Bpc: Self = Self(0b100_0000_0000_0000_0000);
    pub const Type20Bpc: Self = Self(0b1000_0000_0000_0000_0000);
    pub const Type21Bpc: Self = Self(0b1_0000_0000_0000_0000_0000);
    pub const Type22Bpc: Self = Self(0b10_0000_0000_0000_0000_0000);
    pub const Type23Bpc: Self = Self(0b100_0000_0000_0000_0000_0000);
    pub const Type24Bpc: Self = Self(0b1000_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkExportMetalObjectTypeFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ExportMetalObjectTypeFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ExportMetalObjectTypeFlagsEXT, Flags);

impl ExportMetalObjectTypeFlagsEXT {
    pub const MetalDevice: Self = Self(0b1);
    pub const MetalCommandQueue: Self = Self(0b10);
    pub const MetalBuffer: Self = Self(0b100);
    pub const MetalTexture: Self = Self(0b1000);
    pub const MetalIosurface: Self = Self(0b1_0000);
    pub const MetalSharedEvent: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRenderingAttachmentFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct RenderingAttachmentFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(RenderingAttachmentFlagsKHR, Flags);

impl RenderingAttachmentFlagsKHR {
    pub const InputAttachmentFeedback: Self = Self(0b1);
    pub const ResolveSkipTransferFunction: Self = Self(0b10);
    pub const ResolveEnableTransferFunction: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResolveImageFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ResolveImageFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(ResolveImageFlagsKHR, Flags);

impl ResolveImageFlagsKHR {
    pub const SkipTransferFunction: Self = Self(0b1);
    pub const EnableTransferFunction: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct DeviceAddressBindingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(DeviceAddressBindingFlagsEXT, Flags);

impl DeviceAddressBindingFlagsEXT {
    pub const InternalObject: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowGridSizeFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpticalFlowGridSizeFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowGridSizeFlagsNV, Flags);

impl OpticalFlowGridSizeFlagsNV {
    pub const Unknown: Self = Self(0);
    pub const Type1X1: Self = Self(0b1);
    pub const Type2X2: Self = Self(0b10);
    pub const Type4X4: Self = Self(0b100);
    pub const Type8X8: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowUsageFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpticalFlowUsageFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowUsageFlagsNV, Flags);

impl OpticalFlowUsageFlagsNV {
    pub const Unknown: Self = Self(0);
    pub const Input: Self = Self(0b1);
    pub const Output: Self = Self(0b10);
    pub const Hint: Self = Self(0b100);
    pub const Cost: Self = Self(0b1000);
    pub const GlobalFlow: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionCreateFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpticalFlowSessionCreateFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowSessionCreateFlagsNV, Flags);

impl OpticalFlowSessionCreateFlagsNV {
    pub const EnableHint: Self = Self(0b1);
    pub const EnableCost: Self = Self(0b10);
    pub const EnableGlobalFlow: Self = Self(0b100);
    pub const AllowRegions: Self = Self(0b1000);
    pub const BothDirections: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowExecuteFlagBitsNV.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct OpticalFlowExecuteFlagsNV(pub(crate) Flags);
vk_bitflags_wrapped!(OpticalFlowExecuteFlagsNV, Flags);

impl OpticalFlowExecuteFlagsNV {
    pub const DisableTemporalHints: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrameBoundaryFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct FrameBoundaryFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(FrameBoundaryFlagsEXT, Flags);

impl FrameBoundaryFlagsEXT {
    pub const FrameEnd: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentScalingFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PresentScalingFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(PresentScalingFlagsKHR, Flags);

impl PresentScalingFlagsKHR {
    pub const OneToOne: Self = Self(0b1);
    pub const AspectRatioStretch: Self = Self(0b10);
    pub const Stretch: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentGravityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PresentGravityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(PresentGravityFlagsKHR, Flags);

impl PresentGravityFlagsKHR {
    pub const Min: Self = Self(0b1);
    pub const Max: Self = Self(0b10);
    pub const Centered: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCreateFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct ShaderCreateFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(ShaderCreateFlagsEXT, Flags);

impl ShaderCreateFlagsEXT {
    pub const LinkStage: Self = Self(0b1);
    pub const DescriptorHeap: Self = Self(0b100_0000_0000);
    pub const AllowVaryingSubgroupSize: Self = Self(0b10);
    pub const RequireFullSubgroups: Self = Self(0b100);
    pub const NoTaskShader: Self = Self(0b1000);
    pub const DispatchBase: Self = Self(0b1_0000);
    pub const FragmentShadingRateAttachment: Self = Self(0b10_0000);
    pub const FragmentDensityMapAttachment: Self = Self(0b100_0000);
    pub const IndirectBindable: Self = Self(0b1000_0000);
    pub const Type64BitIndexing: Self = Self(0b1000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTileShadingRenderPassFlagBitsQCOM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct TileShadingRenderPassFlagsQCOM(pub(crate) Flags);
vk_bitflags_wrapped!(TileShadingRenderPassFlagsQCOM, Flags);

impl TileShadingRenderPassFlagsQCOM {
    pub const Enable: Self = Self(0b1);
    pub const PerTileExecution: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceSchedulingControlsFlagBitsARM.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PhysicalDeviceSchedulingControlsFlagsARM(pub(crate) Flags64);
vk_bitflags_wrapped!(PhysicalDeviceSchedulingControlsFlagsARM, Flags64);

impl PhysicalDeviceSchedulingControlsFlagsARM {
    pub const ShaderCoreCount: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentStageFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PresentStageFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(PresentStageFlagsEXT, Flags);

impl PresentStageFlagsEXT {
    pub const QueueOperationsEnd: Self = Self(0b1);
    pub const RequestDequeued: Self = Self(0b10);
    pub const ImageFirstPixelOut: Self = Self(0b100);
    pub const ImageFirstPixelVisible: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPastPresentationTimingFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PastPresentationTimingFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(PastPresentationTimingFlagsEXT, Flags);

impl PastPresentationTimingFlagsEXT {
    pub const AllowPartialResults: Self = Self(0b1);
    pub const AllowOutOfOrderResults: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentTimingInfoFlagBitsEXT.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct PresentTimingInfoFlagsEXT(pub(crate) Flags);
vk_bitflags_wrapped!(PresentTimingInfoFlagsEXT, Flags);

impl PresentTimingInfoFlagsEXT {
    pub const PresentAtRelativeTime: Self = Self(0b1);
    pub const PresentAtNearestRefreshCycle: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodecOperationFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoCodecOperationFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoCodecOperationFlagsKHR, Flags);

impl VideoCodecOperationFlagsKHR {
    pub const None: Self = Self(0);
    pub const EncodeH264: Self = Self(0b1_0000_0000_0000_0000);
    pub const EncodeH265: Self = Self(0b10_0000_0000_0000_0000);
    pub const DecodeH264: Self = Self(0b1);
    pub const DecodeH265: Self = Self(0b10);
    pub const DecodeAv1: Self = Self(0b100);
    pub const EncodeAv1: Self = Self(0b100_0000_0000_0000_0000);
    pub const DecodeVp9: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCapabilityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoCapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoCapabilityFlagsKHR, Flags);

impl VideoCapabilityFlagsKHR {
    pub const ProtectedContent: Self = Self(0b1);
    pub const SeparateReferenceImages: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionCreateFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoSessionCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoSessionCreateFlagsKHR, Flags);

impl VideoSessionCreateFlagsKHR {
    pub const ProtectedContent: Self = Self(0b1);
    pub const AllowEncodeParameterOptimizations: Self = Self(0b10);
    pub const InlineQueries: Self = Self(0b100);
    pub const AllowEncodeQuantizationDeltaMap: Self = Self(0b1000);
    pub const AllowEncodeEmphasisMap: Self = Self(0b1_0000);
    pub const InlineSessionParameters: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoSessionParametersCreateFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoSessionParametersCreateFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoSessionParametersCreateFlagsKHR, Flags);

impl VideoSessionParametersCreateFlagsKHR {
    pub const QuantizationMapCompatible: Self = Self(0b1);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoCodingControlFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoCodingControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoCodingControlFlagsKHR, Flags);

impl VideoCodingControlFlagsKHR {
    pub const Reset: Self = Self(0b1);
    pub const EncodeRateControl: Self = Self(0b10);
    pub const EncodeQualityLevel: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeUsageFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoDecodeUsageFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoDecodeUsageFlagsKHR, Flags);

impl VideoDecodeUsageFlagsKHR {
    pub const Default: Self = Self(0);
    pub const Transcoding: Self = Self(0b1);
    pub const Offline: Self = Self(0b10);
    pub const Streaming: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeCapabilityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoDecodeCapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoDecodeCapabilityFlagsKHR, Flags);

impl VideoDecodeCapabilityFlagsKHR {
    pub const DpbAndOutputCoincide: Self = Self(0b1);
    pub const DpbAndOutputDistinct: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoDecodeH264PictureLayoutFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoDecodeH264PictureLayoutFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoDecodeH264PictureLayoutFlagsKHR, Flags);

impl VideoDecodeH264PictureLayoutFlagsKHR {
    pub const Progressive: Self = Self(0);
    pub const InterlacedInterleavedLines: Self = Self(0b1);
    pub const InterlacedSeparatePlanes: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeFlagsKHR, Flags);

impl VideoEncodeFlagsKHR {
    pub const IntraRefresh: Self = Self(0b100);
    pub const WithQuantizationDeltaMap: Self = Self(0b1);
    pub const WithEmphasisMap: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeUsageFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeUsageFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeUsageFlagsKHR, Flags);

impl VideoEncodeUsageFlagsKHR {
    pub const Default: Self = Self(0);
    pub const Transcoding: Self = Self(0b1);
    pub const Streaming: Self = Self(0b10);
    pub const Recording: Self = Self(0b100);
    pub const Conferencing: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeContentFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeContentFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeContentFlagsKHR, Flags);

impl VideoEncodeContentFlagsKHR {
    pub const Default: Self = Self(0);
    pub const Camera: Self = Self(0b1);
    pub const Desktop: Self = Self(0b10);
    pub const Rendered: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeCapabilityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeCapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeCapabilityFlagsKHR, Flags);

impl VideoEncodeCapabilityFlagsKHR {
    pub const PrecedingExternallyEncodedBytes: Self = Self(0b1);
    pub const InsufficientBitstreamBufferRangeDetection: Self = Self(0b10);
    pub const QuantizationDeltaMap: Self = Self(0b100);
    pub const EmphasisMap: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeFeedbackFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeFeedbackFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeFeedbackFlagsKHR, Flags);

impl VideoEncodeFeedbackFlagsKHR {
    pub const BitstreamBufferOffset: Self = Self(0b1);
    pub const BitstreamBytesWritten: Self = Self(0b10);
    pub const BitstreamHasOverrides: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeRateControlModeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeRateControlModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeRateControlModeFlagsKHR, Flags);

impl VideoEncodeRateControlModeFlagsKHR {
    pub const Default: Self = Self(0);
    pub const Disabled: Self = Self(0b1);
    pub const Cbr: Self = Self(0b10);
    pub const Vbr: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeIntraRefreshModeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeIntraRefreshModeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeIntraRefreshModeFlagsKHR, Flags);

impl VideoEncodeIntraRefreshModeFlagsKHR {
    pub const None: Self = Self(0);
    pub const PerPicturePartition: Self = Self(0b1);
    pub const BlockBased: Self = Self(0b10);
    pub const BlockRowBased: Self = Self(0b100);
    pub const BlockColumnBased: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoChromaSubsamplingFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoChromaSubsamplingFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoChromaSubsamplingFlagsKHR, Flags);

impl VideoChromaSubsamplingFlagsKHR {
    pub const Invalid: Self = Self(0);
    pub const Monochrome: Self = Self(0b1);
    pub const Type420: Self = Self(0b10);
    pub const Type422: Self = Self(0b100);
    pub const Type444: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoComponentBitDepthFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoComponentBitDepthFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoComponentBitDepthFlagsKHR, Flags);

impl VideoComponentBitDepthFlagsKHR {
    pub const Invalid: Self = Self(0);
    pub const Type8: Self = Self(0b1);
    pub const Type10: Self = Self(0b100);
    pub const Type12: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264CapabilityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH264CapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH264CapabilityFlagsKHR, Flags);

impl VideoEncodeH264CapabilityFlagsKHR {
    pub const HrdCompliance: Self = Self(0b1);
    pub const PredictionWeightTableGenerated: Self = Self(0b10);
    pub const RowUnalignedSlice: Self = Self(0b100);
    pub const DifferentSliceType: Self = Self(0b1000);
    pub const BFrameInL0List: Self = Self(0b1_0000);
    pub const BFrameInL1List: Self = Self(0b10_0000);
    pub const PerPictureTypeMinMaxQp: Self = Self(0b100_0000);
    pub const PerSliceConstantQp: Self = Self(0b1000_0000);
    pub const GeneratePrefixNalu: Self = Self(0b1_0000_0000);
    pub const BPictureIntraRefresh: Self = Self(0b100_0000_0000);
    pub const MbQpDiffWraparound: Self = Self(0b10_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264StdFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH264StdFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH264StdFlagsKHR, Flags);

impl VideoEncodeH264StdFlagsKHR {
    pub const SeparateColorPlaneFlagSet: Self = Self(0b1);
    pub const QpprimeYZeroTransformBypassFlagSet: Self = Self(0b10);
    pub const ScalingMatrixPresentFlagSet: Self = Self(0b100);
    pub const ChromaQpIndexOffset: Self = Self(0b1000);
    pub const SecondChromaQpIndexOffset: Self = Self(0b1_0000);
    pub const PicInitQpMinus26: Self = Self(0b10_0000);
    pub const WeightedPredFlagSet: Self = Self(0b100_0000);
    pub const WeightedBipredIdcExplicit: Self = Self(0b1000_0000);
    pub const WeightedBipredIdcImplicit: Self = Self(0b1_0000_0000);
    pub const Transform8X8ModeFlagSet: Self = Self(0b10_0000_0000);
    pub const DirectSpatialMvPredFlagUnset: Self = Self(0b100_0000_0000);
    pub const EntropyCodingModeFlagUnset: Self = Self(0b1000_0000_0000);
    pub const EntropyCodingModeFlagSet: Self = Self(0b1_0000_0000_0000);
    pub const Direct8X8InferenceFlagUnset: Self = Self(0b10_0000_0000_0000);
    pub const ConstrainedIntraPredFlagSet: Self = Self(0b100_0000_0000_0000);
    pub const DeblockingFilterDisabled: Self = Self(0b1000_0000_0000_0000);
    pub const DeblockingFilterEnabled: Self = Self(0b1_0000_0000_0000_0000);
    pub const DeblockingFilterPartial: Self = Self(0b10_0000_0000_0000_0000);
    pub const SliceQpDelta: Self = Self(0b1000_0000_0000_0000_0000);
    pub const DifferentSliceQpDelta: Self = Self(0b1_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH264RateControlFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH264RateControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH264RateControlFlagsKHR, Flags);

impl VideoEncodeH264RateControlFlagsKHR {
    pub const AttemptHrdCompliance: Self = Self(0b1);
    pub const RegularGop: Self = Self(0b10);
    pub const ReferencePatternFlat: Self = Self(0b100);
    pub const ReferencePatternDyadic: Self = Self(0b1000);
    pub const TemporalLayerPatternDyadic: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CapabilityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH265CapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265CapabilityFlagsKHR, Flags);

impl VideoEncodeH265CapabilityFlagsKHR {
    pub const HrdCompliance: Self = Self(0b1);
    pub const PredictionWeightTableGenerated: Self = Self(0b10);
    pub const RowUnalignedSliceSegment: Self = Self(0b100);
    pub const DifferentSliceSegmentType: Self = Self(0b1000);
    pub const BFrameInL0List: Self = Self(0b1_0000);
    pub const BFrameInL1List: Self = Self(0b10_0000);
    pub const PerPictureTypeMinMaxQp: Self = Self(0b100_0000);
    pub const PerSliceSegmentConstantQp: Self = Self(0b1000_0000);
    pub const MultipleTilesPerSliceSegment: Self = Self(0b1_0000_0000);
    pub const MultipleSliceSegmentsPerTile: Self = Self(0b10_0000_0000);
    pub const BPictureIntraRefresh: Self = Self(0b1000_0000_0000);
    pub const CuQpDiffWraparound: Self = Self(0b100_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265StdFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH265StdFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265StdFlagsKHR, Flags);

impl VideoEncodeH265StdFlagsKHR {
    pub const SeparateColorPlaneFlagSet: Self = Self(0b1);
    pub const SampleAdaptiveOffsetEnabledFlagSet: Self = Self(0b10);
    pub const ScalingListDataPresentFlagSet: Self = Self(0b100);
    pub const PcmEnabledFlagSet: Self = Self(0b1000);
    pub const SpsTemporalMvpEnabledFlagSet: Self = Self(0b1_0000);
    pub const InitQpMinus26: Self = Self(0b10_0000);
    pub const WeightedPredFlagSet: Self = Self(0b100_0000);
    pub const WeightedBipredFlagSet: Self = Self(0b1000_0000);
    pub const Log2ParallelMergeLevelMinus2: Self = Self(0b1_0000_0000);
    pub const SignDataHidingEnabledFlagSet: Self = Self(0b10_0000_0000);
    pub const TransformSkipEnabledFlagSet: Self = Self(0b100_0000_0000);
    pub const TransformSkipEnabledFlagUnset: Self = Self(0b1000_0000_0000);
    pub const PpsSliceChromaQpOffsetsPresentFlagSet: Self = Self(0b1_0000_0000_0000);
    pub const TransquantBypassEnabledFlagSet: Self = Self(0b10_0000_0000_0000);
    pub const ConstrainedIntraPredFlagSet: Self = Self(0b100_0000_0000_0000);
    pub const EntropyCodingSyncEnabledFlagSet: Self = Self(0b1000_0000_0000_0000);
    pub const DeblockingFilterOverrideEnabledFlagSet: Self = Self(0b1_0000_0000_0000_0000);
    pub const DependentSliceSegmentsEnabledFlagSet: Self = Self(0b10_0000_0000_0000_0000);
    pub const DependentSliceSegmentFlagSet: Self = Self(0b100_0000_0000_0000_0000);
    pub const SliceQpDelta: Self = Self(0b1000_0000_0000_0000_0000);
    pub const DifferentSliceQpDelta: Self = Self(0b1_0000_0000_0000_0000_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265RateControlFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH265RateControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265RateControlFlagsKHR, Flags);

impl VideoEncodeH265RateControlFlagsKHR {
    pub const AttemptHrdCompliance: Self = Self(0b1);
    pub const RegularGop: Self = Self(0b10);
    pub const ReferencePatternFlat: Self = Self(0b100);
    pub const ReferencePatternDyadic: Self = Self(0b1000);
    pub const TemporalSubLayerPatternDyadic: Self = Self(0b1_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265CtbSizeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH265CtbSizeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265CtbSizeFlagsKHR, Flags);

impl VideoEncodeH265CtbSizeFlagsKHR {
    pub const Type16: Self = Self(0b1);
    pub const Type32: Self = Self(0b10);
    pub const Type64: Self = Self(0b100);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeH265TransformBlockSizeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeH265TransformBlockSizeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeH265TransformBlockSizeFlagsKHR, Flags);

impl VideoEncodeH265TransformBlockSizeFlagsKHR {
    pub const Type4: Self = Self(0b1);
    pub const Type8: Self = Self(0b10);
    pub const Type16: Self = Self(0b100);
    pub const Type32: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1CapabilityFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeAV1CapabilityFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeAV1CapabilityFlagsKHR, Flags);

impl VideoEncodeAV1CapabilityFlagsKHR {
    pub const PerRateControlGroupMinMaxQIndex: Self = Self(0b1);
    pub const GenerateObuExtensionHeader: Self = Self(0b10);
    pub const PrimaryReferenceCdfOnly: Self = Self(0b100);
    pub const FrameSizeOverride: Self = Self(0b1000);
    pub const MotionVectorScaling: Self = Self(0b1_0000);
    pub const CompoundPredictionIntraRefresh: Self = Self(0b10_0000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1StdFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeAV1StdFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeAV1StdFlagsKHR, Flags);

impl VideoEncodeAV1StdFlagsKHR {
    pub const UniformTileSpacingFlagSet: Self = Self(0b1);
    pub const SkipModePresentUnset: Self = Self(0b10);
    pub const PrimaryRefFrame: Self = Self(0b100);
    pub const DeltaQ: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeAV1RateControlFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeAV1RateControlFlagsKHR, Flags);

impl VideoEncodeAV1RateControlFlagsKHR {
    pub const RegularGop: Self = Self(0b1);
    pub const TemporalLayerPatternDyadic: Self = Self(0b10);
    pub const ReferencePatternFlat: Self = Self(0b100);
    pub const ReferencePatternDyadic: Self = Self(0b1000);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1SuperblockSizeFlagBitsKHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct VideoEncodeAV1SuperblockSizeFlagsKHR(pub(crate) Flags);
vk_bitflags_wrapped!(VideoEncodeAV1SuperblockSizeFlagsKHR, Flags);

impl VideoEncodeAV1SuperblockSizeFlagsKHR {
    pub const Type64: Self = Self(0b1);
    pub const Type128: Self = Self(0b10);
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccessFlagBits3KHR.html>
#[repr(transparent)]
#[derive(Clone, Copy, Hash, PartialEq, Eq)]
pub struct AccessFlags3KHR(pub(crate) Flags64);
vk_bitflags_wrapped!(AccessFlags3KHR, Flags64);

impl AccessFlags3KHR {
    pub const None: Self = Self(0);
}
