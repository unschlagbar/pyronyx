// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/enums.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

#![allow(non_camel_case_types)]

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageLayout.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ImageLayout {
    /// Implicit layout an image is when its contents are undefined due to various reasons (e.g. right after creation)
    #[default]
    Undefined = 0,
    /// General layout when image can be used for any kind of access
    General = 1,
    /// Optimal layout when image is only used for color attachment read/write
    ColorAttachmentOptimal = 2,
    /// Optimal layout when image is only used for depth/stencil attachment read/write
    DepthStencilAttachmentOptimal = 3,
    /// Optimal layout when image is used for read only depth/stencil attachment and shader access
    DepthStencilReadOnlyOptimal = 4,
    /// Optimal layout when image is used for read only shader access
    ShaderReadOnlyOptimal = 5,
    /// Optimal layout when image is used only as source of transfer operations
    TransferSrcOptimal = 6,
    /// Optimal layout when image is used only as destination of transfer operations
    TransferDstOptimal = 7,
    /// Initial layout used when the data is populated by the CPU
    Preinitialized = 8,
    DepthReadOnlyStencilAttachmentOptimal = 1000117000,
    DepthAttachmentStencilReadOnlyOptimal = 1000117001,
    DepthAttachmentOptimal = 1000241000,
    DepthReadOnlyOptimal = 1000241001,
    StencilAttachmentOptimal = 1000241002,
    StencilReadOnlyOptimal = 1000241003,
    ReadOnlyOptimal = 1000314000,
    AttachmentOptimal = 1000314001,
    RenderingLocalRead = 1000232000,
    PresentSrcKHR = 1000001002,
    VideoDecodeDstKHR = 1000024000,
    VideoDecodeSrcKHR = 1000024001,
    VideoDecodeDpbKHR = 1000024002,
    SharedPresentKHR = 1000111000,
    FragmentDensityMapOptimalEXT = 1000218000,
    FragmentShadingRateAttachmentOptimalKHR = 1000164003,
    VideoEncodeDstKHR = 1000299000,
    VideoEncodeSrcKHR = 1000299001,
    VideoEncodeDpbKHR = 1000299002,
    AttachmentFeedbackLoopOptimalEXT = 1000339000,
    TensorAliasingARM = 1000460000,
    VideoEncodeQuantizationMapKHR = 1000553000,
    ZeroInitializedEXT = 1000620000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentLoadOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AttachmentLoadOp {
    #[default]
    Load = 0,
    Clear = 1,
    DontCare = 2,
    None = 1000400000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAttachmentStoreOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AttachmentStoreOp {
    #[default]
    Store = 0,
    DontCare = 1,
    None = 1000301000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ImageType {
    #[default]
    Type1d = 0,
    Type2d = 1,
    Type3d = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageTiling.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ImageTiling {
    #[default]
    Optimal = 0,
    Linear = 1,
    DrmFormatModifierEXT = 1000158000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkImageViewType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ImageViewType {
    #[default]
    Type1d = 0,
    Type2d = 1,
    Type3d = 2,
    Cube = 3,
    Type1dArray = 4,
    Type2dArray = 5,
    CubeArray = 6,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCommandBufferLevel.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CommandBufferLevel {
    #[default]
    Primary = 0,
    Secondary = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentSwizzle.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ComponentSwizzle {
    #[default]
    Identity = 0,
    Zero = 1,
    One = 2,
    R = 3,
    G = 4,
    B = 5,
    A = 6,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DescriptorType {
    #[default]
    Sampler = 0,
    CombinedImageSampler = 1,
    SampledImage = 2,
    StorageImage = 3,
    UniformTexelBuffer = 4,
    StorageTexelBuffer = 5,
    UniformBuffer = 6,
    StorageBuffer = 7,
    UniformBufferDynamic = 8,
    StorageBufferDynamic = 9,
    InputAttachment = 10,
    InlineUniformBlock = 1000138000,
    AccelerationStructureKHR = 1000150000,
    AccelerationStructureNV = 1000165000,
    SampleWeightImageQCOM = 1000440000,
    BlockMatchImageQCOM = 1000440001,
    TensorARM = 1000460000,
    MutableEXT = 1000351000,
    PartitionedAccelerationStructureNV = 1000570000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum QueryType {
    #[default]
    Occlusion = 0,
    /// Optional
    PipelineStatistics = 1,
    Timestamp = 2,
    ResultStatusOnlyKHR = 1000023000,
    TransformFeedbackStreamEXT = 1000028004,
    PerformanceQueryKHR = 1000116000,
    AccelerationStructureCompactedSizeKHR = 1000150000,
    AccelerationStructureSerializationSizeKHR = 1000150001,
    AccelerationStructureCompactedSizeNV = 1000165000,
    PerformanceQueryINTEL = 1000210000,
    VideoEncodeFeedbackKHR = 1000299000,
    MeshPrimitivesGeneratedEXT = 1000328000,
    PrimitivesGeneratedEXT = 1000382000,
    AccelerationStructureSerializationBottomLevelPointersKHR = 1000386000,
    AccelerationStructureSizeKHR = 1000386001,
    MicromapSerializationSizeEXT = 1000396000,
    MicromapCompactedSizeEXT = 1000396001,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBorderColor.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BorderColor {
    #[default]
    FloatTransparentBlack = 0,
    IntTransparentBlack = 1,
    FloatOpaqueBlack = 2,
    IntOpaqueBlack = 3,
    FloatOpaqueWhite = 4,
    IntOpaqueWhite = 5,
    FloatCustomEXT = 1000287003,
    IntCustomEXT = 1000287004,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineBindPoint.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineBindPoint {
    #[default]
    Graphics = 0,
    Compute = 1,
    ExecutionGraphAMDX = 1000134000,
    RayTracingKHR = 1000165000,
    SubpassShadingHUAWEI = 1000369003,
    DataGraphARM = 1000507000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheHeaderVersion.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineCacheHeaderVersion {
    #[default]
    One = 1,
    SafetyCriticalOne = 1000298001,
    DataGraphQCOM = 1000629000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPrimitiveTopology.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PrimitiveTopology {
    #[default]
    PointList = 0,
    LineList = 1,
    LineStrip = 2,
    TriangleList = 3,
    TriangleStrip = 4,
    TriangleFan = 5,
    LineListWithAdjacency = 6,
    LineStripWithAdjacency = 7,
    TriangleListWithAdjacency = 8,
    TriangleStripWithAdjacency = 9,
    PatchList = 10,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSharingMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SharingMode {
    #[default]
    Exclusive = 0,
    Concurrent = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndexType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum IndexType {
    #[default]
    Uint16 = 0,
    Uint32 = 1,
    Uint8 = 1000265000,
    NoneKHR = 1000165000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFilter.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Filter {
    #[default]
    Nearest = 0,
    Linear = 1,
    CubicEXT = 1000015000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerMipmapMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SamplerMipmapMode {
    /// Choose nearest mip level
    #[default]
    Nearest = 0,
    /// Linear filter between mip levels
    Linear = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerAddressMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SamplerAddressMode {
    #[default]
    Repeat = 0,
    MirroredRepeat = 1,
    ClampToEdge = 2,
    ClampToBorder = 3,
    /// No need to add an extnumber attribute, since this uses a core enum value
    MirrorClampToEdge = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompareOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CompareOp {
    #[default]
    Never = 0,
    Less = 1,
    Equal = 2,
    LessOrEqual = 3,
    Greater = 4,
    NotEqual = 5,
    GreaterOrEqual = 6,
    Always = 7,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPolygonMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PolygonMode {
    #[default]
    Fill = 0,
    Line = 1,
    Point = 2,
    FillRectangleNV = 1000153000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFrontFace.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FrontFace {
    #[default]
    CounterClockwise = 0,
    Clockwise = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendFactor.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BlendFactor {
    #[default]
    Zero = 0,
    One = 1,
    SrcColor = 2,
    OneMinusSrcColor = 3,
    DstColor = 4,
    OneMinusDstColor = 5,
    SrcAlpha = 6,
    OneMinusSrcAlpha = 7,
    DstAlpha = 8,
    OneMinusDstAlpha = 9,
    ConstantColor = 10,
    OneMinusConstantColor = 11,
    ConstantAlpha = 12,
    OneMinusConstantAlpha = 13,
    SrcAlphaSaturate = 14,
    Src1Color = 15,
    OneMinusSrc1Color = 16,
    Src1Alpha = 17,
    OneMinusSrc1Alpha = 18,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BlendOp {
    #[default]
    Add = 0,
    Subtract = 1,
    ReverseSubtract = 2,
    Min = 3,
    Max = 4,
    ZeroEXT = 1000148000,
    SrcEXT = 1000148001,
    DstEXT = 1000148002,
    SrcOverEXT = 1000148003,
    DstOverEXT = 1000148004,
    SrcInEXT = 1000148005,
    DstInEXT = 1000148006,
    SrcOutEXT = 1000148007,
    DstOutEXT = 1000148008,
    SrcAtopEXT = 1000148009,
    DstAtopEXT = 1000148010,
    XorEXT = 1000148011,
    MultiplyEXT = 1000148012,
    ScreenEXT = 1000148013,
    OverlayEXT = 1000148014,
    DarkenEXT = 1000148015,
    LightenEXT = 1000148016,
    ColordodgeEXT = 1000148017,
    ColorburnEXT = 1000148018,
    HardlightEXT = 1000148019,
    SoftlightEXT = 1000148020,
    DifferenceEXT = 1000148021,
    ExclusionEXT = 1000148022,
    InvertEXT = 1000148023,
    InvertRgbEXT = 1000148024,
    LineardodgeEXT = 1000148025,
    LinearburnEXT = 1000148026,
    VividlightEXT = 1000148027,
    LinearlightEXT = 1000148028,
    PinlightEXT = 1000148029,
    HardmixEXT = 1000148030,
    HslHueEXT = 1000148031,
    HslSaturationEXT = 1000148032,
    HslColorEXT = 1000148033,
    HslLuminosityEXT = 1000148034,
    PlusEXT = 1000148035,
    PlusClampedEXT = 1000148036,
    PlusClampedAlphaEXT = 1000148037,
    PlusDarkerEXT = 1000148038,
    MinusEXT = 1000148039,
    MinusClampedEXT = 1000148040,
    ContrastEXT = 1000148041,
    InvertOvgEXT = 1000148042,
    RedEXT = 1000148043,
    GreenEXT = 1000148044,
    BlueEXT = 1000148045,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStencilOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum StencilOp {
    #[default]
    Keep = 0,
    Zero = 1,
    Replace = 2,
    IncrementAndClamp = 3,
    DecrementAndClamp = 4,
    Invert = 5,
    IncrementAndWrap = 6,
    DecrementAndWrap = 7,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLogicOp.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum LogicOp {
    #[default]
    Clear = 0,
    And = 1,
    AndReverse = 2,
    Copy = 3,
    AndInverted = 4,
    NoOp = 5,
    Xor = 6,
    Or = 7,
    Nor = 8,
    Equivalent = 9,
    Invert = 10,
    OrReverse = 11,
    CopyInverted = 12,
    OrInverted = 13,
    Nand = 14,
    Set = 15,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkInternalAllocationType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum InternalAllocationType {
    #[default]
    Executable = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSystemAllocationScope.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SystemAllocationScope {
    #[default]
    Command = 0,
    Object = 1,
    Cache = 2,
    Device = 3,
    Instance = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PhysicalDeviceType {
    #[default]
    Other = 0,
    IntegratedGpu = 1,
    DiscreteGpu = 2,
    VirtualGpu = 3,
    Cpu = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVertexInputRate.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum VertexInputRate {
    #[default]
    Vertex = 0,
    Instance = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFormat.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum Format {
    #[default]
    Undefined = 0,
    R4G4UnormPack8 = 1,
    R4G4B4A4UnormPack16 = 2,
    B4G4R4A4UnormPack16 = 3,
    R5G6B5UnormPack16 = 4,
    B5G6R5UnormPack16 = 5,
    R5G5B5A1UnormPack16 = 6,
    B5G5R5A1UnormPack16 = 7,
    A1R5G5B5UnormPack16 = 8,
    R8Unorm = 9,
    R8Snorm = 10,
    R8Uscaled = 11,
    R8Sscaled = 12,
    R8Uint = 13,
    R8Sint = 14,
    R8Srgb = 15,
    R8G8Unorm = 16,
    R8G8Snorm = 17,
    R8G8Uscaled = 18,
    R8G8Sscaled = 19,
    R8G8Uint = 20,
    R8G8Sint = 21,
    R8G8Srgb = 22,
    R8G8B8Unorm = 23,
    R8G8B8Snorm = 24,
    R8G8B8Uscaled = 25,
    R8G8B8Sscaled = 26,
    R8G8B8Uint = 27,
    R8G8B8Sint = 28,
    R8G8B8Srgb = 29,
    B8G8R8Unorm = 30,
    B8G8R8Snorm = 31,
    B8G8R8Uscaled = 32,
    B8G8R8Sscaled = 33,
    B8G8R8Uint = 34,
    B8G8R8Sint = 35,
    B8G8R8Srgb = 36,
    R8G8B8A8Unorm = 37,
    R8G8B8A8Snorm = 38,
    R8G8B8A8Uscaled = 39,
    R8G8B8A8Sscaled = 40,
    R8G8B8A8Uint = 41,
    R8G8B8A8Sint = 42,
    R8G8B8A8Srgb = 43,
    B8G8R8A8Unorm = 44,
    B8G8R8A8Snorm = 45,
    B8G8R8A8Uscaled = 46,
    B8G8R8A8Sscaled = 47,
    B8G8R8A8Uint = 48,
    B8G8R8A8Sint = 49,
    B8G8R8A8Srgb = 50,
    A8B8G8R8UnormPack32 = 51,
    A8B8G8R8SnormPack32 = 52,
    A8B8G8R8UscaledPack32 = 53,
    A8B8G8R8SscaledPack32 = 54,
    A8B8G8R8UintPack32 = 55,
    A8B8G8R8SintPack32 = 56,
    A8B8G8R8SrgbPack32 = 57,
    A2R10G10B10UnormPack32 = 58,
    A2R10G10B10SnormPack32 = 59,
    A2R10G10B10UscaledPack32 = 60,
    A2R10G10B10SscaledPack32 = 61,
    A2R10G10B10UintPack32 = 62,
    A2R10G10B10SintPack32 = 63,
    A2B10G10R10UnormPack32 = 64,
    A2B10G10R10SnormPack32 = 65,
    A2B10G10R10UscaledPack32 = 66,
    A2B10G10R10SscaledPack32 = 67,
    A2B10G10R10UintPack32 = 68,
    A2B10G10R10SintPack32 = 69,
    R16Unorm = 70,
    R16Snorm = 71,
    R16Uscaled = 72,
    R16Sscaled = 73,
    R16Uint = 74,
    R16Sint = 75,
    R16Sfloat = 76,
    R16G16Unorm = 77,
    R16G16Snorm = 78,
    R16G16Uscaled = 79,
    R16G16Sscaled = 80,
    R16G16Uint = 81,
    R16G16Sint = 82,
    R16G16Sfloat = 83,
    R16G16B16Unorm = 84,
    R16G16B16Snorm = 85,
    R16G16B16Uscaled = 86,
    R16G16B16Sscaled = 87,
    R16G16B16Uint = 88,
    R16G16B16Sint = 89,
    R16G16B16Sfloat = 90,
    R16G16B16A16Unorm = 91,
    R16G16B16A16Snorm = 92,
    R16G16B16A16Uscaled = 93,
    R16G16B16A16Sscaled = 94,
    R16G16B16A16Uint = 95,
    R16G16B16A16Sint = 96,
    R16G16B16A16Sfloat = 97,
    R32Uint = 98,
    R32Sint = 99,
    R32Sfloat = 100,
    R32G32Uint = 101,
    R32G32Sint = 102,
    R32G32Sfloat = 103,
    R32G32B32Uint = 104,
    R32G32B32Sint = 105,
    R32G32B32Sfloat = 106,
    R32G32B32A32Uint = 107,
    R32G32B32A32Sint = 108,
    R32G32B32A32Sfloat = 109,
    R64Uint = 110,
    R64Sint = 111,
    R64Sfloat = 112,
    R64G64Uint = 113,
    R64G64Sint = 114,
    R64G64Sfloat = 115,
    R64G64B64Uint = 116,
    R64G64B64Sint = 117,
    R64G64B64Sfloat = 118,
    R64G64B64A64Uint = 119,
    R64G64B64A64Sint = 120,
    R64G64B64A64Sfloat = 121,
    B10G11R11UfloatPack32 = 122,
    E5B9G9R9UfloatPack32 = 123,
    D16Unorm = 124,
    X8D24UnormPack32 = 125,
    D32Sfloat = 126,
    S8Uint = 127,
    D16UnormS8Uint = 128,
    D24UnormS8Uint = 129,
    D32SfloatS8Uint = 130,
    BC1RgbUnormBlock = 131,
    BC1RgbSrgbBlock = 132,
    BC1RgbaUnormBlock = 133,
    BC1RgbaSrgbBlock = 134,
    BC2UnormBlock = 135,
    BC2SrgbBlock = 136,
    BC3UnormBlock = 137,
    BC3SrgbBlock = 138,
    BC4UnormBlock = 139,
    BC4SnormBlock = 140,
    BC5UnormBlock = 141,
    BC5SnormBlock = 142,
    BC6HUfloatBlock = 143,
    BC6HSfloatBlock = 144,
    BC7UnormBlock = 145,
    BC7SrgbBlock = 146,
    ETC2R8G8B8UnormBlock = 147,
    ETC2R8G8B8SrgbBlock = 148,
    ETC2R8G8B8A1UnormBlock = 149,
    ETC2R8G8B8A1SrgbBlock = 150,
    ETC2R8G8B8A8UnormBlock = 151,
    ETC2R8G8B8A8SrgbBlock = 152,
    EacR11UnormBlock = 153,
    EacR11SnormBlock = 154,
    EacR11G11UnormBlock = 155,
    EacR11G11SnormBlock = 156,
    Astc4x4UnormBlock = 157,
    Astc4x4SrgbBlock = 158,
    Astc5x4UnormBlock = 159,
    Astc5x4SrgbBlock = 160,
    Astc5x5UnormBlock = 161,
    Astc5x5SrgbBlock = 162,
    Astc6x5UnormBlock = 163,
    Astc6x5SrgbBlock = 164,
    Astc6x6UnormBlock = 165,
    Astc6x6SrgbBlock = 166,
    Astc8x5UnormBlock = 167,
    Astc8x5SrgbBlock = 168,
    Astc8x6UnormBlock = 169,
    Astc8x6SrgbBlock = 170,
    Astc8x8UnormBlock = 171,
    Astc8x8SrgbBlock = 172,
    Astc10x5UnormBlock = 173,
    Astc10x5SrgbBlock = 174,
    Astc10x6UnormBlock = 175,
    Astc10x6SrgbBlock = 176,
    Astc10x8UnormBlock = 177,
    Astc10x8SrgbBlock = 178,
    Astc10x10UnormBlock = 179,
    Astc10x10SrgbBlock = 180,
    Astc12x10UnormBlock = 181,
    Astc12x10SrgbBlock = 182,
    Astc12x12UnormBlock = 183,
    Astc12x12SrgbBlock = 184,
    G8B8G8R8422Unorm = 1000156000,
    B8G8R8G8422Unorm = 1000156001,
    G8B8R83Plane420Unorm = 1000156002,
    G8B8R82Plane420Unorm = 1000156003,
    G8B8R83Plane422Unorm = 1000156004,
    G8B8R82Plane422Unorm = 1000156005,
    G8B8R83Plane444Unorm = 1000156006,
    R10X6UnormPack16 = 1000156007,
    R10X6G10X6Unorm2Pack16 = 1000156008,
    R10X6G10X6B10X6A10X6Unorm4Pack16 = 1000156009,
    G10X6B10X6G10X6R10X6422Unorm4Pack16 = 1000156010,
    B10X6G10X6R10X6G10X6422Unorm4Pack16 = 1000156011,
    G10X6B10X6R10X63Plane420Unorm3Pack16 = 1000156012,
    G10X6B10X6R10X62Plane420Unorm3Pack16 = 1000156013,
    G10X6B10X6R10X63Plane422Unorm3Pack16 = 1000156014,
    G10X6B10X6R10X62Plane422Unorm3Pack16 = 1000156015,
    G10X6B10X6R10X63Plane444Unorm3Pack16 = 1000156016,
    R12X4UnormPack16 = 1000156017,
    R12X4G12X4Unorm2Pack16 = 1000156018,
    R12X4G12X4B12X4A12X4Unorm4Pack16 = 1000156019,
    G12X4B12X4G12X4R12X4422Unorm4Pack16 = 1000156020,
    B12X4G12X4R12X4G12X4422Unorm4Pack16 = 1000156021,
    G12X4B12X4R12X43Plane420Unorm3Pack16 = 1000156022,
    G12X4B12X4R12X42Plane420Unorm3Pack16 = 1000156023,
    G12X4B12X4R12X43Plane422Unorm3Pack16 = 1000156024,
    G12X4B12X4R12X42Plane422Unorm3Pack16 = 1000156025,
    G12X4B12X4R12X43Plane444Unorm3Pack16 = 1000156026,
    G16B16G16R16422Unorm = 1000156027,
    B16G16R16G16422Unorm = 1000156028,
    G16B16R163Plane420Unorm = 1000156029,
    G16B16R162Plane420Unorm = 1000156030,
    G16B16R163Plane422Unorm = 1000156031,
    G16B16R162Plane422Unorm = 1000156032,
    G16B16R163Plane444Unorm = 1000156033,
    G8B8R82Plane444Unorm = 1000330000,
    G10X6B10X6R10X62Plane444Unorm3Pack16 = 1000330001,
    G12X4B12X4R12X42Plane444Unorm3Pack16 = 1000330002,
    G16B16R162Plane444Unorm = 1000330003,
    A4R4G4B4UnormPack16 = 1000340000,
    A4B4G4R4UnormPack16 = 1000340001,
    Astc4x4SfloatBlock = 1000066000,
    Astc5x4SfloatBlock = 1000066001,
    Astc5x5SfloatBlock = 1000066002,
    Astc6x5SfloatBlock = 1000066003,
    Astc6x6SfloatBlock = 1000066004,
    Astc8x5SfloatBlock = 1000066005,
    Astc8x6SfloatBlock = 1000066006,
    Astc8x8SfloatBlock = 1000066007,
    Astc10x5SfloatBlock = 1000066008,
    Astc10x6SfloatBlock = 1000066009,
    Astc10x8SfloatBlock = 1000066010,
    Astc10x10SfloatBlock = 1000066011,
    Astc12x10SfloatBlock = 1000066012,
    Astc12x12SfloatBlock = 1000066013,
    A1B5G5R5UnormPack16 = 1000470000,
    A8Unorm = 1000470001,
    PVRTC12BPPUnormBlockIMG = 1000054000,
    PVRTC14BPPUnormBlockIMG = 1000054001,
    PVRTC22BPPUnormBlockIMG = 1000054002,
    PVRTC24BPPUnormBlockIMG = 1000054003,
    PVRTC12BPPSrgbBlockIMG = 1000054004,
    PVRTC14BPPSrgbBlockIMG = 1000054005,
    PVRTC22BPPSrgbBlockIMG = 1000054006,
    PVRTC24BPPSrgbBlockIMG = 1000054007,
    Astc3x3x3UnormBlockEXT = 1000288000,
    Astc3x3x3SrgbBlockEXT = 1000288001,
    Astc3x3x3SfloatBlockEXT = 1000288002,
    Astc4x3x3UnormBlockEXT = 1000288003,
    Astc4x3x3SrgbBlockEXT = 1000288004,
    Astc4x3x3SfloatBlockEXT = 1000288005,
    Astc4x4x3UnormBlockEXT = 1000288006,
    Astc4x4x3SrgbBlockEXT = 1000288007,
    Astc4x4x3SfloatBlockEXT = 1000288008,
    Astc4x4x4UnormBlockEXT = 1000288009,
    Astc4x4x4SrgbBlockEXT = 1000288010,
    Astc4x4x4SfloatBlockEXT = 1000288011,
    Astc5x4x4UnormBlockEXT = 1000288012,
    Astc5x4x4SrgbBlockEXT = 1000288013,
    Astc5x4x4SfloatBlockEXT = 1000288014,
    Astc5x5x4UnormBlockEXT = 1000288015,
    Astc5x5x4SrgbBlockEXT = 1000288016,
    Astc5x5x4SfloatBlockEXT = 1000288017,
    Astc5x5x5UnormBlockEXT = 1000288018,
    Astc5x5x5SrgbBlockEXT = 1000288019,
    Astc5x5x5SfloatBlockEXT = 1000288020,
    Astc6x5x5UnormBlockEXT = 1000288021,
    Astc6x5x5SrgbBlockEXT = 1000288022,
    Astc6x5x5SfloatBlockEXT = 1000288023,
    Astc6x6x5UnormBlockEXT = 1000288024,
    Astc6x6x5SrgbBlockEXT = 1000288025,
    Astc6x6x5SfloatBlockEXT = 1000288026,
    Astc6x6x6UnormBlockEXT = 1000288027,
    Astc6x6x6SrgbBlockEXT = 1000288028,
    Astc6x6x6SfloatBlockEXT = 1000288029,
    R8BoolARM = 1000460000,
    R16G16SFixed5NV = 1000464000,
    R10X6UintPack16ARM = 1000609000,
    R10X6G10X6Uint2Pack16ARM = 1000609001,
    R10X6G10X6B10X6A10X6Uint4Pack16ARM = 1000609002,
    R12X4UintPack16ARM = 1000609003,
    R12X4G12X4Uint2Pack16ARM = 1000609004,
    R12X4G12X4B12X4A12X4Uint4Pack16ARM = 1000609005,
    R14X2UintPack16ARM = 1000609006,
    R14X2G14X2Uint2Pack16ARM = 1000609007,
    R14X2G14X2B14X2A14X2Uint4Pack16ARM = 1000609008,
    R14X2UnormPack16ARM = 1000609009,
    R14X2G14X2Unorm2Pack16ARM = 1000609010,
    R14X2G14X2B14X2A14X2Unorm4Pack16ARM = 1000609011,
    G14X2B14X2R14X22Plane420Unorm3Pack16ARM = 1000609012,
    G14X2B14X2R14X22Plane422Unorm3Pack16ARM = 1000609013,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkStructureType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum StructureType {
    #[default]
    ApplicationInfo = 0,
    InstanceCreateInfo = 1,
    DeviceQueueCreateInfo = 2,
    DeviceCreateInfo = 3,
    SubmitInfo = 4,
    MemoryAllocateInfo = 5,
    MappedMemoryRange = 6,
    BindSparseInfo = 7,
    FenceCreateInfo = 8,
    SemaphoreCreateInfo = 9,
    EventCreateInfo = 10,
    QueryPoolCreateInfo = 11,
    BufferCreateInfo = 12,
    BufferViewCreateInfo = 13,
    ImageCreateInfo = 14,
    ImageViewCreateInfo = 15,
    ShaderModuleCreateInfo = 16,
    PipelineCacheCreateInfo = 17,
    PipelineShaderStageCreateInfo = 18,
    PipelineVertexInputStateCreateInfo = 19,
    PipelineInputAssemblyStateCreateInfo = 20,
    PipelineTessellationStateCreateInfo = 21,
    PipelineViewportStateCreateInfo = 22,
    PipelineRasterizationStateCreateInfo = 23,
    PipelineMultisampleStateCreateInfo = 24,
    PipelineDepthStencilStateCreateInfo = 25,
    PipelineColorBlendStateCreateInfo = 26,
    PipelineDynamicStateCreateInfo = 27,
    GraphicsPipelineCreateInfo = 28,
    ComputePipelineCreateInfo = 29,
    PipelineLayoutCreateInfo = 30,
    SamplerCreateInfo = 31,
    DescriptorSetLayoutCreateInfo = 32,
    DescriptorPoolCreateInfo = 33,
    DescriptorSetAllocateInfo = 34,
    WriteDescriptorSet = 35,
    CopyDescriptorSet = 36,
    FramebufferCreateInfo = 37,
    RenderPassCreateInfo = 38,
    CommandPoolCreateInfo = 39,
    CommandBufferAllocateInfo = 40,
    CommandBufferInheritanceInfo = 41,
    CommandBufferBeginInfo = 42,
    RenderPassBeginInfo = 43,
    BufferMemoryBarrier = 44,
    ImageMemoryBarrier = 45,
    MemoryBarrier = 46,
    /// Reserved for internal use by the loader, layers, and ICDs
    LoaderInstanceCreateInfo = 47,
    /// Reserved for internal use by the loader, layers, and ICDs
    LoaderDeviceCreateInfo = 48,
    BindBufferMemoryInfo = 1000157000,
    BindImageMemoryInfo = 1000157001,
    MemoryDedicatedRequirements = 1000127000,
    MemoryDedicatedAllocateInfo = 1000127001,
    MemoryAllocateFlagsInfo = 1000060000,
    DeviceGroupCommandBufferBeginInfo = 1000060004,
    DeviceGroupSubmitInfo = 1000060005,
    DeviceGroupBindSparseInfo = 1000060006,
    BindBufferMemoryDeviceGroupInfo = 1000060013,
    BindImageMemoryDeviceGroupInfo = 1000060014,
    PhysicalDeviceGroupProperties = 1000070000,
    DeviceGroupDeviceCreateInfo = 1000070001,
    BufferMemoryRequirementsInfo2 = 1000146000,
    ImageMemoryRequirementsInfo2 = 1000146001,
    ImageSparseMemoryRequirementsInfo2 = 1000146002,
    MemoryRequirements2 = 1000146003,
    SparseImageMemoryRequirements2 = 1000146004,
    PhysicalDeviceFeatures2 = 1000059000,
    PhysicalDeviceProperties2 = 1000059001,
    FormatProperties2 = 1000059002,
    ImageFormatProperties2 = 1000059003,
    PhysicalDeviceImageFormatInfo2 = 1000059004,
    QueueFamilyProperties2 = 1000059005,
    PhysicalDeviceMemoryProperties2 = 1000059006,
    SparseImageFormatProperties2 = 1000059007,
    PhysicalDeviceSparseImageFormatInfo2 = 1000059008,
    ImageViewUsageCreateInfo = 1000117002,
    ProtectedSubmitInfo = 1000145000,
    PhysicalDeviceProtectedMemoryFeatures = 1000145001,
    PhysicalDeviceProtectedMemoryProperties = 1000145002,
    DeviceQueueInfo2 = 1000145003,
    PhysicalDeviceExternalImageFormatInfo = 1000071000,
    ExternalImageFormatProperties = 1000071001,
    PhysicalDeviceExternalBufferInfo = 1000071002,
    ExternalBufferProperties = 1000071003,
    PhysicalDeviceIDProperties = 1000071004,
    ExternalMemoryBufferCreateInfo = 1000072000,
    ExternalMemoryImageCreateInfo = 1000072001,
    ExportMemoryAllocateInfo = 1000072002,
    PhysicalDeviceExternalFenceInfo = 1000112000,
    ExternalFenceProperties = 1000112001,
    ExportFenceCreateInfo = 1000113000,
    ExportSemaphoreCreateInfo = 1000077000,
    PhysicalDeviceExternalSemaphoreInfo = 1000076000,
    ExternalSemaphoreProperties = 1000076001,
    PhysicalDeviceSubgroupProperties = 1000094000,
    PhysicalDevice16BitStorageFeatures = 1000083000,
    PhysicalDeviceVariablePointersFeatures = 1000120000,
    DescriptorUpdateTemplateCreateInfo = 1000085000,
    PhysicalDeviceMaintenance3Properties = 1000168000,
    DescriptorSetLayoutSupport = 1000168001,
    SamplerYcbcrConversionCreateInfo = 1000156000,
    SamplerYcbcrConversionInfo = 1000156001,
    BindImagePlaneMemoryInfo = 1000156002,
    ImagePlaneMemoryRequirementsInfo = 1000156003,
    PhysicalDeviceSamplerYcbcrConversionFeatures = 1000156004,
    SamplerYcbcrConversionImageFormatProperties = 1000156005,
    DeviceGroupRenderPassBeginInfo = 1000060003,
    PhysicalDevicePointClippingProperties = 1000117000,
    RenderPassInputAttachmentAspectCreateInfo = 1000117001,
    PipelineTessellationDomainOriginStateCreateInfo = 1000117003,
    RenderPassMultiviewCreateInfo = 1000053000,
    PhysicalDeviceMultiviewFeatures = 1000053001,
    PhysicalDeviceMultiviewProperties = 1000053002,
    PhysicalDeviceShaderDrawParametersFeatures = 1000063000,
    PhysicalDeviceVulkan11Features = 49,
    PhysicalDeviceVulkan11Properties = 50,
    PhysicalDeviceVulkan12Features = 51,
    PhysicalDeviceVulkan12Properties = 52,
    ImageFormatListCreateInfo = 1000147000,
    PhysicalDeviceDriverProperties = 1000196000,
    PhysicalDeviceVulkanMemoryModelFeatures = 1000211000,
    PhysicalDeviceHostQueryResetFeatures = 1000261000,
    PhysicalDeviceTimelineSemaphoreFeatures = 1000207000,
    PhysicalDeviceTimelineSemaphoreProperties = 1000207001,
    SemaphoreTypeCreateInfo = 1000207002,
    TimelineSemaphoreSubmitInfo = 1000207003,
    SemaphoreWaitInfo = 1000207004,
    SemaphoreSignalInfo = 1000207005,
    PhysicalDeviceBufferDeviceAddressFeatures = 1000257000,
    BufferDeviceAddressInfo = 1000244001,
    BufferOpaqueCaptureAddressCreateInfo = 1000257002,
    MemoryOpaqueCaptureAddressAllocateInfo = 1000257003,
    DeviceMemoryOpaqueCaptureAddressInfo = 1000257004,
    PhysicalDevice8BitStorageFeatures = 1000177000,
    PhysicalDeviceShaderAtomicInt64Features = 1000180000,
    PhysicalDeviceShaderFloat16Int8Features = 1000082000,
    PhysicalDeviceFloatControlsProperties = 1000197000,
    DescriptorSetLayoutBindingFlagsCreateInfo = 1000161000,
    PhysicalDeviceDescriptorIndexingFeatures = 1000161001,
    PhysicalDeviceDescriptorIndexingProperties = 1000161002,
    DescriptorSetVariableDescriptorCountAllocateInfo = 1000161003,
    DescriptorSetVariableDescriptorCountLayoutSupport = 1000161004,
    PhysicalDeviceScalarBlockLayoutFeatures = 1000221000,
    PhysicalDeviceSamplerFilterMinmaxProperties = 1000130000,
    SamplerReductionModeCreateInfo = 1000130001,
    PhysicalDeviceUniformBufferStandardLayoutFeatures = 1000253000,
    PhysicalDeviceShaderSubgroupExtendedTypesFeatures = 1000175000,
    AttachmentDescription2 = 1000109000,
    AttachmentReference2 = 1000109001,
    SubpassDescription2 = 1000109002,
    SubpassDependency2 = 1000109003,
    RenderPassCreateInfo2 = 1000109004,
    SubpassBeginInfo = 1000109005,
    SubpassEndInfo = 1000109006,
    PhysicalDeviceDepthStencilResolveProperties = 1000199000,
    SubpassDescriptionDepthStencilResolve = 1000199001,
    ImageStencilUsageCreateInfo = 1000246000,
    PhysicalDeviceImagelessFramebufferFeatures = 1000108000,
    FramebufferAttachmentsCreateInfo = 1000108001,
    FramebufferAttachmentImageInfo = 1000108002,
    RenderPassAttachmentBeginInfo = 1000108003,
    PhysicalDeviceSeparateDepthStencilLayoutsFeatures = 1000241000,
    AttachmentReferenceStencilLayout = 1000241001,
    AttachmentDescriptionStencilLayout = 1000241002,
    PhysicalDeviceVulkan13Features = 53,
    PhysicalDeviceVulkan13Properties = 54,
    PhysicalDeviceToolProperties = 1000245000,
    PhysicalDevicePrivateDataFeatures = 1000295000,
    DevicePrivateDataCreateInfo = 1000295001,
    PrivateDataSlotCreateInfo = 1000295002,
    MemoryBarrier2 = 1000314000,
    BufferMemoryBarrier2 = 1000314001,
    ImageMemoryBarrier2 = 1000314002,
    DependencyInfo = 1000314003,
    SubmitInfo2 = 1000314004,
    SemaphoreSubmitInfo = 1000314005,
    CommandBufferSubmitInfo = 1000314006,
    PhysicalDeviceSynchronization2Features = 1000314007,
    CopyBufferInfo2 = 1000337000,
    CopyImageInfo2 = 1000337001,
    CopyBufferToImageInfo2 = 1000337002,
    CopyImageToBufferInfo2 = 1000337003,
    BufferCopy2 = 1000337006,
    ImageCopy2 = 1000337007,
    BufferImageCopy2 = 1000337009,
    PhysicalDeviceTextureCompressionASTCHDRFeatures = 1000066000,
    FormatProperties3 = 1000360000,
    PhysicalDeviceMaintenance4Features = 1000413000,
    PhysicalDeviceMaintenance4Properties = 1000413001,
    DeviceBufferMemoryRequirements = 1000413002,
    DeviceImageMemoryRequirements = 1000413003,
    PipelineCreationFeedbackCreateInfo = 1000192000,
    PhysicalDeviceShaderTerminateInvocationFeatures = 1000215000,
    PhysicalDeviceShaderDemoteToHelperInvocationFeatures = 1000276000,
    PhysicalDevicePipelineCreationCacheControlFeatures = 1000297000,
    PhysicalDeviceZeroInitializeWorkgroupMemoryFeatures = 1000325000,
    PhysicalDeviceImageRobustnessFeatures = 1000335000,
    PhysicalDeviceSubgroupSizeControlProperties = 1000225000,
    PipelineShaderStageRequiredSubgroupSizeCreateInfo = 1000225001,
    PhysicalDeviceSubgroupSizeControlFeatures = 1000225002,
    PhysicalDeviceInlineUniformBlockFeatures = 1000138000,
    PhysicalDeviceInlineUniformBlockProperties = 1000138001,
    WriteDescriptorSetInlineUniformBlock = 1000138002,
    DescriptorPoolInlineUniformBlockCreateInfo = 1000138003,
    PhysicalDeviceShaderIntegerDotProductFeatures = 1000280000,
    PhysicalDeviceShaderIntegerDotProductProperties = 1000280001,
    PhysicalDeviceTexelBufferAlignmentProperties = 1000281001,
    BlitImageInfo2 = 1000337004,
    ResolveImageInfo2 = 1000337005,
    ImageBlit2 = 1000337008,
    ImageResolve2 = 1000337010,
    RenderingInfo = 1000044000,
    RenderingAttachmentInfo = 1000044001,
    PipelineRenderingCreateInfo = 1000044002,
    PhysicalDeviceDynamicRenderingFeatures = 1000044003,
    CommandBufferInheritanceRenderingInfo = 1000044004,
    PhysicalDeviceVulkan14Features = 55,
    PhysicalDeviceVulkan14Properties = 56,
    DeviceQueueGlobalPriorityCreateInfo = 1000174000,
    PhysicalDeviceGlobalPriorityQueryFeatures = 1000388000,
    QueueFamilyGlobalPriorityProperties = 1000388001,
    PhysicalDeviceIndexTypeUint8Features = 1000265000,
    MemoryMapInfo = 1000271000,
    MemoryUnmapInfo = 1000271001,
    PhysicalDeviceMaintenance5Features = 1000470000,
    PhysicalDeviceMaintenance5Properties = 1000470001,
    DeviceImageSubresourceInfo = 1000470004,
    SubresourceLayout2 = 1000338002,
    ImageSubresource2 = 1000338003,
    BufferUsageFlags2CreateInfo = 1000470006,
    PhysicalDeviceMaintenance6Features = 1000545000,
    PhysicalDeviceMaintenance6Properties = 1000545001,
    BindMemoryStatus = 1000545002,
    PhysicalDeviceHostImageCopyFeatures = 1000270000,
    PhysicalDeviceHostImageCopyProperties = 1000270001,
    MemoryToImageCopy = 1000270002,
    ImageToMemoryCopy = 1000270003,
    CopyImageToMemoryInfo = 1000270004,
    CopyMemoryToImageInfo = 1000270005,
    HostImageLayoutTransitionInfo = 1000270006,
    CopyImageToImageInfo = 1000270007,
    SubresourceHostMemcpySize = 1000270008,
    HostImageCopyDevicePerformanceQuery = 1000270009,
    PhysicalDeviceShaderSubgroupRotateFeatures = 1000416000,
    PhysicalDeviceShaderFloatControls2Features = 1000528000,
    PhysicalDeviceShaderExpectAssumeFeatures = 1000544000,
    PipelineCreateFlags2CreateInfo = 1000470005,
    PhysicalDevicePushDescriptorProperties = 1000080000,
    BindDescriptorSetsInfo = 1000545003,
    PushConstantsInfo = 1000545004,
    PushDescriptorSetInfo = 1000545005,
    PushDescriptorSetWithTemplateInfo = 1000545006,
    PhysicalDevicePipelineProtectedAccessFeatures = 1000466000,
    PipelineRobustnessCreateInfo = 1000068000,
    PhysicalDevicePipelineRobustnessFeatures = 1000068001,
    PhysicalDevicePipelineRobustnessProperties = 1000068002,
    PhysicalDeviceLineRasterizationFeatures = 1000259000,
    PipelineRasterizationLineStateCreateInfo = 1000259001,
    PhysicalDeviceLineRasterizationProperties = 1000259002,
    PhysicalDeviceVertexAttributeDivisorProperties = 1000525000,
    PipelineVertexInputDivisorStateCreateInfo = 1000190001,
    PhysicalDeviceVertexAttributeDivisorFeatures = 1000190002,
    RenderingAreaInfo = 1000470003,
    PhysicalDeviceDynamicRenderingLocalReadFeatures = 1000232000,
    RenderingAttachmentLocationInfo = 1000232001,
    RenderingInputAttachmentIndexInfo = 1000232002,
    PhysicalDeviceVulkanSC10Features = 1000298000,
    PhysicalDeviceVulkanSC10Properties = 1000298001,
    DeviceObjectReservationCreateInfo = 1000298002,
    CommandPoolMemoryReservationCreateInfo = 1000298003,
    CommandPoolMemoryConsumption = 1000298004,
    PipelinePoolSize = 1000298005,
    FaultData = 1000298007,
    FaultCallbackInfo = 1000298008,
    PipelineOfflineCreateInfo = 1000298010,
    SwapchainCreateInfoKHR = 1000001000,
    PresentInfoKHR = 1000001001,
    DeviceGroupPresentCapabilitiesKHR = 1000060007,
    ImageSwapchainCreateInfoKHR = 1000060008,
    BindImageMemorySwapchainInfoKHR = 1000060009,
    AcquireNextImageInfoKHR = 1000060010,
    DeviceGroupPresentInfoKHR = 1000060011,
    DeviceGroupSwapchainCreateInfoKHR = 1000060012,
    DisplayModeCreateInfoKHR = 1000002000,
    DisplaySurfaceCreateInfoKHR = 1000002001,
    DisplayPresentInfoKHR = 1000003000,
    XlibSurfaceCreateInfoKHR = 1000004000,
    XcbSurfaceCreateInfoKHR = 1000005000,
    WaylandSurfaceCreateInfoKHR = 1000006000,
    AndroidSurfaceCreateInfoKHR = 1000008000,
    Win32SurfaceCreateInfoKHR = 1000009000,
    DebugReportCallbackCreateInfoEXT = 1000011000,
    PipelineRasterizationStateRasterizationOrderAMD = 1000018000,
    DebugMarkerObjectNameInfoEXT = 1000022000,
    DebugMarkerObjectTagInfoEXT = 1000022001,
    DebugMarkerMarkerInfoEXT = 1000022002,
    VideoProfileInfoKHR = 1000023000,
    VideoCapabilitiesKHR = 1000023001,
    VideoPictureResourceInfoKHR = 1000023002,
    VideoSessionMemoryRequirementsKHR = 1000023003,
    BindVideoSessionMemoryInfoKHR = 1000023004,
    VideoSessionCreateInfoKHR = 1000023005,
    VideoSessionParametersCreateInfoKHR = 1000023006,
    VideoSessionParametersUpdateInfoKHR = 1000023007,
    VideoBeginCodingInfoKHR = 1000023008,
    VideoEndCodingInfoKHR = 1000023009,
    VideoCodingControlInfoKHR = 1000023010,
    VideoReferenceSlotInfoKHR = 1000023011,
    QueueFamilyVideoPropertiesKHR = 1000023012,
    VideoProfileListInfoKHR = 1000023013,
    PhysicalDeviceVideoFormatInfoKHR = 1000023014,
    VideoFormatPropertiesKHR = 1000023015,
    QueueFamilyQueryResultStatusPropertiesKHR = 1000023016,
    VideoDecodeInfoKHR = 1000024000,
    VideoDecodeCapabilitiesKHR = 1000024001,
    VideoDecodeUsageInfoKHR = 1000024002,
    DedicatedAllocationImageCreateInfoNV = 1000026000,
    DedicatedAllocationBufferCreateInfoNV = 1000026001,
    DedicatedAllocationMemoryAllocateInfoNV = 1000026002,
    PhysicalDeviceTransformFeedbackFeaturesEXT = 1000028000,
    PhysicalDeviceTransformFeedbackPropertiesEXT = 1000028001,
    PipelineRasterizationStateStreamCreateInfoEXT = 1000028002,
    CuModuleCreateInfoNVX = 1000029000,
    CuFunctionCreateInfoNVX = 1000029001,
    CuLaunchInfoNVX = 1000029002,
    CuModuleTexturingModeCreateInfoNVX = 1000029004,
    ImageViewHandleInfoNVX = 1000030000,
    ImageViewAddressPropertiesNVX = 1000030001,
    VideoEncodeH264CapabilitiesKHR = 1000038000,
    VideoEncodeH264SessionParametersCreateInfoKHR = 1000038001,
    VideoEncodeH264SessionParametersAddInfoKHR = 1000038002,
    VideoEncodeH264PictureInfoKHR = 1000038003,
    VideoEncodeH264DpbSlotInfoKHR = 1000038004,
    VideoEncodeH264NaluSliceInfoKHR = 1000038005,
    VideoEncodeH264GopRemainingFrameInfoKHR = 1000038006,
    VideoEncodeH264ProfileInfoKHR = 1000038007,
    VideoEncodeH264RateControlInfoKHR = 1000038008,
    VideoEncodeH264RateControlLayerInfoKHR = 1000038009,
    VideoEncodeH264SessionCreateInfoKHR = 1000038010,
    VideoEncodeH264QualityLevelPropertiesKHR = 1000038011,
    VideoEncodeH264SessionParametersGetInfoKHR = 1000038012,
    VideoEncodeH264SessionParametersFeedbackInfoKHR = 1000038013,
    VideoEncodeH265CapabilitiesKHR = 1000039000,
    VideoEncodeH265SessionParametersCreateInfoKHR = 1000039001,
    VideoEncodeH265SessionParametersAddInfoKHR = 1000039002,
    VideoEncodeH265PictureInfoKHR = 1000039003,
    VideoEncodeH265DpbSlotInfoKHR = 1000039004,
    VideoEncodeH265NaluSliceSegmentInfoKHR = 1000039005,
    VideoEncodeH265GopRemainingFrameInfoKHR = 1000039006,
    VideoEncodeH265ProfileInfoKHR = 1000039007,
    VideoEncodeH265RateControlInfoKHR = 1000039009,
    VideoEncodeH265RateControlLayerInfoKHR = 1000039010,
    VideoEncodeH265SessionCreateInfoKHR = 1000039011,
    VideoEncodeH265QualityLevelPropertiesKHR = 1000039012,
    VideoEncodeH265SessionParametersGetInfoKHR = 1000039013,
    VideoEncodeH265SessionParametersFeedbackInfoKHR = 1000039014,
    VideoDecodeH264CapabilitiesKHR = 1000040000,
    VideoDecodeH264PictureInfoKHR = 1000040001,
    VideoDecodeH264ProfileInfoKHR = 1000040003,
    VideoDecodeH264SessionParametersCreateInfoKHR = 1000040004,
    VideoDecodeH264SessionParametersAddInfoKHR = 1000040005,
    VideoDecodeH264DpbSlotInfoKHR = 1000040006,
    TextureLODGatherFormatPropertiesAMD = 1000041000,
    StreamDescriptorSurfaceCreateInfoGGP = 1000049000,
    PhysicalDeviceCornerSampledImageFeaturesNV = 1000050000,
    PrivateVendorInfoPlaceholderOffset0NV = 1000051000,
    ExternalMemoryImageCreateInfoNV = 1000056000,
    ExportMemoryAllocateInfoNV = 1000056001,
    ImportMemoryWin32HandleInfoNV = 1000057000,
    ExportMemoryWin32HandleInfoNV = 1000057001,
    Win32KeyedMutexAcquireReleaseInfoNV = 1000058000,
    ValidationFlagsEXT = 1000061000,
    ViSurfaceCreateInfoNN = 1000062000,
    ImageViewASTCDecodeModeEXT = 1000067000,
    PhysicalDeviceASTCDecodeFeaturesEXT = 1000067001,
    ImportMemoryWin32HandleInfoKHR = 1000073000,
    ExportMemoryWin32HandleInfoKHR = 1000073001,
    MemoryWin32HandlePropertiesKHR = 1000073002,
    MemoryGetWin32HandleInfoKHR = 1000073003,
    ImportMemoryFdInfoKHR = 1000074000,
    MemoryFdPropertiesKHR = 1000074001,
    MemoryGetFdInfoKHR = 1000074002,
    Win32KeyedMutexAcquireReleaseInfoKHR = 1000075000,
    ImportSemaphoreWin32HandleInfoKHR = 1000078000,
    ExportSemaphoreWin32HandleInfoKHR = 1000078001,
    D3D12FenceSubmitInfoKHR = 1000078002,
    SemaphoreGetWin32HandleInfoKHR = 1000078003,
    ImportSemaphoreFdInfoKHR = 1000079000,
    SemaphoreGetFdInfoKHR = 1000079001,
    CommandBufferInheritanceConditionalRenderingInfoEXT = 1000081000,
    PhysicalDeviceConditionalRenderingFeaturesEXT = 1000081001,
    ConditionalRenderingBeginInfoEXT = 1000081002,
    PresentRegionsKHR = 1000084000,
    PipelineViewportWScalingStateCreateInfoNV = 1000087000,
    SurfaceCapabilities2EXT = 1000090000,
    DisplayPowerInfoEXT = 1000091000,
    DeviceEventInfoEXT = 1000091001,
    DisplayEventInfoEXT = 1000091002,
    SwapchainCounterCreateInfoEXT = 1000091003,
    PresentTimesInfoGOOGLE = 1000092000,
    PhysicalDeviceMultiviewPerViewAttributesPropertiesNVX = 1000097000,
    MultiviewPerViewAttributesInfoNVX = 1000044009,
    PipelineViewportSwizzleStateCreateInfoNV = 1000098000,
    PhysicalDeviceDiscardRectanglePropertiesEXT = 1000099000,
    PipelineDiscardRectangleStateCreateInfoEXT = 1000099001,
    PhysicalDeviceConservativeRasterizationPropertiesEXT = 1000101000,
    PipelineRasterizationConservativeStateCreateInfoEXT = 1000101001,
    PhysicalDeviceDepthClipEnableFeaturesEXT = 1000102000,
    PipelineRasterizationDepthClipStateCreateInfoEXT = 1000102001,
    HdrMetadataEXT = 1000105000,
    PhysicalDeviceRelaxedLineRasterizationFeaturesIMG = 1000110000,
    SharedPresentSurfaceCapabilitiesKHR = 1000111000,
    ImportFenceWin32HandleInfoKHR = 1000114000,
    ExportFenceWin32HandleInfoKHR = 1000114001,
    FenceGetWin32HandleInfoKHR = 1000114002,
    ImportFenceFdInfoKHR = 1000115000,
    FenceGetFdInfoKHR = 1000115001,
    PhysicalDevicePerformanceQueryFeaturesKHR = 1000116000,
    PhysicalDevicePerformanceQueryPropertiesKHR = 1000116001,
    QueryPoolPerformanceCreateInfoKHR = 1000116002,
    PerformanceQuerySubmitInfoKHR = 1000116003,
    AcquireProfilingLockInfoKHR = 1000116004,
    PerformanceCounterKHR = 1000116005,
    PerformanceCounterDescriptionKHR = 1000116006,
    PerformanceQueryReservationInfoKHR = 1000116007,
    PhysicalDeviceSurfaceInfo2KHR = 1000119000,
    SurfaceCapabilities2KHR = 1000119001,
    SurfaceFormat2KHR = 1000119002,
    DisplayProperties2KHR = 1000121000,
    DisplayPlaneProperties2KHR = 1000121001,
    DisplayModeProperties2KHR = 1000121002,
    DisplayPlaneInfo2KHR = 1000121003,
    DisplayPlaneCapabilities2KHR = 1000121004,
    IOSSurfaceCreateInfoMVK = 1000122000,
    MacOSSurfaceCreateInfoMVK = 1000123000,
    DebugUtilsObjectNameInfoEXT = 1000128000,
    DebugUtilsObjectTagInfoEXT = 1000128001,
    DebugUtilsLabelEXT = 1000128002,
    DebugUtilsMessengerCallbackDataEXT = 1000128003,
    DebugUtilsMessengerCreateInfoEXT = 1000128004,
    AndroidHardwareBufferUsageANDROID = 1000129000,
    AndroidHardwareBufferPropertiesANDROID = 1000129001,
    AndroidHardwareBufferFormatPropertiesANDROID = 1000129002,
    ImportAndroidHardwareBufferInfoANDROID = 1000129003,
    MemoryGetAndroidHardwareBufferInfoANDROID = 1000129004,
    ExternalFormatANDROID = 1000129005,
    AndroidHardwareBufferFormatProperties2ANDROID = 1000129006,
    PhysicalDeviceShaderEnqueueFeaturesAMDX = 1000134000,
    PhysicalDeviceShaderEnqueuePropertiesAMDX = 1000134001,
    ExecutionGraphPipelineScratchSizeAMDX = 1000134002,
    ExecutionGraphPipelineCreateInfoAMDX = 1000134003,
    PipelineShaderStageNodeCreateInfoAMDX = 1000134004,
    TexelBufferDescriptorInfoEXT = 1000135000,
    ImageDescriptorInfoEXT = 1000135001,
    ResourceDescriptorInfoEXT = 1000135002,
    BindHeapInfoEXT = 1000135003,
    PushDataInfoEXT = 1000135004,
    DescriptorSetAndBindingMappingEXT = 1000135005,
    ShaderDescriptorSetAndBindingMappingInfoEXT = 1000135006,
    OpaqueCaptureDataCreateInfoEXT = 1000135007,
    PhysicalDeviceDescriptorHeapPropertiesEXT = 1000135008,
    PhysicalDeviceDescriptorHeapFeaturesEXT = 1000135009,
    CommandBufferInheritanceDescriptorHeapInfoEXT = 1000135010,
    SamplerCustomBorderColorIndexCreateInfoEXT = 1000135011,
    IndirectCommandsLayoutPushDataTokenNV = 1000135012,
    SubsampledImageFormatPropertiesEXT = 1000135013,
    PhysicalDeviceDescriptorHeapTensorPropertiesARM = 1000135014,
    AttachmentSampleCountInfoAMD = 1000044008,
    PhysicalDeviceShaderBfloat16FeaturesKHR = 1000141000,
    SampleLocationsInfoEXT = 1000143000,
    RenderPassSampleLocationsBeginInfoEXT = 1000143001,
    PipelineSampleLocationsStateCreateInfoEXT = 1000143002,
    PhysicalDeviceSampleLocationsPropertiesEXT = 1000143003,
    MultisamplePropertiesEXT = 1000143004,
    PhysicalDeviceBlendOperationAdvancedFeaturesEXT = 1000148000,
    PhysicalDeviceBlendOperationAdvancedPropertiesEXT = 1000148001,
    PipelineColorBlendAdvancedStateCreateInfoEXT = 1000148002,
    PipelineCoverageToColorStateCreateInfoNV = 1000149000,
    WriteDescriptorSetAccelerationStructureKHR = 1000150007,
    AccelerationStructureBuildGeometryInfoKHR = 1000150000,
    AccelerationStructureDeviceAddressInfoKHR = 1000150002,
    AccelerationStructureGeometryAabbsDataKHR = 1000150003,
    AccelerationStructureGeometryInstancesDataKHR = 1000150004,
    AccelerationStructureGeometryTrianglesDataKHR = 1000150005,
    AccelerationStructureGeometryKHR = 1000150006,
    AccelerationStructureVersionInfoKHR = 1000150009,
    CopyAccelerationStructureInfoKHR = 1000150010,
    CopyAccelerationStructureToMemoryInfoKHR = 1000150011,
    CopyMemoryToAccelerationStructureInfoKHR = 1000150012,
    PhysicalDeviceAccelerationStructureFeaturesKHR = 1000150013,
    PhysicalDeviceAccelerationStructurePropertiesKHR = 1000150014,
    AccelerationStructureCreateInfoKHR = 1000150017,
    AccelerationStructureBuildSizesInfoKHR = 1000150020,
    PhysicalDeviceRayTracingPipelineFeaturesKHR = 1000347000,
    PhysicalDeviceRayTracingPipelinePropertiesKHR = 1000347001,
    RayTracingPipelineCreateInfoKHR = 1000150015,
    RayTracingShaderGroupCreateInfoKHR = 1000150016,
    RayTracingPipelineInterfaceCreateInfoKHR = 1000150018,
    PhysicalDeviceRayQueryFeaturesKHR = 1000348013,
    PipelineCoverageModulationStateCreateInfoNV = 1000152000,
    PhysicalDeviceShaderSMBuiltinsFeaturesNV = 1000154000,
    PhysicalDeviceShaderSMBuiltinsPropertiesNV = 1000154001,
    DrmFormatModifierPropertiesListEXT = 1000158000,
    PhysicalDeviceImageDrmFormatModifierInfoEXT = 1000158002,
    ImageDrmFormatModifierListCreateInfoEXT = 1000158003,
    ImageDrmFormatModifierExplicitCreateInfoEXT = 1000158004,
    ImageDrmFormatModifierPropertiesEXT = 1000158005,
    DrmFormatModifierPropertiesList2EXT = 1000158006,
    ValidationCacheCreateInfoEXT = 1000160000,
    ShaderModuleValidationCacheCreateInfoEXT = 1000160001,
    PhysicalDevicePortabilitySubsetFeaturesKHR = 1000163000,
    PhysicalDevicePortabilitySubsetPropertiesKHR = 1000163001,
    PipelineViewportShadingRateImageStateCreateInfoNV = 1000164000,
    PhysicalDeviceShadingRateImageFeaturesNV = 1000164001,
    PhysicalDeviceShadingRateImagePropertiesNV = 1000164002,
    PipelineViewportCoarseSampleOrderStateCreateInfoNV = 1000164005,
    RayTracingPipelineCreateInfoNV = 1000165000,
    AccelerationStructureCreateInfoNV = 1000165001,
    GeometryNV = 1000165003,
    GeometryTrianglesNV = 1000165004,
    GeometryAABBNV = 1000165005,
    BindAccelerationStructureMemoryInfoNV = 1000165006,
    WriteDescriptorSetAccelerationStructureNV = 1000165007,
    AccelerationStructureMemoryRequirementsInfoNV = 1000165008,
    PhysicalDeviceRayTracingPropertiesNV = 1000165009,
    RayTracingShaderGroupCreateInfoNV = 1000165011,
    AccelerationStructureInfoNV = 1000165012,
    PhysicalDeviceRepresentativeFragmentTestFeaturesNV = 1000166000,
    PipelineRepresentativeFragmentTestStateCreateInfoNV = 1000166001,
    PhysicalDeviceImageViewImageFormatInfoEXT = 1000170000,
    FilterCubicImageViewImageFormatPropertiesEXT = 1000170001,
    ImportMemoryHostPointerInfoEXT = 1000178000,
    MemoryHostPointerPropertiesEXT = 1000178001,
    PhysicalDeviceExternalMemoryHostPropertiesEXT = 1000178002,
    PhysicalDeviceShaderClockFeaturesKHR = 1000181000,
    PipelineCompilerControlCreateInfoAMD = 1000183000,
    PhysicalDeviceShaderCorePropertiesAMD = 1000185000,
    VideoDecodeH265CapabilitiesKHR = 1000187000,
    VideoDecodeH265SessionParametersCreateInfoKHR = 1000187001,
    VideoDecodeH265SessionParametersAddInfoKHR = 1000187002,
    VideoDecodeH265ProfileInfoKHR = 1000187003,
    VideoDecodeH265PictureInfoKHR = 1000187004,
    VideoDecodeH265DpbSlotInfoKHR = 1000187005,
    DeviceMemoryOverallocationCreateInfoAMD = 1000189000,
    PhysicalDeviceVertexAttributeDivisorPropertiesEXT = 1000190000,
    PresentFrameTokenGGP = 1000191000,
    PhysicalDeviceMeshShaderFeaturesNV = 1000202000,
    PhysicalDeviceMeshShaderPropertiesNV = 1000202001,
    PhysicalDeviceShaderImageFootprintFeaturesNV = 1000204000,
    PipelineViewportExclusiveScissorStateCreateInfoNV = 1000205000,
    PhysicalDeviceExclusiveScissorFeaturesNV = 1000205002,
    CheckpointDataNV = 1000206000,
    QueueFamilyCheckpointPropertiesNV = 1000206001,
    QueueFamilyCheckpointProperties2NV = 1000314008,
    CheckpointData2NV = 1000314009,
    PhysicalDevicePresentTimingFeaturesEXT = 1000208000,
    SwapchainTimingPropertiesEXT = 1000208001,
    SwapchainTimeDomainPropertiesEXT = 1000208002,
    PresentTimingsInfoEXT = 1000208003,
    PresentTimingInfoEXT = 1000208004,
    PastPresentationTimingInfoEXT = 1000208005,
    PastPresentationTimingPropertiesEXT = 1000208006,
    PastPresentationTimingEXT = 1000208007,
    PresentTimingSurfaceCapabilitiesEXT = 1000208008,
    SwapchainCalibratedTimestampInfoEXT = 1000208009,
    PhysicalDeviceShaderIntegerFunctions2FeaturesINTEL = 1000209000,
    QueryPoolPerformanceQueryCreateInfoINTEL = 1000210000,
    InitializePerformanceApiInfoINTEL = 1000210001,
    PerformanceMarkerInfoINTEL = 1000210002,
    PerformanceStreamMarkerInfoINTEL = 1000210003,
    PerformanceOverrideInfoINTEL = 1000210004,
    PerformanceConfigurationAcquireInfoINTEL = 1000210005,
    PhysicalDevicePCIBusInfoPropertiesEXT = 1000212000,
    DisplayNativeHdrSurfaceCapabilitiesAMD = 1000213000,
    SwapchainDisplayNativeHdrCreateInfoAMD = 1000213001,
    ImagePipeSurfaceCreateInfoFUCHSIA = 1000214000,
    MetalSurfaceCreateInfoEXT = 1000217000,
    PhysicalDeviceFragmentDensityMapFeaturesEXT = 1000218000,
    PhysicalDeviceFragmentDensityMapPropertiesEXT = 1000218001,
    RenderPassFragmentDensityMapCreateInfoEXT = 1000218002,
    RenderingFragmentDensityMapAttachmentInfoEXT = 1000044007,
    FragmentShadingRateAttachmentInfoKHR = 1000226000,
    PipelineFragmentShadingRateStateCreateInfoKHR = 1000226001,
    PhysicalDeviceFragmentShadingRatePropertiesKHR = 1000226002,
    PhysicalDeviceFragmentShadingRateFeaturesKHR = 1000226003,
    PhysicalDeviceFragmentShadingRateKHR = 1000226004,
    RenderingFragmentShadingRateAttachmentInfoKHR = 1000044006,
    PhysicalDeviceShaderCoreProperties2AMD = 1000227000,
    PhysicalDeviceCoherentMemoryFeaturesAMD = 1000229000,
    PhysicalDeviceShaderImageAtomicInt64FeaturesEXT = 1000234000,
    PhysicalDeviceShaderQuadControlFeaturesKHR = 1000235000,
    PhysicalDeviceMemoryBudgetPropertiesEXT = 1000237000,
    PhysicalDeviceMemoryPriorityFeaturesEXT = 1000238000,
    MemoryPriorityAllocateInfoEXT = 1000238001,
    SurfaceProtectedCapabilitiesKHR = 1000239000,
    PhysicalDeviceDedicatedAllocationImageAliasingFeaturesNV = 1000240000,
    PhysicalDeviceBufferDeviceAddressFeaturesEXT = 1000244000,
    BufferDeviceAddressCreateInfoEXT = 1000244002,
    ValidationFeaturesEXT = 1000247000,
    PhysicalDevicePresentWaitFeaturesKHR = 1000248000,
    PhysicalDeviceCooperativeMatrixFeaturesNV = 1000249000,
    CooperativeMatrixPropertiesNV = 1000249001,
    PhysicalDeviceCooperativeMatrixPropertiesNV = 1000249002,
    PhysicalDeviceCoverageReductionModeFeaturesNV = 1000250000,
    PipelineCoverageReductionStateCreateInfoNV = 1000250001,
    FramebufferMixedSamplesCombinationNV = 1000250002,
    PhysicalDeviceFragmentShaderInterlockFeaturesEXT = 1000251000,
    PhysicalDeviceYcbcrImageArraysFeaturesEXT = 1000252000,
    PhysicalDeviceProvokingVertexFeaturesEXT = 1000254000,
    PipelineRasterizationProvokingVertexStateCreateInfoEXT = 1000254001,
    PhysicalDeviceProvokingVertexPropertiesEXT = 1000254002,
    SurfaceFullScreenExclusiveInfoEXT = 1000255000,
    SurfaceCapabilitiesFullScreenExclusiveEXT = 1000255002,
    SurfaceFullScreenExclusiveWin32InfoEXT = 1000255001,
    HeadlessSurfaceCreateInfoEXT = 1000256000,
    PhysicalDeviceShaderAtomicFloatFeaturesEXT = 1000260000,
    /// Not promoted to 1.3
    PhysicalDeviceExtendedDynamicStateFeaturesEXT = 1000267000,
    PhysicalDevicePipelineExecutablePropertiesFeaturesKHR = 1000269000,
    PipelineInfoKHR = 1000269001,
    PipelineExecutablePropertiesKHR = 1000269002,
    PipelineExecutableInfoKHR = 1000269003,
    PipelineExecutableStatisticKHR = 1000269004,
    PipelineExecutableInternalRepresentationKHR = 1000269005,
    PhysicalDeviceMapMemoryPlacedFeaturesEXT = 1000272000,
    PhysicalDeviceMapMemoryPlacedPropertiesEXT = 1000272001,
    MemoryMapPlacedInfoEXT = 1000272002,
    PhysicalDeviceShaderAtomicFloat2FeaturesEXT = 1000273000,
    PhysicalDeviceDeviceGeneratedCommandsPropertiesNV = 1000277000,
    GraphicsShaderGroupCreateInfoNV = 1000277001,
    GraphicsPipelineShaderGroupsCreateInfoNV = 1000277002,
    IndirectCommandsLayoutTokenNV = 1000277003,
    IndirectCommandsLayoutCreateInfoNV = 1000277004,
    GeneratedCommandsInfoNV = 1000277005,
    GeneratedCommandsMemoryRequirementsInfoNV = 1000277006,
    PhysicalDeviceDeviceGeneratedCommandsFeaturesNV = 1000277007,
    PhysicalDeviceInheritedViewportScissorFeaturesNV = 1000278000,
    CommandBufferInheritanceViewportScissorInfoNV = 1000278001,
    /// Not promoted to 1.3
    PhysicalDeviceTexelBufferAlignmentFeaturesEXT = 1000281000,
    CommandBufferInheritanceRenderPassTransformInfoQCOM = 1000282000,
    RenderPassTransformBeginInfoQCOM = 1000282001,
    PhysicalDeviceDepthBiasControlFeaturesEXT = 1000283000,
    DepthBiasInfoEXT = 1000283001,
    DepthBiasRepresentationInfoEXT = 1000283002,
    PhysicalDeviceDeviceMemoryReportFeaturesEXT = 1000284000,
    DeviceDeviceMemoryReportCreateInfoEXT = 1000284001,
    DeviceMemoryReportCallbackDataEXT = 1000284002,
    SamplerCustomBorderColorCreateInfoEXT = 1000287000,
    PhysicalDeviceCustomBorderColorPropertiesEXT = 1000287001,
    PhysicalDeviceCustomBorderColorFeaturesEXT = 1000287002,
    PhysicalDeviceTextureCompressionASTC3DFeaturesEXT = 1000288000,
    PipelineLibraryCreateInfoKHR = 1000290000,
    PhysicalDevicePresentBarrierFeaturesNV = 1000292000,
    SurfaceCapabilitiesPresentBarrierNV = 1000292001,
    SwapchainPresentBarrierCreateInfoNV = 1000292002,
    PresentIdKHR = 1000294000,
    PhysicalDevicePresentIdFeaturesKHR = 1000294001,
    VideoEncodeInfoKHR = 1000299000,
    VideoEncodeRateControlInfoKHR = 1000299001,
    VideoEncodeRateControlLayerInfoKHR = 1000299002,
    VideoEncodeCapabilitiesKHR = 1000299003,
    VideoEncodeUsageInfoKHR = 1000299004,
    QueryPoolVideoEncodeFeedbackCreateInfoKHR = 1000299005,
    PhysicalDeviceVideoEncodeQualityLevelInfoKHR = 1000299006,
    VideoEncodeQualityLevelPropertiesKHR = 1000299007,
    VideoEncodeQualityLevelInfoKHR = 1000299008,
    VideoEncodeSessionParametersGetInfoKHR = 1000299009,
    VideoEncodeSessionParametersFeedbackInfoKHR = 1000299010,
    PhysicalDeviceDiagnosticsConfigFeaturesNV = 1000300000,
    DeviceDiagnosticsConfigCreateInfoNV = 1000300001,
    CudaModuleCreateInfoNV = 1000307000,
    CudaFunctionCreateInfoNV = 1000307001,
    CudaLaunchInfoNV = 1000307002,
    PhysicalDeviceCudaKernelLaunchFeaturesNV = 1000307003,
    PhysicalDeviceCudaKernelLaunchPropertiesNV = 1000307004,
    RefreshObjectListKHR = 1000308000,
    PhysicalDeviceTileShadingFeaturesQCOM = 1000309000,
    PhysicalDeviceTileShadingPropertiesQCOM = 1000309001,
    RenderPassTileShadingCreateInfoQCOM = 1000309002,
    PerTileBeginInfoQCOM = 1000309003,
    PerTileEndInfoQCOM = 1000309004,
    DispatchTileInfoQCOM = 1000309005,
    QueryLowLatencySupportNV = 1000310000,
    ExportMetalObjectCreateInfoEXT = 1000311000,
    ExportMetalObjectsInfoEXT = 1000311001,
    ExportMetalDeviceInfoEXT = 1000311002,
    ExportMetalCommandQueueInfoEXT = 1000311003,
    ExportMetalBufferInfoEXT = 1000311004,
    ImportMetalBufferInfoEXT = 1000311005,
    ExportMetalTextureInfoEXT = 1000311006,
    ImportMetalTextureInfoEXT = 1000311007,
    ExportMetalIOSurfaceInfoEXT = 1000311008,
    ImportMetalIOSurfaceInfoEXT = 1000311009,
    ExportMetalSharedEventInfoEXT = 1000311010,
    ImportMetalSharedEventInfoEXT = 1000311011,
    PhysicalDeviceDescriptorBufferPropertiesEXT = 1000316000,
    PhysicalDeviceDescriptorBufferDensityMapPropertiesEXT = 1000316001,
    PhysicalDeviceDescriptorBufferFeaturesEXT = 1000316002,
    DescriptorAddressInfoEXT = 1000316003,
    DescriptorGetInfoEXT = 1000316004,
    BufferCaptureDescriptorDataInfoEXT = 1000316005,
    ImageCaptureDescriptorDataInfoEXT = 1000316006,
    ImageViewCaptureDescriptorDataInfoEXT = 1000316007,
    SamplerCaptureDescriptorDataInfoEXT = 1000316008,
    OpaqueCaptureDescriptorDataCreateInfoEXT = 1000316010,
    DescriptorBufferBindingInfoEXT = 1000316011,
    DescriptorBufferBindingPushDescriptorBufferHandleEXT = 1000316012,
    AccelerationStructureCaptureDescriptorDataInfoEXT = 1000316009,
    PhysicalDeviceGraphicsPipelineLibraryFeaturesEXT = 1000320000,
    PhysicalDeviceGraphicsPipelineLibraryPropertiesEXT = 1000320001,
    GraphicsPipelineLibraryCreateInfoEXT = 1000320002,
    PhysicalDeviceShaderEarlyAndLateFragmentTestsFeaturesAMD = 1000321000,
    PhysicalDeviceFragmentShaderBarycentricFeaturesKHR = 1000203000,
    PhysicalDeviceFragmentShaderBarycentricPropertiesKHR = 1000322000,
    PhysicalDeviceShaderSubgroupUniformControlFlowFeaturesKHR = 1000323000,
    PhysicalDeviceFragmentShadingRateEnumsPropertiesNV = 1000326000,
    PhysicalDeviceFragmentShadingRateEnumsFeaturesNV = 1000326001,
    PipelineFragmentShadingRateEnumStateCreateInfoNV = 1000326002,
    AccelerationStructureGeometryMotionTrianglesDataNV = 1000327000,
    PhysicalDeviceRayTracingMotionBlurFeaturesNV = 1000327001,
    AccelerationStructureMotionInfoNV = 1000327002,
    PhysicalDeviceMeshShaderFeaturesEXT = 1000328000,
    PhysicalDeviceMeshShaderPropertiesEXT = 1000328001,
    PhysicalDeviceYcbcr2Plane444FormatsFeaturesEXT = 1000330000,
    PhysicalDeviceFragmentDensityMap2FeaturesEXT = 1000332000,
    PhysicalDeviceFragmentDensityMap2PropertiesEXT = 1000332001,
    CopyCommandTransformInfoQCOM = 1000333000,
    PhysicalDeviceWorkgroupMemoryExplicitLayoutFeaturesKHR = 1000336000,
    PhysicalDeviceImageCompressionControlFeaturesEXT = 1000338000,
    ImageCompressionControlEXT = 1000338001,
    ImageCompressionPropertiesEXT = 1000338004,
    PhysicalDeviceAttachmentFeedbackLoopLayoutFeaturesEXT = 1000339000,
    PhysicalDevice4444FormatsFeaturesEXT = 1000340000,
    PhysicalDeviceFaultFeaturesEXT = 1000341000,
    DeviceFaultCountsEXT = 1000341001,
    DeviceFaultInfoEXT = 1000341002,
    PhysicalDeviceRGBA10X6FormatsFeaturesEXT = 1000344000,
    DirectFBSurfaceCreateInfoEXT = 1000346000,
    PhysicalDeviceVertexInputDynamicStateFeaturesEXT = 1000352000,
    VertexInputBindingDescription2EXT = 1000352001,
    VertexInputAttributeDescription2EXT = 1000352002,
    PhysicalDeviceDrmPropertiesEXT = 1000353000,
    PhysicalDeviceAddressBindingReportFeaturesEXT = 1000354000,
    DeviceAddressBindingCallbackDataEXT = 1000354001,
    PhysicalDeviceDepthClipControlFeaturesEXT = 1000355000,
    PipelineViewportDepthClipControlCreateInfoEXT = 1000355001,
    PhysicalDevicePrimitiveTopologyListRestartFeaturesEXT = 1000356000,
    ImportMemoryZirconHandleInfoFUCHSIA = 1000364000,
    MemoryZirconHandlePropertiesFUCHSIA = 1000364001,
    MemoryGetZirconHandleInfoFUCHSIA = 1000364002,
    ImportSemaphoreZirconHandleInfoFUCHSIA = 1000365000,
    SemaphoreGetZirconHandleInfoFUCHSIA = 1000365001,
    BufferCollectionCreateInfoFUCHSIA = 1000366000,
    ImportMemoryBufferCollectionFUCHSIA = 1000366001,
    BufferCollectionImageCreateInfoFUCHSIA = 1000366002,
    BufferCollectionPropertiesFUCHSIA = 1000366003,
    BufferConstraintsInfoFUCHSIA = 1000366004,
    BufferCollectionBufferCreateInfoFUCHSIA = 1000366005,
    ImageConstraintsInfoFUCHSIA = 1000366006,
    ImageFormatConstraintsInfoFUCHSIA = 1000366007,
    SysmemColorSpaceFUCHSIA = 1000366008,
    BufferCollectionConstraintsInfoFUCHSIA = 1000366009,
    SubpassShadingPipelineCreateInfoHUAWEI = 1000369000,
    PhysicalDeviceSubpassShadingFeaturesHUAWEI = 1000369001,
    PhysicalDeviceSubpassShadingPropertiesHUAWEI = 1000369002,
    PhysicalDeviceInvocationMaskFeaturesHUAWEI = 1000370000,
    MemoryGetRemoteAddressInfoNV = 1000371000,
    PhysicalDeviceExternalMemoryRDMAFeaturesNV = 1000371001,
    PipelinePropertiesIdentifierEXT = 1000372000,
    PhysicalDevicePipelinePropertiesFeaturesEXT = 1000372001,
    ImportFenceSciSyncInfoNV = 1000373000,
    ExportFenceSciSyncInfoNV = 1000373001,
    FenceGetSciSyncInfoNV = 1000373002,
    SciSyncAttributesInfoNV = 1000373003,
    ImportSemaphoreSciSyncInfoNV = 1000373004,
    ExportSemaphoreSciSyncInfoNV = 1000373005,
    SemaphoreGetSciSyncInfoNV = 1000373006,
    PhysicalDeviceExternalSciSyncFeaturesNV = 1000373007,
    ImportMemorySciBufInfoNV = 1000374000,
    ExportMemorySciBufInfoNV = 1000374001,
    MemoryGetSciBufInfoNV = 1000374002,
    MemorySciBufPropertiesNV = 1000374003,
    PhysicalDeviceExternalMemorySciBufFeaturesNV = 1000374004,
    PhysicalDeviceFrameBoundaryFeaturesEXT = 1000375000,
    FrameBoundaryEXT = 1000375001,
    PhysicalDeviceMultisampledRenderToSingleSampledFeaturesEXT = 1000376000,
    SubpassResolvePerformanceQueryEXT = 1000376001,
    MultisampledRenderToSingleSampledInfoEXT = 1000376002,
    /// Not promoted to 1.3
    PhysicalDeviceExtendedDynamicState2FeaturesEXT = 1000377000,
    ScreenSurfaceCreateInfoQNX = 1000378000,
    PhysicalDeviceColorWriteEnableFeaturesEXT = 1000381000,
    PipelineColorWriteCreateInfoEXT = 1000381001,
    PhysicalDevicePrimitivesGeneratedQueryFeaturesEXT = 1000382000,
    PhysicalDeviceRayTracingMaintenance1FeaturesKHR = 1000386000,
    PhysicalDeviceShaderUntypedPointersFeaturesKHR = 1000387000,
    PhysicalDeviceVideoEncodeRgbConversionFeaturesVALVE = 1000390000,
    VideoEncodeRgbConversionCapabilitiesVALVE = 1000390001,
    VideoEncodeProfileRgbConversionInfoVALVE = 1000390002,
    VideoEncodeSessionRgbConversionCreateInfoVALVE = 1000390003,
    PhysicalDeviceImageViewMinLodFeaturesEXT = 1000391000,
    ImageViewMinLodCreateInfoEXT = 1000391001,
    PhysicalDeviceMultiDrawFeaturesEXT = 1000392000,
    PhysicalDeviceMultiDrawPropertiesEXT = 1000392001,
    PhysicalDeviceImage2DViewOf3DFeaturesEXT = 1000393000,
    PhysicalDeviceShaderTileImageFeaturesEXT = 1000395000,
    PhysicalDeviceShaderTileImagePropertiesEXT = 1000395001,
    MicromapBuildInfoEXT = 1000396000,
    MicromapVersionInfoEXT = 1000396001,
    CopyMicromapInfoEXT = 1000396002,
    CopyMicromapToMemoryInfoEXT = 1000396003,
    CopyMemoryToMicromapInfoEXT = 1000396004,
    PhysicalDeviceOpacityMicromapFeaturesEXT = 1000396005,
    PhysicalDeviceOpacityMicromapPropertiesEXT = 1000396006,
    MicromapCreateInfoEXT = 1000396007,
    MicromapBuildSizesInfoEXT = 1000396008,
    AccelerationStructureTrianglesOpacityMicromapEXT = 1000396009,
    PhysicalDeviceDisplacementMicromapFeaturesNV = 1000397000,
    PhysicalDeviceDisplacementMicromapPropertiesNV = 1000397001,
    AccelerationStructureTrianglesDisplacementMicromapNV = 1000397002,
    PhysicalDeviceClusterCullingShaderFeaturesHUAWEI = 1000404000,
    PhysicalDeviceClusterCullingShaderPropertiesHUAWEI = 1000404001,
    PhysicalDeviceClusterCullingShaderVrsFeaturesHUAWEI = 1000404002,
    PhysicalDeviceBorderColorSwizzleFeaturesEXT = 1000411000,
    SamplerBorderColorComponentMappingCreateInfoEXT = 1000411001,
    PhysicalDevicePageableDeviceLocalMemoryFeaturesEXT = 1000412000,
    PhysicalDeviceShaderCorePropertiesARM = 1000415000,
    DeviceQueueShaderCoreControlCreateInfoARM = 1000417000,
    PhysicalDeviceSchedulingControlsFeaturesARM = 1000417001,
    PhysicalDeviceSchedulingControlsPropertiesARM = 1000417002,
    PhysicalDeviceImageSlicedViewOf3DFeaturesEXT = 1000418000,
    ImageViewSlicedCreateInfoEXT = 1000418001,
    PhysicalDeviceDescriptorSetHostMappingFeaturesVALVE = 1000420000,
    DescriptorSetBindingReferenceVALVE = 1000420001,
    DescriptorSetLayoutHostMappingInfoVALVE = 1000420002,
    PhysicalDeviceNonSeamlessCubeMapFeaturesEXT = 1000422000,
    PhysicalDeviceRenderPassStripedFeaturesARM = 1000424000,
    PhysicalDeviceRenderPassStripedPropertiesARM = 1000424001,
    RenderPassStripeBeginInfoARM = 1000424002,
    RenderPassStripeInfoARM = 1000424003,
    RenderPassStripeSubmitInfoARM = 1000424004,
    PhysicalDeviceCopyMemoryIndirectFeaturesNV = 1000426000,
    PhysicalDeviceDeviceGeneratedCommandsComputeFeaturesNV = 1000428000,
    ComputePipelineIndirectBufferInfoNV = 1000428001,
    PipelineIndirectDeviceAddressInfoNV = 1000428002,
    PhysicalDeviceRayTracingLinearSweptSpheresFeaturesNV = 1000429008,
    AccelerationStructureGeometryLinearSweptSpheresDataNV = 1000429009,
    AccelerationStructureGeometrySpheresDataNV = 1000429010,
    PhysicalDeviceLinearColorAttachmentFeaturesNV = 1000430000,
    PhysicalDeviceShaderMaximalReconvergenceFeaturesKHR = 1000434000,
    ApplicationParametersEXT = 1000435000,
    PhysicalDeviceImageCompressionControlSwapchainFeaturesEXT = 1000437000,
    PhysicalDeviceImageProcessingFeaturesQCOM = 1000440000,
    PhysicalDeviceImageProcessingPropertiesQCOM = 1000440001,
    ImageViewSampleWeightCreateInfoQCOM = 1000440002,
    PhysicalDeviceNestedCommandBufferFeaturesEXT = 1000451000,
    PhysicalDeviceNestedCommandBufferPropertiesEXT = 1000451001,
    NativeBufferUsageOHOS = 1000452000,
    NativeBufferPropertiesOHOS = 1000452001,
    NativeBufferFormatPropertiesOHOS = 1000452002,
    ImportNativeBufferInfoOHOS = 1000452003,
    MemoryGetNativeBufferInfoOHOS = 1000452004,
    ExternalFormatOHOS = 1000452005,
    ExternalMemoryAcquireUnmodifiedEXT = 1000453000,
    PhysicalDeviceExtendedDynamicState3FeaturesEXT = 1000455000,
    PhysicalDeviceExtendedDynamicState3PropertiesEXT = 1000455001,
    PhysicalDeviceSubpassMergeFeedbackFeaturesEXT = 1000458000,
    RenderPassCreationControlEXT = 1000458001,
    RenderPassCreationFeedbackCreateInfoEXT = 1000458002,
    RenderPassSubpassFeedbackCreateInfoEXT = 1000458003,
    DirectDriverLoadingInfoLUNARG = 1000459000,
    DirectDriverLoadingListLUNARG = 1000459001,
    TensorCreateInfoARM = 1000460000,
    TensorViewCreateInfoARM = 1000460001,
    BindTensorMemoryInfoARM = 1000460002,
    WriteDescriptorSetTensorARM = 1000460003,
    PhysicalDeviceTensorPropertiesARM = 1000460004,
    TensorFormatPropertiesARM = 1000460005,
    TensorDescriptionARM = 1000460006,
    TensorMemoryRequirementsInfoARM = 1000460007,
    TensorMemoryBarrierARM = 1000460008,
    PhysicalDeviceTensorFeaturesARM = 1000460009,
    DeviceTensorMemoryRequirementsARM = 1000460010,
    CopyTensorInfoARM = 1000460011,
    TensorCopyARM = 1000460012,
    TensorDependencyInfoARM = 1000460013,
    MemoryDedicatedAllocateInfoTensorARM = 1000460014,
    PhysicalDeviceExternalTensorInfoARM = 1000460015,
    ExternalTensorPropertiesARM = 1000460016,
    ExternalMemoryTensorCreateInfoARM = 1000460017,
    PhysicalDeviceDescriptorBufferTensorFeaturesARM = 1000460018,
    PhysicalDeviceDescriptorBufferTensorPropertiesARM = 1000460019,
    DescriptorGetTensorInfoARM = 1000460020,
    TensorCaptureDescriptorDataInfoARM = 1000460021,
    TensorViewCaptureDescriptorDataInfoARM = 1000460022,
    FrameBoundaryTensorsARM = 1000460023,
    PhysicalDeviceShaderModuleIdentifierFeaturesEXT = 1000462000,
    PhysicalDeviceShaderModuleIdentifierPropertiesEXT = 1000462001,
    PipelineShaderStageModuleIdentifierCreateInfoEXT = 1000462002,
    ShaderModuleIdentifierEXT = 1000462003,
    PhysicalDeviceRasterizationOrderAttachmentAccessFeaturesEXT = 1000342000,
    PhysicalDeviceOpticalFlowFeaturesNV = 1000464000,
    PhysicalDeviceOpticalFlowPropertiesNV = 1000464001,
    OpticalFlowImageFormatInfoNV = 1000464002,
    OpticalFlowImageFormatPropertiesNV = 1000464003,
    OpticalFlowSessionCreateInfoNV = 1000464004,
    OpticalFlowExecuteInfoNV = 1000464005,
    OpticalFlowSessionCreatePrivateDataInfoNV = 1000464010,
    PhysicalDeviceLegacyDitheringFeaturesEXT = 1000465000,
    PhysicalDeviceExternalFormatResolveFeaturesANDROID = 1000468000,
    PhysicalDeviceExternalFormatResolvePropertiesANDROID = 1000468001,
    AndroidHardwareBufferFormatResolvePropertiesANDROID = 1000468002,
    PhysicalDeviceAntiLagFeaturesAMD = 1000476000,
    AntiLagDataAMD = 1000476001,
    AntiLagPresentationInfoAMD = 1000476002,
    PhysicalDeviceDenseGeometryFormatFeaturesAMDX = 1000478000,
    AccelerationStructureDenseGeometryFormatTrianglesDataAMDX = 1000478001,
    SurfaceCapabilitiesPresentId2KHR = 1000479000,
    PresentId2KHR = 1000479001,
    PhysicalDevicePresentId2FeaturesKHR = 1000479002,
    SurfaceCapabilitiesPresentWait2KHR = 1000480000,
    PhysicalDevicePresentWait2FeaturesKHR = 1000480001,
    PresentWait2InfoKHR = 1000480002,
    PhysicalDeviceRayTracingPositionFetchFeaturesKHR = 1000481000,
    PhysicalDeviceShaderObjectFeaturesEXT = 1000482000,
    PhysicalDeviceShaderObjectPropertiesEXT = 1000482001,
    ShaderCreateInfoEXT = 1000482002,
    PhysicalDevicePipelineBinaryFeaturesKHR = 1000483000,
    PipelineBinaryCreateInfoKHR = 1000483001,
    PipelineBinaryInfoKHR = 1000483002,
    PipelineBinaryKeyKHR = 1000483003,
    PhysicalDevicePipelineBinaryPropertiesKHR = 1000483004,
    ReleaseCapturedPipelineDataInfoKHR = 1000483005,
    PipelineBinaryDataInfoKHR = 1000483006,
    PipelineCreateInfoKHR = 1000483007,
    DevicePipelineBinaryInternalCacheControlKHR = 1000483008,
    PipelineBinaryHandlesInfoKHR = 1000483009,
    PhysicalDeviceTilePropertiesFeaturesQCOM = 1000484000,
    TilePropertiesQCOM = 1000484001,
    PhysicalDeviceAmigoProfilingFeaturesSEC = 1000485000,
    AmigoProfilingSubmitInfoSEC = 1000485001,
    SurfacePresentModeKHR = 1000274000,
    SurfacePresentScalingCapabilitiesKHR = 1000274001,
    SurfacePresentModeCompatibilityKHR = 1000274002,
    PhysicalDeviceSwapchainMaintenance1FeaturesKHR = 1000275000,
    SwapchainPresentFenceInfoKHR = 1000275001,
    SwapchainPresentModesCreateInfoKHR = 1000275002,
    SwapchainPresentModeInfoKHR = 1000275003,
    SwapchainPresentScalingCreateInfoKHR = 1000275004,
    ReleaseSwapchainImagesInfoKHR = 1000275005,
    PhysicalDeviceMultiviewPerViewViewportsFeaturesQCOM = 1000488000,
    SemaphoreSciSyncPoolCreateInfoNV = 1000489000,
    SemaphoreSciSyncCreateInfoNV = 1000489001,
    PhysicalDeviceExternalSciSync2FeaturesNV = 1000489002,
    DeviceSemaphoreSciSyncPoolReservationCreateInfoNV = 1000489003,
    PhysicalDeviceRayTracingInvocationReorderFeaturesNV = 1000490000,
    PhysicalDeviceRayTracingInvocationReorderPropertiesNV = 1000490001,
    PhysicalDeviceCooperativeVectorFeaturesNV = 1000491000,
    PhysicalDeviceCooperativeVectorPropertiesNV = 1000491001,
    CooperativeVectorPropertiesNV = 1000491002,
    ConvertCooperativeVectorMatrixInfoNV = 1000491004,
    PhysicalDeviceExtendedSparseAddressSpaceFeaturesNV = 1000492000,
    PhysicalDeviceExtendedSparseAddressSpacePropertiesNV = 1000492001,
    PhysicalDeviceMutableDescriptorTypeFeaturesEXT = 1000351000,
    MutableDescriptorTypeCreateInfoEXT = 1000351002,
    PhysicalDeviceLegacyVertexAttributesFeaturesEXT = 1000495000,
    PhysicalDeviceLegacyVertexAttributesPropertiesEXT = 1000495001,
    LayerSettingsCreateInfoEXT = 1000496000,
    PhysicalDeviceShaderCoreBuiltinsFeaturesARM = 1000497000,
    PhysicalDeviceShaderCoreBuiltinsPropertiesARM = 1000497001,
    PhysicalDevicePipelineLibraryGroupHandlesFeaturesEXT = 1000498000,
    PhysicalDeviceDynamicRenderingUnusedAttachmentsFeaturesEXT = 1000499000,
    PhysicalDeviceInternallySynchronizedQueuesFeaturesKHR = 1000504000,
    LatencySleepModeInfoNV = 1000505000,
    LatencySleepInfoNV = 1000505001,
    SetLatencyMarkerInfoNV = 1000505002,
    GetLatencyMarkerInfoNV = 1000505003,
    LatencyTimingsFrameReportNV = 1000505004,
    LatencySubmissionPresentIdNV = 1000505005,
    OutOfBandQueueTypeInfoNV = 1000505006,
    SwapchainLatencyCreateInfoNV = 1000505007,
    LatencySurfaceCapabilitiesNV = 1000505008,
    PhysicalDeviceCooperativeMatrixFeaturesKHR = 1000506000,
    CooperativeMatrixPropertiesKHR = 1000506001,
    PhysicalDeviceCooperativeMatrixPropertiesKHR = 1000506002,
    DataGraphPipelineCreateInfoARM = 1000507000,
    DataGraphPipelineSessionCreateInfoARM = 1000507001,
    DataGraphPipelineResourceInfoARM = 1000507002,
    DataGraphPipelineConstantARM = 1000507003,
    DataGraphPipelineSessionMemoryRequirementsInfoARM = 1000507004,
    BindDataGraphPipelineSessionMemoryInfoARM = 1000507005,
    PhysicalDeviceDataGraphFeaturesARM = 1000507006,
    DataGraphPipelineShaderModuleCreateInfoARM = 1000507007,
    DataGraphPipelinePropertyQueryResultARM = 1000507008,
    DataGraphPipelineInfoARM = 1000507009,
    DataGraphPipelineCompilerControlCreateInfoARM = 1000507010,
    DataGraphPipelineSessionBindPointRequirementsInfoARM = 1000507011,
    DataGraphPipelineSessionBindPointRequirementARM = 1000507012,
    DataGraphPipelineIdentifierCreateInfoARM = 1000507013,
    DataGraphPipelineDispatchInfoARM = 1000507014,
    DataGraphProcessingEngineCreateInfoARM = 1000507016,
    QueueFamilyDataGraphProcessingEnginePropertiesARM = 1000507017,
    QueueFamilyDataGraphPropertiesARM = 1000507018,
    PhysicalDeviceQueueFamilyDataGraphProcessingEngineInfoARM = 1000507019,
    DataGraphPipelineConstantTensorSemiStructuredSparsityInfoARM = 1000507015,
    PhysicalDeviceMultiviewPerViewRenderAreasFeaturesQCOM = 1000510000,
    MultiviewPerViewRenderAreasRenderPassBeginInfoQCOM = 1000510001,
    PhysicalDeviceComputeShaderDerivativesFeaturesKHR = 1000201000,
    PhysicalDeviceComputeShaderDerivativesPropertiesKHR = 1000511000,
    VideoDecodeAV1CapabilitiesKHR = 1000512000,
    VideoDecodeAV1PictureInfoKHR = 1000512001,
    VideoDecodeAV1ProfileInfoKHR = 1000512003,
    VideoDecodeAV1SessionParametersCreateInfoKHR = 1000512004,
    VideoDecodeAV1DpbSlotInfoKHR = 1000512005,
    VideoEncodeAV1CapabilitiesKHR = 1000513000,
    VideoEncodeAV1SessionParametersCreateInfoKHR = 1000513001,
    VideoEncodeAV1PictureInfoKHR = 1000513002,
    VideoEncodeAV1DpbSlotInfoKHR = 1000513003,
    PhysicalDeviceVideoEncodeAV1FeaturesKHR = 1000513004,
    VideoEncodeAV1ProfileInfoKHR = 1000513005,
    VideoEncodeAV1RateControlInfoKHR = 1000513006,
    VideoEncodeAV1RateControlLayerInfoKHR = 1000513007,
    VideoEncodeAV1QualityLevelPropertiesKHR = 1000513008,
    VideoEncodeAV1SessionCreateInfoKHR = 1000513009,
    VideoEncodeAV1GopRemainingFrameInfoKHR = 1000513010,
    PhysicalDeviceVideoDecodeVP9FeaturesKHR = 1000514000,
    VideoDecodeVP9CapabilitiesKHR = 1000514001,
    VideoDecodeVP9PictureInfoKHR = 1000514002,
    VideoDecodeVP9ProfileInfoKHR = 1000514003,
    PhysicalDeviceVideoMaintenance1FeaturesKHR = 1000515000,
    VideoInlineQueryInfoKHR = 1000515001,
    PhysicalDevicePerStageDescriptorSetFeaturesNV = 1000516000,
    PhysicalDeviceImageProcessing2FeaturesQCOM = 1000518000,
    PhysicalDeviceImageProcessing2PropertiesQCOM = 1000518001,
    SamplerBlockMatchWindowCreateInfoQCOM = 1000518002,
    SamplerCubicWeightsCreateInfoQCOM = 1000519000,
    PhysicalDeviceCubicWeightsFeaturesQCOM = 1000519001,
    BlitImageCubicWeightsInfoQCOM = 1000519002,
    PhysicalDeviceYcbcrDegammaFeaturesQCOM = 1000520000,
    SamplerYcbcrConversionYcbcrDegammaCreateInfoQCOM = 1000520001,
    PhysicalDeviceCubicClampFeaturesQCOM = 1000521000,
    PhysicalDeviceAttachmentFeedbackLoopDynamicStateFeaturesEXT = 1000524000,
    PhysicalDeviceUnifiedImageLayoutsFeaturesKHR = 1000527000,
    AttachmentFeedbackLoopInfoEXT = 1000527001,
    ScreenBufferPropertiesQNX = 1000529000,
    ScreenBufferFormatPropertiesQNX = 1000529001,
    ImportScreenBufferInfoQNX = 1000529002,
    ExternalFormatQNX = 1000529003,
    PhysicalDeviceExternalMemoryScreenBufferFeaturesQNX = 1000529004,
    PhysicalDeviceLayeredDriverPropertiesMSFT = 1000530000,
    CalibratedTimestampInfoKHR = 1000184000,
    SetDescriptorBufferOffsetsInfoEXT = 1000545007,
    BindDescriptorBufferEmbeddedSamplersInfoEXT = 1000545008,
    PhysicalDeviceDescriptorPoolOverallocationFeaturesNV = 1000546000,
    PhysicalDeviceTileMemoryHeapFeaturesQCOM = 1000547000,
    PhysicalDeviceTileMemoryHeapPropertiesQCOM = 1000547001,
    TileMemoryRequirementsQCOM = 1000547002,
    TileMemoryBindInfoQCOM = 1000547003,
    TileMemorySizeInfoQCOM = 1000547004,
    PhysicalDeviceCopyMemoryIndirectFeaturesKHR = 1000549000,
    PhysicalDeviceCopyMemoryIndirectPropertiesKHR = 1000426001,
    CopyMemoryIndirectInfoKHR = 1000549002,
    CopyMemoryToImageIndirectInfoKHR = 1000549003,
    PhysicalDeviceMemoryDecompressionFeaturesEXT = 1000427000,
    PhysicalDeviceMemoryDecompressionPropertiesEXT = 1000427001,
    DecompressMemoryInfoEXT = 1000550002,
    DisplaySurfaceStereoCreateInfoNV = 1000551000,
    DisplayModeStereoPropertiesNV = 1000551001,
    VideoEncodeIntraRefreshCapabilitiesKHR = 1000552000,
    VideoEncodeSessionIntraRefreshCreateInfoKHR = 1000552001,
    VideoEncodeIntraRefreshInfoKHR = 1000552002,
    VideoReferenceIntraRefreshInfoKHR = 1000552003,
    PhysicalDeviceVideoEncodeIntraRefreshFeaturesKHR = 1000552004,
    VideoEncodeQuantizationMapCapabilitiesKHR = 1000553000,
    VideoFormatQuantizationMapPropertiesKHR = 1000553001,
    VideoEncodeQuantizationMapInfoKHR = 1000553002,
    VideoEncodeQuantizationMapSessionParametersCreateInfoKHR = 1000553005,
    PhysicalDeviceVideoEncodeQuantizationMapFeaturesKHR = 1000553009,
    VideoEncodeH264QuantizationMapCapabilitiesKHR = 1000553003,
    VideoEncodeH265QuantizationMapCapabilitiesKHR = 1000553004,
    VideoFormatH265QuantizationMapPropertiesKHR = 1000553006,
    VideoEncodeAV1QuantizationMapCapabilitiesKHR = 1000553007,
    VideoFormatAV1QuantizationMapPropertiesKHR = 1000553008,
    PhysicalDeviceRawAccessChainsFeaturesNV = 1000555000,
    ExternalComputeQueueDeviceCreateInfoNV = 1000556000,
    ExternalComputeQueueCreateInfoNV = 1000556001,
    ExternalComputeQueueDataParamsNV = 1000556002,
    PhysicalDeviceExternalComputeQueuePropertiesNV = 1000556003,
    PhysicalDeviceShaderRelaxedExtendedInstructionFeaturesKHR = 1000558000,
    PhysicalDeviceCommandBufferInheritanceFeaturesNV = 1000559000,
    PhysicalDeviceMaintenance7FeaturesKHR = 1000562000,
    PhysicalDeviceMaintenance7PropertiesKHR = 1000562001,
    PhysicalDeviceLayeredApiPropertiesListKHR = 1000562002,
    PhysicalDeviceLayeredApiPropertiesKHR = 1000562003,
    PhysicalDeviceLayeredApiVulkanPropertiesKHR = 1000562004,
    PhysicalDeviceShaderAtomicFloat16VectorFeaturesNV = 1000563000,
    PhysicalDeviceShaderReplicatedCompositesFeaturesEXT = 1000564000,
    PhysicalDeviceShaderFloat8FeaturesEXT = 1000567000,
    PhysicalDeviceRayTracingValidationFeaturesNV = 1000568000,
    PhysicalDeviceClusterAccelerationStructureFeaturesNV = 1000569000,
    PhysicalDeviceClusterAccelerationStructurePropertiesNV = 1000569001,
    ClusterAccelerationStructureClustersBottomLevelInputNV = 1000569002,
    ClusterAccelerationStructureTriangleClusterInputNV = 1000569003,
    ClusterAccelerationStructureMoveObjectsInputNV = 1000569004,
    ClusterAccelerationStructureInputInfoNV = 1000569005,
    ClusterAccelerationStructureCommandsInfoNV = 1000569006,
    RayTracingPipelineClusterAccelerationStructureCreateInfoNV = 1000569007,
    PhysicalDevicePartitionedAccelerationStructureFeaturesNV = 1000570000,
    PhysicalDevicePartitionedAccelerationStructurePropertiesNV = 1000570001,
    WriteDescriptorSetPartitionedAccelerationStructureNV = 1000570002,
    PartitionedAccelerationStructureInstancesInputNV = 1000570003,
    BuildPartitionedAccelerationStructureInfoNV = 1000570004,
    PartitionedAccelerationStructureFlagsNV = 1000570005,
    PhysicalDeviceDeviceGeneratedCommandsFeaturesEXT = 1000572000,
    PhysicalDeviceDeviceGeneratedCommandsPropertiesEXT = 1000572001,
    GeneratedCommandsMemoryRequirementsInfoEXT = 1000572002,
    IndirectExecutionSetCreateInfoEXT = 1000572003,
    GeneratedCommandsInfoEXT = 1000572004,
    IndirectCommandsLayoutCreateInfoEXT = 1000572006,
    IndirectCommandsLayoutTokenEXT = 1000572007,
    WriteIndirectExecutionSetPipelineEXT = 1000572008,
    WriteIndirectExecutionSetShaderEXT = 1000572009,
    IndirectExecutionSetPipelineInfoEXT = 1000572010,
    IndirectExecutionSetShaderInfoEXT = 1000572011,
    IndirectExecutionSetShaderLayoutInfoEXT = 1000572012,
    GeneratedCommandsPipelineInfoEXT = 1000572013,
    GeneratedCommandsShaderInfoEXT = 1000572014,
    PhysicalDeviceMaintenance8FeaturesKHR = 1000574000,
    MemoryBarrierAccessFlags3KHR = 1000574002,
    PhysicalDeviceImageAlignmentControlFeaturesMESA = 1000575000,
    PhysicalDeviceImageAlignmentControlPropertiesMESA = 1000575001,
    ImageAlignmentControlCreateInfoMESA = 1000575002,
    PhysicalDeviceShaderFmaFeaturesKHR = 1000579000,
    PushConstantBankInfoNV = 1000580000,
    PhysicalDevicePushConstantBankFeaturesNV = 1000580001,
    PhysicalDevicePushConstantBankPropertiesNV = 1000580002,
    PhysicalDeviceRayTracingInvocationReorderFeaturesEXT = 1000581000,
    PhysicalDeviceRayTracingInvocationReorderPropertiesEXT = 1000581001,
    PhysicalDeviceDepthClampControlFeaturesEXT = 1000582000,
    PipelineViewportDepthClampControlCreateInfoEXT = 1000582001,
    PhysicalDeviceMaintenance9FeaturesKHR = 1000584000,
    PhysicalDeviceMaintenance9PropertiesKHR = 1000584001,
    QueueFamilyOwnershipTransferPropertiesKHR = 1000584002,
    PhysicalDeviceVideoMaintenance2FeaturesKHR = 1000586000,
    VideoDecodeH264InlineSessionParametersInfoKHR = 1000586001,
    VideoDecodeH265InlineSessionParametersInfoKHR = 1000586002,
    VideoDecodeAV1InlineSessionParametersInfoKHR = 1000586003,
    SurfaceCreateInfoOHOS = 1000685000,
    PhysicalDeviceHdrVividFeaturesHUAWEI = 1000590000,
    HdrVividDynamicMetadataHUAWEI = 1000590001,
    PhysicalDeviceCooperativeMatrix2FeaturesNV = 1000593000,
    CooperativeMatrixFlexibleDimensionsPropertiesNV = 1000593001,
    PhysicalDeviceCooperativeMatrix2PropertiesNV = 1000593002,
    PhysicalDevicePipelineOpacityMicromapFeaturesARM = 1000596000,
    ImportMemoryMetalHandleInfoEXT = 1000602000,
    MemoryMetalHandlePropertiesEXT = 1000602001,
    MemoryGetMetalHandleInfoEXT = 1000602002,
    PhysicalDeviceDepthClampZeroOneFeaturesKHR = 1000421000,
    PhysicalDevicePerformanceCountersByRegionFeaturesARM = 1000605000,
    PhysicalDevicePerformanceCountersByRegionPropertiesARM = 1000605001,
    PerformanceCounterARM = 1000605002,
    PerformanceCounterDescriptionARM = 1000605003,
    RenderPassPerformanceCountersByRegionBeginInfoARM = 1000605004,
    PhysicalDeviceVertexAttributeRobustnessFeaturesEXT = 1000608000,
    PhysicalDeviceFormatPackFeaturesARM = 1000609000,
    PhysicalDeviceFragmentDensityMapLayeredFeaturesVALVE = 1000611000,
    PhysicalDeviceFragmentDensityMapLayeredPropertiesVALVE = 1000611001,
    PipelineFragmentDensityMapLayeredCreateInfoVALVE = 1000611002,
    PhysicalDeviceRobustness2FeaturesKHR = 1000286000,
    PhysicalDeviceRobustness2PropertiesKHR = 1000286001,
    SetPresentConfigNV = 1000613000,
    PhysicalDevicePresentMeteringFeaturesNV = 1000613001,
    PhysicalDeviceFragmentDensityMapOffsetFeaturesEXT = 1000425000,
    PhysicalDeviceFragmentDensityMapOffsetPropertiesEXT = 1000425001,
    RenderPassFragmentDensityMapOffsetEndInfoEXT = 1000425002,
    PhysicalDeviceZeroInitializeDeviceMemoryFeaturesEXT = 1000620000,
    PhysicalDevicePresentModeFifoLatestReadyFeaturesKHR = 1000361000,
    PhysicalDeviceShader64BitIndexingFeaturesEXT = 1000627000,
    PhysicalDeviceCustomResolveFeaturesEXT = 1000628000,
    BeginCustomResolveInfoEXT = 1000628001,
    CustomResolveCreateInfoEXT = 1000628002,
    PhysicalDeviceDataGraphModelFeaturesQCOM = 1000629000,
    DataGraphPipelineBuiltinModelCreateInfoQCOM = 1000629001,
    PhysicalDeviceMaintenance10FeaturesKHR = 1000630000,
    PhysicalDeviceMaintenance10PropertiesKHR = 1000630001,
    RenderingAttachmentFlagsInfoKHR = 1000630002,
    RenderingEndInfoKHR = 1000619003,
    ResolveImageModeInfoKHR = 1000630004,
    PhysicalDeviceShaderLongVectorFeaturesEXT = 1000635000,
    PhysicalDeviceShaderLongVectorPropertiesEXT = 1000635001,
    PhysicalDevicePipelineCacheIncrementalModeFeaturesSEC = 1000637000,
    PhysicalDeviceShaderUniformBufferUnsizedArrayFeaturesEXT = 1000642000,
    ComputeOccupancyPriorityParametersNV = 1000645000,
    PhysicalDeviceComputeOccupancyPriorityFeaturesNV = 1000645001,
    PhysicalDeviceShaderSubgroupPartitionedFeaturesEXT = 1000662000,
}

impl StructureType {
    #[allow(non_upper_case_globals)]
    pub const ApplicationInfo: StructureType = StructureType::PhysicalDeviceVariablePointerFeatures;
}
impl StructureType {
    #[allow(non_upper_case_globals)]
    pub const PhysicalDeviceVariablePointerFeatures: StructureType = StructureType::ApplicationInfo;
}
impl StructureType {
    #[allow(non_upper_case_globals)]
    pub const PhysicalDeviceShaderDrawParameterFeatures: StructureType =
        StructureType::ApplicationInfo;
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassContents.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SubpassContents {
    #[default]
    Inline = 0,
    SecondaryCommandBuffers = 1,
    InlineAndSecondaryCommandBuffersKHR = 1000451000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkResult.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum vkResult {
    /// Command completed successfully
    #[default]
    Success = 0,
    /// A fence or query has not yet completed
    NotReady = 1,
    /// A wait operation has not completed in the specified time
    Timeout = 2,
    /// An event is signaled
    EventSet = 3,
    /// An event is unsignaled
    EventReset = 4,
    /// A return array was too small for the result
    Incomplete = 5,
    /// A host memory allocation has failed
    ErrorOutOfHostMemory = -1,
    /// A device memory allocation has failed
    ErrorOutOfDeviceMemory = -2,
    /// Initialization of an object has failed
    ErrorInitializationFailed = -3,
    /// The logical device has been lost. See <<devsandqueues-lost-device>>
    ErrorDeviceLost = -4,
    /// Mapping of a memory object has failed
    ErrorMemoryMapFailed = -5,
    /// Layer specified does not exist
    ErrorLayerNotPresent = -6,
    /// Extension specified does not exist
    ErrorExtensionNotPresent = -7,
    /// Requested feature is not available on this device
    ErrorFeatureNotPresent = -8,
    /// Unable to find a Vulkan driver
    ErrorIncompatibleDriver = -9,
    /// Too many objects of the type have already been created
    ErrorTooManyObjects = -10,
    /// Requested format is not supported on this device
    ErrorFormatNotSupported = -11,
    /// A requested pool allocation has failed due to fragmentation of the pool's memory
    ErrorFragmentedPool = -12,
    /// An unknown error has occurred, due to an implementation or application bug
    ErrorUnknown = -13,
    ErrorValidationFailed = -1000011001,
    ErrorOutOfPoolMemory = -1000069000,
    ErrorInvalidExternalHandle = -1000072003,
    ErrorInvalidOpaqueCaptureAddress = -1000257000,
    ErrorFragmentation = -1000161000,
    PipelineCompileRequired = 1000297000,
    ErrorNotPermitted = -1000174001,
    ErrorInvalidPipelineCacheData = -1000298000,
    ErrorNoPipelineMatch = -1000298001,
    ErrorSurfaceLostKHR = -1000000000,
    ErrorNativeWindowInUseKHR = -1000000001,
    SuboptimalKHR = 1000001003,
    ErrorOutOfDateKHR = -1000001004,
    ErrorIncompatibleDisplayKHR = -1000003001,
    ErrorInvalidShaderNV = -1000012000,
    ErrorImageUsageNotSupportedKHR = -1000023000,
    ErrorVideoPictureLayoutNotSupportedKHR = -1000023001,
    ErrorVideoProfileOperationNotSupportedKHR = -1000023002,
    ErrorVideoProfileFormatNotSupportedKHR = -1000023003,
    ErrorVideoProfileCodecNotSupportedKHR = -1000023004,
    ErrorVideoStdVersionNotSupportedKHR = -1000023005,
    ErrorInvalidDrmFormatModifierPlaneLayoutEXT = -1000158000,
    ErrorPresentTimingQueueFullEXT = -1000208000,
    ErrorFullScreenExclusiveModeLostEXT = -1000255000,
    ThreadIdleKHR = 1000268000,
    ThreadDoneKHR = 1000268001,
    OperationDeferredKHR = 1000268002,
    OperationNotDeferredKHR = 1000268003,
    ErrorInvalidVideoStdParametersKHR = -1000299000,
    ErrorCompressionExhaustedEXT = -1000338000,
    IncompatibleShaderBinaryEXT = 1000482000,
    PipelineBinaryMissingKHR = 1000483000,
    ErrorNotEnoughSpaceKHR = -1000483000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDynamicState.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DynamicState {
    #[default]
    Viewport = 0,
    Scissor = 1,
    LineWidth = 2,
    DepthBias = 3,
    BlendConstants = 4,
    DepthBounds = 5,
    StencilCompareMask = 6,
    StencilWriteMask = 7,
    StencilReference = 8,
    CullMode = 1000267000,
    FrontFace = 1000267001,
    PrimitiveTopology = 1000267002,
    ViewportWithCount = 1000267003,
    ScissorWithCount = 1000267004,
    VertexInputBindingStride = 1000267005,
    DepthTestEnable = 1000267006,
    DepthWriteEnable = 1000267007,
    DepthCompareOp = 1000267008,
    DepthBoundsTestEnable = 1000267009,
    StencilTestEnable = 1000267010,
    StencilOp = 1000267011,
    RasterizerDiscardEnable = 1000377001,
    DepthBiasEnable = 1000377002,
    PrimitiveRestartEnable = 1000377004,
    LineStipple = 1000259000,
    ViewportWScalingNV = 1000087000,
    DiscardRectangleEXT = 1000099000,
    DiscardRectangleEnableEXT = 1000099001,
    DiscardRectangleModeEXT = 1000099002,
    SampleLocationsEXT = 1000143000,
    RayTracingPipelineStackSizeKHR = 1000347000,
    ViewportShadingRatePaletteNV = 1000164004,
    ViewportCoarseSampleOrderNV = 1000164006,
    ExclusiveScissorEnableNV = 1000205000,
    ExclusiveScissorNV = 1000205001,
    FragmentShadingRateKHR = 1000226000,
    VertexInputEXT = 1000352000,
    /// Not promoted to 1.3
    PatchControlPointsEXT = 1000377000,
    /// Not promoted to 1.3
    LogicOpEXT = 1000377003,
    ColorWriteEnableEXT = 1000381000,
    DepthClampEnableEXT = 1000455003,
    PolygonModeEXT = 1000455004,
    RasterizationSamplesEXT = 1000455005,
    SampleMaskEXT = 1000455006,
    AlphaToCoverageEnableEXT = 1000455007,
    AlphaToOneEnableEXT = 1000455008,
    LogicOpEnableEXT = 1000455009,
    ColorBlendEnableEXT = 1000455010,
    ColorBlendEquationEXT = 1000455011,
    ColorWriteMaskEXT = 1000455012,
    TessellationDomainOriginEXT = 1000455002,
    RasterizationStreamEXT = 1000455013,
    ConservativeRasterizationModeEXT = 1000455014,
    ExtraPrimitiveOverestimationSizeEXT = 1000455015,
    DepthClipEnableEXT = 1000455016,
    SampleLocationsEnableEXT = 1000455017,
    ColorBlendAdvancedEXT = 1000455018,
    ProvokingVertexModeEXT = 1000455019,
    LineRasterizationModeEXT = 1000455020,
    LineStippleEnableEXT = 1000455021,
    DepthClipNegativeOneToOneEXT = 1000455022,
    ViewportWScalingEnableNV = 1000455023,
    ViewportSwizzleNV = 1000455024,
    CoverageToColorEnableNV = 1000455025,
    CoverageToColorLocationNV = 1000455026,
    CoverageModulationModeNV = 1000455027,
    CoverageModulationTableEnableNV = 1000455028,
    CoverageModulationTableNV = 1000455029,
    ShadingRateImageEnableNV = 1000455030,
    RepresentativeFragmentTestEnableNV = 1000455031,
    CoverageReductionModeNV = 1000455032,
    AttachmentFeedbackLoopEnableEXT = 1000524000,
    DepthClampRangeEXT = 1000582000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorUpdateTemplateType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DescriptorUpdateTemplateType {
    /// Create descriptor update template for descriptor set updates
    #[default]
    DescriptorSet = 0,
    PushDescriptors = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkObjectType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ObjectType {
    #[default]
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,
    DescriptorUpdateTemplate = 1000085000,
    SamplerYcbcrConversion = 1000156000,
    PrivateDataSlot = 1000295000,
    SurfaceKHR = 1000000000,
    SwapchainKHR = 1000001000,
    DisplayKHR = 1000002000,
    DisplayModeKHR = 1000002001,
    DebugReportCallbackEXT = 1000011000,
    /// VkVideoSessionKHR
    VideoSessionKHR = 1000023000,
    /// VkVideoSessionParametersKHR
    VideoSessionParametersKHR = 1000023001,
    CuModuleNVX = 1000029000,
    CuFunctionNVX = 1000029001,
    DebugUtilsMessengerEXT = 1000128000,
    AccelerationStructureKHR = 1000150000,
    ValidationCacheEXT = 1000160000,
    AccelerationStructureNV = 1000165000,
    PerformanceConfigurationINTEL = 1000210000,
    DeferredOperationKHR = 1000268000,
    IndirectCommandsLayoutNV = 1000277000,
    CudaModuleNV = 1000307000,
    CudaFunctionNV = 1000307001,
    /// VkBufferCollectionFUCHSIA
    BufferCollectionFUCHSIA = 1000366000,
    MicromapEXT = 1000396000,
    TensorARM = 1000460000,
    TensorViewARM = 1000460001,
    OpticalFlowSessionNV = 1000464000,
    ShaderEXT = 1000482000,
    PipelineBinaryKHR = 1000483000,
    /// VkSemaphoreSciSyncPoolNV
    SemaphoreSciSyncPoolNV = 1000489000,
    DataGraphPipelineSessionARM = 1000507000,
    ExternalComputeQueueNV = 1000556000,
    IndirectCommandsLayoutEXT = 1000572000,
    IndirectExecutionSetEXT = 1000572001,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingInvocationReorderModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum RayTracingInvocationReorderModeEXT {
    #[default]
    None = 0,
    Reorder = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingLssIndexingModeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum RayTracingLssIndexingModeNV {
    #[default]
    List = 0,
    Successive = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingLssPrimitiveEndCapsModeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum RayTracingLssPrimitiveEndCapsModeNV {
    #[default]
    None = 0,
    Chained = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDirectDriverLoadingModeLUNARG.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DirectDriverLoadingModeLUNARG {
    #[default]
    Exclusive = 0,
    Inclusive = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagModeAMD.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AntiLagModeAMD {
    #[default]
    DriverControl = 0,
    On = 1,
    Off = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAntiLagStageAMD.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AntiLagStageAMD {
    #[default]
    Input = 0,
    Present = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSemaphoreType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SemaphoreType {
    #[default]
    Binary = 0,
    Timeline = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPresentModeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PresentModeKHR {
    #[default]
    Immediate = 0,
    Mailbox = 1,
    Fifo = 2,
    FifoRelaxed = 3,
    SharedDemandRefresh = 1000111000,
    SharedContinuousRefresh = 1000111001,
    FifoLatestReady = 1000361000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkColorSpaceKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ColorSpaceKHR {
    #[default]
    SrgbNonlinear = 0,
    DisplayP3Nonlinear = 1000104001,
    ExtendedSrgbLinear = 1000104002,
    DisplayP3Linear = 1000104003,
    DciP3Nonlinear = 1000104004,
    Bt709Linear = 1000104005,
    Bt709Nonlinear = 1000104006,
    Bt2020Linear = 1000104007,
    Hdr10St2084 = 1000104008,
    Dolbyvision = 1000104009,
    Hdr10Hlg = 1000104010,
    AdobergbLinear = 1000104011,
    AdobergbNonlinear = 1000104012,
    PassThrough = 1000104013,
    ExtendedSrgbNonlinear = 1000104014,
    DisplayNative = 1000213000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplaySurfaceStereoTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DisplaySurfaceStereoTypeNV {
    #[default]
    None = 0,
    OnboardDin = 1,
    Hdmi3d = 2,
    InbandDisplayport = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTimeDomainKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum TimeDomainKHR {
    #[default]
    Device = 0,
    ClockMonotonic = 1,
    ClockMonotonicRaw = 2,
    QueryPerformanceCounter = 3,
    PresentStageLocal = 1000208000,
    SwapchainLocal = 1000208001,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDebugReportObjectTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DebugReportObjectTypeEXT {
    #[default]
    Unknown = 0,
    Instance = 1,
    PhysicalDevice = 2,
    Device = 3,
    Queue = 4,
    Semaphore = 5,
    CommandBuffer = 6,
    Fence = 7,
    DeviceMemory = 8,
    Buffer = 9,
    Image = 10,
    Event = 11,
    QueryPool = 12,
    BufferView = 13,
    ImageView = 14,
    ShaderModule = 15,
    PipelineCache = 16,
    PipelineLayout = 17,
    RenderPass = 18,
    Pipeline = 19,
    DescriptorSetLayout = 20,
    Sampler = 21,
    DescriptorPool = 22,
    DescriptorSet = 23,
    Framebuffer = 24,
    CommandPool = 25,
    SurfaceKhr = 26,
    SwapchainKhr = 27,
    DebugReportCallbackExt = 28,
    DisplayKhr = 29,
    DisplayModeKhr = 30,
    ValidationCacheExt = 33,
    SamplerYcbcrConversion = 1000156000,
    DescriptorUpdateTemplate = 1000085000,
    CuModuleNvx = 1000029000,
    CuFunctionNvx = 1000029001,
    AccelerationStructureKhr = 1000150000,
    AccelerationStructureNv = 1000165000,
    CudaModuleNv = 1000307000,
    CudaFunctionNv = 1000307001,
    BufferCollectionFuchsia = 1000366000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceMemoryReportEventTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DeviceMemoryReportEventTypeEXT {
    #[default]
    Allocate = 0,
    Free = 1,
    Import = 2,
    Unimport = 3,
    AllocationFailed = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRasterizationOrderAMD.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum RasterizationOrderAMD {
    #[default]
    Strict = 0,
    Relaxed = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ClusterAccelerationStructureTypeNV {
    #[default]
    ClustersBottomLevel = 0,
    TriangleCluster = 1,
    TriangleClusterTemplate = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ClusterAccelerationStructureOpTypeNV {
    #[default]
    MoveObjects = 0,
    BuildClustersBottomLevel = 1,
    BuildTriangleCluster = 2,
    BuildTriangleClusterTemplate = 3,
    InstantiateTriangleCluster = 4,
    GetClusterTemplateIndices = 5,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkClusterAccelerationStructureOpModeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ClusterAccelerationStructureOpModeNV {
    #[default]
    ImplicitDestinations = 0,
    ExplicitDestinations = 1,
    ComputeSizes = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCheckEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ValidationCheckEXT {
    #[default]
    All = 0,
    Shaders = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationFeatureEnableEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ValidationFeatureEnableEXT {
    #[default]
    GpuAssisted = 0,
    GpuAssistedReserveBindingSlot = 1,
    BestPractices = 2,
    DebugPrintf = 3,
    SynchronizationValidation = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationFeatureDisableEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ValidationFeatureDisableEXT {
    #[default]
    All = 0,
    Shaders = 1,
    ThreadSafety = 2,
    ApiParameters = 3,
    ObjectLifetimes = 4,
    CoreChecks = 5,
    UniqueHandles = 6,
    ShaderValidationCache = 7,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLayerSettingTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum LayerSettingTypeEXT {
    #[default]
    Bool32 = 0,
    Int32 = 1,
    Int64 = 2,
    Uint32 = 3,
    Uint64 = 4,
    Float32 = 5,
    Float64 = 6,
    String = 7,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsTokenTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum IndirectCommandsTokenTypeNV {
    #[default]
    ShaderGroup = 0,
    StateFlags = 1,
    IndexBuffer = 2,
    VertexBuffer = 3,
    PushConstant = 4,
    DrawIndexed = 5,
    Draw = 6,
    DrawTasks = 7,
    PushData = 1000135000,
    DrawMeshTasks = 1000328000,
    Pipeline = 1000428003,
    Dispatch = 1000428004,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayPowerStateEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DisplayPowerStateEXT {
    #[default]
    Off = 0,
    Suspend = 1,
    On = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceEventTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DeviceEventTypeEXT {
    #[default]
    DisplayHotplug = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplayEventTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DisplayEventTypeEXT {
    #[default]
    FirstPixelOut = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkViewportCoordinateSwizzleNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ViewportCoordinateSwizzleNV {
    #[default]
    PositiveX = 0,
    NegativeX = 1,
    PositiveY = 2,
    NegativeY = 3,
    PositiveZ = 4,
    NegativeZ = 5,
    PositiveW = 6,
    NegativeW = 7,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDiscardRectangleModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DiscardRectangleModeEXT {
    #[default]
    Inclusive = 0,
    Exclusive = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPointClippingBehavior.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PointClippingBehavior {
    #[default]
    AllClipPlanes = 0,
    UserClipPlanesOnly = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerReductionMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SamplerReductionMode {
    #[default]
    WeightedAverage = 0,
    Min = 1,
    Max = 2,
    WeightedAverageRangeclampQCOM = 1000521000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTessellationDomainOrigin.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum TessellationDomainOrigin {
    #[default]
    UpperLeft = 0,
    LowerLeft = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrModelConversion.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SamplerYcbcrModelConversion {
    #[default]
    RgbIdentity = 0,
    /// just range expansion
    YcbcrIdentity = 1,
    /// aka HD YUV
    Ycbcr709 = 2,
    /// aka SD YUV
    Ycbcr601 = 3,
    /// aka UHD YUV
    Ycbcr2020 = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSamplerYcbcrRange.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SamplerYcbcrRange {
    /// Luma 0..1 maps to 0..255, chroma -0.5..0.5 to 1..255 (clamped)
    #[default]
    ItuFull = 0,
    /// Luma 0..1 maps to 16..235, chroma -0.5..0.5 to 16..240
    ItuNarrow = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkChromaLocation.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ChromaLocation {
    #[default]
    CositedEven = 0,
    Midpoint = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBlendOverlapEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BlendOverlapEXT {
    #[default]
    Uncorrelated = 0,
    Disjoint = 1,
    Conjoint = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCoverageModulationModeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CoverageModulationModeNV {
    #[default]
    None = 0,
    Rgb = 1,
    Alpha = 2,
    Rgba = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCoverageReductionModeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CoverageReductionModeNV {
    #[default]
    Merge = 0,
    Truncate = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkValidationCacheHeaderVersionEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ValidationCacheHeaderVersionEXT {
    #[default]
    One = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderInfoTypeAMD.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ShaderInfoTypeAMD {
    #[default]
    Statistics = 0,
    Binary = 1,
    Disassembly = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueueGlobalPriority.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum QueueGlobalPriority {
    #[default]
    Low = 128,
    Medium = 256,
    High = 512,
    Realtime = 1024,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkConservativeRasterizationModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ConservativeRasterizationModeEXT {
    #[default]
    Disabled = 0,
    Overestimate = 1,
    Underestimate = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVendorId.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum VendorId {
    /// Khronos vendor ID
    #[default]
    Khronos = 65536,
    /// Vivante vendor ID
    VIV = 65537,
    /// VeriSilicon vendor ID
    VSI = 65538,
    /// Kazan Software Renderer
    Kazan = 65539,
    /// Codeplay Software Ltd. vendor ID
    Codeplay = 65540,
    /// Mesa vendor ID
    MESA = 65541,
    /// PoCL vendor ID
    Pocl = 65542,
    /// Mobileye vendor ID
    Mobileye = 65543,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDriverId.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DriverId {
    /// Advanced Micro Devices, Inc.
    #[default]
    AmdProprietary = 1,
    /// Advanced Micro Devices, Inc.
    AmdOpenSource = 2,
    /// Mesa open source project
    MesaRadv = 3,
    /// NVIDIA Corporation
    NvidiaProprietary = 4,
    /// Intel Corporation
    IntelProprietaryWindows = 5,
    /// Intel Corporation
    IntelOpenSourceMESA = 6,
    /// Imagination Technologies
    ImaginationProprietary = 7,
    /// Qualcomm Technologies, Inc.
    QualcommProprietary = 8,
    /// Arm Limited
    ArmProprietary = 9,
    /// Google LLC
    GoogleSwiftshader = 10,
    /// Google LLC
    GgpProprietary = 11,
    /// Broadcom Inc.
    BroadcomProprietary = 12,
    /// Mesa
    MesaLlvmpipe = 13,
    /// MoltenVK
    Moltenvk = 14,
    /// Core Avionics & Industrial Inc.
    CoreaviProprietary = 15,
    /// Juice Technologies, Inc.
    JuiceProprietary = 16,
    /// Verisilicon, Inc.
    VerisiliconProprietary = 17,
    /// Mesa open source project
    MesaTurnip = 18,
    /// Mesa open source project
    MesaV3dv = 19,
    /// Mesa open source project
    MesaPanvk = 20,
    /// Samsung Electronics Co., Ltd.
    SamsungProprietary = 21,
    /// Mesa open source project
    MesaVenus = 22,
    /// Mesa open source project
    MesaDozen = 23,
    /// Mesa open source project
    MesaNvk = 24,
    /// Imagination Technologies
    ImaginationOpenSourceMESA = 25,
    /// Mesa open source project
    MesaHoneykrisp = 26,
    /// Vulkan SC Emulation on Vulkan
    VulkanScEmulationOnVulkan = 27,
    /// Mesa open source project
    MesaKosmickrisp = 28,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShadingRatePaletteEntryNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ShadingRatePaletteEntryNV {
    #[default]
    NoInvocations = 0,
    Type16InvocationsPerPixel = 1,
    Type8InvocationsPerPixel = 2,
    Type4InvocationsPerPixel = 3,
    Type2InvocationsPerPixel = 4,
    Type1InvocationPerPixel = 5,
    Type1InvocationPer2x1Pixels = 6,
    Type1InvocationPer1x2Pixels = 7,
    Type1InvocationPer2x2Pixels = 8,
    Type1InvocationPer4x2Pixels = 9,
    Type1InvocationPer2x4Pixels = 10,
    Type1InvocationPer4x4Pixels = 11,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCoarseSampleOrderTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CoarseSampleOrderTypeNV {
    #[default]
    Default = 0,
    Custom = 1,
    PixelMajor = 2,
    SampleMajor = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyAccelerationStructureModeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CopyAccelerationStructureModeKHR {
    #[default]
    Clone = 0,
    Compact = 1,
    Serialize = 2,
    Deserialize = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildAccelerationStructureModeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BuildAccelerationStructureModeKHR {
    #[default]
    Build = 0,
    Update = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureTypeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AccelerationStructureTypeKHR {
    #[default]
    TopLevel = 0,
    BottomLevel = 1,
    Generic = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkGeometryTypeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum GeometryTypeKHR {
    #[default]
    Triangles = 0,
    Aabbs = 1,
    Instances = 2,
    Spheres = 1000429004,
    LinearSweptSpheres = 1000429005,
    DenseGeometryFormatTriangles = 1000478000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMemoryRequirementsTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AccelerationStructureMemoryRequirementsTypeNV {
    #[default]
    Object = 0,
    BuildScratch = 1,
    UpdateScratch = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureBuildTypeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AccelerationStructureBuildTypeKHR {
    #[default]
    Host = 0,
    Device = 1,
    HostOrDevice = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkRayTracingShaderGroupTypeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum RayTracingShaderGroupTypeKHR {
    #[default]
    General = 0,
    TrianglesHitGroup = 1,
    ProceduralHitGroup = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureCompatibilityKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AccelerationStructureCompatibilityKHR {
    #[default]
    Compatible = 0,
    Incompatible = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderGroupShaderKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ShaderGroupShaderKHR {
    #[default]
    General = 0,
    ClosestHit = 1,
    AnyHit = 2,
    Intersection = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMemoryOverallocationBehaviorAMD.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum MemoryOverallocationBehaviorAMD {
    #[default]
    Default = 0,
    Allowed = 1,
    Disallowed = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFullScreenExclusiveEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FullScreenExclusiveEXT {
    #[default]
    Default = 0,
    Allowed = 1,
    Disallowed = 2,
    ApplicationControlled = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterScopeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceCounterScopeKHR {
    #[default]
    CommandBuffer = 0,
    RenderPass = 1,
    Command = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterUnitKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceCounterUnitKHR {
    #[default]
    Generic = 0,
    Percentage = 1,
    Nanoseconds = 2,
    Bytes = 3,
    BytesPerSecond = 4,
    Kelvin = 5,
    Watts = 6,
    Volts = 7,
    Amps = 8,
    Hertz = 9,
    Cycles = 10,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceCounterStorageKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceCounterStorageKHR {
    #[default]
    Int32 = 0,
    Int64 = 1,
    Uint32 = 2,
    Uint64 = 3,
    Float32 = 4,
    Float64 = 5,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceConfigurationTypeINTEL.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceConfigurationTypeINTEL {
    #[default]
    CommandQueueMetricsDiscoveryActivated = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryPoolSamplingModeINTEL.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum QueryPoolSamplingModeINTEL {
    #[default]
    Manual = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceOverrideTypeINTEL.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceOverrideTypeINTEL {
    #[default]
    NullHardware = 0,
    FlushGpuCaches = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceParameterTypeINTEL.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceParameterTypeINTEL {
    #[default]
    HwCountersSupported = 0,
    StreamMarkerValidBits = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPerformanceValueTypeINTEL.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PerformanceValueTypeINTEL {
    #[default]
    Uint32 = 0,
    Uint64 = 1,
    Float = 2,
    Bool = 3,
    String = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderFloatControlsIndependence.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ShaderFloatControlsIndependence {
    #[default]
    Type32BitOnly = 0,
    All = 1,
    None = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineExecutableStatisticFormatKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineExecutableStatisticFormatKHR {
    #[default]
    Bool32 = 0,
    Int64 = 1,
    Uint64 = 2,
    Float64 = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLineRasterizationMode.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum LineRasterizationMode {
    #[default]
    Default = 0,
    Rectangular = 1,
    Bresenham = 2,
    RectangularSmooth = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultLevel.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FaultLevel {
    #[default]
    Unassigned = 0,
    Critical = 1,
    Recoverable = 2,
    Warning = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultType.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FaultType {
    #[default]
    Invalid = 0,
    Unassigned = 1,
    Implementation = 2,
    System = 3,
    PhysicalDevice = 4,
    CommandBufferFull = 5,
    InvalidApiUsage = 6,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFaultQueryBehavior.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FaultQueryBehavior {
    #[default]
    GetAndClearAllFaults = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineMatchControl.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineMatchControl {
    #[default]
    ApplicationUuidExactMatch = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateCombinerOpKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FragmentShadingRateCombinerOpKHR {
    #[default]
    Keep = 0,
    Replace = 1,
    Min = 2,
    Max = 3,
    Mul = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FragmentShadingRateNV {
    #[default]
    Type1InvocationPerPixel = 0,
    Type1InvocationPer1x2Pixels = 1,
    Type1InvocationPer2x1Pixels = 4,
    Type1InvocationPer2x2Pixels = 5,
    Type1InvocationPer2x4Pixels = 6,
    Type1InvocationPer4x2Pixels = 9,
    Type1InvocationPer4x4Pixels = 10,
    Type2InvocationsPerPixel = 11,
    Type4InvocationsPerPixel = 12,
    Type8InvocationsPerPixel = 13,
    Type16InvocationsPerPixel = 14,
    NoInvocations = 15,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkFragmentShadingRateTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum FragmentShadingRateTypeNV {
    #[default]
    FragmentSize = 0,
    Enums = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSubpassMergeStatusEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SubpassMergeStatusEXT {
    #[default]
    Merged = 0,
    Disallowed = 1,
    NotMergedSideEffects = 2,
    NotMergedSamplesMismatch = 3,
    NotMergedViewsMismatch = 4,
    NotMergedAliasing = 5,
    NotMergedDependencies = 6,
    NotMergedIncompatibleInputAttachment = 7,
    NotMergedTooManyAttachments = 8,
    NotMergedInsufficientStorage = 9,
    NotMergedDepthStencilCount = 10,
    NotMergedResolveAttachmentReuse = 11,
    NotMergedSingleSubpass = 12,
    NotMergedUnspecified = 13,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSciSyncClientTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SciSyncClientTypeNV {
    #[default]
    Signaler = 0,
    Waiter = 1,
    SignalerWaiter = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkSciSyncPrimitiveTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum SciSyncPrimitiveTypeNV {
    #[default]
    Fence = 0,
    Semaphore = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkProvokingVertexModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ProvokingVertexModeEXT {
    #[default]
    FirstVertex = 0,
    LastVertex = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineCacheValidationVersion.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineCacheValidationVersion {
    #[default]
    SafetyCriticalOne = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkAccelerationStructureMotionInstanceTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum AccelerationStructureMotionInstanceTypeNV {
    #[default]
    Static = 0,
    MatrixMotion = 1,
    SrtMotion = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceAddressBindingTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DeviceAddressBindingTypeEXT {
    #[default]
    Bind = 0,
    Unbind = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkQueryResultStatusKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum QueryResultStatusKHR {
    #[default]
    Error = -1,
    NotReady = 0,
    Complete = 1,
    InsufficientBitstreamBufferRange = -1000299000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeTuningModeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum VideoEncodeTuningModeKHR {
    #[default]
    Default = 0,
    HighQuality = 1,
    LowLatency = 2,
    UltraLowLatency = 3,
    Lossless = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPartitionedAccelerationStructureOpTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PartitionedAccelerationStructureOpTypeNV {
    #[default]
    WriteInstance = 0,
    UpdateInstance = 1,
    WritePartitionTranslation = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1PredictionModeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum VideoEncodeAV1PredictionModeKHR {
    #[default]
    IntraOnly = 0,
    SingleReference = 1,
    UnidirectionalCompound = 2,
    BidirectionalCompound = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkVideoEncodeAV1RateControlGroupKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum VideoEncodeAV1RateControlGroupKHR {
    #[default]
    Intra = 0,
    Predictive = 1,
    Bipredictive = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessBufferBehavior.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineRobustnessBufferBehavior {
    #[default]
    DeviceDefault = 0,
    Disabled = 1,
    RobustBufferAccess = 2,
    RobustBufferAccess2 = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPipelineRobustnessImageBehavior.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PipelineRobustnessImageBehavior {
    #[default]
    DeviceDefault = 0,
    Disabled = 1,
    RobustImageAccess = 2,
    RobustImageAccess2 = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowPerformanceLevelNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum OpticalFlowPerformanceLevelNV {
    #[default]
    Unknown = 0,
    Slow = 1,
    Medium = 2,
    Fast = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpticalFlowSessionBindingPointNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum OpticalFlowSessionBindingPointNV {
    #[default]
    Unknown = 0,
    Input = 1,
    Reference = 2,
    Hint = 3,
    FlowVector = 4,
    BackwardFlowVector = 5,
    Cost = 6,
    BackwardCost = 7,
    GlobalFlow = 8,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkMicromapTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum MicromapTypeEXT {
    #[default]
    OpacityMicromap = 0,
    DisplacementMicromap = 1000397000,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCopyMicromapModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CopyMicromapModeEXT {
    #[default]
    Clone = 0,
    Serialize = 1,
    Deserialize = 2,
    Compact = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBuildMicromapModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BuildMicromapModeEXT {
    #[default]
    Build = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpacityMicromapFormatEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum OpacityMicromapFormatEXT {
    #[default]
    Type2State = 1,
    Type4State = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOpacityMicromapSpecialIndexEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum OpacityMicromapSpecialIndexEXT {
    #[default]
    FullyTransparent = -1,
    FullyOpaque = -2,
    FullyUnknownTransparent = -3,
    FullyUnknownOpaque = -4,
    ClusterGeometryDisableOpacityMicromap = -5,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthBiasRepresentationEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DepthBiasRepresentationEXT {
    #[default]
    LeastRepresentableValueFormat = 0,
    LeastRepresentableValueForceUnorm = 1,
    Float = 2,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultAddressTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DeviceFaultAddressTypeEXT {
    /// Currently unused
    #[default]
    None = 0,
    ReadInvalid = 1,
    WriteInvalid = 2,
    ExecuteInvalid = 3,
    InstructionPointerUnknown = 4,
    InstructionPointerInvalid = 5,
    InstructionPointerFault = 6,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDeviceFaultVendorBinaryHeaderVersionEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DeviceFaultVendorBinaryHeaderVersionEXT {
    #[default]
    One = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectExecutionSetInfoTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum IndirectExecutionSetInfoTypeEXT {
    #[default]
    Pipelines = 0,
    ShaderObjects = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkIndirectCommandsTokenTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum IndirectCommandsTokenTypeEXT {
    #[default]
    ExecutionSet = 0,
    PushConstant = 1,
    SequenceIndex = 2,
    IndexBuffer = 3,
    VertexBuffer = 4,
    DrawIndexed = 5,
    Draw = 6,
    DrawIndexedCount = 7,
    DrawCount = 8,
    Dispatch = 9,
    PushData = 1000135000,
    PushDataSequenceIndex = 1000135001,
    DrawMeshTasksNv = 1000202002,
    DrawMeshTasksCountNv = 1000202003,
    DrawMeshTasks = 1000328000,
    DrawMeshTasksCount = 1000328001,
    TraceRays2 = 1000386004,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDisplacementMicromapFormatNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DisplacementMicromapFormatNV {
    #[default]
    Type64Triangles64Bytes = 1,
    Type256Triangles128Bytes = 2,
    Type1024Triangles128Bytes = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkShaderCodeTypeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ShaderCodeTypeEXT {
    #[default]
    Binary = 0,
    Spirv = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkScopeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ScopeKHR {
    #[default]
    Device = 1,
    Workgroup = 2,
    Subgroup = 3,
    QueueFamily = 5,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkComponentTypeKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum ComponentTypeKHR {
    #[default]
    Float16 = 0,
    Float32 = 1,
    Float64 = 2,
    Sint8 = 3,
    Sint16 = 4,
    Sint32 = 5,
    Sint64 = 6,
    Uint8 = 7,
    Uint16 = 8,
    Uint32 = 9,
    Uint64 = 10,
    Bfloat16 = 1000141000,
    Sint8Packed = 1000491000,
    Uint8Packed = 1000491001,
    Float8E4m3 = 1000491002,
    Float8E5m2 = 1000491003,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCubicFilterWeightsQCOM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CubicFilterWeightsQCOM {
    #[default]
    CatmullRom = 0,
    ZeroTangentCardinal = 1,
    BSpline = 2,
    MitchellNetravali = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkBlockMatchWindowCompareModeQCOM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum BlockMatchWindowCompareModeQCOM {
    #[default]
    Min = 0,
    Max = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceLayeredApiKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PhysicalDeviceLayeredApiKHR {
    #[default]
    Vulkan = 0,
    D3d12 = 1,
    Metal = 2,
    Opengl = 3,
    Opengles = 4,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLayeredDriverUnderlyingApiMSFT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum LayeredDriverUnderlyingApiMSFT {
    #[default]
    None = 0,
    D3d12 = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkLatencyMarkerNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum LatencyMarkerNV {
    #[default]
    SimulationStart = 0,
    SimulationEnd = 1,
    RendersubmitStart = 2,
    RendersubmitEnd = 3,
    PresentStart = 4,
    PresentEnd = 5,
    InputSample = 6,
    TriggerFlash = 7,
    OutOfBandRendersubmitStart = 8,
    OutOfBandRendersubmitEnd = 9,
    OutOfBandPresentStart = 10,
    OutOfBandPresentEnd = 11,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkOutOfBandQueueTypeNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum OutOfBandQueueTypeNV {
    #[default]
    Render = 0,
    Present = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCompressedTriangleFormatAMDX.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CompressedTriangleFormatAMDX {
    #[default]
    Dgf1 = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDepthClampModeEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DepthClampModeEXT {
    #[default]
    ViewportRange = 0,
    UserDefinedRange = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkCooperativeVectorMatrixLayoutNV.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum CooperativeVectorMatrixLayoutNV {
    #[default]
    RowMajor = 0,
    ColumnMajor = 1,
    InferencingOptimal = 2,
    TrainingOptimal = 3,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkTensorTilingARM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum TensorTilingARM {
    #[default]
    Optimal = 0,
    Linear = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDefaultVertexAttributeValueKHR.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DefaultVertexAttributeValueKHR {
    #[default]
    ZeroZeroZeroZero = 0,
    ZeroZeroZeroOne = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointARM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DataGraphPipelineSessionBindPointARM {
    #[default]
    Transient = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelineSessionBindPointTypeARM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DataGraphPipelineSessionBindPointTypeARM {
    #[default]
    Memory = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphPipelinePropertyARM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DataGraphPipelinePropertyARM {
    #[default]
    CreationLog = 0,
    Identifier = 1,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphProcessingEngineTypeARM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PhysicalDeviceDataGraphProcessingEngineTypeARM {
    #[default]
    Default = 0,
    Neural = 1000629000,
    Compute = 1000629001,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkPhysicalDeviceDataGraphOperationTypeARM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum PhysicalDeviceDataGraphOperationTypeARM {
    #[default]
    SpirvExtendedInstructionSet = 0,
    NeuralModel = 1000629000,
    BuiltinModel = 1000629001,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDataGraphModelCacheTypeQCOM.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DataGraphModelCacheTypeQCOM {
    #[default]
    GenericBinary = 0,
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VkDescriptorMappingSourceEXT.html>
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub enum DescriptorMappingSourceEXT {
    #[default]
    HeapWithConstantOffset = 0,
    HeapWithPushIndex = 1,
    HeapWithIndirectIndex = 2,
    HeapWithIndirectIndexArray = 3,
    ResourceHeapData = 4,
    PushData = 5,
    PushAddress = 6,
    IndirectAddress = 7,
    HeapWithShaderRecordIndex = 8,
    ShaderRecordData = 9,
    ShaderRecordAddress = 10,
}
