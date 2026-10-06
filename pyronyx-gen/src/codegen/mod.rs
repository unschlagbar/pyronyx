pub mod bitflags;
pub mod bitflags_debug;
pub mod commands;
pub mod constants;
pub mod enums;
pub mod extensions;
pub mod fns;
pub mod impls;
pub mod types;
pub mod video;

use std::collections::HashSet;
use std::fs;
use std::path::Path;

use heck::ToSnakeCase;

use crate::parse::c_to_rust;

pub struct Writer {
    buf: String,
}

impl Writer {
    pub fn new() -> Self {
        Self {
            buf: String::with_capacity(1024 * 256),
        }
    }

    pub fn ln(&mut self, s: &str) {
        self.buf.push_str(s);
        self.buf.push('\n');
    }

    pub fn blank(&mut self) {
        self.buf.push('\n');
    }

    pub fn len(&mut self) -> usize {
        self.buf.len()
    }

    pub fn save(self, path: impl AsRef<Path>) {
        let path = path.as_ref();
        if let Some(parent) = path.parent() {
            fs::create_dir_all(parent).unwrap();
        }
        fs::write(path, self.buf)
            .unwrap_or_else(|e| panic!("Konnte {:?} nicht schreiben: {}", path, e));
    }

    pub fn into_string(self) -> String {
        self.buf
    }
}

pub fn file_header(w: &mut Writer, section: &str) {
    w.ln("// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    w.ln(&format!("// Auto generated from pyronyx-gen — {section}"));
    w.ln("// Do not Edit! Execute `cargo run pyronyx-gen`");
    w.ln("// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!");
    w.blank();
}

pub fn rust_name(name: &str) -> String {
    let name = c_to_rust(name);
    let name = name
        .strip_prefix("Vk")
        .unwrap_or(name.strip_prefix("StdVideo").unwrap_or(name));
    name.replace("FlagBits", "Flags")
}

pub fn const_name(name: &str) -> &str {
    name.strip_prefix("VK_")
        .unwrap_or(name.strip_prefix("STD_VIDEO_").unwrap_or(name))
}

pub fn rust_member(name: &str, ps: &mut HashSet<String>) -> String {
    match name {
        "type" => "ty".to_string(),
        name => {
            let name = name.to_snake_case().replace("std_", "");
            if let Some(witout_pt) = name.strip_prefix("p_") {
                ps.insert(witout_pt.to_string());
                witout_pt.to_string()
            } else if let Some(witout_ppt) = name.strip_prefix("pp_")
                && !ps.contains(witout_ppt)
            {
                witout_ppt.to_string()
            } else {
                name
            }
        }
    }
}

pub fn simple_rust_member(name: &str) -> String {
    match name {
        "type" => "ty".to_string(),
        name => {
            let name = name.to_snake_case();
            if let Some(witout_pt) = name.strip_prefix("p_") {
                witout_pt.to_string()
            } else {
                name
            }
        }
    }
}

pub const fn a_lifetime(lifetime: bool) -> &'static str {
    if lifetime { "<'a>" } else { "" }
}

pub const fn u_lifetime(lifetime: bool) -> &'static str {
    if lifetime { "<'_>" } else { "" }
}

pub const VENDORS: &[&str] = &[
    "Img",
    "Amd",
    "Amdx",
    "Arm",
    "Fsl",
    "Brcm",
    "Nxp",
    "Nv",
    "Nvx",
    "Viv",
    "Vsi",
    "Kdab",
    "Android",
    "Chromium",
    "Fuchsia",
    "Ggp",
    "Google",
    "Qcom",
    "Lunarg",
    "Nzxt",
    "Samsung",
    "Sec",
    "Tizen",
    "Renderdoc",
    "Nn",
    "Mvk",
    "Khr",
    "Khx",
    "Ext",
    "Mesa",
    "Intel",
    "Huawei",
    "Ohos",
    "Valve",
    "Qnx",
    "Juice",
    "Fb",
    "Rastergrid",
    "Msft",
    "Shady",
    "Fredemmott",
    "Mtk",
    "Openxr",
];

pub const HAND_WRITTEN_FNS: &[(&str, &str)] = &[
    ("Instance", "vkGetInstanceProcAddr"),
    ("Instance", "vkEnumeratePhysicalDevices"),
    ("PhysicalDevice", "vkCreateDevice"),
    ("Device", "vkAllocateCommandBuffers"),
    ("Device", "vkGetDeviceProcAddr"),
    ("Device", "vkGetDeviceQueue"),
    ("Device", "vkGetDeviceQueue2"),
];

pub const RETURNS_SUBOPTIMAL: &[&str] = &["vkAcquireNextImageKHR", "vkAcquireNextImage2KHR"];

/// Bitfield groups (consecutive `:N` members filling one u32) → hand-written
/// packed type in `pyronyx/src/vk/packed.rs`. Widths are listed LSB → MSB.
/// A new layout in the registry requires a new packed type there plus an entry here.
pub const PACKED_TYPES: &[(&[u32], &str)] = &[
    (&[24, 8], "Packed24_8"),
    (&[24, 5, 3], "Packed24_5_3"),
    (&[9, 9, 6, 4, 4], "Packed9_9_6_4_4"),
];

/// Overrides for merged bitfield member names where joining the original
/// member names with `_and_` would be unreadable. Keyed by the joined name.
pub const PACKED_FIELD_NAMES: &[(&str, &str)] = &[(
    "triangle_count_and_vertex_count_and_position_truncate_bit_count_and_index_type_and_opacity_micromap_index_type",
    "triangle_cluster_info_packed",
)];

/// "function Name", "additional params", "body lines", "remove return"
pub const ASSERT_FNS: &[(&str, &str)] = &[("vkGetQueryPoolResults", "")];

/// Commands whose untyped `void*` parameter is a caller-provided byte buffer rather
/// than a pointer to a single opaque object.
///
/// The registry gives these no `len` attribute, so the shape cannot be derived from
/// the XML: `vkGetSemaphoreSciSyncObjNV`'s `pHandle` has the identical C type but
/// points at one object, and turning it into a slice would be a lie. Anything listed
/// here becomes `&mut [u8]`; every other unsized `void*` stays a raw pointer.
///
/// This mirrors what ash does by hand for the same commands.
pub const BYTE_SLICE_FNS: &[&str] = &[
    "vkGetBufferOpaqueCaptureDescriptorDataEXT",
    "vkGetImageOpaqueCaptureDescriptorDataEXT",
    "vkGetImageViewOpaqueCaptureDescriptorDataEXT",
    "vkGetSamplerOpaqueCaptureDescriptorDataEXT",
    "vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
    "vkGetTensorOpaqueCaptureDescriptorDataARM",
    "vkGetTensorViewOpaqueCaptureDescriptorDataARM",
    "vkGetShaderInstrumentationValuesARM",
];

pub fn is_byte_slice_fn(cmd_name: &str) -> bool {
    BYTE_SLICE_FNS.contains(&cmd_name)
}

fn find_assert_fn(cmd_name: &str) -> Option<&'static str> {
    ASSERT_FNS
        .iter()
        .find(|(name, _)| *name == cmd_name)
        .copied()
        .map(|g| g.1)
}

/// Functions with a true mutable-slice input buffer for which a `{fn_name}_len`
/// companion method is also generated, so callers can query the required size
/// without having to compute it manually.
///
/// Fields: `("vk_fn_name", "rust_count_expression")`
///
/// The count expression is pasted verbatim into `{expr} as usize` inside the
/// companion body. It may reference any `Single` parameter by its Rust name,
/// e.g. `"allocate_info.descriptor_set_count"`.
pub const LEN_FNS: &[&str] = &[
    "vkGetPhysicalDeviceQueueFamilyProperties2",
    "vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
    "vkGetPhysicalDeviceSparseImageFormatProperties2",
    "vkGetPhysicalDeviceToolProperties",
    "vkEnumeratePhysicalDeviceGroups",
    "vkGetDeviceImageSparseMemoryRequirements",
    "vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR",
    "vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV",
    "vkGetQueueCheckpointDataNV",
    "vkGetPhysicalDeviceCalibrateableTimeDomainsKHR",
    "vkGetShaderInfoAMD",
    "vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM",
    "vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM",
    "vkGetDataGraphPipelineAvailablePropertiesARM",
    "vkGetDataGraphPipelineSessionBindPointRequirementsARM",
    "vkGetDisplayModeProperties2KHR",
    "vkGetDisplayModePropertiesKHR",
    "vkGetDisplayPlaneSupportedDisplaysKHR",
    "vkGetEncodedVideoSessionParametersKHR",
    "vkGetFaultData",
    "vkGetFramebufferTilePropertiesQCOM",
    "vkGetGpaSessionResultsAMD",
    "vkGetImageSparseMemoryRequirements2",
    "vkGetPastPresentationTimingGOOGLE",
    "vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV",
    "vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR",
    "vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
    "vkGetPhysicalDeviceCooperativeVectorPropertiesNV",
    "vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
    "vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
    "vkGetPhysicalDeviceDisplayProperties2KHR",
    "vkGetPhysicalDeviceDisplayPropertiesKHR",
    "vkGetPhysicalDeviceFragmentShadingRatesKHR",
    "vkGetPhysicalDevicePresentRectanglesKHR",
    "vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
    "vkGetPhysicalDeviceRefreshableObjectTypesKHR",
    "vkGetPhysicalDeviceSurfaceFormats2KHR",
    "vkGetPhysicalDeviceVideoFormatPropertiesKHR",
    "vkGetPipelineBinaryDataKHR",
    "vkGetPipelineExecutableInternalRepresentationsKHR",
    "vkGetPipelineExecutablePropertiesKHR",
    "vkGetPipelineExecutableStatisticsKHR",
    "vkGetQueueCheckpointData2NV",
    "vkGetValidationCacheDataEXT",
    "vkGetVideoSessionMemoryRequirementsKHR",
    "vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM",
    "vkGetDeviceFaultReportsKHR",
];

pub fn find_len_fn(cmd_name: &str) -> Option<&'static str> {
    LEN_FNS.iter().find(|&&name| name == cmd_name).copied()
}

/// Element types must not start with `sType`: `Vec` fills uninitialised memory, but the driver
/// reads `sType`/`pNext` of each element. Those commands belong in [`LEN_FNS`].
pub const VEC_FNS: &[&str] = &[
    "vkGetPhysicalDeviceQueueFamilyProperties",
    "vkEnumerateDeviceExtensionProperties",
    "vkGetPhysicalDeviceSurfaceFormatsKHR",
    "vkGetPhysicalDeviceSurfacePresentModesKHR",
    "vkGetSwapchainImagesKHR",
    "vkGetPipelineCacheData",
    "vkGetImageSparseMemoryRequirements",
    "vkEnumerateDeviceLayerProperties",
    "vkGetPhysicalDeviceSparseImageFormatProperties",
    "vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
    "vkGetPhysicalDeviceSurfacePresentModes2EXT",
    "vkGetShaderBinaryDataEXT",
    "vkGetPhysicalDeviceToolPropertiesEXT",
    "vkGetCudaModuleCacheNV",
];

pub const COPY: &[&str] = &[
    "Offset2D",
    "Offset3D",
    "Extent2D",
    "Extent3D",
    "Rect2D",
    "ClearRect",
    "SurfaceFormatKHR",
];

const LATEXMATH_LEN_MAP: &[(&str, &str)] = &[
    (r"latexmath:[\textrm{codeSize} \over 4]", "code_size / 4"),
    (
        r"latexmath:[\lceil{\mathit{rasterizationSamples} \over 32}\rceil]",
        "(rasterization_samples / 32).ceil()",
    ),
    (
        r"latexmath:[2 \times \mathtt{VK\_UUID\_SIZE}]",
        "2 * UUID_SIZE",
    ),
    (
        r"latexmath:[\lceil{\mathit{maxVertexAttribDivisor} + 1} \over 32\rceil]",
        "((max_vertex_attrib_divisor + 1) / 32).ceil()",
    ),
    (
        r"latexmath:[\lceil{\mathit{displacedMicromapPrimitiveCount} \over 4}\rceil]",
        "(displaced_micromap_primitive_count / 4).ceil()",
    ),
    (
        r"latexmath:[\lceil{rayCount \over 4}\rceil]",
        "(ray_count / 4).ceil()",
    ),
];
