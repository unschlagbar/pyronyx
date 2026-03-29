// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/constants.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_MAKE_API_VERSION.html>
pub const fn make_api_version(variant: u32, major: u32, minor: u32, patch: u32) -> u32 {
    ((variant) << 29) | ((major) << 22) | ((minor) << 12) | (patch)
}
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_VARIANT.html>
pub const fn api_version_variant(version: u32) -> u32 {
    (version) >> 29
}
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MAJOR.html>
pub const fn api_version_major(version: u32) -> u32 {
    ((version) >> 22) & 0x7fu32
}
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_MINOR.html>
pub const fn api_version_minor(version: u32) -> u32 {
    ((version) >> 12) & 0x3ffu32
}
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_PATCH.html>
pub const fn api_version_patch(version: u32) -> u32 {
    (version) & 0xfffu32
}
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_0.html>
pub const API_VERSION_1_0: u32 = make_api_version(0, 1, 0, 0);
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_1.html>
pub const API_VERSION_1_1: u32 = make_api_version(0, 1, 1, 0);
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_2.html>
pub const API_VERSION_1_2: u32 = make_api_version(0, 1, 2, 0);
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_3.html>
pub const API_VERSION_1_3: u32 = make_api_version(0, 1, 3, 0);
/// <https://docs.vulkan.org/refpages/latest/refpages/source/VK_API_VERSION_1_4.html>
pub const API_VERSION_1_4: u32 = make_api_version(0, 1, 4, 0);

pub const MAX_PHYSICAL_DEVICE_NAME_SIZE: u32 = 256;
pub const UUID_SIZE: u32 = 16;
pub const LUID_SIZE: u32 = 8;
pub const MAX_EXTENSION_NAME_SIZE: u32 = 256;
pub const MAX_DESCRIPTION_SIZE: u32 = 256;
pub const MAX_MEMORY_TYPES: u32 = 32;
/// The maximum number of unique memory heaps, each of which supporting 1 or more memory types
pub const MAX_MEMORY_HEAPS: u32 = 16;
pub const LOD_CLAMP_NONE: f32 = 1000.0;
pub const REMAINING_MIP_LEVELS: u32 = !0;
pub const REMAINING_ARRAY_LAYERS: u32 = !0;
pub const REMAINING_3D_SLICES_EXT: u32 = !0;
pub const WHOLE_SIZE: u64 = !0;
pub const ATTACHMENT_UNUSED: u32 = !0;
pub const TRUE: u32 = 1;
pub const FALSE: u32 = 0;
pub const QUEUE_FAMILY_IGNORED: u32 = !0;
pub const QUEUE_FAMILY_EXTERNAL: u32 = !1;
pub const QUEUE_FAMILY_FOREIGN_EXT: u32 = !2;
pub const SUBPASS_EXTERNAL: u32 = !0;
pub const MAX_DEVICE_GROUP_SIZE: u32 = 32;
pub const MAX_DRIVER_NAME_SIZE: u32 = 256;
pub const MAX_DRIVER_INFO_SIZE: u32 = 256;
pub const SHADER_UNUSED_KHR: u32 = !0;
pub const MAX_GLOBAL_PRIORITY_SIZE: u32 = 16;
pub const MAX_SHADER_MODULE_IDENTIFIER_SIZE_EXT: u32 = 32;
pub const MAX_PIPELINE_BINARY_KEY_SIZE_KHR: u32 = 32;
pub const MAX_VIDEO_AV1_REFERENCES_PER_FRAME_KHR: u32 = 7;
pub const MAX_VIDEO_VP9_REFERENCES_PER_FRAME_KHR: u32 = 3;
pub const SHADER_INDEX_UNUSED_AMDX: u32 = !0;
pub const PARTITIONED_ACCELERATION_STRUCTURE_PARTITION_INDEX_GLOBAL_NV: u32 = !0;
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_ALIGNMENT_AMDX: u32 = 128;
pub const COMPRESSED_TRIANGLE_FORMAT_DGF1_BYTE_STRIDE_AMDX: u32 = 128;
pub const MAX_PHYSICAL_DEVICE_DATA_GRAPH_OPERATION_SET_NAME_SIZE_ARM: u32 = 128;
pub const DATA_GRAPH_MODEL_TOOLCHAIN_VERSION_LENGTH_QCOM: u32 = 3;
pub const COMPUTE_OCCUPANCY_PRIORITY_LOW_NV: f32 = 0.25f32;
pub const COMPUTE_OCCUPANCY_PRIORITY_NORMAL_NV: f32 = 0.50f32;
pub const COMPUTE_OCCUPANCY_PRIORITY_HIGH_NV: f32 = 0.75f32;
