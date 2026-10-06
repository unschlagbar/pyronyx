// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/commands.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use super::vk::*;
use crate::utils::to_option;
use core::ffi::{CStr, c_char, c_void};
#[derive(Clone)]
pub struct InstanceFn {
    pub v1_0: InstanceFnv1_0,
    pub v1_1: InstanceFnv1_1,
    pub ext_debug_report: InstanceFnExtDebugReport,
    pub ext_debug_utils: InstanceFnExtDebugUtils,
    pub ext_directfb_surface: InstanceFnExtDirectfbSurface,
    pub ext_headless_surface: InstanceFnExtHeadlessSurface,
    pub ext_metal_surface: InstanceFnExtMetalSurface,
    pub fuchsia_imagepipe_surface: InstanceFnFuchsiaImagepipeSurface,
    pub ggp_stream_descriptor_surface: InstanceFnGgpStreamDescriptorSurface,
    pub khr_android_surface: InstanceFnKhrAndroidSurface,
    pub khr_display: InstanceFnKhrDisplay,
    pub khr_surface: InstanceFnKhrSurface,
    pub khr_wayland_surface: InstanceFnKhrWaylandSurface,
    pub khr_win32_surface: InstanceFnKhrWin32Surface,
    pub khr_xcb_surface: InstanceFnKhrXcbSurface,
    pub khr_xlib_surface: InstanceFnKhrXlibSurface,
    pub mvk_ios_surface: InstanceFnMvkIosSurface,
    pub mvk_macos_surface: InstanceFnMvkMacosSurface,
    pub nn_vi_surface: InstanceFnNnViSurface,
    pub ohos_surface: InstanceFnOhosSurface,
    pub qnx_screen_surface: InstanceFnQnxScreenSurface,
    pub sec_ubm_surface: InstanceFnSecUbmSurface,
}

impl InstanceFn {
    /// A table with no functions loaded; every call through it panics.
    pub const EMPTY: Self = Self {
        v1_0: InstanceFnv1_0::EMPTY,
        v1_1: InstanceFnv1_1::EMPTY,
        ext_debug_report: InstanceFnExtDebugReport::EMPTY,
        ext_debug_utils: InstanceFnExtDebugUtils::EMPTY,
        ext_directfb_surface: InstanceFnExtDirectfbSurface::EMPTY,
        ext_headless_surface: InstanceFnExtHeadlessSurface::EMPTY,
        ext_metal_surface: InstanceFnExtMetalSurface::EMPTY,
        fuchsia_imagepipe_surface: InstanceFnFuchsiaImagepipeSurface::EMPTY,
        ggp_stream_descriptor_surface: InstanceFnGgpStreamDescriptorSurface::EMPTY,
        khr_android_surface: InstanceFnKhrAndroidSurface::EMPTY,
        khr_display: InstanceFnKhrDisplay::EMPTY,
        khr_surface: InstanceFnKhrSurface::EMPTY,
        khr_wayland_surface: InstanceFnKhrWaylandSurface::EMPTY,
        khr_win32_surface: InstanceFnKhrWin32Surface::EMPTY,
        khr_xcb_surface: InstanceFnKhrXcbSurface::EMPTY,
        khr_xlib_surface: InstanceFnKhrXlibSurface::EMPTY,
        mvk_ios_surface: InstanceFnMvkIosSurface::EMPTY,
        mvk_macos_surface: InstanceFnMvkMacosSurface::EMPTY,
        nn_vi_surface: InstanceFnNnViSurface::EMPTY,
        ohos_surface: InstanceFnOhosSurface::EMPTY,
        qnx_screen_surface: InstanceFnQnxScreenSurface::EMPTY,
        sec_ubm_surface: InstanceFnSecUbmSurface::EMPTY,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        let mut out = Self {
            v1_0: InstanceFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                InstanceFnv1_1::load(&mut loader)
            } else {
                InstanceFnv1_1::EMPTY
            },
            ext_debug_report: InstanceFnExtDebugReport::EMPTY,
            ext_debug_utils: InstanceFnExtDebugUtils::EMPTY,
            ext_directfb_surface: InstanceFnExtDirectfbSurface::EMPTY,
            ext_headless_surface: InstanceFnExtHeadlessSurface::EMPTY,
            ext_metal_surface: InstanceFnExtMetalSurface::EMPTY,
            fuchsia_imagepipe_surface: InstanceFnFuchsiaImagepipeSurface::EMPTY,
            ggp_stream_descriptor_surface: InstanceFnGgpStreamDescriptorSurface::EMPTY,
            khr_android_surface: InstanceFnKhrAndroidSurface::EMPTY,
            khr_display: InstanceFnKhrDisplay::EMPTY,
            khr_surface: InstanceFnKhrSurface::EMPTY,
            khr_wayland_surface: InstanceFnKhrWaylandSurface::EMPTY,
            khr_win32_surface: InstanceFnKhrWin32Surface::EMPTY,
            khr_xcb_surface: InstanceFnKhrXcbSurface::EMPTY,
            khr_xlib_surface: InstanceFnKhrXlibSurface::EMPTY,
            mvk_ios_surface: InstanceFnMvkIosSurface::EMPTY,
            mvk_macos_surface: InstanceFnMvkMacosSurface::EMPTY,
            nn_vi_surface: InstanceFnNnViSurface::EMPTY,
            ohos_surface: InstanceFnOhosSurface::EMPTY,
            qnx_screen_surface: InstanceFnQnxScreenSurface::EMPTY,
            sec_ubm_surface: InstanceFnSecUbmSurface::EMPTY,
        };
        for &ext in extensions {
            match unsafe { CStr::from_ptr(ext) }.to_bytes() {
                b"VK_KHR_surface" => {
                    out.khr_surface.destroy_surface_khr = to_option(loader(c"vkDestroySurfaceKHR"));
                }
                b"VK_KHR_display" => {
                    out.khr_display.create_display_plane_surface_khr =
                        to_option(loader(c"vkCreateDisplayPlaneSurfaceKHR"));
                }
                b"VK_KHR_xlib_surface" => {
                    out.khr_xlib_surface.create_xlib_surface_khr =
                        to_option(loader(c"vkCreateXlibSurfaceKHR"));
                }
                b"VK_KHR_xcb_surface" => {
                    out.khr_xcb_surface.create_xcb_surface_khr =
                        to_option(loader(c"vkCreateXcbSurfaceKHR"));
                }
                b"VK_KHR_wayland_surface" => {
                    out.khr_wayland_surface.create_wayland_surface_khr =
                        to_option(loader(c"vkCreateWaylandSurfaceKHR"));
                }
                b"VK_KHR_android_surface" => {
                    out.khr_android_surface.create_android_surface_khr =
                        to_option(loader(c"vkCreateAndroidSurfaceKHR"));
                }
                b"VK_KHR_win32_surface" => {
                    out.khr_win32_surface.create_win32_surface_khr =
                        to_option(loader(c"vkCreateWin32SurfaceKHR"));
                }
                b"VK_EXT_debug_report" => {
                    out.ext_debug_report.create_debug_report_callback_ext =
                        to_option(loader(c"vkCreateDebugReportCallbackEXT"));
                    out.ext_debug_report.destroy_debug_report_callback_ext =
                        to_option(loader(c"vkDestroyDebugReportCallbackEXT"));
                    out.ext_debug_report.debug_report_message_ext =
                        to_option(loader(c"vkDebugReportMessageEXT"));
                }
                b"VK_GGP_stream_descriptor_surface" => {
                    out.ggp_stream_descriptor_surface
                        .create_stream_descriptor_surface_ggp =
                        to_option(loader(c"vkCreateStreamDescriptorSurfaceGGP"));
                }
                b"VK_NN_vi_surface" => {
                    out.nn_vi_surface.create_vi_surface_nn =
                        to_option(loader(c"vkCreateViSurfaceNN"));
                }
                b"VK_KHR_device_group_creation" => {
                    if out.v1_1.enumerate_physical_device_groups.is_none() {
                        out.v1_1.enumerate_physical_device_groups =
                            to_option(loader(c"vkEnumeratePhysicalDeviceGroupsKHR"));
                    }
                }
                b"VK_MVK_ios_surface" => {
                    out.mvk_ios_surface.create_ios_surface_mvk =
                        to_option(loader(c"vkCreateIOSSurfaceMVK"));
                }
                b"VK_MVK_macos_surface" => {
                    out.mvk_macos_surface.create_mac_os_surface_mvk =
                        to_option(loader(c"vkCreateMacOSSurfaceMVK"));
                }
                b"VK_EXT_debug_utils" => {
                    out.ext_debug_utils.create_debug_utils_messenger_ext =
                        to_option(loader(c"vkCreateDebugUtilsMessengerEXT"));
                    out.ext_debug_utils.destroy_debug_utils_messenger_ext =
                        to_option(loader(c"vkDestroyDebugUtilsMessengerEXT"));
                    out.ext_debug_utils.submit_debug_utils_message_ext =
                        to_option(loader(c"vkSubmitDebugUtilsMessageEXT"));
                }
                b"VK_FUCHSIA_imagepipe_surface" => {
                    out.fuchsia_imagepipe_surface
                        .create_image_pipe_surface_fuchsia =
                        to_option(loader(c"vkCreateImagePipeSurfaceFUCHSIA"));
                }
                b"VK_EXT_metal_surface" => {
                    out.ext_metal_surface.create_metal_surface_ext =
                        to_option(loader(c"vkCreateMetalSurfaceEXT"));
                }
                b"VK_EXT_headless_surface" => {
                    out.ext_headless_surface.create_headless_surface_ext =
                        to_option(loader(c"vkCreateHeadlessSurfaceEXT"));
                }
                b"VK_EXT_directfb_surface" => {
                    out.ext_directfb_surface.create_direct_fb_surface_ext =
                        to_option(loader(c"vkCreateDirectFBSurfaceEXT"));
                }
                b"VK_QNX_screen_surface" => {
                    out.qnx_screen_surface.create_screen_surface_qnx =
                        to_option(loader(c"vkCreateScreenSurfaceQNX"));
                }
                b"VK_OHOS_surface" => {
                    out.ohos_surface.create_surface_ohos =
                        to_option(loader(c"vkCreateSurfaceOHOS"));
                }
                b"VK_SEC_ubm_surface" => {
                    out.sec_ubm_surface.create_ubm_surface_sec =
                        to_option(loader(c"vkCreateUbmSurfaceSEC"));
                }
                _ => (),
            }
        }
        out
    }
}

#[derive(Clone, Default)]
pub struct InstanceFnv1_0 {
    pub destroy_instance: Option<vkDestroyInstance>,
    pub enumerate_physical_devices: Option<vkEnumeratePhysicalDevices>,
}

impl InstanceFnv1_0 {
    pub const EMPTY: Self = Self {
        destroy_instance: None,
        enumerate_physical_devices: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            destroy_instance: to_option(loader(c"vkDestroyInstance")),
            enumerate_physical_devices: to_option(loader(c"vkEnumeratePhysicalDevices")),
        }
    }
}

#[derive(Clone, Default)]
pub struct InstanceFnv1_1 {
    pub enumerate_physical_device_groups: Option<vkEnumeratePhysicalDeviceGroups>,
}

impl InstanceFnv1_1 {
    pub const EMPTY: Self = Self {
        enumerate_physical_device_groups: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            enumerate_physical_device_groups: to_option(loader(c"vkEnumeratePhysicalDeviceGroups")),
        }
    }
}

#[derive(Clone, Default)]
pub struct InstanceFnExtDebugReport {
    pub create_debug_report_callback_ext: Option<vkCreateDebugReportCallbackEXT>,
    pub destroy_debug_report_callback_ext: Option<vkDestroyDebugReportCallbackEXT>,
    pub debug_report_message_ext: Option<vkDebugReportMessageEXT>,
}

impl InstanceFnExtDebugReport {
    pub const EMPTY: Self = Self {
        create_debug_report_callback_ext: None,
        destroy_debug_report_callback_ext: None,
        debug_report_message_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnExtDebugUtils {
    pub create_debug_utils_messenger_ext: Option<vkCreateDebugUtilsMessengerEXT>,
    pub destroy_debug_utils_messenger_ext: Option<vkDestroyDebugUtilsMessengerEXT>,
    pub submit_debug_utils_message_ext: Option<vkSubmitDebugUtilsMessageEXT>,
}

impl InstanceFnExtDebugUtils {
    pub const EMPTY: Self = Self {
        create_debug_utils_messenger_ext: None,
        destroy_debug_utils_messenger_ext: None,
        submit_debug_utils_message_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnExtDirectfbSurface {
    pub create_direct_fb_surface_ext: Option<vkCreateDirectFBSurfaceEXT>,
}

impl InstanceFnExtDirectfbSurface {
    pub const EMPTY: Self = Self {
        create_direct_fb_surface_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnExtHeadlessSurface {
    pub create_headless_surface_ext: Option<vkCreateHeadlessSurfaceEXT>,
}

impl InstanceFnExtHeadlessSurface {
    pub const EMPTY: Self = Self {
        create_headless_surface_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnExtMetalSurface {
    pub create_metal_surface_ext: Option<vkCreateMetalSurfaceEXT>,
}

impl InstanceFnExtMetalSurface {
    pub const EMPTY: Self = Self {
        create_metal_surface_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnFuchsiaImagepipeSurface {
    pub create_image_pipe_surface_fuchsia: Option<vkCreateImagePipeSurfaceFUCHSIA>,
}

impl InstanceFnFuchsiaImagepipeSurface {
    pub const EMPTY: Self = Self {
        create_image_pipe_surface_fuchsia: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnGgpStreamDescriptorSurface {
    pub create_stream_descriptor_surface_ggp: Option<vkCreateStreamDescriptorSurfaceGGP>,
}

impl InstanceFnGgpStreamDescriptorSurface {
    pub const EMPTY: Self = Self {
        create_stream_descriptor_surface_ggp: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrAndroidSurface {
    pub create_android_surface_khr: Option<vkCreateAndroidSurfaceKHR>,
}

impl InstanceFnKhrAndroidSurface {
    pub const EMPTY: Self = Self {
        create_android_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrDisplay {
    pub create_display_plane_surface_khr: Option<vkCreateDisplayPlaneSurfaceKHR>,
}

impl InstanceFnKhrDisplay {
    pub const EMPTY: Self = Self {
        create_display_plane_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrSurface {
    pub destroy_surface_khr: Option<vkDestroySurfaceKHR>,
}

impl InstanceFnKhrSurface {
    pub const EMPTY: Self = Self {
        destroy_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrWaylandSurface {
    pub create_wayland_surface_khr: Option<vkCreateWaylandSurfaceKHR>,
}

impl InstanceFnKhrWaylandSurface {
    pub const EMPTY: Self = Self {
        create_wayland_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrWin32Surface {
    pub create_win32_surface_khr: Option<vkCreateWin32SurfaceKHR>,
}

impl InstanceFnKhrWin32Surface {
    pub const EMPTY: Self = Self {
        create_win32_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrXcbSurface {
    pub create_xcb_surface_khr: Option<vkCreateXcbSurfaceKHR>,
}

impl InstanceFnKhrXcbSurface {
    pub const EMPTY: Self = Self {
        create_xcb_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnKhrXlibSurface {
    pub create_xlib_surface_khr: Option<vkCreateXlibSurfaceKHR>,
}

impl InstanceFnKhrXlibSurface {
    pub const EMPTY: Self = Self {
        create_xlib_surface_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnMvkIosSurface {
    pub create_ios_surface_mvk: Option<vkCreateIOSSurfaceMVK>,
}

impl InstanceFnMvkIosSurface {
    pub const EMPTY: Self = Self {
        create_ios_surface_mvk: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnMvkMacosSurface {
    pub create_mac_os_surface_mvk: Option<vkCreateMacOSSurfaceMVK>,
}

impl InstanceFnMvkMacosSurface {
    pub const EMPTY: Self = Self {
        create_mac_os_surface_mvk: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnNnViSurface {
    pub create_vi_surface_nn: Option<vkCreateViSurfaceNN>,
}

impl InstanceFnNnViSurface {
    pub const EMPTY: Self = Self {
        create_vi_surface_nn: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnOhosSurface {
    pub create_surface_ohos: Option<vkCreateSurfaceOHOS>,
}

impl InstanceFnOhosSurface {
    pub const EMPTY: Self = Self {
        create_surface_ohos: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnQnxScreenSurface {
    pub create_screen_surface_qnx: Option<vkCreateScreenSurfaceQNX>,
}

impl InstanceFnQnxScreenSurface {
    pub const EMPTY: Self = Self {
        create_screen_surface_qnx: None,
    };
}

#[derive(Clone, Default)]
pub struct InstanceFnSecUbmSurface {
    pub create_ubm_surface_sec: Option<vkCreateUbmSurfaceSEC>,
}

impl InstanceFnSecUbmSurface {
    pub const EMPTY: Self = Self {
        create_ubm_surface_sec: None,
    };
}

#[derive(Clone)]
pub struct PhysicalDeviceFn {
    pub v1_0: PhysicalDeviceFnv1_0,
    pub v1_1: PhysicalDeviceFnv1_1,
    pub v1_3: PhysicalDeviceFnv1_3,
    pub arm_data_graph: PhysicalDeviceFnArmDataGraph,
    pub arm_data_graph_optical_flow: PhysicalDeviceFnArmDataGraphOpticalFlow,
    pub arm_performance_counters_by_region: PhysicalDeviceFnArmPerformanceCountersByRegion,
    pub arm_shader_instrumentation: PhysicalDeviceFnArmShaderInstrumentation,
    pub arm_tensors: PhysicalDeviceFnArmTensors,
    pub ext_acquire_drm_display: PhysicalDeviceFnExtAcquireDrmDisplay,
    pub ext_acquire_xlib_display: PhysicalDeviceFnExtAcquireXlibDisplay,
    pub ext_descriptor_heap: PhysicalDeviceFnExtDescriptorHeap,
    pub ext_direct_mode_display: PhysicalDeviceFnExtDirectModeDisplay,
    pub ext_directfb_surface: PhysicalDeviceFnExtDirectfbSurface,
    pub ext_display_surface_counter: PhysicalDeviceFnExtDisplaySurfaceCounter,
    pub ext_full_screen_exclusive: PhysicalDeviceFnExtFullScreenExclusive,
    pub ext_sample_locations: PhysicalDeviceFnExtSampleLocations,
    pub khr_calibrated_timestamps: PhysicalDeviceFnKhrCalibratedTimestamps,
    pub khr_cooperative_matrix: PhysicalDeviceFnKhrCooperativeMatrix,
    pub khr_device_group: PhysicalDeviceFnKhrDeviceGroup,
    pub khr_display: PhysicalDeviceFnKhrDisplay,
    pub khr_fragment_shading_rate: PhysicalDeviceFnKhrFragmentShadingRate,
    pub khr_get_display_properties2: PhysicalDeviceFnKhrGetDisplayProperties2,
    pub khr_get_surface_capabilities2: PhysicalDeviceFnKhrGetSurfaceCapabilities2,
    pub khr_object_refresh: PhysicalDeviceFnKhrObjectRefresh,
    pub khr_performance_query: PhysicalDeviceFnKhrPerformanceQuery,
    pub khr_surface: PhysicalDeviceFnKhrSurface,
    pub khr_video_encode_queue: PhysicalDeviceFnKhrVideoEncodeQueue,
    pub khr_video_queue: PhysicalDeviceFnKhrVideoQueue,
    pub khr_wayland_surface: PhysicalDeviceFnKhrWaylandSurface,
    pub khr_win32_surface: PhysicalDeviceFnKhrWin32Surface,
    pub khr_xcb_surface: PhysicalDeviceFnKhrXcbSurface,
    pub khr_xlib_surface: PhysicalDeviceFnKhrXlibSurface,
    pub nv_acquire_winrt_display: PhysicalDeviceFnNvAcquireWinrtDisplay,
    pub nv_cooperative_matrix: PhysicalDeviceFnNvCooperativeMatrix,
    pub nv_cooperative_matrix2: PhysicalDeviceFnNvCooperativeMatrix2,
    pub nv_cooperative_vector: PhysicalDeviceFnNvCooperativeVector,
    pub nv_coverage_reduction_mode: PhysicalDeviceFnNvCoverageReductionMode,
    pub nv_external_memory_capabilities: PhysicalDeviceFnNvExternalMemoryCapabilities,
    pub nv_external_memory_sci_buf: PhysicalDeviceFnNvExternalMemorySciBuf,
    pub nv_external_sci_sync2: PhysicalDeviceFnNvExternalSciSync2,
    pub nv_optical_flow: PhysicalDeviceFnNvOpticalFlow,
    pub qnx_screen_surface: PhysicalDeviceFnQnxScreenSurface,
    pub sec_ubm_surface: PhysicalDeviceFnSecUbmSurface,
}

impl PhysicalDeviceFn {
    /// A table with no functions loaded; every call through it panics.
    pub const EMPTY: Self = Self {
        v1_0: PhysicalDeviceFnv1_0::EMPTY,
        v1_1: PhysicalDeviceFnv1_1::EMPTY,
        v1_3: PhysicalDeviceFnv1_3::EMPTY,
        arm_data_graph: PhysicalDeviceFnArmDataGraph::EMPTY,
        arm_data_graph_optical_flow: PhysicalDeviceFnArmDataGraphOpticalFlow::EMPTY,
        arm_performance_counters_by_region: PhysicalDeviceFnArmPerformanceCountersByRegion::EMPTY,
        arm_shader_instrumentation: PhysicalDeviceFnArmShaderInstrumentation::EMPTY,
        arm_tensors: PhysicalDeviceFnArmTensors::EMPTY,
        ext_acquire_drm_display: PhysicalDeviceFnExtAcquireDrmDisplay::EMPTY,
        ext_acquire_xlib_display: PhysicalDeviceFnExtAcquireXlibDisplay::EMPTY,
        ext_descriptor_heap: PhysicalDeviceFnExtDescriptorHeap::EMPTY,
        ext_direct_mode_display: PhysicalDeviceFnExtDirectModeDisplay::EMPTY,
        ext_directfb_surface: PhysicalDeviceFnExtDirectfbSurface::EMPTY,
        ext_display_surface_counter: PhysicalDeviceFnExtDisplaySurfaceCounter::EMPTY,
        ext_full_screen_exclusive: PhysicalDeviceFnExtFullScreenExclusive::EMPTY,
        ext_sample_locations: PhysicalDeviceFnExtSampleLocations::EMPTY,
        khr_calibrated_timestamps: PhysicalDeviceFnKhrCalibratedTimestamps::EMPTY,
        khr_cooperative_matrix: PhysicalDeviceFnKhrCooperativeMatrix::EMPTY,
        khr_device_group: PhysicalDeviceFnKhrDeviceGroup::EMPTY,
        khr_display: PhysicalDeviceFnKhrDisplay::EMPTY,
        khr_fragment_shading_rate: PhysicalDeviceFnKhrFragmentShadingRate::EMPTY,
        khr_get_display_properties2: PhysicalDeviceFnKhrGetDisplayProperties2::EMPTY,
        khr_get_surface_capabilities2: PhysicalDeviceFnKhrGetSurfaceCapabilities2::EMPTY,
        khr_object_refresh: PhysicalDeviceFnKhrObjectRefresh::EMPTY,
        khr_performance_query: PhysicalDeviceFnKhrPerformanceQuery::EMPTY,
        khr_surface: PhysicalDeviceFnKhrSurface::EMPTY,
        khr_video_encode_queue: PhysicalDeviceFnKhrVideoEncodeQueue::EMPTY,
        khr_video_queue: PhysicalDeviceFnKhrVideoQueue::EMPTY,
        khr_wayland_surface: PhysicalDeviceFnKhrWaylandSurface::EMPTY,
        khr_win32_surface: PhysicalDeviceFnKhrWin32Surface::EMPTY,
        khr_xcb_surface: PhysicalDeviceFnKhrXcbSurface::EMPTY,
        khr_xlib_surface: PhysicalDeviceFnKhrXlibSurface::EMPTY,
        nv_acquire_winrt_display: PhysicalDeviceFnNvAcquireWinrtDisplay::EMPTY,
        nv_cooperative_matrix: PhysicalDeviceFnNvCooperativeMatrix::EMPTY,
        nv_cooperative_matrix2: PhysicalDeviceFnNvCooperativeMatrix2::EMPTY,
        nv_cooperative_vector: PhysicalDeviceFnNvCooperativeVector::EMPTY,
        nv_coverage_reduction_mode: PhysicalDeviceFnNvCoverageReductionMode::EMPTY,
        nv_external_memory_capabilities: PhysicalDeviceFnNvExternalMemoryCapabilities::EMPTY,
        nv_external_memory_sci_buf: PhysicalDeviceFnNvExternalMemorySciBuf::EMPTY,
        nv_external_sci_sync2: PhysicalDeviceFnNvExternalSciSync2::EMPTY,
        nv_optical_flow: PhysicalDeviceFnNvOpticalFlow::EMPTY,
        qnx_screen_surface: PhysicalDeviceFnQnxScreenSurface::EMPTY,
        sec_ubm_surface: PhysicalDeviceFnSecUbmSurface::EMPTY,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        let mut out = Self {
            v1_0: PhysicalDeviceFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                PhysicalDeviceFnv1_1::load(&mut loader)
            } else {
                PhysicalDeviceFnv1_1::EMPTY
            },
            v1_3: if api_version >= API_VERSION_1_3 {
                PhysicalDeviceFnv1_3::load(&mut loader)
            } else {
                PhysicalDeviceFnv1_3::EMPTY
            },
            arm_data_graph: PhysicalDeviceFnArmDataGraph::EMPTY,
            arm_data_graph_optical_flow: PhysicalDeviceFnArmDataGraphOpticalFlow::EMPTY,
            arm_performance_counters_by_region:
                PhysicalDeviceFnArmPerformanceCountersByRegion::EMPTY,
            arm_shader_instrumentation: PhysicalDeviceFnArmShaderInstrumentation::EMPTY,
            arm_tensors: PhysicalDeviceFnArmTensors::EMPTY,
            ext_acquire_drm_display: PhysicalDeviceFnExtAcquireDrmDisplay::EMPTY,
            ext_acquire_xlib_display: PhysicalDeviceFnExtAcquireXlibDisplay::EMPTY,
            ext_descriptor_heap: PhysicalDeviceFnExtDescriptorHeap::EMPTY,
            ext_direct_mode_display: PhysicalDeviceFnExtDirectModeDisplay::EMPTY,
            ext_directfb_surface: PhysicalDeviceFnExtDirectfbSurface::EMPTY,
            ext_display_surface_counter: PhysicalDeviceFnExtDisplaySurfaceCounter::EMPTY,
            ext_full_screen_exclusive: PhysicalDeviceFnExtFullScreenExclusive::EMPTY,
            ext_sample_locations: PhysicalDeviceFnExtSampleLocations::EMPTY,
            khr_calibrated_timestamps: PhysicalDeviceFnKhrCalibratedTimestamps::EMPTY,
            khr_cooperative_matrix: PhysicalDeviceFnKhrCooperativeMatrix::EMPTY,
            khr_device_group: PhysicalDeviceFnKhrDeviceGroup::EMPTY,
            khr_display: PhysicalDeviceFnKhrDisplay::EMPTY,
            khr_fragment_shading_rate: PhysicalDeviceFnKhrFragmentShadingRate::EMPTY,
            khr_get_display_properties2: PhysicalDeviceFnKhrGetDisplayProperties2::EMPTY,
            khr_get_surface_capabilities2: PhysicalDeviceFnKhrGetSurfaceCapabilities2::EMPTY,
            khr_object_refresh: PhysicalDeviceFnKhrObjectRefresh::EMPTY,
            khr_performance_query: PhysicalDeviceFnKhrPerformanceQuery::EMPTY,
            khr_surface: PhysicalDeviceFnKhrSurface::EMPTY,
            khr_video_encode_queue: PhysicalDeviceFnKhrVideoEncodeQueue::EMPTY,
            khr_video_queue: PhysicalDeviceFnKhrVideoQueue::EMPTY,
            khr_wayland_surface: PhysicalDeviceFnKhrWaylandSurface::EMPTY,
            khr_win32_surface: PhysicalDeviceFnKhrWin32Surface::EMPTY,
            khr_xcb_surface: PhysicalDeviceFnKhrXcbSurface::EMPTY,
            khr_xlib_surface: PhysicalDeviceFnKhrXlibSurface::EMPTY,
            nv_acquire_winrt_display: PhysicalDeviceFnNvAcquireWinrtDisplay::EMPTY,
            nv_cooperative_matrix: PhysicalDeviceFnNvCooperativeMatrix::EMPTY,
            nv_cooperative_matrix2: PhysicalDeviceFnNvCooperativeMatrix2::EMPTY,
            nv_cooperative_vector: PhysicalDeviceFnNvCooperativeVector::EMPTY,
            nv_coverage_reduction_mode: PhysicalDeviceFnNvCoverageReductionMode::EMPTY,
            nv_external_memory_capabilities: PhysicalDeviceFnNvExternalMemoryCapabilities::EMPTY,
            nv_external_memory_sci_buf: PhysicalDeviceFnNvExternalMemorySciBuf::EMPTY,
            nv_external_sci_sync2: PhysicalDeviceFnNvExternalSciSync2::EMPTY,
            nv_optical_flow: PhysicalDeviceFnNvOpticalFlow::EMPTY,
            qnx_screen_surface: PhysicalDeviceFnQnxScreenSurface::EMPTY,
            sec_ubm_surface: PhysicalDeviceFnSecUbmSurface::EMPTY,
        };
        if out
            .khr_device_group
            .get_physical_device_present_rectangles_khr
            .is_none()
        {
            out.khr_device_group
                .get_physical_device_present_rectangles_khr =
                to_option(loader(c"vkGetPhysicalDevicePresentRectanglesKHR"));
        }
        out.khr_video_queue
            .get_physical_device_video_capabilities_khr =
            to_option(loader(c"vkGetPhysicalDeviceVideoCapabilitiesKHR"));
        out.khr_video_queue
            .get_physical_device_video_format_properties_khr =
            to_option(loader(c"vkGetPhysicalDeviceVideoFormatPropertiesKHR"));
        if out
            .khr_device_group
            .get_physical_device_present_rectangles_khr
            .is_none()
        {
            out.khr_device_group
                .get_physical_device_present_rectangles_khr =
                to_option(loader(c"vkGetPhysicalDevicePresentRectanglesKHR"));
        }
        out.khr_performance_query
            .enumerate_physical_device_queue_family_performance_query_counters_khr = to_option(
            loader(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR"),
        );
        out.khr_performance_query
            .get_physical_device_queue_family_performance_query_passes_khr = to_option(loader(
            c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
        ));
        out.ext_descriptor_heap
            .get_physical_device_descriptor_size_ext =
            to_option(loader(c"vkGetPhysicalDeviceDescriptorSizeEXT"));
        out.ext_sample_locations
            .get_physical_device_multisample_properties_ext =
            to_option(loader(c"vkGetPhysicalDeviceMultisamplePropertiesEXT"));
        if out
            .khr_calibrated_timestamps
            .get_physical_device_calibrateable_time_domains_khr
            .is_none()
        {
            out.khr_calibrated_timestamps
                .get_physical_device_calibrateable_time_domains_khr =
                to_option(loader(c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT"));
        }
        out.khr_fragment_shading_rate
            .get_physical_device_fragment_shading_rates_khr =
            to_option(loader(c"vkGetPhysicalDeviceFragmentShadingRatesKHR"));
        if out.v1_3.get_physical_device_tool_properties.is_none() {
            out.v1_3.get_physical_device_tool_properties =
                to_option(loader(c"vkGetPhysicalDeviceToolPropertiesEXT"));
        }
        out.nv_cooperative_matrix
            .get_physical_device_cooperative_matrix_properties_nv =
            to_option(loader(c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV"));
        out.nv_coverage_reduction_mode
            .get_physical_device_supported_framebuffer_mixed_samples_combinations_nv = to_option(
            loader(c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV"),
        );
        out.ext_full_screen_exclusive
            .get_physical_device_surface_present_modes2_ext =
            to_option(loader(c"vkGetPhysicalDeviceSurfacePresentModes2EXT"));
        out.khr_video_encode_queue
            .get_physical_device_video_encode_quality_level_properties_khr = to_option(loader(
            c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR",
        ));
        out.khr_object_refresh
            .get_physical_device_refreshable_object_types_khr =
            to_option(loader(c"vkGetPhysicalDeviceRefreshableObjectTypesKHR"));
        out.nv_acquire_winrt_display.acquire_winrt_display_nv =
            to_option(loader(c"vkAcquireWinrtDisplayNV"));
        out.nv_acquire_winrt_display.get_winrt_display_nv =
            to_option(loader(c"vkGetWinrtDisplayNV"));
        if out
            .nv_external_sci_sync2
            .get_physical_device_sci_sync_attributes_nv
            .is_none()
        {
            out.nv_external_sci_sync2
                .get_physical_device_sci_sync_attributes_nv =
                to_option(loader(c"vkGetPhysicalDeviceSciSyncAttributesNV"));
        }
        out.nv_external_memory_sci_buf
            .get_physical_device_external_memory_sci_buf_properties_nv = to_option(loader(
            c"vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV",
        ));
        out.nv_external_memory_sci_buf
            .get_physical_device_sci_buf_attributes_nv =
            to_option(loader(c"vkGetPhysicalDeviceSciBufAttributesNV"));
        out.arm_tensors
            .get_physical_device_external_tensor_properties_arm =
            to_option(loader(c"vkGetPhysicalDeviceExternalTensorPropertiesARM"));
        out.nv_optical_flow
            .get_physical_device_optical_flow_image_formats_nv =
            to_option(loader(c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV"));
        if out
            .nv_external_sci_sync2
            .get_physical_device_sci_sync_attributes_nv
            .is_none()
        {
            out.nv_external_sci_sync2
                .get_physical_device_sci_sync_attributes_nv =
                to_option(loader(c"vkGetPhysicalDeviceSciSyncAttributesNV"));
        }
        out.nv_cooperative_vector
            .get_physical_device_cooperative_vector_properties_nv =
            to_option(loader(c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV"));
        out.khr_cooperative_matrix
            .get_physical_device_cooperative_matrix_properties_khr =
            to_option(loader(c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR"));
        out.arm_data_graph
            .get_physical_device_queue_family_data_graph_properties_arm = to_option(loader(
            c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
        ));
        out.arm_data_graph
            .get_physical_device_queue_family_data_graph_processing_engine_properties_arm =
            to_option(loader(
                c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM",
            ));
        if out
            .arm_data_graph_optical_flow
            .get_physical_device_queue_family_data_graph_engine_operation_properties_arm
            .is_none()
        {
            out.arm_data_graph_optical_flow
                .get_physical_device_queue_family_data_graph_engine_operation_properties_arm =
                to_option(loader(
                    c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM",
                ));
        }
        if out
            .khr_calibrated_timestamps
            .get_physical_device_calibrateable_time_domains_khr
            .is_none()
        {
            out.khr_calibrated_timestamps
                .get_physical_device_calibrateable_time_domains_khr =
                to_option(loader(c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR"));
        }
        out.nv_cooperative_matrix2
            .get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv = to_option(
            loader(c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV"),
        );
        out.arm_performance_counters_by_region
            .enumerate_physical_device_queue_family_performance_counters_by_region_arm = to_option(
            loader(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM"),
        );
        out.arm_shader_instrumentation
            .enumerate_physical_device_shader_instrumentation_metrics_arm = to_option(loader(
            c"vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM",
        ));
        out.arm_data_graph_optical_flow
            .get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm = to_option(
            loader(c"vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM"),
        );
        if out
            .arm_data_graph_optical_flow
            .get_physical_device_queue_family_data_graph_engine_operation_properties_arm
            .is_none()
        {
            out.arm_data_graph_optical_flow
                .get_physical_device_queue_family_data_graph_engine_operation_properties_arm =
                to_option(loader(
                    c"vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM",
                ));
        }
        for &ext in extensions {
            match unsafe { CStr::from_ptr(ext) }.to_bytes() {
                b"VK_KHR_surface" => {
                    out.khr_surface.get_physical_device_surface_support_khr =
                        to_option(loader(c"vkGetPhysicalDeviceSurfaceSupportKHR"));
                    out.khr_surface.get_physical_device_surface_capabilities_khr =
                        to_option(loader(c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR"));
                    out.khr_surface.get_physical_device_surface_formats_khr =
                        to_option(loader(c"vkGetPhysicalDeviceSurfaceFormatsKHR"));
                    out.khr_surface
                        .get_physical_device_surface_present_modes_khr =
                        to_option(loader(c"vkGetPhysicalDeviceSurfacePresentModesKHR"));
                }
                b"VK_KHR_display" => {
                    out.khr_display.get_physical_device_display_properties_khr =
                        to_option(loader(c"vkGetPhysicalDeviceDisplayPropertiesKHR"));
                    out.khr_display
                        .get_physical_device_display_plane_properties_khr =
                        to_option(loader(c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR"));
                    out.khr_display.get_display_plane_supported_displays_khr =
                        to_option(loader(c"vkGetDisplayPlaneSupportedDisplaysKHR"));
                    out.khr_display.get_display_mode_properties_khr =
                        to_option(loader(c"vkGetDisplayModePropertiesKHR"));
                    out.khr_display.create_display_mode_khr =
                        to_option(loader(c"vkCreateDisplayModeKHR"));
                    out.khr_display.get_display_plane_capabilities_khr =
                        to_option(loader(c"vkGetDisplayPlaneCapabilitiesKHR"));
                }
                b"VK_KHR_xlib_surface" => {
                    out.khr_xlib_surface
                        .get_physical_device_xlib_presentation_support_khr =
                        to_option(loader(c"vkGetPhysicalDeviceXlibPresentationSupportKHR"));
                }
                b"VK_KHR_xcb_surface" => {
                    out.khr_xcb_surface
                        .get_physical_device_xcb_presentation_support_khr =
                        to_option(loader(c"vkGetPhysicalDeviceXcbPresentationSupportKHR"));
                }
                b"VK_KHR_wayland_surface" => {
                    out.khr_wayland_surface
                        .get_physical_device_wayland_presentation_support_khr =
                        to_option(loader(c"vkGetPhysicalDeviceWaylandPresentationSupportKHR"));
                }
                b"VK_KHR_win32_surface" => {
                    out.khr_win32_surface
                        .get_physical_device_win32_presentation_support_khr =
                        to_option(loader(c"vkGetPhysicalDeviceWin32PresentationSupportKHR"));
                }
                b"VK_NV_external_memory_capabilities" => {
                    out.nv_external_memory_capabilities
                        .get_physical_device_external_image_format_properties_nv = to_option(
                        loader(c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV"),
                    );
                }
                b"VK_KHR_get_physical_device_properties2" => {
                    if out.v1_1.get_physical_device_features2.is_none() {
                        out.v1_1.get_physical_device_features2 =
                            to_option(loader(c"vkGetPhysicalDeviceFeatures2KHR"));
                    }
                    if out.v1_1.get_physical_device_properties2.is_none() {
                        out.v1_1.get_physical_device_properties2 =
                            to_option(loader(c"vkGetPhysicalDeviceProperties2KHR"));
                    }
                    if out.v1_1.get_physical_device_format_properties2.is_none() {
                        out.v1_1.get_physical_device_format_properties2 =
                            to_option(loader(c"vkGetPhysicalDeviceFormatProperties2KHR"));
                    }
                    if out
                        .v1_1
                        .get_physical_device_image_format_properties2
                        .is_none()
                    {
                        out.v1_1.get_physical_device_image_format_properties2 =
                            to_option(loader(c"vkGetPhysicalDeviceImageFormatProperties2KHR"));
                    }
                    if out
                        .v1_1
                        .get_physical_device_queue_family_properties2
                        .is_none()
                    {
                        out.v1_1.get_physical_device_queue_family_properties2 =
                            to_option(loader(c"vkGetPhysicalDeviceQueueFamilyProperties2KHR"));
                    }
                    if out.v1_1.get_physical_device_memory_properties2.is_none() {
                        out.v1_1.get_physical_device_memory_properties2 =
                            to_option(loader(c"vkGetPhysicalDeviceMemoryProperties2KHR"));
                    }
                    if out
                        .v1_1
                        .get_physical_device_sparse_image_format_properties2
                        .is_none()
                    {
                        out.v1_1.get_physical_device_sparse_image_format_properties2 = to_option(
                            loader(c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR"),
                        );
                    }
                }
                b"VK_KHR_external_memory_capabilities" => {
                    if out
                        .v1_1
                        .get_physical_device_external_buffer_properties
                        .is_none()
                    {
                        out.v1_1.get_physical_device_external_buffer_properties =
                            to_option(loader(c"vkGetPhysicalDeviceExternalBufferPropertiesKHR"));
                    }
                }
                b"VK_KHR_external_semaphore_capabilities" => {
                    if out
                        .v1_1
                        .get_physical_device_external_semaphore_properties
                        .is_none()
                    {
                        out.v1_1.get_physical_device_external_semaphore_properties =
                            to_option(loader(c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR"));
                    }
                }
                b"VK_EXT_direct_mode_display" => {
                    out.ext_direct_mode_display.release_display_ext =
                        to_option(loader(c"vkReleaseDisplayEXT"));
                }
                b"VK_EXT_acquire_xlib_display" => {
                    out.ext_acquire_xlib_display.acquire_xlib_display_ext =
                        to_option(loader(c"vkAcquireXlibDisplayEXT"));
                    out.ext_acquire_xlib_display.get_rand_r_output_display_ext =
                        to_option(loader(c"vkGetRandROutputDisplayEXT"));
                }
                b"VK_EXT_display_surface_counter" => {
                    out.ext_display_surface_counter
                        .get_physical_device_surface_capabilities2_ext =
                        to_option(loader(c"vkGetPhysicalDeviceSurfaceCapabilities2EXT"));
                }
                b"VK_KHR_external_fence_capabilities" => {
                    if out
                        .v1_1
                        .get_physical_device_external_fence_properties
                        .is_none()
                    {
                        out.v1_1.get_physical_device_external_fence_properties =
                            to_option(loader(c"vkGetPhysicalDeviceExternalFencePropertiesKHR"));
                    }
                }
                b"VK_KHR_get_surface_capabilities2" => {
                    out.khr_get_surface_capabilities2
                        .get_physical_device_surface_capabilities2_khr =
                        to_option(loader(c"vkGetPhysicalDeviceSurfaceCapabilities2KHR"));
                    out.khr_get_surface_capabilities2
                        .get_physical_device_surface_formats2_khr =
                        to_option(loader(c"vkGetPhysicalDeviceSurfaceFormats2KHR"));
                }
                b"VK_KHR_get_display_properties2" => {
                    out.khr_get_display_properties2
                        .get_physical_device_display_properties2_khr =
                        to_option(loader(c"vkGetPhysicalDeviceDisplayProperties2KHR"));
                    out.khr_get_display_properties2
                        .get_physical_device_display_plane_properties2_khr =
                        to_option(loader(c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR"));
                    out.khr_get_display_properties2
                        .get_display_mode_properties2_khr =
                        to_option(loader(c"vkGetDisplayModeProperties2KHR"));
                    out.khr_get_display_properties2
                        .get_display_plane_capabilities2_khr =
                        to_option(loader(c"vkGetDisplayPlaneCapabilities2KHR"));
                }
                b"VK_EXT_acquire_drm_display" => {
                    out.ext_acquire_drm_display.acquire_drm_display_ext =
                        to_option(loader(c"vkAcquireDrmDisplayEXT"));
                    out.ext_acquire_drm_display.get_drm_display_ext =
                        to_option(loader(c"vkGetDrmDisplayEXT"));
                }
                b"VK_EXT_directfb_surface" => {
                    out.ext_directfb_surface
                        .get_physical_device_direct_fb_presentation_support_ext =
                        to_option(loader(c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT"));
                }
                b"VK_QNX_screen_surface" => {
                    out.qnx_screen_surface
                        .get_physical_device_screen_presentation_support_qnx =
                        to_option(loader(c"vkGetPhysicalDeviceScreenPresentationSupportQNX"));
                }
                b"VK_SEC_ubm_surface" => {
                    out.sec_ubm_surface
                        .get_physical_device_ubm_presentation_support_sec =
                        to_option(loader(c"vkGetPhysicalDeviceUbmPresentationSupportSEC"));
                }
                _ => (),
            }
        }
        out
    }
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnv1_0 {
    pub get_physical_device_properties: Option<vkGetPhysicalDeviceProperties>,
    pub get_physical_device_queue_family_properties:
        Option<vkGetPhysicalDeviceQueueFamilyProperties>,
    pub get_physical_device_memory_properties: Option<vkGetPhysicalDeviceMemoryProperties>,
    pub get_physical_device_features: Option<vkGetPhysicalDeviceFeatures>,
    pub get_physical_device_format_properties: Option<vkGetPhysicalDeviceFormatProperties>,
    pub get_physical_device_image_format_properties:
        Option<vkGetPhysicalDeviceImageFormatProperties>,
    pub create_device: Option<vkCreateDevice>,
    pub enumerate_device_layer_properties: Option<vkEnumerateDeviceLayerProperties>,
    pub enumerate_device_extension_properties: Option<vkEnumerateDeviceExtensionProperties>,
    pub get_physical_device_sparse_image_format_properties:
        Option<vkGetPhysicalDeviceSparseImageFormatProperties>,
}

impl PhysicalDeviceFnv1_0 {
    pub const EMPTY: Self = Self {
        get_physical_device_properties: None,
        get_physical_device_queue_family_properties: None,
        get_physical_device_memory_properties: None,
        get_physical_device_features: None,
        get_physical_device_format_properties: None,
        get_physical_device_image_format_properties: None,
        create_device: None,
        enumerate_device_layer_properties: None,
        enumerate_device_extension_properties: None,
        get_physical_device_sparse_image_format_properties: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_properties: to_option(loader(c"vkGetPhysicalDeviceProperties")),
            get_physical_device_queue_family_properties: to_option(loader(
                c"vkGetPhysicalDeviceQueueFamilyProperties",
            )),
            get_physical_device_memory_properties: to_option(loader(
                c"vkGetPhysicalDeviceMemoryProperties",
            )),
            get_physical_device_features: to_option(loader(c"vkGetPhysicalDeviceFeatures")),
            get_physical_device_format_properties: to_option(loader(
                c"vkGetPhysicalDeviceFormatProperties",
            )),
            get_physical_device_image_format_properties: to_option(loader(
                c"vkGetPhysicalDeviceImageFormatProperties",
            )),
            create_device: to_option(loader(c"vkCreateDevice")),
            enumerate_device_layer_properties: to_option(loader(
                c"vkEnumerateDeviceLayerProperties",
            )),
            enumerate_device_extension_properties: to_option(loader(
                c"vkEnumerateDeviceExtensionProperties",
            )),
            get_physical_device_sparse_image_format_properties: to_option(loader(
                c"vkGetPhysicalDeviceSparseImageFormatProperties",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnv1_1 {
    pub get_physical_device_features2: Option<vkGetPhysicalDeviceFeatures2>,
    pub get_physical_device_properties2: Option<vkGetPhysicalDeviceProperties2>,
    pub get_physical_device_format_properties2: Option<vkGetPhysicalDeviceFormatProperties2>,
    pub get_physical_device_image_format_properties2:
        Option<vkGetPhysicalDeviceImageFormatProperties2>,
    pub get_physical_device_queue_family_properties2:
        Option<vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub get_physical_device_memory_properties2: Option<vkGetPhysicalDeviceMemoryProperties2>,
    pub get_physical_device_sparse_image_format_properties2:
        Option<vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub get_physical_device_external_buffer_properties:
        Option<vkGetPhysicalDeviceExternalBufferProperties>,
    pub get_physical_device_external_semaphore_properties:
        Option<vkGetPhysicalDeviceExternalSemaphoreProperties>,
    pub get_physical_device_external_fence_properties:
        Option<vkGetPhysicalDeviceExternalFenceProperties>,
}

impl PhysicalDeviceFnv1_1 {
    pub const EMPTY: Self = Self {
        get_physical_device_features2: None,
        get_physical_device_properties2: None,
        get_physical_device_format_properties2: None,
        get_physical_device_image_format_properties2: None,
        get_physical_device_queue_family_properties2: None,
        get_physical_device_memory_properties2: None,
        get_physical_device_sparse_image_format_properties2: None,
        get_physical_device_external_buffer_properties: None,
        get_physical_device_external_semaphore_properties: None,
        get_physical_device_external_fence_properties: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_features2: to_option(loader(c"vkGetPhysicalDeviceFeatures2")),
            get_physical_device_properties2: to_option(loader(c"vkGetPhysicalDeviceProperties2")),
            get_physical_device_format_properties2: to_option(loader(
                c"vkGetPhysicalDeviceFormatProperties2",
            )),
            get_physical_device_image_format_properties2: to_option(loader(
                c"vkGetPhysicalDeviceImageFormatProperties2",
            )),
            get_physical_device_queue_family_properties2: to_option(loader(
                c"vkGetPhysicalDeviceQueueFamilyProperties2",
            )),
            get_physical_device_memory_properties2: to_option(loader(
                c"vkGetPhysicalDeviceMemoryProperties2",
            )),
            get_physical_device_sparse_image_format_properties2: to_option(loader(
                c"vkGetPhysicalDeviceSparseImageFormatProperties2",
            )),
            get_physical_device_external_buffer_properties: to_option(loader(
                c"vkGetPhysicalDeviceExternalBufferProperties",
            )),
            get_physical_device_external_semaphore_properties: to_option(loader(
                c"vkGetPhysicalDeviceExternalSemaphoreProperties",
            )),
            get_physical_device_external_fence_properties: to_option(loader(
                c"vkGetPhysicalDeviceExternalFenceProperties",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnv1_3 {
    pub get_physical_device_tool_properties: Option<vkGetPhysicalDeviceToolProperties>,
}

impl PhysicalDeviceFnv1_3 {
    pub const EMPTY: Self = Self {
        get_physical_device_tool_properties: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_tool_properties: to_option(loader(
                c"vkGetPhysicalDeviceToolProperties",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnArmDataGraph {
    pub get_physical_device_queue_family_data_graph_properties_arm:
        Option<vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM>,
    pub get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
        Option<vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM>,
}

impl PhysicalDeviceFnArmDataGraph {
    pub const EMPTY: Self = Self {
        get_physical_device_queue_family_data_graph_properties_arm: None,
        get_physical_device_queue_family_data_graph_processing_engine_properties_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnArmDataGraphOpticalFlow {
    pub get_physical_device_queue_family_data_graph_engine_operation_properties_arm:
        Option<vkGetPhysicalDeviceQueueFamilyDataGraphEngineOperationPropertiesARM>,
    pub get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm:
        Option<vkGetPhysicalDeviceQueueFamilyDataGraphOpticalFlowImageFormatsARM>,
}

impl PhysicalDeviceFnArmDataGraphOpticalFlow {
    pub const EMPTY: Self = Self {
        get_physical_device_queue_family_data_graph_engine_operation_properties_arm: None,
        get_physical_device_queue_family_data_graph_optical_flow_image_formats_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnArmPerformanceCountersByRegion {
    pub enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        Option<vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM>,
}

impl PhysicalDeviceFnArmPerformanceCountersByRegion {
    pub const EMPTY: Self = Self {
        enumerate_physical_device_queue_family_performance_counters_by_region_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnArmShaderInstrumentation {
    pub enumerate_physical_device_shader_instrumentation_metrics_arm:
        Option<vkEnumeratePhysicalDeviceShaderInstrumentationMetricsARM>,
}

impl PhysicalDeviceFnArmShaderInstrumentation {
    pub const EMPTY: Self = Self {
        enumerate_physical_device_shader_instrumentation_metrics_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnArmTensors {
    pub get_physical_device_external_tensor_properties_arm:
        Option<vkGetPhysicalDeviceExternalTensorPropertiesARM>,
}

impl PhysicalDeviceFnArmTensors {
    pub const EMPTY: Self = Self {
        get_physical_device_external_tensor_properties_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtAcquireDrmDisplay {
    pub acquire_drm_display_ext: Option<vkAcquireDrmDisplayEXT>,
    pub get_drm_display_ext: Option<vkGetDrmDisplayEXT>,
}

impl PhysicalDeviceFnExtAcquireDrmDisplay {
    pub const EMPTY: Self = Self {
        acquire_drm_display_ext: None,
        get_drm_display_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtAcquireXlibDisplay {
    pub acquire_xlib_display_ext: Option<vkAcquireXlibDisplayEXT>,
    pub get_rand_r_output_display_ext: Option<vkGetRandROutputDisplayEXT>,
}

impl PhysicalDeviceFnExtAcquireXlibDisplay {
    pub const EMPTY: Self = Self {
        acquire_xlib_display_ext: None,
        get_rand_r_output_display_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtDescriptorHeap {
    pub get_physical_device_descriptor_size_ext: Option<vkGetPhysicalDeviceDescriptorSizeEXT>,
}

impl PhysicalDeviceFnExtDescriptorHeap {
    pub const EMPTY: Self = Self {
        get_physical_device_descriptor_size_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtDirectModeDisplay {
    pub release_display_ext: Option<vkReleaseDisplayEXT>,
}

impl PhysicalDeviceFnExtDirectModeDisplay {
    pub const EMPTY: Self = Self {
        release_display_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtDirectfbSurface {
    pub get_physical_device_direct_fb_presentation_support_ext:
        Option<vkGetPhysicalDeviceDirectFBPresentationSupportEXT>,
}

impl PhysicalDeviceFnExtDirectfbSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_direct_fb_presentation_support_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtDisplaySurfaceCounter {
    pub get_physical_device_surface_capabilities2_ext:
        Option<vkGetPhysicalDeviceSurfaceCapabilities2EXT>,
}

impl PhysicalDeviceFnExtDisplaySurfaceCounter {
    pub const EMPTY: Self = Self {
        get_physical_device_surface_capabilities2_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtFullScreenExclusive {
    pub get_physical_device_surface_present_modes2_ext:
        Option<vkGetPhysicalDeviceSurfacePresentModes2EXT>,
}

impl PhysicalDeviceFnExtFullScreenExclusive {
    pub const EMPTY: Self = Self {
        get_physical_device_surface_present_modes2_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnExtSampleLocations {
    pub get_physical_device_multisample_properties_ext:
        Option<vkGetPhysicalDeviceMultisamplePropertiesEXT>,
}

impl PhysicalDeviceFnExtSampleLocations {
    pub const EMPTY: Self = Self {
        get_physical_device_multisample_properties_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrCalibratedTimestamps {
    pub get_physical_device_calibrateable_time_domains_khr:
        Option<vkGetPhysicalDeviceCalibrateableTimeDomainsKHR>,
}

impl PhysicalDeviceFnKhrCalibratedTimestamps {
    pub const EMPTY: Self = Self {
        get_physical_device_calibrateable_time_domains_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrCooperativeMatrix {
    pub get_physical_device_cooperative_matrix_properties_khr:
        Option<vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR>,
}

impl PhysicalDeviceFnKhrCooperativeMatrix {
    pub const EMPTY: Self = Self {
        get_physical_device_cooperative_matrix_properties_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrDeviceGroup {
    pub get_physical_device_present_rectangles_khr: Option<vkGetPhysicalDevicePresentRectanglesKHR>,
}

impl PhysicalDeviceFnKhrDeviceGroup {
    pub const EMPTY: Self = Self {
        get_physical_device_present_rectangles_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrDisplay {
    pub get_physical_device_display_properties_khr: Option<vkGetPhysicalDeviceDisplayPropertiesKHR>,
    pub get_physical_device_display_plane_properties_khr:
        Option<vkGetPhysicalDeviceDisplayPlanePropertiesKHR>,
    pub get_display_plane_supported_displays_khr: Option<vkGetDisplayPlaneSupportedDisplaysKHR>,
    pub get_display_mode_properties_khr: Option<vkGetDisplayModePropertiesKHR>,
    pub create_display_mode_khr: Option<vkCreateDisplayModeKHR>,
    pub get_display_plane_capabilities_khr: Option<vkGetDisplayPlaneCapabilitiesKHR>,
}

impl PhysicalDeviceFnKhrDisplay {
    pub const EMPTY: Self = Self {
        get_physical_device_display_properties_khr: None,
        get_physical_device_display_plane_properties_khr: None,
        get_display_plane_supported_displays_khr: None,
        get_display_mode_properties_khr: None,
        create_display_mode_khr: None,
        get_display_plane_capabilities_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrFragmentShadingRate {
    pub get_physical_device_fragment_shading_rates_khr:
        Option<vkGetPhysicalDeviceFragmentShadingRatesKHR>,
}

impl PhysicalDeviceFnKhrFragmentShadingRate {
    pub const EMPTY: Self = Self {
        get_physical_device_fragment_shading_rates_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrGetDisplayProperties2 {
    pub get_physical_device_display_properties2_khr:
        Option<vkGetPhysicalDeviceDisplayProperties2KHR>,
    pub get_physical_device_display_plane_properties2_khr:
        Option<vkGetPhysicalDeviceDisplayPlaneProperties2KHR>,
    pub get_display_mode_properties2_khr: Option<vkGetDisplayModeProperties2KHR>,
    pub get_display_plane_capabilities2_khr: Option<vkGetDisplayPlaneCapabilities2KHR>,
}

impl PhysicalDeviceFnKhrGetDisplayProperties2 {
    pub const EMPTY: Self = Self {
        get_physical_device_display_properties2_khr: None,
        get_physical_device_display_plane_properties2_khr: None,
        get_display_mode_properties2_khr: None,
        get_display_plane_capabilities2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrGetSurfaceCapabilities2 {
    pub get_physical_device_surface_capabilities2_khr:
        Option<vkGetPhysicalDeviceSurfaceCapabilities2KHR>,
    pub get_physical_device_surface_formats2_khr: Option<vkGetPhysicalDeviceSurfaceFormats2KHR>,
}

impl PhysicalDeviceFnKhrGetSurfaceCapabilities2 {
    pub const EMPTY: Self = Self {
        get_physical_device_surface_capabilities2_khr: None,
        get_physical_device_surface_formats2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrObjectRefresh {
    pub get_physical_device_refreshable_object_types_khr:
        Option<vkGetPhysicalDeviceRefreshableObjectTypesKHR>,
}

impl PhysicalDeviceFnKhrObjectRefresh {
    pub const EMPTY: Self = Self {
        get_physical_device_refreshable_object_types_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrPerformanceQuery {
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        Option<vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR>,
    pub get_physical_device_queue_family_performance_query_passes_khr:
        Option<vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR>,
}

impl PhysicalDeviceFnKhrPerformanceQuery {
    pub const EMPTY: Self = Self {
        enumerate_physical_device_queue_family_performance_query_counters_khr: None,
        get_physical_device_queue_family_performance_query_passes_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrSurface {
    pub get_physical_device_surface_support_khr: Option<vkGetPhysicalDeviceSurfaceSupportKHR>,
    pub get_physical_device_surface_capabilities_khr:
        Option<vkGetPhysicalDeviceSurfaceCapabilitiesKHR>,
    pub get_physical_device_surface_formats_khr: Option<vkGetPhysicalDeviceSurfaceFormatsKHR>,
    pub get_physical_device_surface_present_modes_khr:
        Option<vkGetPhysicalDeviceSurfacePresentModesKHR>,
}

impl PhysicalDeviceFnKhrSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_surface_support_khr: None,
        get_physical_device_surface_capabilities_khr: None,
        get_physical_device_surface_formats_khr: None,
        get_physical_device_surface_present_modes_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrVideoEncodeQueue {
    pub get_physical_device_video_encode_quality_level_properties_khr:
        Option<vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR>,
}

impl PhysicalDeviceFnKhrVideoEncodeQueue {
    pub const EMPTY: Self = Self {
        get_physical_device_video_encode_quality_level_properties_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrVideoQueue {
    pub get_physical_device_video_capabilities_khr: Option<vkGetPhysicalDeviceVideoCapabilitiesKHR>,
    pub get_physical_device_video_format_properties_khr:
        Option<vkGetPhysicalDeviceVideoFormatPropertiesKHR>,
}

impl PhysicalDeviceFnKhrVideoQueue {
    pub const EMPTY: Self = Self {
        get_physical_device_video_capabilities_khr: None,
        get_physical_device_video_format_properties_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrWaylandSurface {
    pub get_physical_device_wayland_presentation_support_khr:
        Option<vkGetPhysicalDeviceWaylandPresentationSupportKHR>,
}

impl PhysicalDeviceFnKhrWaylandSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_wayland_presentation_support_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrWin32Surface {
    pub get_physical_device_win32_presentation_support_khr:
        Option<vkGetPhysicalDeviceWin32PresentationSupportKHR>,
}

impl PhysicalDeviceFnKhrWin32Surface {
    pub const EMPTY: Self = Self {
        get_physical_device_win32_presentation_support_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrXcbSurface {
    pub get_physical_device_xcb_presentation_support_khr:
        Option<vkGetPhysicalDeviceXcbPresentationSupportKHR>,
}

impl PhysicalDeviceFnKhrXcbSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_xcb_presentation_support_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnKhrXlibSurface {
    pub get_physical_device_xlib_presentation_support_khr:
        Option<vkGetPhysicalDeviceXlibPresentationSupportKHR>,
}

impl PhysicalDeviceFnKhrXlibSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_xlib_presentation_support_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvAcquireWinrtDisplay {
    pub acquire_winrt_display_nv: Option<vkAcquireWinrtDisplayNV>,
    pub get_winrt_display_nv: Option<vkGetWinrtDisplayNV>,
}

impl PhysicalDeviceFnNvAcquireWinrtDisplay {
    pub const EMPTY: Self = Self {
        acquire_winrt_display_nv: None,
        get_winrt_display_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvCooperativeMatrix {
    pub get_physical_device_cooperative_matrix_properties_nv:
        Option<vkGetPhysicalDeviceCooperativeMatrixPropertiesNV>,
}

impl PhysicalDeviceFnNvCooperativeMatrix {
    pub const EMPTY: Self = Self {
        get_physical_device_cooperative_matrix_properties_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvCooperativeMatrix2 {
    pub get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv:
        Option<vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV>,
}

impl PhysicalDeviceFnNvCooperativeMatrix2 {
    pub const EMPTY: Self = Self {
        get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvCooperativeVector {
    pub get_physical_device_cooperative_vector_properties_nv:
        Option<vkGetPhysicalDeviceCooperativeVectorPropertiesNV>,
}

impl PhysicalDeviceFnNvCooperativeVector {
    pub const EMPTY: Self = Self {
        get_physical_device_cooperative_vector_properties_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvCoverageReductionMode {
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        Option<vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV>,
}

impl PhysicalDeviceFnNvCoverageReductionMode {
    pub const EMPTY: Self = Self {
        get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvExternalMemoryCapabilities {
    pub get_physical_device_external_image_format_properties_nv:
        Option<vkGetPhysicalDeviceExternalImageFormatPropertiesNV>,
}

impl PhysicalDeviceFnNvExternalMemoryCapabilities {
    pub const EMPTY: Self = Self {
        get_physical_device_external_image_format_properties_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvExternalMemorySciBuf {
    pub get_physical_device_external_memory_sci_buf_properties_nv:
        Option<vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV>,
    pub get_physical_device_sci_buf_attributes_nv: Option<vkGetPhysicalDeviceSciBufAttributesNV>,
}

impl PhysicalDeviceFnNvExternalMemorySciBuf {
    pub const EMPTY: Self = Self {
        get_physical_device_external_memory_sci_buf_properties_nv: None,
        get_physical_device_sci_buf_attributes_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvExternalSciSync2 {
    pub get_physical_device_sci_sync_attributes_nv: Option<vkGetPhysicalDeviceSciSyncAttributesNV>,
}

impl PhysicalDeviceFnNvExternalSciSync2 {
    pub const EMPTY: Self = Self {
        get_physical_device_sci_sync_attributes_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnNvOpticalFlow {
    pub get_physical_device_optical_flow_image_formats_nv:
        Option<vkGetPhysicalDeviceOpticalFlowImageFormatsNV>,
}

impl PhysicalDeviceFnNvOpticalFlow {
    pub const EMPTY: Self = Self {
        get_physical_device_optical_flow_image_formats_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnQnxScreenSurface {
    pub get_physical_device_screen_presentation_support_qnx:
        Option<vkGetPhysicalDeviceScreenPresentationSupportQNX>,
}

impl PhysicalDeviceFnQnxScreenSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_screen_presentation_support_qnx: None,
    };
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnSecUbmSurface {
    pub get_physical_device_ubm_presentation_support_sec:
        Option<vkGetPhysicalDeviceUbmPresentationSupportSEC>,
}

impl PhysicalDeviceFnSecUbmSurface {
    pub const EMPTY: Self = Self {
        get_physical_device_ubm_presentation_support_sec: None,
    };
}

#[derive(Clone)]
pub struct DeviceFn {
    pub v1_0: DeviceFnv1_0,
    pub v1_1: DeviceFnv1_1,
    pub v1_2: DeviceFnv1_2,
    pub v1_3: DeviceFnv1_3,
    pub v1_4: DeviceFnv1_4,
    pub amd_anti_lag: DeviceFnAmdAntiLag,
    pub amd_display_native_hdr: DeviceFnAmdDisplayNativeHdr,
    pub amd_gpa_interface: DeviceFnAmdGpaInterface,
    pub amd_shader_info: DeviceFnAmdShaderInfo,
    pub amdx_shader_enqueue: DeviceFnAmdxShaderEnqueue,
    pub android_external_memory_android_hardware_buffer:
        DeviceFnAndroidExternalMemoryAndroidHardwareBuffer,
    pub arm_data_graph: DeviceFnArmDataGraph,
    pub arm_shader_instrumentation: DeviceFnArmShaderInstrumentation,
    pub arm_tensors: DeviceFnArmTensors,
    pub ext_debug_marker: DeviceFnExtDebugMarker,
    pub ext_debug_utils: DeviceFnExtDebugUtils,
    pub ext_descriptor_buffer: DeviceFnExtDescriptorBuffer,
    pub ext_descriptor_heap: DeviceFnExtDescriptorHeap,
    pub ext_device_fault: DeviceFnExtDeviceFault,
    pub ext_device_generated_commands: DeviceFnExtDeviceGeneratedCommands,
    pub ext_display_control: DeviceFnExtDisplayControl,
    pub ext_external_memory_host: DeviceFnExtExternalMemoryHost,
    pub ext_external_memory_metal: DeviceFnExtExternalMemoryMetal,
    pub ext_full_screen_exclusive: DeviceFnExtFullScreenExclusive,
    pub ext_hdr_metadata: DeviceFnExtHdrMetadata,
    pub ext_image_drm_format_modifier: DeviceFnExtImageDrmFormatModifier,
    pub ext_metal_objects: DeviceFnExtMetalObjects,
    pub ext_opacity_micromap: DeviceFnExtOpacityMicromap,
    pub ext_pageable_device_local_memory: DeviceFnExtPageableDeviceLocalMemory,
    pub ext_pipeline_properties: DeviceFnExtPipelineProperties,
    pub ext_present_timing: DeviceFnExtPresentTiming,
    pub ext_shader_module_identifier: DeviceFnExtShaderModuleIdentifier,
    pub ext_shader_object: DeviceFnExtShaderObject,
    pub ext_validation_cache: DeviceFnExtValidationCache,
    pub fuchsia_buffer_collection: DeviceFnFuchsiaBufferCollection,
    pub fuchsia_external_memory: DeviceFnFuchsiaExternalMemory,
    pub fuchsia_external_semaphore: DeviceFnFuchsiaExternalSemaphore,
    pub google_display_timing: DeviceFnGoogleDisplayTiming,
    pub huawei_subpass_shading: DeviceFnHuaweiSubpassShading,
    pub intel_performance_query: DeviceFnIntelPerformanceQuery,
    pub khr_acceleration_structure: DeviceFnKhrAccelerationStructure,
    pub khr_calibrated_timestamps: DeviceFnKhrCalibratedTimestamps,
    pub khr_deferred_host_operations: DeviceFnKhrDeferredHostOperations,
    pub khr_device_address_commands: DeviceFnKhrDeviceAddressCommands,
    pub khr_device_fault: DeviceFnKhrDeviceFault,
    pub khr_device_group: DeviceFnKhrDeviceGroup,
    pub khr_display_swapchain: DeviceFnKhrDisplaySwapchain,
    pub khr_external_fence_fd: DeviceFnKhrExternalFenceFd,
    pub khr_external_fence_win32: DeviceFnKhrExternalFenceWin32,
    pub khr_external_memory_fd: DeviceFnKhrExternalMemoryFd,
    pub khr_external_memory_win32: DeviceFnKhrExternalMemoryWin32,
    pub khr_external_semaphore_fd: DeviceFnKhrExternalSemaphoreFd,
    pub khr_external_semaphore_win32: DeviceFnKhrExternalSemaphoreWin32,
    pub khr_performance_query: DeviceFnKhrPerformanceQuery,
    pub khr_pipeline_binary: DeviceFnKhrPipelineBinary,
    pub khr_pipeline_executable_properties: DeviceFnKhrPipelineExecutableProperties,
    pub khr_present_wait: DeviceFnKhrPresentWait,
    pub khr_present_wait2: DeviceFnKhrPresentWait2,
    pub khr_ray_tracing_pipeline: DeviceFnKhrRayTracingPipeline,
    pub khr_shared_presentable_image: DeviceFnKhrSharedPresentableImage,
    pub khr_swapchain: DeviceFnKhrSwapchain,
    pub khr_swapchain_maintenance1: DeviceFnKhrSwapchainMaintenance1,
    pub khr_video_encode_queue: DeviceFnKhrVideoEncodeQueue,
    pub khr_video_queue: DeviceFnKhrVideoQueue,
    pub nv_cluster_acceleration_structure: DeviceFnNvClusterAccelerationStructure,
    pub nv_cooperative_vector: DeviceFnNvCooperativeVector,
    pub nv_cuda_kernel_launch: DeviceFnNvCudaKernelLaunch,
    pub nv_device_generated_commands: DeviceFnNvDeviceGeneratedCommands,
    pub nv_device_generated_commands_compute: DeviceFnNvDeviceGeneratedCommandsCompute,
    pub nv_external_compute_queue: DeviceFnNvExternalComputeQueue,
    pub nv_external_memory_rdma: DeviceFnNvExternalMemoryRdma,
    pub nv_external_memory_sci_buf: DeviceFnNvExternalMemorySciBuf,
    pub nv_external_memory_win32: DeviceFnNvExternalMemoryWin32,
    pub nv_external_sci_sync: DeviceFnNvExternalSciSync,
    pub nv_external_sci_sync2: DeviceFnNvExternalSciSync2,
    pub nv_low_latency: DeviceFnNvLowLatency,
    pub nv_low_latency2: DeviceFnNvLowLatency2,
    pub nv_optical_flow: DeviceFnNvOpticalFlow,
    pub nv_partitioned_acceleration_structure: DeviceFnNvPartitionedAccelerationStructure,
    pub nv_ray_tracing: DeviceFnNvRayTracing,
    pub nvx_binary_import: DeviceFnNvxBinaryImport,
    pub nvx_image_view_handle: DeviceFnNvxImageViewHandle,
    pub ohos_external_memory: DeviceFnOhosExternalMemory,
    pub qcom_tile_properties: DeviceFnQcomTileProperties,
    pub qnx_external_memory_screen_buffer: DeviceFnQnxExternalMemoryScreenBuffer,
    pub valve_descriptor_set_host_mapping: DeviceFnValveDescriptorSetHostMapping,
}

impl DeviceFn {
    /// A table with no functions loaded; every call through it panics.
    pub const EMPTY: Self = Self {
        v1_0: DeviceFnv1_0::EMPTY,
        v1_1: DeviceFnv1_1::EMPTY,
        v1_2: DeviceFnv1_2::EMPTY,
        v1_3: DeviceFnv1_3::EMPTY,
        v1_4: DeviceFnv1_4::EMPTY,
        amd_anti_lag: DeviceFnAmdAntiLag::EMPTY,
        amd_display_native_hdr: DeviceFnAmdDisplayNativeHdr::EMPTY,
        amd_gpa_interface: DeviceFnAmdGpaInterface::EMPTY,
        amd_shader_info: DeviceFnAmdShaderInfo::EMPTY,
        amdx_shader_enqueue: DeviceFnAmdxShaderEnqueue::EMPTY,
        android_external_memory_android_hardware_buffer:
            DeviceFnAndroidExternalMemoryAndroidHardwareBuffer::EMPTY,
        arm_data_graph: DeviceFnArmDataGraph::EMPTY,
        arm_shader_instrumentation: DeviceFnArmShaderInstrumentation::EMPTY,
        arm_tensors: DeviceFnArmTensors::EMPTY,
        ext_debug_marker: DeviceFnExtDebugMarker::EMPTY,
        ext_debug_utils: DeviceFnExtDebugUtils::EMPTY,
        ext_descriptor_buffer: DeviceFnExtDescriptorBuffer::EMPTY,
        ext_descriptor_heap: DeviceFnExtDescriptorHeap::EMPTY,
        ext_device_fault: DeviceFnExtDeviceFault::EMPTY,
        ext_device_generated_commands: DeviceFnExtDeviceGeneratedCommands::EMPTY,
        ext_display_control: DeviceFnExtDisplayControl::EMPTY,
        ext_external_memory_host: DeviceFnExtExternalMemoryHost::EMPTY,
        ext_external_memory_metal: DeviceFnExtExternalMemoryMetal::EMPTY,
        ext_full_screen_exclusive: DeviceFnExtFullScreenExclusive::EMPTY,
        ext_hdr_metadata: DeviceFnExtHdrMetadata::EMPTY,
        ext_image_drm_format_modifier: DeviceFnExtImageDrmFormatModifier::EMPTY,
        ext_metal_objects: DeviceFnExtMetalObjects::EMPTY,
        ext_opacity_micromap: DeviceFnExtOpacityMicromap::EMPTY,
        ext_pageable_device_local_memory: DeviceFnExtPageableDeviceLocalMemory::EMPTY,
        ext_pipeline_properties: DeviceFnExtPipelineProperties::EMPTY,
        ext_present_timing: DeviceFnExtPresentTiming::EMPTY,
        ext_shader_module_identifier: DeviceFnExtShaderModuleIdentifier::EMPTY,
        ext_shader_object: DeviceFnExtShaderObject::EMPTY,
        ext_validation_cache: DeviceFnExtValidationCache::EMPTY,
        fuchsia_buffer_collection: DeviceFnFuchsiaBufferCollection::EMPTY,
        fuchsia_external_memory: DeviceFnFuchsiaExternalMemory::EMPTY,
        fuchsia_external_semaphore: DeviceFnFuchsiaExternalSemaphore::EMPTY,
        google_display_timing: DeviceFnGoogleDisplayTiming::EMPTY,
        huawei_subpass_shading: DeviceFnHuaweiSubpassShading::EMPTY,
        intel_performance_query: DeviceFnIntelPerformanceQuery::EMPTY,
        khr_acceleration_structure: DeviceFnKhrAccelerationStructure::EMPTY,
        khr_calibrated_timestamps: DeviceFnKhrCalibratedTimestamps::EMPTY,
        khr_deferred_host_operations: DeviceFnKhrDeferredHostOperations::EMPTY,
        khr_device_address_commands: DeviceFnKhrDeviceAddressCommands::EMPTY,
        khr_device_fault: DeviceFnKhrDeviceFault::EMPTY,
        khr_device_group: DeviceFnKhrDeviceGroup::EMPTY,
        khr_display_swapchain: DeviceFnKhrDisplaySwapchain::EMPTY,
        khr_external_fence_fd: DeviceFnKhrExternalFenceFd::EMPTY,
        khr_external_fence_win32: DeviceFnKhrExternalFenceWin32::EMPTY,
        khr_external_memory_fd: DeviceFnKhrExternalMemoryFd::EMPTY,
        khr_external_memory_win32: DeviceFnKhrExternalMemoryWin32::EMPTY,
        khr_external_semaphore_fd: DeviceFnKhrExternalSemaphoreFd::EMPTY,
        khr_external_semaphore_win32: DeviceFnKhrExternalSemaphoreWin32::EMPTY,
        khr_performance_query: DeviceFnKhrPerformanceQuery::EMPTY,
        khr_pipeline_binary: DeviceFnKhrPipelineBinary::EMPTY,
        khr_pipeline_executable_properties: DeviceFnKhrPipelineExecutableProperties::EMPTY,
        khr_present_wait: DeviceFnKhrPresentWait::EMPTY,
        khr_present_wait2: DeviceFnKhrPresentWait2::EMPTY,
        khr_ray_tracing_pipeline: DeviceFnKhrRayTracingPipeline::EMPTY,
        khr_shared_presentable_image: DeviceFnKhrSharedPresentableImage::EMPTY,
        khr_swapchain: DeviceFnKhrSwapchain::EMPTY,
        khr_swapchain_maintenance1: DeviceFnKhrSwapchainMaintenance1::EMPTY,
        khr_video_encode_queue: DeviceFnKhrVideoEncodeQueue::EMPTY,
        khr_video_queue: DeviceFnKhrVideoQueue::EMPTY,
        nv_cluster_acceleration_structure: DeviceFnNvClusterAccelerationStructure::EMPTY,
        nv_cooperative_vector: DeviceFnNvCooperativeVector::EMPTY,
        nv_cuda_kernel_launch: DeviceFnNvCudaKernelLaunch::EMPTY,
        nv_device_generated_commands: DeviceFnNvDeviceGeneratedCommands::EMPTY,
        nv_device_generated_commands_compute: DeviceFnNvDeviceGeneratedCommandsCompute::EMPTY,
        nv_external_compute_queue: DeviceFnNvExternalComputeQueue::EMPTY,
        nv_external_memory_rdma: DeviceFnNvExternalMemoryRdma::EMPTY,
        nv_external_memory_sci_buf: DeviceFnNvExternalMemorySciBuf::EMPTY,
        nv_external_memory_win32: DeviceFnNvExternalMemoryWin32::EMPTY,
        nv_external_sci_sync: DeviceFnNvExternalSciSync::EMPTY,
        nv_external_sci_sync2: DeviceFnNvExternalSciSync2::EMPTY,
        nv_low_latency: DeviceFnNvLowLatency::EMPTY,
        nv_low_latency2: DeviceFnNvLowLatency2::EMPTY,
        nv_optical_flow: DeviceFnNvOpticalFlow::EMPTY,
        nv_partitioned_acceleration_structure: DeviceFnNvPartitionedAccelerationStructure::EMPTY,
        nv_ray_tracing: DeviceFnNvRayTracing::EMPTY,
        nvx_binary_import: DeviceFnNvxBinaryImport::EMPTY,
        nvx_image_view_handle: DeviceFnNvxImageViewHandle::EMPTY,
        ohos_external_memory: DeviceFnOhosExternalMemory::EMPTY,
        qcom_tile_properties: DeviceFnQcomTileProperties::EMPTY,
        qnx_external_memory_screen_buffer: DeviceFnQnxExternalMemoryScreenBuffer::EMPTY,
        valve_descriptor_set_host_mapping: DeviceFnValveDescriptorSetHostMapping::EMPTY,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        let mut out = Self {
            v1_0: DeviceFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                DeviceFnv1_1::load(&mut loader)
            } else {
                DeviceFnv1_1::EMPTY
            },
            v1_2: if api_version >= API_VERSION_1_2 {
                DeviceFnv1_2::load(&mut loader)
            } else {
                DeviceFnv1_2::EMPTY
            },
            v1_3: if api_version >= API_VERSION_1_3 {
                DeviceFnv1_3::load(&mut loader)
            } else {
                DeviceFnv1_3::EMPTY
            },
            v1_4: if api_version >= API_VERSION_1_4 {
                DeviceFnv1_4::load(&mut loader)
            } else {
                DeviceFnv1_4::EMPTY
            },
            amd_anti_lag: DeviceFnAmdAntiLag::EMPTY,
            amd_display_native_hdr: DeviceFnAmdDisplayNativeHdr::EMPTY,
            amd_gpa_interface: DeviceFnAmdGpaInterface::EMPTY,
            amd_shader_info: DeviceFnAmdShaderInfo::EMPTY,
            amdx_shader_enqueue: DeviceFnAmdxShaderEnqueue::EMPTY,
            android_external_memory_android_hardware_buffer:
                DeviceFnAndroidExternalMemoryAndroidHardwareBuffer::EMPTY,
            arm_data_graph: DeviceFnArmDataGraph::EMPTY,
            arm_shader_instrumentation: DeviceFnArmShaderInstrumentation::EMPTY,
            arm_tensors: DeviceFnArmTensors::EMPTY,
            ext_debug_marker: DeviceFnExtDebugMarker::EMPTY,
            ext_debug_utils: DeviceFnExtDebugUtils::EMPTY,
            ext_descriptor_buffer: DeviceFnExtDescriptorBuffer::EMPTY,
            ext_descriptor_heap: DeviceFnExtDescriptorHeap::EMPTY,
            ext_device_fault: DeviceFnExtDeviceFault::EMPTY,
            ext_device_generated_commands: DeviceFnExtDeviceGeneratedCommands::EMPTY,
            ext_display_control: DeviceFnExtDisplayControl::EMPTY,
            ext_external_memory_host: DeviceFnExtExternalMemoryHost::EMPTY,
            ext_external_memory_metal: DeviceFnExtExternalMemoryMetal::EMPTY,
            ext_full_screen_exclusive: DeviceFnExtFullScreenExclusive::EMPTY,
            ext_hdr_metadata: DeviceFnExtHdrMetadata::EMPTY,
            ext_image_drm_format_modifier: DeviceFnExtImageDrmFormatModifier::EMPTY,
            ext_metal_objects: DeviceFnExtMetalObjects::EMPTY,
            ext_opacity_micromap: DeviceFnExtOpacityMicromap::EMPTY,
            ext_pageable_device_local_memory: DeviceFnExtPageableDeviceLocalMemory::EMPTY,
            ext_pipeline_properties: DeviceFnExtPipelineProperties::EMPTY,
            ext_present_timing: DeviceFnExtPresentTiming::EMPTY,
            ext_shader_module_identifier: DeviceFnExtShaderModuleIdentifier::EMPTY,
            ext_shader_object: DeviceFnExtShaderObject::EMPTY,
            ext_validation_cache: DeviceFnExtValidationCache::EMPTY,
            fuchsia_buffer_collection: DeviceFnFuchsiaBufferCollection::EMPTY,
            fuchsia_external_memory: DeviceFnFuchsiaExternalMemory::EMPTY,
            fuchsia_external_semaphore: DeviceFnFuchsiaExternalSemaphore::EMPTY,
            google_display_timing: DeviceFnGoogleDisplayTiming::EMPTY,
            huawei_subpass_shading: DeviceFnHuaweiSubpassShading::EMPTY,
            intel_performance_query: DeviceFnIntelPerformanceQuery::EMPTY,
            khr_acceleration_structure: DeviceFnKhrAccelerationStructure::EMPTY,
            khr_calibrated_timestamps: DeviceFnKhrCalibratedTimestamps::EMPTY,
            khr_deferred_host_operations: DeviceFnKhrDeferredHostOperations::EMPTY,
            khr_device_address_commands: DeviceFnKhrDeviceAddressCommands::EMPTY,
            khr_device_fault: DeviceFnKhrDeviceFault::EMPTY,
            khr_device_group: DeviceFnKhrDeviceGroup::EMPTY,
            khr_display_swapchain: DeviceFnKhrDisplaySwapchain::EMPTY,
            khr_external_fence_fd: DeviceFnKhrExternalFenceFd::EMPTY,
            khr_external_fence_win32: DeviceFnKhrExternalFenceWin32::EMPTY,
            khr_external_memory_fd: DeviceFnKhrExternalMemoryFd::EMPTY,
            khr_external_memory_win32: DeviceFnKhrExternalMemoryWin32::EMPTY,
            khr_external_semaphore_fd: DeviceFnKhrExternalSemaphoreFd::EMPTY,
            khr_external_semaphore_win32: DeviceFnKhrExternalSemaphoreWin32::EMPTY,
            khr_performance_query: DeviceFnKhrPerformanceQuery::EMPTY,
            khr_pipeline_binary: DeviceFnKhrPipelineBinary::EMPTY,
            khr_pipeline_executable_properties: DeviceFnKhrPipelineExecutableProperties::EMPTY,
            khr_present_wait: DeviceFnKhrPresentWait::EMPTY,
            khr_present_wait2: DeviceFnKhrPresentWait2::EMPTY,
            khr_ray_tracing_pipeline: DeviceFnKhrRayTracingPipeline::EMPTY,
            khr_shared_presentable_image: DeviceFnKhrSharedPresentableImage::EMPTY,
            khr_swapchain: DeviceFnKhrSwapchain::EMPTY,
            khr_swapchain_maintenance1: DeviceFnKhrSwapchainMaintenance1::EMPTY,
            khr_video_encode_queue: DeviceFnKhrVideoEncodeQueue::EMPTY,
            khr_video_queue: DeviceFnKhrVideoQueue::EMPTY,
            nv_cluster_acceleration_structure: DeviceFnNvClusterAccelerationStructure::EMPTY,
            nv_cooperative_vector: DeviceFnNvCooperativeVector::EMPTY,
            nv_cuda_kernel_launch: DeviceFnNvCudaKernelLaunch::EMPTY,
            nv_device_generated_commands: DeviceFnNvDeviceGeneratedCommands::EMPTY,
            nv_device_generated_commands_compute: DeviceFnNvDeviceGeneratedCommandsCompute::EMPTY,
            nv_external_compute_queue: DeviceFnNvExternalComputeQueue::EMPTY,
            nv_external_memory_rdma: DeviceFnNvExternalMemoryRdma::EMPTY,
            nv_external_memory_sci_buf: DeviceFnNvExternalMemorySciBuf::EMPTY,
            nv_external_memory_win32: DeviceFnNvExternalMemoryWin32::EMPTY,
            nv_external_sci_sync: DeviceFnNvExternalSciSync::EMPTY,
            nv_external_sci_sync2: DeviceFnNvExternalSciSync2::EMPTY,
            nv_low_latency: DeviceFnNvLowLatency::EMPTY,
            nv_low_latency2: DeviceFnNvLowLatency2::EMPTY,
            nv_optical_flow: DeviceFnNvOpticalFlow::EMPTY,
            nv_partitioned_acceleration_structure:
                DeviceFnNvPartitionedAccelerationStructure::EMPTY,
            nv_ray_tracing: DeviceFnNvRayTracing::EMPTY,
            nvx_binary_import: DeviceFnNvxBinaryImport::EMPTY,
            nvx_image_view_handle: DeviceFnNvxImageViewHandle::EMPTY,
            ohos_external_memory: DeviceFnOhosExternalMemory::EMPTY,
            qcom_tile_properties: DeviceFnQcomTileProperties::EMPTY,
            qnx_external_memory_screen_buffer: DeviceFnQnxExternalMemoryScreenBuffer::EMPTY,
            valve_descriptor_set_host_mapping: DeviceFnValveDescriptorSetHostMapping::EMPTY,
        };
        out.ext_debug_utils.set_debug_utils_object_name_ext =
            to_option(loader(c"vkSetDebugUtilsObjectNameEXT"));
        out.ext_debug_utils.set_debug_utils_object_tag_ext =
            to_option(loader(c"vkSetDebugUtilsObjectTagEXT"));
        for &ext in extensions {
            match unsafe { CStr::from_ptr(ext) }.to_bytes() {
                b"VK_KHR_swapchain" => {
                    out.khr_swapchain.create_swapchain_khr =
                        to_option(loader(c"vkCreateSwapchainKHR"));
                    out.khr_swapchain.destroy_swapchain_khr =
                        to_option(loader(c"vkDestroySwapchainKHR"));
                    out.khr_swapchain.get_swapchain_images_khr =
                        to_option(loader(c"vkGetSwapchainImagesKHR"));
                    out.khr_swapchain.acquire_next_image_khr =
                        to_option(loader(c"vkAcquireNextImageKHR"));
                    if out
                        .khr_device_group
                        .get_device_group_present_capabilities_khr
                        .is_none()
                    {
                        out.khr_device_group
                            .get_device_group_present_capabilities_khr =
                            to_option(loader(c"vkGetDeviceGroupPresentCapabilitiesKHR"));
                    }
                    if out
                        .khr_device_group
                        .get_device_group_surface_present_modes_khr
                        .is_none()
                    {
                        out.khr_device_group
                            .get_device_group_surface_present_modes_khr =
                            to_option(loader(c"vkGetDeviceGroupSurfacePresentModesKHR"));
                    }
                    if out.khr_device_group.acquire_next_image2_khr.is_none() {
                        out.khr_device_group.acquire_next_image2_khr =
                            to_option(loader(c"vkAcquireNextImage2KHR"));
                    }
                }
                b"VK_KHR_display_swapchain" => {
                    out.khr_display_swapchain.create_shared_swapchains_khr =
                        to_option(loader(c"vkCreateSharedSwapchainsKHR"));
                }
                b"VK_EXT_debug_marker" => {
                    out.ext_debug_marker.debug_marker_set_object_tag_ext =
                        to_option(loader(c"vkDebugMarkerSetObjectTagEXT"));
                    out.ext_debug_marker.debug_marker_set_object_name_ext =
                        to_option(loader(c"vkDebugMarkerSetObjectNameEXT"));
                }
                b"VK_KHR_video_queue" => {
                    out.khr_video_queue.create_video_session_khr =
                        to_option(loader(c"vkCreateVideoSessionKHR"));
                    out.khr_video_queue.destroy_video_session_khr =
                        to_option(loader(c"vkDestroyVideoSessionKHR"));
                    out.khr_video_queue
                        .get_video_session_memory_requirements_khr =
                        to_option(loader(c"vkGetVideoSessionMemoryRequirementsKHR"));
                    out.khr_video_queue.bind_video_session_memory_khr =
                        to_option(loader(c"vkBindVideoSessionMemoryKHR"));
                    out.khr_video_queue.create_video_session_parameters_khr =
                        to_option(loader(c"vkCreateVideoSessionParametersKHR"));
                    out.khr_video_queue.update_video_session_parameters_khr =
                        to_option(loader(c"vkUpdateVideoSessionParametersKHR"));
                    out.khr_video_queue.destroy_video_session_parameters_khr =
                        to_option(loader(c"vkDestroyVideoSessionParametersKHR"));
                }
                b"VK_NVX_binary_import" => {
                    out.nvx_binary_import.create_cu_module_nvx =
                        to_option(loader(c"vkCreateCuModuleNVX"));
                    out.nvx_binary_import.create_cu_function_nvx =
                        to_option(loader(c"vkCreateCuFunctionNVX"));
                    out.nvx_binary_import.destroy_cu_module_nvx =
                        to_option(loader(c"vkDestroyCuModuleNVX"));
                    out.nvx_binary_import.destroy_cu_function_nvx =
                        to_option(loader(c"vkDestroyCuFunctionNVX"));
                }
                b"VK_NVX_image_view_handle" => {
                    out.nvx_image_view_handle.get_image_view_handle_nvx =
                        to_option(loader(c"vkGetImageViewHandleNVX"));
                    out.nvx_image_view_handle.get_image_view_handle64_nvx =
                        to_option(loader(c"vkGetImageViewHandle64NVX"));
                    out.nvx_image_view_handle.get_image_view_address_nvx =
                        to_option(loader(c"vkGetImageViewAddressNVX"));
                    out.nvx_image_view_handle
                        .get_device_combined_image_sampler_index_nvx =
                        to_option(loader(c"vkGetDeviceCombinedImageSamplerIndexNVX"));
                }
                b"VK_AMD_shader_info" => {
                    out.amd_shader_info.get_shader_info_amd =
                        to_option(loader(c"vkGetShaderInfoAMD"));
                }
                b"VK_NV_external_memory_win32" => {
                    out.nv_external_memory_win32.get_memory_win32_handle_nv =
                        to_option(loader(c"vkGetMemoryWin32HandleNV"));
                }
                b"VK_KHR_device_group" => {
                    if out.v1_1.get_device_group_peer_memory_features.is_none() {
                        out.v1_1.get_device_group_peer_memory_features =
                            to_option(loader(c"vkGetDeviceGroupPeerMemoryFeaturesKHR"));
                    }
                    if out
                        .khr_device_group
                        .get_device_group_present_capabilities_khr
                        .is_none()
                    {
                        out.khr_device_group
                            .get_device_group_present_capabilities_khr =
                            to_option(loader(c"vkGetDeviceGroupPresentCapabilitiesKHR"));
                    }
                    if out
                        .khr_device_group
                        .get_device_group_surface_present_modes_khr
                        .is_none()
                    {
                        out.khr_device_group
                            .get_device_group_surface_present_modes_khr =
                            to_option(loader(c"vkGetDeviceGroupSurfacePresentModesKHR"));
                    }
                    if out.khr_device_group.acquire_next_image2_khr.is_none() {
                        out.khr_device_group.acquire_next_image2_khr =
                            to_option(loader(c"vkAcquireNextImage2KHR"));
                    }
                }
                b"VK_KHR_maintenance1" => {
                    if out.v1_1.trim_command_pool.is_none() {
                        out.v1_1.trim_command_pool = to_option(loader(c"vkTrimCommandPoolKHR"));
                    }
                }
                b"VK_KHR_external_memory_win32" => {
                    out.khr_external_memory_win32.get_memory_win32_handle_khr =
                        to_option(loader(c"vkGetMemoryWin32HandleKHR"));
                    out.khr_external_memory_win32
                        .get_memory_win32_handle_properties_khr =
                        to_option(loader(c"vkGetMemoryWin32HandlePropertiesKHR"));
                }
                b"VK_KHR_external_memory_fd" => {
                    out.khr_external_memory_fd.get_memory_fd_khr =
                        to_option(loader(c"vkGetMemoryFdKHR"));
                    out.khr_external_memory_fd.get_memory_fd_properties_khr =
                        to_option(loader(c"vkGetMemoryFdPropertiesKHR"));
                }
                b"VK_KHR_external_semaphore_win32" => {
                    out.khr_external_semaphore_win32
                        .import_semaphore_win32_handle_khr =
                        to_option(loader(c"vkImportSemaphoreWin32HandleKHR"));
                    out.khr_external_semaphore_win32
                        .get_semaphore_win32_handle_khr =
                        to_option(loader(c"vkGetSemaphoreWin32HandleKHR"));
                }
                b"VK_KHR_external_semaphore_fd" => {
                    out.khr_external_semaphore_fd.import_semaphore_fd_khr =
                        to_option(loader(c"vkImportSemaphoreFdKHR"));
                    out.khr_external_semaphore_fd.get_semaphore_fd_khr =
                        to_option(loader(c"vkGetSemaphoreFdKHR"));
                }
                b"VK_KHR_descriptor_update_template" => {
                    if out.v1_1.create_descriptor_update_template.is_none() {
                        out.v1_1.create_descriptor_update_template =
                            to_option(loader(c"vkCreateDescriptorUpdateTemplateKHR"));
                    }
                    if out.v1_1.destroy_descriptor_update_template.is_none() {
                        out.v1_1.destroy_descriptor_update_template =
                            to_option(loader(c"vkDestroyDescriptorUpdateTemplateKHR"));
                    }
                    if out.v1_1.update_descriptor_set_with_template.is_none() {
                        out.v1_1.update_descriptor_set_with_template =
                            to_option(loader(c"vkUpdateDescriptorSetWithTemplateKHR"));
                    }
                }
                b"VK_EXT_display_control" => {
                    out.ext_display_control.display_power_control_ext =
                        to_option(loader(c"vkDisplayPowerControlEXT"));
                    out.ext_display_control.register_device_event_ext =
                        to_option(loader(c"vkRegisterDeviceEventEXT"));
                    out.ext_display_control.register_display_event_ext =
                        to_option(loader(c"vkRegisterDisplayEventEXT"));
                    out.ext_display_control.get_swapchain_counter_ext =
                        to_option(loader(c"vkGetSwapchainCounterEXT"));
                }
                b"VK_GOOGLE_display_timing" => {
                    out.google_display_timing.get_refresh_cycle_duration_google =
                        to_option(loader(c"vkGetRefreshCycleDurationGOOGLE"));
                    out.google_display_timing
                        .get_past_presentation_timing_google =
                        to_option(loader(c"vkGetPastPresentationTimingGOOGLE"));
                }
                b"VK_EXT_hdr_metadata" => {
                    out.ext_hdr_metadata.set_hdr_metadata_ext =
                        to_option(loader(c"vkSetHdrMetadataEXT"));
                }
                b"VK_KHR_create_renderpass2" => {
                    if out.v1_2.create_render_pass2.is_none() {
                        out.v1_2.create_render_pass2 = to_option(loader(c"vkCreateRenderPass2KHR"));
                    }
                }
                b"VK_KHR_shared_presentable_image" => {
                    out.khr_shared_presentable_image.get_swapchain_status_khr =
                        to_option(loader(c"vkGetSwapchainStatusKHR"));
                }
                b"VK_KHR_external_fence_win32" => {
                    out.khr_external_fence_win32.import_fence_win32_handle_khr =
                        to_option(loader(c"vkImportFenceWin32HandleKHR"));
                    out.khr_external_fence_win32.get_fence_win32_handle_khr =
                        to_option(loader(c"vkGetFenceWin32HandleKHR"));
                }
                b"VK_KHR_external_fence_fd" => {
                    out.khr_external_fence_fd.import_fence_fd_khr =
                        to_option(loader(c"vkImportFenceFdKHR"));
                    out.khr_external_fence_fd.get_fence_fd_khr =
                        to_option(loader(c"vkGetFenceFdKHR"));
                }
                b"VK_KHR_performance_query" => {
                    out.khr_performance_query.acquire_profiling_lock_khr =
                        to_option(loader(c"vkAcquireProfilingLockKHR"));
                    out.khr_performance_query.release_profiling_lock_khr =
                        to_option(loader(c"vkReleaseProfilingLockKHR"));
                }
                b"VK_ANDROID_external_memory_android_hardware_buffer" => {
                    out.android_external_memory_android_hardware_buffer
                        .get_android_hardware_buffer_properties_android =
                        to_option(loader(c"vkGetAndroidHardwareBufferPropertiesANDROID"));
                    out.android_external_memory_android_hardware_buffer
                        .get_memory_android_hardware_buffer_android =
                        to_option(loader(c"vkGetMemoryAndroidHardwareBufferANDROID"));
                }
                b"VK_AMD_gpa_interface" => {
                    out.amd_gpa_interface.create_gpa_session_amd =
                        to_option(loader(c"vkCreateGpaSessionAMD"));
                    out.amd_gpa_interface.destroy_gpa_session_amd =
                        to_option(loader(c"vkDestroyGpaSessionAMD"));
                    out.amd_gpa_interface.set_gpa_device_clock_mode_amd =
                        to_option(loader(c"vkSetGpaDeviceClockModeAMD"));
                    out.amd_gpa_interface.get_gpa_device_clock_info_amd =
                        to_option(loader(c"vkGetGpaDeviceClockInfoAMD"));
                    out.amd_gpa_interface.get_gpa_session_status_amd =
                        to_option(loader(c"vkGetGpaSessionStatusAMD"));
                    out.amd_gpa_interface.get_gpa_session_results_amd =
                        to_option(loader(c"vkGetGpaSessionResultsAMD"));
                    out.amd_gpa_interface.reset_gpa_session_amd =
                        to_option(loader(c"vkResetGpaSessionAMD"));
                }
                b"VK_AMDX_shader_enqueue" => {
                    out.amdx_shader_enqueue
                        .create_execution_graph_pipelines_amdx =
                        to_option(loader(c"vkCreateExecutionGraphPipelinesAMDX"));
                    out.amdx_shader_enqueue
                        .get_execution_graph_pipeline_scratch_size_amdx =
                        to_option(loader(c"vkGetExecutionGraphPipelineScratchSizeAMDX"));
                    out.amdx_shader_enqueue
                        .get_execution_graph_pipeline_node_index_amdx =
                        to_option(loader(c"vkGetExecutionGraphPipelineNodeIndexAMDX"));
                }
                b"VK_EXT_descriptor_heap" => {
                    out.ext_descriptor_heap.write_sampler_descriptors_ext =
                        to_option(loader(c"vkWriteSamplerDescriptorsEXT"));
                    out.ext_descriptor_heap.write_resource_descriptors_ext =
                        to_option(loader(c"vkWriteResourceDescriptorsEXT"));
                    out.ext_descriptor_heap.get_image_opaque_capture_data_ext =
                        to_option(loader(c"vkGetImageOpaqueCaptureDataEXT"));
                    out.ext_descriptor_heap.register_custom_border_color_ext =
                        to_option(loader(c"vkRegisterCustomBorderColorEXT"));
                    out.ext_descriptor_heap.unregister_custom_border_color_ext =
                        to_option(loader(c"vkUnregisterCustomBorderColorEXT"));
                    out.ext_descriptor_heap.get_tensor_opaque_capture_data_arm =
                        to_option(loader(c"vkGetTensorOpaqueCaptureDataARM"));
                }
                b"VK_KHR_get_memory_requirements2" => {
                    if out.v1_1.get_image_memory_requirements2.is_none() {
                        out.v1_1.get_image_memory_requirements2 =
                            to_option(loader(c"vkGetImageMemoryRequirements2KHR"));
                    }
                    if out.v1_1.get_buffer_memory_requirements2.is_none() {
                        out.v1_1.get_buffer_memory_requirements2 =
                            to_option(loader(c"vkGetBufferMemoryRequirements2KHR"));
                    }
                    if out.v1_1.get_image_sparse_memory_requirements2.is_none() {
                        out.v1_1.get_image_sparse_memory_requirements2 =
                            to_option(loader(c"vkGetImageSparseMemoryRequirements2KHR"));
                    }
                }
                b"VK_KHR_acceleration_structure" => {
                    out.khr_acceleration_structure
                        .create_acceleration_structure_khr =
                        to_option(loader(c"vkCreateAccelerationStructureKHR"));
                    out.khr_acceleration_structure
                        .destroy_acceleration_structure_khr =
                        to_option(loader(c"vkDestroyAccelerationStructureKHR"));
                    out.khr_acceleration_structure
                        .build_acceleration_structures_khr =
                        to_option(loader(c"vkBuildAccelerationStructuresKHR"));
                    out.khr_acceleration_structure
                        .copy_acceleration_structure_khr =
                        to_option(loader(c"vkCopyAccelerationStructureKHR"));
                    out.khr_acceleration_structure
                        .copy_acceleration_structure_to_memory_khr =
                        to_option(loader(c"vkCopyAccelerationStructureToMemoryKHR"));
                    out.khr_acceleration_structure
                        .copy_memory_to_acceleration_structure_khr =
                        to_option(loader(c"vkCopyMemoryToAccelerationStructureKHR"));
                    out.khr_acceleration_structure
                        .write_acceleration_structures_properties_khr =
                        to_option(loader(c"vkWriteAccelerationStructuresPropertiesKHR"));
                    out.khr_acceleration_structure
                        .get_acceleration_structure_device_address_khr =
                        to_option(loader(c"vkGetAccelerationStructureDeviceAddressKHR"));
                    out.khr_acceleration_structure
                        .get_device_acceleration_structure_compatibility_khr =
                        to_option(loader(c"vkGetDeviceAccelerationStructureCompatibilityKHR"));
                    out.khr_acceleration_structure
                        .get_acceleration_structure_build_sizes_khr =
                        to_option(loader(c"vkGetAccelerationStructureBuildSizesKHR"));
                }
                b"VK_KHR_ray_tracing_pipeline" => {
                    out.khr_ray_tracing_pipeline
                        .create_ray_tracing_pipelines_khr =
                        to_option(loader(c"vkCreateRayTracingPipelinesKHR"));
                    if out
                        .khr_ray_tracing_pipeline
                        .get_ray_tracing_shader_group_handles_khr
                        .is_none()
                    {
                        out.khr_ray_tracing_pipeline
                            .get_ray_tracing_shader_group_handles_khr =
                            to_option(loader(c"vkGetRayTracingShaderGroupHandlesKHR"));
                    }
                    out.khr_ray_tracing_pipeline
                        .get_ray_tracing_capture_replay_shader_group_handles_khr =
                        to_option(loader(c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR"));
                    out.khr_ray_tracing_pipeline
                        .get_ray_tracing_shader_group_stack_size_khr =
                        to_option(loader(c"vkGetRayTracingShaderGroupStackSizeKHR"));
                }
                b"VK_KHR_sampler_ycbcr_conversion" => {
                    if out.v1_1.create_sampler_ycbcr_conversion.is_none() {
                        out.v1_1.create_sampler_ycbcr_conversion =
                            to_option(loader(c"vkCreateSamplerYcbcrConversionKHR"));
                    }
                    if out.v1_1.destroy_sampler_ycbcr_conversion.is_none() {
                        out.v1_1.destroy_sampler_ycbcr_conversion =
                            to_option(loader(c"vkDestroySamplerYcbcrConversionKHR"));
                    }
                }
                b"VK_KHR_bind_memory2" => {
                    if out.v1_1.bind_buffer_memory2.is_none() {
                        out.v1_1.bind_buffer_memory2 = to_option(loader(c"vkBindBufferMemory2KHR"));
                    }
                    if out.v1_1.bind_image_memory2.is_none() {
                        out.v1_1.bind_image_memory2 = to_option(loader(c"vkBindImageMemory2KHR"));
                    }
                }
                b"VK_EXT_image_drm_format_modifier" => {
                    out.ext_image_drm_format_modifier
                        .get_image_drm_format_modifier_properties_ext =
                        to_option(loader(c"vkGetImageDrmFormatModifierPropertiesEXT"));
                }
                b"VK_EXT_validation_cache" => {
                    out.ext_validation_cache.create_validation_cache_ext =
                        to_option(loader(c"vkCreateValidationCacheEXT"));
                    out.ext_validation_cache.destroy_validation_cache_ext =
                        to_option(loader(c"vkDestroyValidationCacheEXT"));
                    out.ext_validation_cache.merge_validation_caches_ext =
                        to_option(loader(c"vkMergeValidationCachesEXT"));
                    out.ext_validation_cache.get_validation_cache_data_ext =
                        to_option(loader(c"vkGetValidationCacheDataEXT"));
                }
                b"VK_NV_ray_tracing" => {
                    out.nv_ray_tracing.create_acceleration_structure_nv =
                        to_option(loader(c"vkCreateAccelerationStructureNV"));
                    out.nv_ray_tracing.destroy_acceleration_structure_nv =
                        to_option(loader(c"vkDestroyAccelerationStructureNV"));
                    out.nv_ray_tracing
                        .get_acceleration_structure_memory_requirements_nv =
                        to_option(loader(c"vkGetAccelerationStructureMemoryRequirementsNV"));
                    out.nv_ray_tracing.bind_acceleration_structure_memory_nv =
                        to_option(loader(c"vkBindAccelerationStructureMemoryNV"));
                    out.nv_ray_tracing.create_ray_tracing_pipelines_nv =
                        to_option(loader(c"vkCreateRayTracingPipelinesNV"));
                    if out
                        .khr_ray_tracing_pipeline
                        .get_ray_tracing_shader_group_handles_khr
                        .is_none()
                    {
                        out.khr_ray_tracing_pipeline
                            .get_ray_tracing_shader_group_handles_khr =
                            to_option(loader(c"vkGetRayTracingShaderGroupHandlesNV"));
                    }
                    out.nv_ray_tracing.get_acceleration_structure_handle_nv =
                        to_option(loader(c"vkGetAccelerationStructureHandleNV"));
                    out.nv_ray_tracing.compile_deferred_nv =
                        to_option(loader(c"vkCompileDeferredNV"));
                }
                b"VK_KHR_maintenance3" => {
                    if out.v1_1.get_descriptor_set_layout_support.is_none() {
                        out.v1_1.get_descriptor_set_layout_support =
                            to_option(loader(c"vkGetDescriptorSetLayoutSupportKHR"));
                    }
                }
                b"VK_EXT_external_memory_host" => {
                    out.ext_external_memory_host
                        .get_memory_host_pointer_properties_ext =
                        to_option(loader(c"vkGetMemoryHostPointerPropertiesEXT"));
                }
                b"VK_EXT_calibrated_timestamps" => {
                    if out
                        .khr_calibrated_timestamps
                        .get_calibrated_timestamps_khr
                        .is_none()
                    {
                        out.khr_calibrated_timestamps.get_calibrated_timestamps_khr =
                            to_option(loader(c"vkGetCalibratedTimestampsEXT"));
                    }
                }
                b"VK_KHR_timeline_semaphore" => {
                    if out.v1_2.get_semaphore_counter_value.is_none() {
                        out.v1_2.get_semaphore_counter_value =
                            to_option(loader(c"vkGetSemaphoreCounterValueKHR"));
                    }
                    if out.v1_2.wait_semaphores.is_none() {
                        out.v1_2.wait_semaphores = to_option(loader(c"vkWaitSemaphoresKHR"));
                    }
                    if out.v1_2.signal_semaphore.is_none() {
                        out.v1_2.signal_semaphore = to_option(loader(c"vkSignalSemaphoreKHR"));
                    }
                }
                b"VK_EXT_present_timing" => {
                    out.ext_present_timing
                        .set_swapchain_present_timing_queue_size_ext =
                        to_option(loader(c"vkSetSwapchainPresentTimingQueueSizeEXT"));
                    out.ext_present_timing.get_swapchain_timing_properties_ext =
                        to_option(loader(c"vkGetSwapchainTimingPropertiesEXT"));
                    out.ext_present_timing
                        .get_swapchain_time_domain_properties_ext =
                        to_option(loader(c"vkGetSwapchainTimeDomainPropertiesEXT"));
                    out.ext_present_timing.get_past_presentation_timing_ext =
                        to_option(loader(c"vkGetPastPresentationTimingEXT"));
                }
                b"VK_INTEL_performance_query" => {
                    out.intel_performance_query.initialize_performance_api_intel =
                        to_option(loader(c"vkInitializePerformanceApiINTEL"));
                    out.intel_performance_query
                        .uninitialize_performance_api_intel =
                        to_option(loader(c"vkUninitializePerformanceApiINTEL"));
                    out.intel_performance_query
                        .acquire_performance_configuration_intel =
                        to_option(loader(c"vkAcquirePerformanceConfigurationINTEL"));
                    out.intel_performance_query
                        .release_performance_configuration_intel =
                        to_option(loader(c"vkReleasePerformanceConfigurationINTEL"));
                    out.intel_performance_query.get_performance_parameter_intel =
                        to_option(loader(c"vkGetPerformanceParameterINTEL"));
                }
                b"VK_AMD_display_native_hdr" => {
                    out.amd_display_native_hdr.set_local_dimming_amd =
                        to_option(loader(c"vkSetLocalDimmingAMD"));
                }
                b"VK_EXT_buffer_device_address" => {
                    if out.v1_2.get_buffer_device_address.is_none() {
                        out.v1_2.get_buffer_device_address =
                            to_option(loader(c"vkGetBufferDeviceAddressEXT"));
                    }
                }
                b"VK_KHR_present_wait" => {
                    out.khr_present_wait.wait_for_present_khr =
                        to_option(loader(c"vkWaitForPresentKHR"));
                }
                b"VK_EXT_full_screen_exclusive" => {
                    out.ext_full_screen_exclusive
                        .acquire_full_screen_exclusive_mode_ext =
                        to_option(loader(c"vkAcquireFullScreenExclusiveModeEXT"));
                    out.ext_full_screen_exclusive
                        .release_full_screen_exclusive_mode_ext =
                        to_option(loader(c"vkReleaseFullScreenExclusiveModeEXT"));
                    out.ext_full_screen_exclusive
                        .get_device_group_surface_present_modes2_ext =
                        to_option(loader(c"vkGetDeviceGroupSurfacePresentModes2EXT"));
                }
                b"VK_KHR_buffer_device_address" => {
                    if out.v1_2.get_buffer_device_address.is_none() {
                        out.v1_2.get_buffer_device_address =
                            to_option(loader(c"vkGetBufferDeviceAddressKHR"));
                    }
                    if out.v1_2.get_buffer_opaque_capture_address.is_none() {
                        out.v1_2.get_buffer_opaque_capture_address =
                            to_option(loader(c"vkGetBufferOpaqueCaptureAddressKHR"));
                    }
                    if out.v1_2.get_device_memory_opaque_capture_address.is_none() {
                        out.v1_2.get_device_memory_opaque_capture_address =
                            to_option(loader(c"vkGetDeviceMemoryOpaqueCaptureAddressKHR"));
                    }
                }
                b"VK_EXT_host_query_reset" => {
                    if out.v1_2.reset_query_pool.is_none() {
                        out.v1_2.reset_query_pool = to_option(loader(c"vkResetQueryPoolEXT"));
                    }
                }
                b"VK_KHR_deferred_host_operations" => {
                    out.khr_deferred_host_operations
                        .create_deferred_operation_khr =
                        to_option(loader(c"vkCreateDeferredOperationKHR"));
                    out.khr_deferred_host_operations
                        .destroy_deferred_operation_khr =
                        to_option(loader(c"vkDestroyDeferredOperationKHR"));
                    out.khr_deferred_host_operations
                        .get_deferred_operation_max_concurrency_khr =
                        to_option(loader(c"vkGetDeferredOperationMaxConcurrencyKHR"));
                    out.khr_deferred_host_operations
                        .get_deferred_operation_result_khr =
                        to_option(loader(c"vkGetDeferredOperationResultKHR"));
                    out.khr_deferred_host_operations.deferred_operation_join_khr =
                        to_option(loader(c"vkDeferredOperationJoinKHR"));
                }
                b"VK_KHR_pipeline_executable_properties" => {
                    out.khr_pipeline_executable_properties
                        .get_pipeline_executable_properties_khr =
                        to_option(loader(c"vkGetPipelineExecutablePropertiesKHR"));
                    out.khr_pipeline_executable_properties
                        .get_pipeline_executable_statistics_khr =
                        to_option(loader(c"vkGetPipelineExecutableStatisticsKHR"));
                    out.khr_pipeline_executable_properties
                        .get_pipeline_executable_internal_representations_khr =
                        to_option(loader(c"vkGetPipelineExecutableInternalRepresentationsKHR"));
                }
                b"VK_EXT_host_image_copy" => {
                    if out.v1_4.copy_memory_to_image.is_none() {
                        out.v1_4.copy_memory_to_image =
                            to_option(loader(c"vkCopyMemoryToImageEXT"));
                    }
                    if out.v1_4.copy_image_to_memory.is_none() {
                        out.v1_4.copy_image_to_memory =
                            to_option(loader(c"vkCopyImageToMemoryEXT"));
                    }
                    if out.v1_4.copy_image_to_image.is_none() {
                        out.v1_4.copy_image_to_image = to_option(loader(c"vkCopyImageToImageEXT"));
                    }
                    if out.v1_4.transition_image_layout.is_none() {
                        out.v1_4.transition_image_layout =
                            to_option(loader(c"vkTransitionImageLayoutEXT"));
                    }
                    if out.v1_4.get_image_subresource_layout2.is_none() {
                        out.v1_4.get_image_subresource_layout2 =
                            to_option(loader(c"vkGetImageSubresourceLayout2EXT"));
                    }
                }
                b"VK_KHR_map_memory2" => {
                    if out.v1_4.map_memory2.is_none() {
                        out.v1_4.map_memory2 = to_option(loader(c"vkMapMemory2KHR"));
                    }
                    if out.v1_4.unmap_memory2.is_none() {
                        out.v1_4.unmap_memory2 = to_option(loader(c"vkUnmapMemory2KHR"));
                    }
                }
                b"VK_EXT_swapchain_maintenance1" => {
                    if out
                        .khr_swapchain_maintenance1
                        .release_swapchain_images_khr
                        .is_none()
                    {
                        out.khr_swapchain_maintenance1.release_swapchain_images_khr =
                            to_option(loader(c"vkReleaseSwapchainImagesEXT"));
                    }
                }
                b"VK_NV_device_generated_commands" => {
                    out.nv_device_generated_commands
                        .get_generated_commands_memory_requirements_nv =
                        to_option(loader(c"vkGetGeneratedCommandsMemoryRequirementsNV"));
                    out.nv_device_generated_commands
                        .create_indirect_commands_layout_nv =
                        to_option(loader(c"vkCreateIndirectCommandsLayoutNV"));
                    out.nv_device_generated_commands
                        .destroy_indirect_commands_layout_nv =
                        to_option(loader(c"vkDestroyIndirectCommandsLayoutNV"));
                }
                b"VK_EXT_private_data" => {
                    if out.v1_3.create_private_data_slot.is_none() {
                        out.v1_3.create_private_data_slot =
                            to_option(loader(c"vkCreatePrivateDataSlotEXT"));
                    }
                    if out.v1_3.destroy_private_data_slot.is_none() {
                        out.v1_3.destroy_private_data_slot =
                            to_option(loader(c"vkDestroyPrivateDataSlotEXT"));
                    }
                    if out.v1_3.set_private_data.is_none() {
                        out.v1_3.set_private_data = to_option(loader(c"vkSetPrivateDataEXT"));
                    }
                    if out.v1_3.get_private_data.is_none() {
                        out.v1_3.get_private_data = to_option(loader(c"vkGetPrivateDataEXT"));
                    }
                }
                b"VK_KHR_video_encode_queue" => {
                    out.khr_video_encode_queue
                        .get_encoded_video_session_parameters_khr =
                        to_option(loader(c"vkGetEncodedVideoSessionParametersKHR"));
                }
                b"VK_NV_cuda_kernel_launch" => {
                    out.nv_cuda_kernel_launch.create_cuda_module_nv =
                        to_option(loader(c"vkCreateCudaModuleNV"));
                    out.nv_cuda_kernel_launch.get_cuda_module_cache_nv =
                        to_option(loader(c"vkGetCudaModuleCacheNV"));
                    out.nv_cuda_kernel_launch.create_cuda_function_nv =
                        to_option(loader(c"vkCreateCudaFunctionNV"));
                    out.nv_cuda_kernel_launch.destroy_cuda_module_nv =
                        to_option(loader(c"vkDestroyCudaModuleNV"));
                    out.nv_cuda_kernel_launch.destroy_cuda_function_nv =
                        to_option(loader(c"vkDestroyCudaFunctionNV"));
                }
                b"VK_NV_low_latency" => {
                    out.nv_low_latency.set_latency_sleep_mode_legacy_nv =
                        to_option(loader(c"vkSetLatencySleepModeLegacyNV"));
                    out.nv_low_latency.latency_sleep_legacy_nv =
                        to_option(loader(c"vkLatencySleepLegacyNV"));
                    out.nv_low_latency.set_latency_marker_legacy_nv =
                        to_option(loader(c"vkSetLatencyMarkerLegacyNV"));
                    out.nv_low_latency.get_latency_timings_legacy_nv =
                        to_option(loader(c"vkGetLatencyTimingsLegacyNV"));
                    out.nv_low_latency.get_sleep_status_legacy_nv =
                        to_option(loader(c"vkGetSleepStatusLegacyNV"));
                    out.nv_low_latency.shutdown_latency_device_legacy_nv =
                        to_option(loader(c"vkShutdownLatencyDeviceLegacyNV"));
                }
                b"VK_EXT_metal_objects" => {
                    out.ext_metal_objects.export_metal_objects_ext =
                        to_option(loader(c"vkExportMetalObjectsEXT"));
                }
                b"VK_EXT_descriptor_buffer" => {
                    out.ext_descriptor_buffer.get_descriptor_set_layout_size_ext =
                        to_option(loader(c"vkGetDescriptorSetLayoutSizeEXT"));
                    out.ext_descriptor_buffer
                        .get_descriptor_set_layout_binding_offset_ext =
                        to_option(loader(c"vkGetDescriptorSetLayoutBindingOffsetEXT"));
                    out.ext_descriptor_buffer.get_descriptor_ext =
                        to_option(loader(c"vkGetDescriptorEXT"));
                    out.ext_descriptor_buffer
                        .get_buffer_opaque_capture_descriptor_data_ext =
                        to_option(loader(c"vkGetBufferOpaqueCaptureDescriptorDataEXT"));
                    out.ext_descriptor_buffer
                        .get_image_opaque_capture_descriptor_data_ext =
                        to_option(loader(c"vkGetImageOpaqueCaptureDescriptorDataEXT"));
                    out.ext_descriptor_buffer
                        .get_image_view_opaque_capture_descriptor_data_ext =
                        to_option(loader(c"vkGetImageViewOpaqueCaptureDescriptorDataEXT"));
                    out.ext_descriptor_buffer
                        .get_sampler_opaque_capture_descriptor_data_ext =
                        to_option(loader(c"vkGetSamplerOpaqueCaptureDescriptorDataEXT"));
                    out.ext_descriptor_buffer
                        .get_acceleration_structure_opaque_capture_descriptor_data_ext = to_option(
                        loader(c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT"),
                    );
                }
                b"VK_KHR_device_address_commands" => {
                    out.khr_device_address_commands
                        .create_acceleration_structure2_khr =
                        to_option(loader(c"vkCreateAccelerationStructure2KHR"));
                }
                b"VK_EXT_image_compression_control" => {
                    if out.v1_4.get_image_subresource_layout2.is_none() {
                        out.v1_4.get_image_subresource_layout2 =
                            to_option(loader(c"vkGetImageSubresourceLayout2EXT"));
                    }
                }
                b"VK_EXT_device_fault" => {
                    out.ext_device_fault.get_device_fault_info_ext =
                        to_option(loader(c"vkGetDeviceFaultInfoEXT"));
                }
                b"VK_FUCHSIA_external_memory" => {
                    out.fuchsia_external_memory.get_memory_zircon_handle_fuchsia =
                        to_option(loader(c"vkGetMemoryZirconHandleFUCHSIA"));
                    out.fuchsia_external_memory
                        .get_memory_zircon_handle_properties_fuchsia =
                        to_option(loader(c"vkGetMemoryZirconHandlePropertiesFUCHSIA"));
                }
                b"VK_FUCHSIA_external_semaphore" => {
                    out.fuchsia_external_semaphore
                        .import_semaphore_zircon_handle_fuchsia =
                        to_option(loader(c"vkImportSemaphoreZirconHandleFUCHSIA"));
                    out.fuchsia_external_semaphore
                        .get_semaphore_zircon_handle_fuchsia =
                        to_option(loader(c"vkGetSemaphoreZirconHandleFUCHSIA"));
                }
                b"VK_FUCHSIA_buffer_collection" => {
                    out.fuchsia_buffer_collection
                        .create_buffer_collection_fuchsia =
                        to_option(loader(c"vkCreateBufferCollectionFUCHSIA"));
                    out.fuchsia_buffer_collection
                        .set_buffer_collection_image_constraints_fuchsia =
                        to_option(loader(c"vkSetBufferCollectionImageConstraintsFUCHSIA"));
                    out.fuchsia_buffer_collection
                        .set_buffer_collection_buffer_constraints_fuchsia =
                        to_option(loader(c"vkSetBufferCollectionBufferConstraintsFUCHSIA"));
                    out.fuchsia_buffer_collection
                        .destroy_buffer_collection_fuchsia =
                        to_option(loader(c"vkDestroyBufferCollectionFUCHSIA"));
                    out.fuchsia_buffer_collection
                        .get_buffer_collection_properties_fuchsia =
                        to_option(loader(c"vkGetBufferCollectionPropertiesFUCHSIA"));
                }
                b"VK_HUAWEI_subpass_shading" => {
                    out.huawei_subpass_shading
                        .get_device_subpass_shading_max_workgroup_size_huawei =
                        to_option(loader(c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI"));
                }
                b"VK_NV_external_memory_rdma" => {
                    out.nv_external_memory_rdma.get_memory_remote_address_nv =
                        to_option(loader(c"vkGetMemoryRemoteAddressNV"));
                }
                b"VK_EXT_pipeline_properties" => {
                    out.ext_pipeline_properties.get_pipeline_properties_ext =
                        to_option(loader(c"vkGetPipelinePropertiesEXT"));
                }
                b"VK_NV_external_sci_sync" => {
                    if out
                        .nv_external_sci_sync2
                        .get_fence_sci_sync_fence_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.get_fence_sci_sync_fence_nv =
                            to_option(loader(c"vkGetFenceSciSyncFenceNV"));
                    }
                    if out
                        .nv_external_sci_sync2
                        .get_fence_sci_sync_obj_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.get_fence_sci_sync_obj_nv =
                            to_option(loader(c"vkGetFenceSciSyncObjNV"));
                    }
                    if out
                        .nv_external_sci_sync2
                        .import_fence_sci_sync_fence_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.import_fence_sci_sync_fence_nv =
                            to_option(loader(c"vkImportFenceSciSyncFenceNV"));
                    }
                    if out
                        .nv_external_sci_sync2
                        .import_fence_sci_sync_obj_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.import_fence_sci_sync_obj_nv =
                            to_option(loader(c"vkImportFenceSciSyncObjNV"));
                    }
                    out.nv_external_sci_sync.get_semaphore_sci_sync_obj_nv =
                        to_option(loader(c"vkGetSemaphoreSciSyncObjNV"));
                    out.nv_external_sci_sync.import_semaphore_sci_sync_obj_nv =
                        to_option(loader(c"vkImportSemaphoreSciSyncObjNV"));
                }
                b"VK_NV_external_memory_sci_buf" => {
                    out.nv_external_memory_sci_buf.get_memory_sci_buf_nv =
                        to_option(loader(c"vkGetMemorySciBufNV"));
                }
                b"VK_EXT_opacity_micromap" => {
                    out.ext_opacity_micromap.create_micromap_ext =
                        to_option(loader(c"vkCreateMicromapEXT"));
                    out.ext_opacity_micromap.destroy_micromap_ext =
                        to_option(loader(c"vkDestroyMicromapEXT"));
                    out.ext_opacity_micromap.build_micromaps_ext =
                        to_option(loader(c"vkBuildMicromapsEXT"));
                    out.ext_opacity_micromap.copy_micromap_ext =
                        to_option(loader(c"vkCopyMicromapEXT"));
                    out.ext_opacity_micromap.copy_micromap_to_memory_ext =
                        to_option(loader(c"vkCopyMicromapToMemoryEXT"));
                    out.ext_opacity_micromap.copy_memory_to_micromap_ext =
                        to_option(loader(c"vkCopyMemoryToMicromapEXT"));
                    out.ext_opacity_micromap.write_micromaps_properties_ext =
                        to_option(loader(c"vkWriteMicromapsPropertiesEXT"));
                    out.ext_opacity_micromap
                        .get_device_micromap_compatibility_ext =
                        to_option(loader(c"vkGetDeviceMicromapCompatibilityEXT"));
                    out.ext_opacity_micromap.get_micromap_build_sizes_ext =
                        to_option(loader(c"vkGetMicromapBuildSizesEXT"));
                }
                b"VK_EXT_pageable_device_local_memory" => {
                    out.ext_pageable_device_local_memory
                        .set_device_memory_priority_ext =
                        to_option(loader(c"vkSetDeviceMemoryPriorityEXT"));
                }
                b"VK_KHR_maintenance4" => {
                    if out.v1_3.get_device_buffer_memory_requirements.is_none() {
                        out.v1_3.get_device_buffer_memory_requirements =
                            to_option(loader(c"vkGetDeviceBufferMemoryRequirementsKHR"));
                    }
                    if out.v1_3.get_device_image_memory_requirements.is_none() {
                        out.v1_3.get_device_image_memory_requirements =
                            to_option(loader(c"vkGetDeviceImageMemoryRequirementsKHR"));
                    }
                    if out
                        .v1_3
                        .get_device_image_sparse_memory_requirements
                        .is_none()
                    {
                        out.v1_3.get_device_image_sparse_memory_requirements =
                            to_option(loader(c"vkGetDeviceImageSparseMemoryRequirementsKHR"));
                    }
                }
                b"VK_VALVE_descriptor_set_host_mapping" => {
                    out.valve_descriptor_set_host_mapping
                        .get_descriptor_set_layout_host_mapping_info_valve =
                        to_option(loader(c"vkGetDescriptorSetLayoutHostMappingInfoVALVE"));
                    out.valve_descriptor_set_host_mapping
                        .get_descriptor_set_host_mapping_valve =
                        to_option(loader(c"vkGetDescriptorSetHostMappingVALVE"));
                }
                b"VK_NV_device_generated_commands_compute" => {
                    out.nv_device_generated_commands_compute
                        .get_pipeline_indirect_memory_requirements_nv =
                        to_option(loader(c"vkGetPipelineIndirectMemoryRequirementsNV"));
                    out.nv_device_generated_commands_compute
                        .get_pipeline_indirect_device_address_nv =
                        to_option(loader(c"vkGetPipelineIndirectDeviceAddressNV"));
                }
                b"VK_OHOS_external_memory" => {
                    out.ohos_external_memory.get_native_buffer_properties_ohos =
                        to_option(loader(c"vkGetNativeBufferPropertiesOHOS"));
                    out.ohos_external_memory.get_memory_native_buffer_ohos =
                        to_option(loader(c"vkGetMemoryNativeBufferOHOS"));
                }
                b"VK_ARM_tensors" => {
                    out.arm_tensors.create_tensor_arm = to_option(loader(c"vkCreateTensorARM"));
                    out.arm_tensors.destroy_tensor_arm = to_option(loader(c"vkDestroyTensorARM"));
                    out.arm_tensors.create_tensor_view_arm =
                        to_option(loader(c"vkCreateTensorViewARM"));
                    out.arm_tensors.destroy_tensor_view_arm =
                        to_option(loader(c"vkDestroyTensorViewARM"));
                    out.arm_tensors.get_tensor_memory_requirements_arm =
                        to_option(loader(c"vkGetTensorMemoryRequirementsARM"));
                    out.arm_tensors.bind_tensor_memory_arm =
                        to_option(loader(c"vkBindTensorMemoryARM"));
                    out.arm_tensors.get_device_tensor_memory_requirements_arm =
                        to_option(loader(c"vkGetDeviceTensorMemoryRequirementsARM"));
                    out.arm_tensors
                        .get_tensor_opaque_capture_descriptor_data_arm =
                        to_option(loader(c"vkGetTensorOpaqueCaptureDescriptorDataARM"));
                    out.arm_tensors
                        .get_tensor_view_opaque_capture_descriptor_data_arm =
                        to_option(loader(c"vkGetTensorViewOpaqueCaptureDescriptorDataARM"));
                }
                b"VK_EXT_shader_module_identifier" => {
                    out.ext_shader_module_identifier
                        .get_shader_module_identifier_ext =
                        to_option(loader(c"vkGetShaderModuleIdentifierEXT"));
                    out.ext_shader_module_identifier
                        .get_shader_module_create_info_identifier_ext =
                        to_option(loader(c"vkGetShaderModuleCreateInfoIdentifierEXT"));
                }
                b"VK_NV_optical_flow" => {
                    out.nv_optical_flow.create_optical_flow_session_nv =
                        to_option(loader(c"vkCreateOpticalFlowSessionNV"));
                    out.nv_optical_flow.destroy_optical_flow_session_nv =
                        to_option(loader(c"vkDestroyOpticalFlowSessionNV"));
                    out.nv_optical_flow.bind_optical_flow_session_image_nv =
                        to_option(loader(c"vkBindOpticalFlowSessionImageNV"));
                }
                b"VK_KHR_maintenance5" => {
                    if out.v1_4.get_rendering_area_granularity.is_none() {
                        out.v1_4.get_rendering_area_granularity =
                            to_option(loader(c"vkGetRenderingAreaGranularityKHR"));
                    }
                    if out.v1_4.get_device_image_subresource_layout.is_none() {
                        out.v1_4.get_device_image_subresource_layout =
                            to_option(loader(c"vkGetDeviceImageSubresourceLayoutKHR"));
                    }
                    if out.v1_4.get_image_subresource_layout2.is_none() {
                        out.v1_4.get_image_subresource_layout2 =
                            to_option(loader(c"vkGetImageSubresourceLayout2KHR"));
                    }
                }
                b"VK_AMD_anti_lag" => {
                    out.amd_anti_lag.anti_lag_update_amd = to_option(loader(c"vkAntiLagUpdateAMD"));
                }
                b"VK_KHR_present_wait2" => {
                    out.khr_present_wait2.wait_for_present2_khr =
                        to_option(loader(c"vkWaitForPresent2KHR"));
                }
                b"VK_EXT_shader_object" => {
                    out.ext_shader_object.create_shaders_ext =
                        to_option(loader(c"vkCreateShadersEXT"));
                    out.ext_shader_object.destroy_shader_ext =
                        to_option(loader(c"vkDestroyShaderEXT"));
                    out.ext_shader_object.get_shader_binary_data_ext =
                        to_option(loader(c"vkGetShaderBinaryDataEXT"));
                }
                b"VK_KHR_pipeline_binary" => {
                    out.khr_pipeline_binary.create_pipeline_binaries_khr =
                        to_option(loader(c"vkCreatePipelineBinariesKHR"));
                    out.khr_pipeline_binary.destroy_pipeline_binary_khr =
                        to_option(loader(c"vkDestroyPipelineBinaryKHR"));
                    out.khr_pipeline_binary.get_pipeline_key_khr =
                        to_option(loader(c"vkGetPipelineKeyKHR"));
                    out.khr_pipeline_binary.get_pipeline_binary_data_khr =
                        to_option(loader(c"vkGetPipelineBinaryDataKHR"));
                    out.khr_pipeline_binary.release_captured_pipeline_data_khr =
                        to_option(loader(c"vkReleaseCapturedPipelineDataKHR"));
                }
                b"VK_QCOM_tile_properties" => {
                    out.qcom_tile_properties
                        .get_framebuffer_tile_properties_qcom =
                        to_option(loader(c"vkGetFramebufferTilePropertiesQCOM"));
                    out.qcom_tile_properties
                        .get_dynamic_rendering_tile_properties_qcom =
                        to_option(loader(c"vkGetDynamicRenderingTilePropertiesQCOM"));
                }
                b"VK_KHR_swapchain_maintenance1" => {
                    if out
                        .khr_swapchain_maintenance1
                        .release_swapchain_images_khr
                        .is_none()
                    {
                        out.khr_swapchain_maintenance1.release_swapchain_images_khr =
                            to_option(loader(c"vkReleaseSwapchainImagesKHR"));
                    }
                }
                b"VK_NV_external_sci_sync2" => {
                    out.nv_external_sci_sync2.create_semaphore_sci_sync_pool_nv =
                        to_option(loader(c"vkCreateSemaphoreSciSyncPoolNV"));
                    out.nv_external_sci_sync2.destroy_semaphore_sci_sync_pool_nv =
                        to_option(loader(c"vkDestroySemaphoreSciSyncPoolNV"));
                    if out
                        .nv_external_sci_sync2
                        .get_fence_sci_sync_fence_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.get_fence_sci_sync_fence_nv =
                            to_option(loader(c"vkGetFenceSciSyncFenceNV"));
                    }
                    if out
                        .nv_external_sci_sync2
                        .get_fence_sci_sync_obj_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.get_fence_sci_sync_obj_nv =
                            to_option(loader(c"vkGetFenceSciSyncObjNV"));
                    }
                    if out
                        .nv_external_sci_sync2
                        .import_fence_sci_sync_fence_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.import_fence_sci_sync_fence_nv =
                            to_option(loader(c"vkImportFenceSciSyncFenceNV"));
                    }
                    if out
                        .nv_external_sci_sync2
                        .import_fence_sci_sync_obj_nv
                        .is_none()
                    {
                        out.nv_external_sci_sync2.import_fence_sci_sync_obj_nv =
                            to_option(loader(c"vkImportFenceSciSyncObjNV"));
                    }
                }
                b"VK_NV_cooperative_vector" => {
                    out.nv_cooperative_vector
                        .convert_cooperative_vector_matrix_nv =
                        to_option(loader(c"vkConvertCooperativeVectorMatrixNV"));
                }
                b"VK_NV_low_latency2" => {
                    out.nv_low_latency2.set_latency_sleep_mode_nv =
                        to_option(loader(c"vkSetLatencySleepModeNV"));
                    out.nv_low_latency2.latency_sleep_nv = to_option(loader(c"vkLatencySleepNV"));
                    out.nv_low_latency2.set_latency_marker_nv =
                        to_option(loader(c"vkSetLatencyMarkerNV"));
                    out.nv_low_latency2.get_latency_timings_nv =
                        to_option(loader(c"vkGetLatencyTimingsNV"));
                }
                b"VK_ARM_data_graph" => {
                    out.arm_data_graph.create_data_graph_pipelines_arm =
                        to_option(loader(c"vkCreateDataGraphPipelinesARM"));
                    out.arm_data_graph.create_data_graph_pipeline_session_arm =
                        to_option(loader(c"vkCreateDataGraphPipelineSessionARM"));
                    out.arm_data_graph
                        .get_data_graph_pipeline_session_bind_point_requirements_arm = to_option(
                        loader(c"vkGetDataGraphPipelineSessionBindPointRequirementsARM"),
                    );
                    out.arm_data_graph
                        .get_data_graph_pipeline_session_memory_requirements_arm = to_option(
                        loader(c"vkGetDataGraphPipelineSessionMemoryRequirementsARM"),
                    );
                    out.arm_data_graph
                        .bind_data_graph_pipeline_session_memory_arm =
                        to_option(loader(c"vkBindDataGraphPipelineSessionMemoryARM"));
                    out.arm_data_graph.destroy_data_graph_pipeline_session_arm =
                        to_option(loader(c"vkDestroyDataGraphPipelineSessionARM"));
                    out.arm_data_graph
                        .get_data_graph_pipeline_available_properties_arm =
                        to_option(loader(c"vkGetDataGraphPipelineAvailablePropertiesARM"));
                    out.arm_data_graph.get_data_graph_pipeline_properties_arm =
                        to_option(loader(c"vkGetDataGraphPipelinePropertiesARM"));
                }
                b"VK_QNX_external_memory_screen_buffer" => {
                    out.qnx_external_memory_screen_buffer
                        .get_screen_buffer_properties_qnx =
                        to_option(loader(c"vkGetScreenBufferPropertiesQNX"));
                }
                b"VK_KHR_calibrated_timestamps" => {
                    if out
                        .khr_calibrated_timestamps
                        .get_calibrated_timestamps_khr
                        .is_none()
                    {
                        out.khr_calibrated_timestamps.get_calibrated_timestamps_khr =
                            to_option(loader(c"vkGetCalibratedTimestampsKHR"));
                    }
                }
                b"VK_NV_external_compute_queue" => {
                    out.nv_external_compute_queue
                        .create_external_compute_queue_nv =
                        to_option(loader(c"vkCreateExternalComputeQueueNV"));
                    out.nv_external_compute_queue
                        .destroy_external_compute_queue_nv =
                        to_option(loader(c"vkDestroyExternalComputeQueueNV"));
                }
                b"VK_NV_cluster_acceleration_structure" => {
                    out.nv_cluster_acceleration_structure
                        .get_cluster_acceleration_structure_build_sizes_nv =
                        to_option(loader(c"vkGetClusterAccelerationStructureBuildSizesNV"));
                }
                b"VK_NV_partitioned_acceleration_structure" => {
                    out.nv_partitioned_acceleration_structure
                        .get_partitioned_acceleration_structures_build_sizes_nv = to_option(
                        loader(c"vkGetPartitionedAccelerationStructuresBuildSizesNV"),
                    );
                }
                b"VK_EXT_device_generated_commands" => {
                    out.ext_device_generated_commands
                        .get_generated_commands_memory_requirements_ext =
                        to_option(loader(c"vkGetGeneratedCommandsMemoryRequirementsEXT"));
                    out.ext_device_generated_commands
                        .create_indirect_commands_layout_ext =
                        to_option(loader(c"vkCreateIndirectCommandsLayoutEXT"));
                    out.ext_device_generated_commands
                        .destroy_indirect_commands_layout_ext =
                        to_option(loader(c"vkDestroyIndirectCommandsLayoutEXT"));
                    out.ext_device_generated_commands
                        .create_indirect_execution_set_ext =
                        to_option(loader(c"vkCreateIndirectExecutionSetEXT"));
                    out.ext_device_generated_commands
                        .destroy_indirect_execution_set_ext =
                        to_option(loader(c"vkDestroyIndirectExecutionSetEXT"));
                    out.ext_device_generated_commands
                        .update_indirect_execution_set_pipeline_ext =
                        to_option(loader(c"vkUpdateIndirectExecutionSetPipelineEXT"));
                    out.ext_device_generated_commands
                        .update_indirect_execution_set_shader_ext =
                        to_option(loader(c"vkUpdateIndirectExecutionSetShaderEXT"));
                }
                b"VK_KHR_device_fault" => {
                    out.khr_device_fault.get_device_fault_reports_khr =
                        to_option(loader(c"vkGetDeviceFaultReportsKHR"));
                    out.khr_device_fault.get_device_fault_debug_info_khr =
                        to_option(loader(c"vkGetDeviceFaultDebugInfoKHR"));
                }
                b"VK_EXT_external_memory_metal" => {
                    out.ext_external_memory_metal.get_memory_metal_handle_ext =
                        to_option(loader(c"vkGetMemoryMetalHandleEXT"));
                    out.ext_external_memory_metal
                        .get_memory_metal_handle_properties_ext =
                        to_option(loader(c"vkGetMemoryMetalHandlePropertiesEXT"));
                }
                b"VK_ARM_shader_instrumentation" => {
                    out.arm_shader_instrumentation
                        .create_shader_instrumentation_arm =
                        to_option(loader(c"vkCreateShaderInstrumentationARM"));
                    out.arm_shader_instrumentation
                        .destroy_shader_instrumentation_arm =
                        to_option(loader(c"vkDestroyShaderInstrumentationARM"));
                    out.arm_shader_instrumentation
                        .get_shader_instrumentation_values_arm =
                        to_option(loader(c"vkGetShaderInstrumentationValuesARM"));
                    out.arm_shader_instrumentation
                        .clear_shader_instrumentation_metrics_arm =
                        to_option(loader(c"vkClearShaderInstrumentationMetricsARM"));
                }
                _ => (),
            }
        }
        out
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_0 {
    pub destroy_device: Option<vkDestroyDevice>,
    pub get_device_queue: Option<vkGetDeviceQueue>,
    pub device_wait_idle: Option<vkDeviceWaitIdle>,
    pub allocate_memory: Option<vkAllocateMemory>,
    pub free_memory: Option<vkFreeMemory>,
    pub map_memory: Option<vkMapMemory>,
    pub unmap_memory: Option<vkUnmapMemory>,
    pub flush_mapped_memory_ranges: Option<vkFlushMappedMemoryRanges>,
    pub invalidate_mapped_memory_ranges: Option<vkInvalidateMappedMemoryRanges>,
    pub get_device_memory_commitment: Option<vkGetDeviceMemoryCommitment>,
    pub get_buffer_memory_requirements: Option<vkGetBufferMemoryRequirements>,
    pub bind_buffer_memory: Option<vkBindBufferMemory>,
    pub get_image_memory_requirements: Option<vkGetImageMemoryRequirements>,
    pub bind_image_memory: Option<vkBindImageMemory>,
    pub get_image_sparse_memory_requirements: Option<vkGetImageSparseMemoryRequirements>,
    pub create_fence: Option<vkCreateFence>,
    pub destroy_fence: Option<vkDestroyFence>,
    pub reset_fences: Option<vkResetFences>,
    pub get_fence_status: Option<vkGetFenceStatus>,
    pub wait_for_fences: Option<vkWaitForFences>,
    pub create_semaphore: Option<vkCreateSemaphore>,
    pub destroy_semaphore: Option<vkDestroySemaphore>,
    pub create_event: Option<vkCreateEvent>,
    pub destroy_event: Option<vkDestroyEvent>,
    pub get_event_status: Option<vkGetEventStatus>,
    pub set_event: Option<vkSetEvent>,
    pub reset_event: Option<vkResetEvent>,
    pub create_query_pool: Option<vkCreateQueryPool>,
    pub destroy_query_pool: Option<vkDestroyQueryPool>,
    pub get_query_pool_results: Option<vkGetQueryPoolResults>,
    pub create_buffer: Option<vkCreateBuffer>,
    pub destroy_buffer: Option<vkDestroyBuffer>,
    pub create_buffer_view: Option<vkCreateBufferView>,
    pub destroy_buffer_view: Option<vkDestroyBufferView>,
    pub create_image: Option<vkCreateImage>,
    pub destroy_image: Option<vkDestroyImage>,
    pub get_image_subresource_layout: Option<vkGetImageSubresourceLayout>,
    pub create_image_view: Option<vkCreateImageView>,
    pub destroy_image_view: Option<vkDestroyImageView>,
    pub create_shader_module: Option<vkCreateShaderModule>,
    pub destroy_shader_module: Option<vkDestroyShaderModule>,
    pub create_pipeline_cache: Option<vkCreatePipelineCache>,
    pub destroy_pipeline_cache: Option<vkDestroyPipelineCache>,
    pub get_pipeline_cache_data: Option<vkGetPipelineCacheData>,
    pub merge_pipeline_caches: Option<vkMergePipelineCaches>,
    pub create_graphics_pipelines: Option<vkCreateGraphicsPipelines>,
    pub create_compute_pipelines: Option<vkCreateComputePipelines>,
    pub destroy_pipeline: Option<vkDestroyPipeline>,
    pub create_pipeline_layout: Option<vkCreatePipelineLayout>,
    pub destroy_pipeline_layout: Option<vkDestroyPipelineLayout>,
    pub create_sampler: Option<vkCreateSampler>,
    pub destroy_sampler: Option<vkDestroySampler>,
    pub create_descriptor_set_layout: Option<vkCreateDescriptorSetLayout>,
    pub destroy_descriptor_set_layout: Option<vkDestroyDescriptorSetLayout>,
    pub create_descriptor_pool: Option<vkCreateDescriptorPool>,
    pub destroy_descriptor_pool: Option<vkDestroyDescriptorPool>,
    pub reset_descriptor_pool: Option<vkResetDescriptorPool>,
    pub allocate_descriptor_sets: Option<vkAllocateDescriptorSets>,
    pub free_descriptor_sets: Option<vkFreeDescriptorSets>,
    pub update_descriptor_sets: Option<vkUpdateDescriptorSets>,
    pub create_framebuffer: Option<vkCreateFramebuffer>,
    pub destroy_framebuffer: Option<vkDestroyFramebuffer>,
    pub create_render_pass: Option<vkCreateRenderPass>,
    pub destroy_render_pass: Option<vkDestroyRenderPass>,
    pub get_render_area_granularity: Option<vkGetRenderAreaGranularity>,
    pub create_command_pool: Option<vkCreateCommandPool>,
    pub destroy_command_pool: Option<vkDestroyCommandPool>,
    pub reset_command_pool: Option<vkResetCommandPool>,
    pub allocate_command_buffers: Option<vkAllocateCommandBuffers>,
    pub free_command_buffers: Option<vkFreeCommandBuffers>,
    pub get_fault_data: Option<vkGetFaultData>,
    pub get_command_pool_memory_consumption: Option<vkGetCommandPoolMemoryConsumption>,
}

impl DeviceFnv1_0 {
    pub const EMPTY: Self = Self {
        destroy_device: None,
        get_device_queue: None,
        device_wait_idle: None,
        allocate_memory: None,
        free_memory: None,
        map_memory: None,
        unmap_memory: None,
        flush_mapped_memory_ranges: None,
        invalidate_mapped_memory_ranges: None,
        get_device_memory_commitment: None,
        get_buffer_memory_requirements: None,
        bind_buffer_memory: None,
        get_image_memory_requirements: None,
        bind_image_memory: None,
        get_image_sparse_memory_requirements: None,
        create_fence: None,
        destroy_fence: None,
        reset_fences: None,
        get_fence_status: None,
        wait_for_fences: None,
        create_semaphore: None,
        destroy_semaphore: None,
        create_event: None,
        destroy_event: None,
        get_event_status: None,
        set_event: None,
        reset_event: None,
        create_query_pool: None,
        destroy_query_pool: None,
        get_query_pool_results: None,
        create_buffer: None,
        destroy_buffer: None,
        create_buffer_view: None,
        destroy_buffer_view: None,
        create_image: None,
        destroy_image: None,
        get_image_subresource_layout: None,
        create_image_view: None,
        destroy_image_view: None,
        create_shader_module: None,
        destroy_shader_module: None,
        create_pipeline_cache: None,
        destroy_pipeline_cache: None,
        get_pipeline_cache_data: None,
        merge_pipeline_caches: None,
        create_graphics_pipelines: None,
        create_compute_pipelines: None,
        destroy_pipeline: None,
        create_pipeline_layout: None,
        destroy_pipeline_layout: None,
        create_sampler: None,
        destroy_sampler: None,
        create_descriptor_set_layout: None,
        destroy_descriptor_set_layout: None,
        create_descriptor_pool: None,
        destroy_descriptor_pool: None,
        reset_descriptor_pool: None,
        allocate_descriptor_sets: None,
        free_descriptor_sets: None,
        update_descriptor_sets: None,
        create_framebuffer: None,
        destroy_framebuffer: None,
        create_render_pass: None,
        destroy_render_pass: None,
        get_render_area_granularity: None,
        create_command_pool: None,
        destroy_command_pool: None,
        reset_command_pool: None,
        allocate_command_buffers: None,
        free_command_buffers: None,
        get_fault_data: None,
        get_command_pool_memory_consumption: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            destroy_device: to_option(loader(c"vkDestroyDevice")),
            get_device_queue: to_option(loader(c"vkGetDeviceQueue")),
            device_wait_idle: to_option(loader(c"vkDeviceWaitIdle")),
            allocate_memory: to_option(loader(c"vkAllocateMemory")),
            free_memory: to_option(loader(c"vkFreeMemory")),
            map_memory: to_option(loader(c"vkMapMemory")),
            unmap_memory: to_option(loader(c"vkUnmapMemory")),
            flush_mapped_memory_ranges: to_option(loader(c"vkFlushMappedMemoryRanges")),
            invalidate_mapped_memory_ranges: to_option(loader(c"vkInvalidateMappedMemoryRanges")),
            get_device_memory_commitment: to_option(loader(c"vkGetDeviceMemoryCommitment")),
            get_buffer_memory_requirements: to_option(loader(c"vkGetBufferMemoryRequirements")),
            bind_buffer_memory: to_option(loader(c"vkBindBufferMemory")),
            get_image_memory_requirements: to_option(loader(c"vkGetImageMemoryRequirements")),
            bind_image_memory: to_option(loader(c"vkBindImageMemory")),
            get_image_sparse_memory_requirements: to_option(loader(
                c"vkGetImageSparseMemoryRequirements",
            )),
            create_fence: to_option(loader(c"vkCreateFence")),
            destroy_fence: to_option(loader(c"vkDestroyFence")),
            reset_fences: to_option(loader(c"vkResetFences")),
            get_fence_status: to_option(loader(c"vkGetFenceStatus")),
            wait_for_fences: to_option(loader(c"vkWaitForFences")),
            create_semaphore: to_option(loader(c"vkCreateSemaphore")),
            destroy_semaphore: to_option(loader(c"vkDestroySemaphore")),
            create_event: to_option(loader(c"vkCreateEvent")),
            destroy_event: to_option(loader(c"vkDestroyEvent")),
            get_event_status: to_option(loader(c"vkGetEventStatus")),
            set_event: to_option(loader(c"vkSetEvent")),
            reset_event: to_option(loader(c"vkResetEvent")),
            create_query_pool: to_option(loader(c"vkCreateQueryPool")),
            destroy_query_pool: to_option(loader(c"vkDestroyQueryPool")),
            get_query_pool_results: to_option(loader(c"vkGetQueryPoolResults")),
            create_buffer: to_option(loader(c"vkCreateBuffer")),
            destroy_buffer: to_option(loader(c"vkDestroyBuffer")),
            create_buffer_view: to_option(loader(c"vkCreateBufferView")),
            destroy_buffer_view: to_option(loader(c"vkDestroyBufferView")),
            create_image: to_option(loader(c"vkCreateImage")),
            destroy_image: to_option(loader(c"vkDestroyImage")),
            get_image_subresource_layout: to_option(loader(c"vkGetImageSubresourceLayout")),
            create_image_view: to_option(loader(c"vkCreateImageView")),
            destroy_image_view: to_option(loader(c"vkDestroyImageView")),
            create_shader_module: to_option(loader(c"vkCreateShaderModule")),
            destroy_shader_module: to_option(loader(c"vkDestroyShaderModule")),
            create_pipeline_cache: to_option(loader(c"vkCreatePipelineCache")),
            destroy_pipeline_cache: to_option(loader(c"vkDestroyPipelineCache")),
            get_pipeline_cache_data: to_option(loader(c"vkGetPipelineCacheData")),
            merge_pipeline_caches: to_option(loader(c"vkMergePipelineCaches")),
            create_graphics_pipelines: to_option(loader(c"vkCreateGraphicsPipelines")),
            create_compute_pipelines: to_option(loader(c"vkCreateComputePipelines")),
            destroy_pipeline: to_option(loader(c"vkDestroyPipeline")),
            create_pipeline_layout: to_option(loader(c"vkCreatePipelineLayout")),
            destroy_pipeline_layout: to_option(loader(c"vkDestroyPipelineLayout")),
            create_sampler: to_option(loader(c"vkCreateSampler")),
            destroy_sampler: to_option(loader(c"vkDestroySampler")),
            create_descriptor_set_layout: to_option(loader(c"vkCreateDescriptorSetLayout")),
            destroy_descriptor_set_layout: to_option(loader(c"vkDestroyDescriptorSetLayout")),
            create_descriptor_pool: to_option(loader(c"vkCreateDescriptorPool")),
            destroy_descriptor_pool: to_option(loader(c"vkDestroyDescriptorPool")),
            reset_descriptor_pool: to_option(loader(c"vkResetDescriptorPool")),
            allocate_descriptor_sets: to_option(loader(c"vkAllocateDescriptorSets")),
            free_descriptor_sets: to_option(loader(c"vkFreeDescriptorSets")),
            update_descriptor_sets: to_option(loader(c"vkUpdateDescriptorSets")),
            create_framebuffer: to_option(loader(c"vkCreateFramebuffer")),
            destroy_framebuffer: to_option(loader(c"vkDestroyFramebuffer")),
            create_render_pass: to_option(loader(c"vkCreateRenderPass")),
            destroy_render_pass: to_option(loader(c"vkDestroyRenderPass")),
            get_render_area_granularity: to_option(loader(c"vkGetRenderAreaGranularity")),
            create_command_pool: to_option(loader(c"vkCreateCommandPool")),
            destroy_command_pool: to_option(loader(c"vkDestroyCommandPool")),
            reset_command_pool: to_option(loader(c"vkResetCommandPool")),
            allocate_command_buffers: to_option(loader(c"vkAllocateCommandBuffers")),
            free_command_buffers: to_option(loader(c"vkFreeCommandBuffers")),
            get_fault_data: to_option(loader(c"vkGetFaultData")),
            get_command_pool_memory_consumption: to_option(loader(
                c"vkGetCommandPoolMemoryConsumption",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_1 {
    pub trim_command_pool: Option<vkTrimCommandPool>,
    pub get_device_group_peer_memory_features: Option<vkGetDeviceGroupPeerMemoryFeatures>,
    pub bind_buffer_memory2: Option<vkBindBufferMemory2>,
    pub bind_image_memory2: Option<vkBindImageMemory2>,
    pub create_descriptor_update_template: Option<vkCreateDescriptorUpdateTemplate>,
    pub destroy_descriptor_update_template: Option<vkDestroyDescriptorUpdateTemplate>,
    pub update_descriptor_set_with_template: Option<vkUpdateDescriptorSetWithTemplate>,
    pub get_buffer_memory_requirements2: Option<vkGetBufferMemoryRequirements2>,
    pub get_image_memory_requirements2: Option<vkGetImageMemoryRequirements2>,
    pub get_image_sparse_memory_requirements2: Option<vkGetImageSparseMemoryRequirements2>,
    pub create_sampler_ycbcr_conversion: Option<vkCreateSamplerYcbcrConversion>,
    pub destroy_sampler_ycbcr_conversion: Option<vkDestroySamplerYcbcrConversion>,
    pub get_device_queue2: Option<vkGetDeviceQueue2>,
    pub get_descriptor_set_layout_support: Option<vkGetDescriptorSetLayoutSupport>,
}

impl DeviceFnv1_1 {
    pub const EMPTY: Self = Self {
        trim_command_pool: None,
        get_device_group_peer_memory_features: None,
        bind_buffer_memory2: None,
        bind_image_memory2: None,
        create_descriptor_update_template: None,
        destroy_descriptor_update_template: None,
        update_descriptor_set_with_template: None,
        get_buffer_memory_requirements2: None,
        get_image_memory_requirements2: None,
        get_image_sparse_memory_requirements2: None,
        create_sampler_ycbcr_conversion: None,
        destroy_sampler_ycbcr_conversion: None,
        get_device_queue2: None,
        get_descriptor_set_layout_support: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            trim_command_pool: to_option(loader(c"vkTrimCommandPool")),
            get_device_group_peer_memory_features: to_option(loader(
                c"vkGetDeviceGroupPeerMemoryFeatures",
            )),
            bind_buffer_memory2: to_option(loader(c"vkBindBufferMemory2")),
            bind_image_memory2: to_option(loader(c"vkBindImageMemory2")),
            create_descriptor_update_template: to_option(loader(
                c"vkCreateDescriptorUpdateTemplate",
            )),
            destroy_descriptor_update_template: to_option(loader(
                c"vkDestroyDescriptorUpdateTemplate",
            )),
            update_descriptor_set_with_template: to_option(loader(
                c"vkUpdateDescriptorSetWithTemplate",
            )),
            get_buffer_memory_requirements2: to_option(loader(c"vkGetBufferMemoryRequirements2")),
            get_image_memory_requirements2: to_option(loader(c"vkGetImageMemoryRequirements2")),
            get_image_sparse_memory_requirements2: to_option(loader(
                c"vkGetImageSparseMemoryRequirements2",
            )),
            create_sampler_ycbcr_conversion: to_option(loader(c"vkCreateSamplerYcbcrConversion")),
            destroy_sampler_ycbcr_conversion: to_option(loader(c"vkDestroySamplerYcbcrConversion")),
            get_device_queue2: to_option(loader(c"vkGetDeviceQueue2")),
            get_descriptor_set_layout_support: to_option(loader(
                c"vkGetDescriptorSetLayoutSupport",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_2 {
    pub reset_query_pool: Option<vkResetQueryPool>,
    pub create_render_pass2: Option<vkCreateRenderPass2>,
    pub get_semaphore_counter_value: Option<vkGetSemaphoreCounterValue>,
    pub wait_semaphores: Option<vkWaitSemaphores>,
    pub signal_semaphore: Option<vkSignalSemaphore>,
    pub get_buffer_opaque_capture_address: Option<vkGetBufferOpaqueCaptureAddress>,
    pub get_buffer_device_address: Option<vkGetBufferDeviceAddress>,
    pub get_device_memory_opaque_capture_address: Option<vkGetDeviceMemoryOpaqueCaptureAddress>,
}

impl DeviceFnv1_2 {
    pub const EMPTY: Self = Self {
        reset_query_pool: None,
        create_render_pass2: None,
        get_semaphore_counter_value: None,
        wait_semaphores: None,
        signal_semaphore: None,
        get_buffer_opaque_capture_address: None,
        get_buffer_device_address: None,
        get_device_memory_opaque_capture_address: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            reset_query_pool: to_option(loader(c"vkResetQueryPool")),
            create_render_pass2: to_option(loader(c"vkCreateRenderPass2")),
            get_semaphore_counter_value: to_option(loader(c"vkGetSemaphoreCounterValue")),
            wait_semaphores: to_option(loader(c"vkWaitSemaphores")),
            signal_semaphore: to_option(loader(c"vkSignalSemaphore")),
            get_buffer_opaque_capture_address: to_option(loader(
                c"vkGetBufferOpaqueCaptureAddress",
            )),
            get_buffer_device_address: to_option(loader(c"vkGetBufferDeviceAddress")),
            get_device_memory_opaque_capture_address: to_option(loader(
                c"vkGetDeviceMemoryOpaqueCaptureAddress",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_3 {
    pub get_device_buffer_memory_requirements: Option<vkGetDeviceBufferMemoryRequirements>,
    pub get_device_image_memory_requirements: Option<vkGetDeviceImageMemoryRequirements>,
    pub get_device_image_sparse_memory_requirements:
        Option<vkGetDeviceImageSparseMemoryRequirements>,
    pub create_private_data_slot: Option<vkCreatePrivateDataSlot>,
    pub destroy_private_data_slot: Option<vkDestroyPrivateDataSlot>,
    pub set_private_data: Option<vkSetPrivateData>,
    pub get_private_data: Option<vkGetPrivateData>,
}

impl DeviceFnv1_3 {
    pub const EMPTY: Self = Self {
        get_device_buffer_memory_requirements: None,
        get_device_image_memory_requirements: None,
        get_device_image_sparse_memory_requirements: None,
        create_private_data_slot: None,
        destroy_private_data_slot: None,
        set_private_data: None,
        get_private_data: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_device_buffer_memory_requirements: to_option(loader(
                c"vkGetDeviceBufferMemoryRequirements",
            )),
            get_device_image_memory_requirements: to_option(loader(
                c"vkGetDeviceImageMemoryRequirements",
            )),
            get_device_image_sparse_memory_requirements: to_option(loader(
                c"vkGetDeviceImageSparseMemoryRequirements",
            )),
            create_private_data_slot: to_option(loader(c"vkCreatePrivateDataSlot")),
            destroy_private_data_slot: to_option(loader(c"vkDestroyPrivateDataSlot")),
            set_private_data: to_option(loader(c"vkSetPrivateData")),
            get_private_data: to_option(loader(c"vkGetPrivateData")),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_4 {
    pub get_rendering_area_granularity: Option<vkGetRenderingAreaGranularity>,
    pub copy_memory_to_image: Option<vkCopyMemoryToImage>,
    pub copy_image_to_memory: Option<vkCopyImageToMemory>,
    pub copy_image_to_image: Option<vkCopyImageToImage>,
    pub transition_image_layout: Option<vkTransitionImageLayout>,
    pub get_image_subresource_layout2: Option<vkGetImageSubresourceLayout2>,
    pub get_device_image_subresource_layout: Option<vkGetDeviceImageSubresourceLayout>,
    pub map_memory2: Option<vkMapMemory2>,
    pub unmap_memory2: Option<vkUnmapMemory2>,
}

impl DeviceFnv1_4 {
    pub const EMPTY: Self = Self {
        get_rendering_area_granularity: None,
        copy_memory_to_image: None,
        copy_image_to_memory: None,
        copy_image_to_image: None,
        transition_image_layout: None,
        get_image_subresource_layout2: None,
        get_device_image_subresource_layout: None,
        map_memory2: None,
        unmap_memory2: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_rendering_area_granularity: to_option(loader(c"vkGetRenderingAreaGranularity")),
            copy_memory_to_image: to_option(loader(c"vkCopyMemoryToImage")),
            copy_image_to_memory: to_option(loader(c"vkCopyImageToMemory")),
            copy_image_to_image: to_option(loader(c"vkCopyImageToImage")),
            transition_image_layout: to_option(loader(c"vkTransitionImageLayout")),
            get_image_subresource_layout2: to_option(loader(c"vkGetImageSubresourceLayout2")),
            get_device_image_subresource_layout: to_option(loader(
                c"vkGetDeviceImageSubresourceLayout",
            )),
            map_memory2: to_option(loader(c"vkMapMemory2")),
            unmap_memory2: to_option(loader(c"vkUnmapMemory2")),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnAmdAntiLag {
    pub anti_lag_update_amd: Option<vkAntiLagUpdateAMD>,
}

impl DeviceFnAmdAntiLag {
    pub const EMPTY: Self = Self {
        anti_lag_update_amd: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnAmdDisplayNativeHdr {
    pub set_local_dimming_amd: Option<vkSetLocalDimmingAMD>,
}

impl DeviceFnAmdDisplayNativeHdr {
    pub const EMPTY: Self = Self {
        set_local_dimming_amd: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnAmdGpaInterface {
    pub create_gpa_session_amd: Option<vkCreateGpaSessionAMD>,
    pub destroy_gpa_session_amd: Option<vkDestroyGpaSessionAMD>,
    pub set_gpa_device_clock_mode_amd: Option<vkSetGpaDeviceClockModeAMD>,
    pub get_gpa_device_clock_info_amd: Option<vkGetGpaDeviceClockInfoAMD>,
    pub get_gpa_session_status_amd: Option<vkGetGpaSessionStatusAMD>,
    pub get_gpa_session_results_amd: Option<vkGetGpaSessionResultsAMD>,
    pub reset_gpa_session_amd: Option<vkResetGpaSessionAMD>,
}

impl DeviceFnAmdGpaInterface {
    pub const EMPTY: Self = Self {
        create_gpa_session_amd: None,
        destroy_gpa_session_amd: None,
        set_gpa_device_clock_mode_amd: None,
        get_gpa_device_clock_info_amd: None,
        get_gpa_session_status_amd: None,
        get_gpa_session_results_amd: None,
        reset_gpa_session_amd: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnAmdShaderInfo {
    pub get_shader_info_amd: Option<vkGetShaderInfoAMD>,
}

impl DeviceFnAmdShaderInfo {
    pub const EMPTY: Self = Self {
        get_shader_info_amd: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnAmdxShaderEnqueue {
    pub get_execution_graph_pipeline_scratch_size_amdx:
        Option<vkGetExecutionGraphPipelineScratchSizeAMDX>,
    pub get_execution_graph_pipeline_node_index_amdx:
        Option<vkGetExecutionGraphPipelineNodeIndexAMDX>,
    pub create_execution_graph_pipelines_amdx: Option<vkCreateExecutionGraphPipelinesAMDX>,
}

impl DeviceFnAmdxShaderEnqueue {
    pub const EMPTY: Self = Self {
        get_execution_graph_pipeline_scratch_size_amdx: None,
        get_execution_graph_pipeline_node_index_amdx: None,
        create_execution_graph_pipelines_amdx: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnAndroidExternalMemoryAndroidHardwareBuffer {
    pub get_android_hardware_buffer_properties_android:
        Option<vkGetAndroidHardwareBufferPropertiesANDROID>,
    pub get_memory_android_hardware_buffer_android: Option<vkGetMemoryAndroidHardwareBufferANDROID>,
}

impl DeviceFnAndroidExternalMemoryAndroidHardwareBuffer {
    pub const EMPTY: Self = Self {
        get_android_hardware_buffer_properties_android: None,
        get_memory_android_hardware_buffer_android: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnArmDataGraph {
    pub create_data_graph_pipelines_arm: Option<vkCreateDataGraphPipelinesARM>,
    pub create_data_graph_pipeline_session_arm: Option<vkCreateDataGraphPipelineSessionARM>,
    pub get_data_graph_pipeline_session_bind_point_requirements_arm:
        Option<vkGetDataGraphPipelineSessionBindPointRequirementsARM>,
    pub get_data_graph_pipeline_session_memory_requirements_arm:
        Option<vkGetDataGraphPipelineSessionMemoryRequirementsARM>,
    pub bind_data_graph_pipeline_session_memory_arm:
        Option<vkBindDataGraphPipelineSessionMemoryARM>,
    pub destroy_data_graph_pipeline_session_arm: Option<vkDestroyDataGraphPipelineSessionARM>,
    pub get_data_graph_pipeline_available_properties_arm:
        Option<vkGetDataGraphPipelineAvailablePropertiesARM>,
    pub get_data_graph_pipeline_properties_arm: Option<vkGetDataGraphPipelinePropertiesARM>,
}

impl DeviceFnArmDataGraph {
    pub const EMPTY: Self = Self {
        create_data_graph_pipelines_arm: None,
        create_data_graph_pipeline_session_arm: None,
        get_data_graph_pipeline_session_bind_point_requirements_arm: None,
        get_data_graph_pipeline_session_memory_requirements_arm: None,
        bind_data_graph_pipeline_session_memory_arm: None,
        destroy_data_graph_pipeline_session_arm: None,
        get_data_graph_pipeline_available_properties_arm: None,
        get_data_graph_pipeline_properties_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnArmShaderInstrumentation {
    pub create_shader_instrumentation_arm: Option<vkCreateShaderInstrumentationARM>,
    pub destroy_shader_instrumentation_arm: Option<vkDestroyShaderInstrumentationARM>,
    pub get_shader_instrumentation_values_arm: Option<vkGetShaderInstrumentationValuesARM>,
    pub clear_shader_instrumentation_metrics_arm: Option<vkClearShaderInstrumentationMetricsARM>,
}

impl DeviceFnArmShaderInstrumentation {
    pub const EMPTY: Self = Self {
        create_shader_instrumentation_arm: None,
        destroy_shader_instrumentation_arm: None,
        get_shader_instrumentation_values_arm: None,
        clear_shader_instrumentation_metrics_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnArmTensors {
    pub create_tensor_arm: Option<vkCreateTensorARM>,
    pub destroy_tensor_arm: Option<vkDestroyTensorARM>,
    pub create_tensor_view_arm: Option<vkCreateTensorViewARM>,
    pub destroy_tensor_view_arm: Option<vkDestroyTensorViewARM>,
    pub get_tensor_memory_requirements_arm: Option<vkGetTensorMemoryRequirementsARM>,
    pub bind_tensor_memory_arm: Option<vkBindTensorMemoryARM>,
    pub get_device_tensor_memory_requirements_arm: Option<vkGetDeviceTensorMemoryRequirementsARM>,
    pub get_tensor_opaque_capture_descriptor_data_arm:
        Option<vkGetTensorOpaqueCaptureDescriptorDataARM>,
    pub get_tensor_view_opaque_capture_descriptor_data_arm:
        Option<vkGetTensorViewOpaqueCaptureDescriptorDataARM>,
}

impl DeviceFnArmTensors {
    pub const EMPTY: Self = Self {
        create_tensor_arm: None,
        destroy_tensor_arm: None,
        create_tensor_view_arm: None,
        destroy_tensor_view_arm: None,
        get_tensor_memory_requirements_arm: None,
        bind_tensor_memory_arm: None,
        get_device_tensor_memory_requirements_arm: None,
        get_tensor_opaque_capture_descriptor_data_arm: None,
        get_tensor_view_opaque_capture_descriptor_data_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDebugMarker {
    pub debug_marker_set_object_name_ext: Option<vkDebugMarkerSetObjectNameEXT>,
    pub debug_marker_set_object_tag_ext: Option<vkDebugMarkerSetObjectTagEXT>,
}

impl DeviceFnExtDebugMarker {
    pub const EMPTY: Self = Self {
        debug_marker_set_object_name_ext: None,
        debug_marker_set_object_tag_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDebugUtils {
    pub set_debug_utils_object_name_ext: Option<vkSetDebugUtilsObjectNameEXT>,
    pub set_debug_utils_object_tag_ext: Option<vkSetDebugUtilsObjectTagEXT>,
}

impl DeviceFnExtDebugUtils {
    pub const EMPTY: Self = Self {
        set_debug_utils_object_name_ext: None,
        set_debug_utils_object_tag_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDescriptorBuffer {
    pub get_descriptor_set_layout_size_ext: Option<vkGetDescriptorSetLayoutSizeEXT>,
    pub get_descriptor_set_layout_binding_offset_ext:
        Option<vkGetDescriptorSetLayoutBindingOffsetEXT>,
    pub get_descriptor_ext: Option<vkGetDescriptorEXT>,
    pub get_buffer_opaque_capture_descriptor_data_ext:
        Option<vkGetBufferOpaqueCaptureDescriptorDataEXT>,
    pub get_image_opaque_capture_descriptor_data_ext:
        Option<vkGetImageOpaqueCaptureDescriptorDataEXT>,
    pub get_image_view_opaque_capture_descriptor_data_ext:
        Option<vkGetImageViewOpaqueCaptureDescriptorDataEXT>,
    pub get_sampler_opaque_capture_descriptor_data_ext:
        Option<vkGetSamplerOpaqueCaptureDescriptorDataEXT>,
    pub get_acceleration_structure_opaque_capture_descriptor_data_ext:
        Option<vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT>,
}

impl DeviceFnExtDescriptorBuffer {
    pub const EMPTY: Self = Self {
        get_descriptor_set_layout_size_ext: None,
        get_descriptor_set_layout_binding_offset_ext: None,
        get_descriptor_ext: None,
        get_buffer_opaque_capture_descriptor_data_ext: None,
        get_image_opaque_capture_descriptor_data_ext: None,
        get_image_view_opaque_capture_descriptor_data_ext: None,
        get_sampler_opaque_capture_descriptor_data_ext: None,
        get_acceleration_structure_opaque_capture_descriptor_data_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDescriptorHeap {
    pub write_sampler_descriptors_ext: Option<vkWriteSamplerDescriptorsEXT>,
    pub write_resource_descriptors_ext: Option<vkWriteResourceDescriptorsEXT>,
    pub register_custom_border_color_ext: Option<vkRegisterCustomBorderColorEXT>,
    pub unregister_custom_border_color_ext: Option<vkUnregisterCustomBorderColorEXT>,
    pub get_image_opaque_capture_data_ext: Option<vkGetImageOpaqueCaptureDataEXT>,
    pub get_tensor_opaque_capture_data_arm: Option<vkGetTensorOpaqueCaptureDataARM>,
}

impl DeviceFnExtDescriptorHeap {
    pub const EMPTY: Self = Self {
        write_sampler_descriptors_ext: None,
        write_resource_descriptors_ext: None,
        register_custom_border_color_ext: None,
        unregister_custom_border_color_ext: None,
        get_image_opaque_capture_data_ext: None,
        get_tensor_opaque_capture_data_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDeviceFault {
    pub get_device_fault_info_ext: Option<vkGetDeviceFaultInfoEXT>,
}

impl DeviceFnExtDeviceFault {
    pub const EMPTY: Self = Self {
        get_device_fault_info_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDeviceGeneratedCommands {
    pub get_generated_commands_memory_requirements_ext:
        Option<vkGetGeneratedCommandsMemoryRequirementsEXT>,
    pub create_indirect_commands_layout_ext: Option<vkCreateIndirectCommandsLayoutEXT>,
    pub destroy_indirect_commands_layout_ext: Option<vkDestroyIndirectCommandsLayoutEXT>,
    pub create_indirect_execution_set_ext: Option<vkCreateIndirectExecutionSetEXT>,
    pub destroy_indirect_execution_set_ext: Option<vkDestroyIndirectExecutionSetEXT>,
    pub update_indirect_execution_set_pipeline_ext: Option<vkUpdateIndirectExecutionSetPipelineEXT>,
    pub update_indirect_execution_set_shader_ext: Option<vkUpdateIndirectExecutionSetShaderEXT>,
}

impl DeviceFnExtDeviceGeneratedCommands {
    pub const EMPTY: Self = Self {
        get_generated_commands_memory_requirements_ext: None,
        create_indirect_commands_layout_ext: None,
        destroy_indirect_commands_layout_ext: None,
        create_indirect_execution_set_ext: None,
        destroy_indirect_execution_set_ext: None,
        update_indirect_execution_set_pipeline_ext: None,
        update_indirect_execution_set_shader_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtDisplayControl {
    pub display_power_control_ext: Option<vkDisplayPowerControlEXT>,
    pub register_device_event_ext: Option<vkRegisterDeviceEventEXT>,
    pub register_display_event_ext: Option<vkRegisterDisplayEventEXT>,
    pub get_swapchain_counter_ext: Option<vkGetSwapchainCounterEXT>,
}

impl DeviceFnExtDisplayControl {
    pub const EMPTY: Self = Self {
        display_power_control_ext: None,
        register_device_event_ext: None,
        register_display_event_ext: None,
        get_swapchain_counter_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtExternalMemoryHost {
    pub get_memory_host_pointer_properties_ext: Option<vkGetMemoryHostPointerPropertiesEXT>,
}

impl DeviceFnExtExternalMemoryHost {
    pub const EMPTY: Self = Self {
        get_memory_host_pointer_properties_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtExternalMemoryMetal {
    pub get_memory_metal_handle_ext: Option<vkGetMemoryMetalHandleEXT>,
    pub get_memory_metal_handle_properties_ext: Option<vkGetMemoryMetalHandlePropertiesEXT>,
}

impl DeviceFnExtExternalMemoryMetal {
    pub const EMPTY: Self = Self {
        get_memory_metal_handle_ext: None,
        get_memory_metal_handle_properties_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtFullScreenExclusive {
    pub get_device_group_surface_present_modes2_ext:
        Option<vkGetDeviceGroupSurfacePresentModes2EXT>,
    pub acquire_full_screen_exclusive_mode_ext: Option<vkAcquireFullScreenExclusiveModeEXT>,
    pub release_full_screen_exclusive_mode_ext: Option<vkReleaseFullScreenExclusiveModeEXT>,
}

impl DeviceFnExtFullScreenExclusive {
    pub const EMPTY: Self = Self {
        get_device_group_surface_present_modes2_ext: None,
        acquire_full_screen_exclusive_mode_ext: None,
        release_full_screen_exclusive_mode_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtHdrMetadata {
    pub set_hdr_metadata_ext: Option<vkSetHdrMetadataEXT>,
}

impl DeviceFnExtHdrMetadata {
    pub const EMPTY: Self = Self {
        set_hdr_metadata_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtImageDrmFormatModifier {
    pub get_image_drm_format_modifier_properties_ext:
        Option<vkGetImageDrmFormatModifierPropertiesEXT>,
}

impl DeviceFnExtImageDrmFormatModifier {
    pub const EMPTY: Self = Self {
        get_image_drm_format_modifier_properties_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtMetalObjects {
    pub export_metal_objects_ext: Option<vkExportMetalObjectsEXT>,
}

impl DeviceFnExtMetalObjects {
    pub const EMPTY: Self = Self {
        export_metal_objects_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtOpacityMicromap {
    pub create_micromap_ext: Option<vkCreateMicromapEXT>,
    pub build_micromaps_ext: Option<vkBuildMicromapsEXT>,
    pub destroy_micromap_ext: Option<vkDestroyMicromapEXT>,
    pub copy_micromap_ext: Option<vkCopyMicromapEXT>,
    pub copy_micromap_to_memory_ext: Option<vkCopyMicromapToMemoryEXT>,
    pub copy_memory_to_micromap_ext: Option<vkCopyMemoryToMicromapEXT>,
    pub write_micromaps_properties_ext: Option<vkWriteMicromapsPropertiesEXT>,
    pub get_device_micromap_compatibility_ext: Option<vkGetDeviceMicromapCompatibilityEXT>,
    pub get_micromap_build_sizes_ext: Option<vkGetMicromapBuildSizesEXT>,
}

impl DeviceFnExtOpacityMicromap {
    pub const EMPTY: Self = Self {
        create_micromap_ext: None,
        build_micromaps_ext: None,
        destroy_micromap_ext: None,
        copy_micromap_ext: None,
        copy_micromap_to_memory_ext: None,
        copy_memory_to_micromap_ext: None,
        write_micromaps_properties_ext: None,
        get_device_micromap_compatibility_ext: None,
        get_micromap_build_sizes_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtPageableDeviceLocalMemory {
    pub set_device_memory_priority_ext: Option<vkSetDeviceMemoryPriorityEXT>,
}

impl DeviceFnExtPageableDeviceLocalMemory {
    pub const EMPTY: Self = Self {
        set_device_memory_priority_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtPipelineProperties {
    pub get_pipeline_properties_ext: Option<vkGetPipelinePropertiesEXT>,
}

impl DeviceFnExtPipelineProperties {
    pub const EMPTY: Self = Self {
        get_pipeline_properties_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtPresentTiming {
    pub set_swapchain_present_timing_queue_size_ext:
        Option<vkSetSwapchainPresentTimingQueueSizeEXT>,
    pub get_swapchain_timing_properties_ext: Option<vkGetSwapchainTimingPropertiesEXT>,
    pub get_swapchain_time_domain_properties_ext: Option<vkGetSwapchainTimeDomainPropertiesEXT>,
    pub get_past_presentation_timing_ext: Option<vkGetPastPresentationTimingEXT>,
}

impl DeviceFnExtPresentTiming {
    pub const EMPTY: Self = Self {
        set_swapchain_present_timing_queue_size_ext: None,
        get_swapchain_timing_properties_ext: None,
        get_swapchain_time_domain_properties_ext: None,
        get_past_presentation_timing_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtShaderModuleIdentifier {
    pub get_shader_module_identifier_ext: Option<vkGetShaderModuleIdentifierEXT>,
    pub get_shader_module_create_info_identifier_ext:
        Option<vkGetShaderModuleCreateInfoIdentifierEXT>,
}

impl DeviceFnExtShaderModuleIdentifier {
    pub const EMPTY: Self = Self {
        get_shader_module_identifier_ext: None,
        get_shader_module_create_info_identifier_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtShaderObject {
    pub create_shaders_ext: Option<vkCreateShadersEXT>,
    pub destroy_shader_ext: Option<vkDestroyShaderEXT>,
    pub get_shader_binary_data_ext: Option<vkGetShaderBinaryDataEXT>,
}

impl DeviceFnExtShaderObject {
    pub const EMPTY: Self = Self {
        create_shaders_ext: None,
        destroy_shader_ext: None,
        get_shader_binary_data_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnExtValidationCache {
    pub create_validation_cache_ext: Option<vkCreateValidationCacheEXT>,
    pub destroy_validation_cache_ext: Option<vkDestroyValidationCacheEXT>,
    pub get_validation_cache_data_ext: Option<vkGetValidationCacheDataEXT>,
    pub merge_validation_caches_ext: Option<vkMergeValidationCachesEXT>,
}

impl DeviceFnExtValidationCache {
    pub const EMPTY: Self = Self {
        create_validation_cache_ext: None,
        destroy_validation_cache_ext: None,
        get_validation_cache_data_ext: None,
        merge_validation_caches_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnFuchsiaBufferCollection {
    pub create_buffer_collection_fuchsia: Option<vkCreateBufferCollectionFUCHSIA>,
    pub set_buffer_collection_buffer_constraints_fuchsia:
        Option<vkSetBufferCollectionBufferConstraintsFUCHSIA>,
    pub set_buffer_collection_image_constraints_fuchsia:
        Option<vkSetBufferCollectionImageConstraintsFUCHSIA>,
    pub destroy_buffer_collection_fuchsia: Option<vkDestroyBufferCollectionFUCHSIA>,
    pub get_buffer_collection_properties_fuchsia: Option<vkGetBufferCollectionPropertiesFUCHSIA>,
}

impl DeviceFnFuchsiaBufferCollection {
    pub const EMPTY: Self = Self {
        create_buffer_collection_fuchsia: None,
        set_buffer_collection_buffer_constraints_fuchsia: None,
        set_buffer_collection_image_constraints_fuchsia: None,
        destroy_buffer_collection_fuchsia: None,
        get_buffer_collection_properties_fuchsia: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnFuchsiaExternalMemory {
    pub get_memory_zircon_handle_fuchsia: Option<vkGetMemoryZirconHandleFUCHSIA>,
    pub get_memory_zircon_handle_properties_fuchsia:
        Option<vkGetMemoryZirconHandlePropertiesFUCHSIA>,
}

impl DeviceFnFuchsiaExternalMemory {
    pub const EMPTY: Self = Self {
        get_memory_zircon_handle_fuchsia: None,
        get_memory_zircon_handle_properties_fuchsia: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnFuchsiaExternalSemaphore {
    pub get_semaphore_zircon_handle_fuchsia: Option<vkGetSemaphoreZirconHandleFUCHSIA>,
    pub import_semaphore_zircon_handle_fuchsia: Option<vkImportSemaphoreZirconHandleFUCHSIA>,
}

impl DeviceFnFuchsiaExternalSemaphore {
    pub const EMPTY: Self = Self {
        get_semaphore_zircon_handle_fuchsia: None,
        import_semaphore_zircon_handle_fuchsia: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnGoogleDisplayTiming {
    pub get_refresh_cycle_duration_google: Option<vkGetRefreshCycleDurationGOOGLE>,
    pub get_past_presentation_timing_google: Option<vkGetPastPresentationTimingGOOGLE>,
}

impl DeviceFnGoogleDisplayTiming {
    pub const EMPTY: Self = Self {
        get_refresh_cycle_duration_google: None,
        get_past_presentation_timing_google: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnHuaweiSubpassShading {
    pub get_device_subpass_shading_max_workgroup_size_huawei:
        Option<vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI>,
}

impl DeviceFnHuaweiSubpassShading {
    pub const EMPTY: Self = Self {
        get_device_subpass_shading_max_workgroup_size_huawei: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnIntelPerformanceQuery {
    pub initialize_performance_api_intel: Option<vkInitializePerformanceApiINTEL>,
    pub uninitialize_performance_api_intel: Option<vkUninitializePerformanceApiINTEL>,
    pub acquire_performance_configuration_intel: Option<vkAcquirePerformanceConfigurationINTEL>,
    pub release_performance_configuration_intel: Option<vkReleasePerformanceConfigurationINTEL>,
    pub get_performance_parameter_intel: Option<vkGetPerformanceParameterINTEL>,
}

impl DeviceFnIntelPerformanceQuery {
    pub const EMPTY: Self = Self {
        initialize_performance_api_intel: None,
        uninitialize_performance_api_intel: None,
        acquire_performance_configuration_intel: None,
        release_performance_configuration_intel: None,
        get_performance_parameter_intel: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrAccelerationStructure {
    pub destroy_acceleration_structure_khr: Option<vkDestroyAccelerationStructureKHR>,
    pub copy_acceleration_structure_khr: Option<vkCopyAccelerationStructureKHR>,
    pub copy_acceleration_structure_to_memory_khr: Option<vkCopyAccelerationStructureToMemoryKHR>,
    pub copy_memory_to_acceleration_structure_khr: Option<vkCopyMemoryToAccelerationStructureKHR>,
    pub write_acceleration_structures_properties_khr:
        Option<vkWriteAccelerationStructuresPropertiesKHR>,
    pub get_device_acceleration_structure_compatibility_khr:
        Option<vkGetDeviceAccelerationStructureCompatibilityKHR>,
    pub create_acceleration_structure_khr: Option<vkCreateAccelerationStructureKHR>,
    pub build_acceleration_structures_khr: Option<vkBuildAccelerationStructuresKHR>,
    pub get_acceleration_structure_device_address_khr:
        Option<vkGetAccelerationStructureDeviceAddressKHR>,
    pub get_acceleration_structure_build_sizes_khr: Option<vkGetAccelerationStructureBuildSizesKHR>,
}

impl DeviceFnKhrAccelerationStructure {
    pub const EMPTY: Self = Self {
        destroy_acceleration_structure_khr: None,
        copy_acceleration_structure_khr: None,
        copy_acceleration_structure_to_memory_khr: None,
        copy_memory_to_acceleration_structure_khr: None,
        write_acceleration_structures_properties_khr: None,
        get_device_acceleration_structure_compatibility_khr: None,
        create_acceleration_structure_khr: None,
        build_acceleration_structures_khr: None,
        get_acceleration_structure_device_address_khr: None,
        get_acceleration_structure_build_sizes_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrCalibratedTimestamps {
    pub get_calibrated_timestamps_khr: Option<vkGetCalibratedTimestampsKHR>,
}

impl DeviceFnKhrCalibratedTimestamps {
    pub const EMPTY: Self = Self {
        get_calibrated_timestamps_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrDeferredHostOperations {
    pub create_deferred_operation_khr: Option<vkCreateDeferredOperationKHR>,
    pub destroy_deferred_operation_khr: Option<vkDestroyDeferredOperationKHR>,
    pub get_deferred_operation_max_concurrency_khr: Option<vkGetDeferredOperationMaxConcurrencyKHR>,
    pub get_deferred_operation_result_khr: Option<vkGetDeferredOperationResultKHR>,
    pub deferred_operation_join_khr: Option<vkDeferredOperationJoinKHR>,
}

impl DeviceFnKhrDeferredHostOperations {
    pub const EMPTY: Self = Self {
        create_deferred_operation_khr: None,
        destroy_deferred_operation_khr: None,
        get_deferred_operation_max_concurrency_khr: None,
        get_deferred_operation_result_khr: None,
        deferred_operation_join_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrDeviceAddressCommands {
    pub create_acceleration_structure2_khr: Option<vkCreateAccelerationStructure2KHR>,
}

impl DeviceFnKhrDeviceAddressCommands {
    pub const EMPTY: Self = Self {
        create_acceleration_structure2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrDeviceFault {
    pub get_device_fault_reports_khr: Option<vkGetDeviceFaultReportsKHR>,
    pub get_device_fault_debug_info_khr: Option<vkGetDeviceFaultDebugInfoKHR>,
}

impl DeviceFnKhrDeviceFault {
    pub const EMPTY: Self = Self {
        get_device_fault_reports_khr: None,
        get_device_fault_debug_info_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrDeviceGroup {
    pub get_device_group_present_capabilities_khr: Option<vkGetDeviceGroupPresentCapabilitiesKHR>,
    pub get_device_group_surface_present_modes_khr: Option<vkGetDeviceGroupSurfacePresentModesKHR>,
    pub acquire_next_image2_khr: Option<vkAcquireNextImage2KHR>,
}

impl DeviceFnKhrDeviceGroup {
    pub const EMPTY: Self = Self {
        get_device_group_present_capabilities_khr: None,
        get_device_group_surface_present_modes_khr: None,
        acquire_next_image2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrDisplaySwapchain {
    pub create_shared_swapchains_khr: Option<vkCreateSharedSwapchainsKHR>,
}

impl DeviceFnKhrDisplaySwapchain {
    pub const EMPTY: Self = Self {
        create_shared_swapchains_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrExternalFenceFd {
    pub get_fence_fd_khr: Option<vkGetFenceFdKHR>,
    pub import_fence_fd_khr: Option<vkImportFenceFdKHR>,
}

impl DeviceFnKhrExternalFenceFd {
    pub const EMPTY: Self = Self {
        get_fence_fd_khr: None,
        import_fence_fd_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrExternalFenceWin32 {
    pub get_fence_win32_handle_khr: Option<vkGetFenceWin32HandleKHR>,
    pub import_fence_win32_handle_khr: Option<vkImportFenceWin32HandleKHR>,
}

impl DeviceFnKhrExternalFenceWin32 {
    pub const EMPTY: Self = Self {
        get_fence_win32_handle_khr: None,
        import_fence_win32_handle_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrExternalMemoryFd {
    pub get_memory_fd_khr: Option<vkGetMemoryFdKHR>,
    pub get_memory_fd_properties_khr: Option<vkGetMemoryFdPropertiesKHR>,
}

impl DeviceFnKhrExternalMemoryFd {
    pub const EMPTY: Self = Self {
        get_memory_fd_khr: None,
        get_memory_fd_properties_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrExternalMemoryWin32 {
    pub get_memory_win32_handle_khr: Option<vkGetMemoryWin32HandleKHR>,
    pub get_memory_win32_handle_properties_khr: Option<vkGetMemoryWin32HandlePropertiesKHR>,
}

impl DeviceFnKhrExternalMemoryWin32 {
    pub const EMPTY: Self = Self {
        get_memory_win32_handle_khr: None,
        get_memory_win32_handle_properties_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrExternalSemaphoreFd {
    pub get_semaphore_fd_khr: Option<vkGetSemaphoreFdKHR>,
    pub import_semaphore_fd_khr: Option<vkImportSemaphoreFdKHR>,
}

impl DeviceFnKhrExternalSemaphoreFd {
    pub const EMPTY: Self = Self {
        get_semaphore_fd_khr: None,
        import_semaphore_fd_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrExternalSemaphoreWin32 {
    pub get_semaphore_win32_handle_khr: Option<vkGetSemaphoreWin32HandleKHR>,
    pub import_semaphore_win32_handle_khr: Option<vkImportSemaphoreWin32HandleKHR>,
}

impl DeviceFnKhrExternalSemaphoreWin32 {
    pub const EMPTY: Self = Self {
        get_semaphore_win32_handle_khr: None,
        import_semaphore_win32_handle_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrPerformanceQuery {
    pub acquire_profiling_lock_khr: Option<vkAcquireProfilingLockKHR>,
    pub release_profiling_lock_khr: Option<vkReleaseProfilingLockKHR>,
}

impl DeviceFnKhrPerformanceQuery {
    pub const EMPTY: Self = Self {
        acquire_profiling_lock_khr: None,
        release_profiling_lock_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrPipelineBinary {
    pub create_pipeline_binaries_khr: Option<vkCreatePipelineBinariesKHR>,
    pub destroy_pipeline_binary_khr: Option<vkDestroyPipelineBinaryKHR>,
    pub get_pipeline_key_khr: Option<vkGetPipelineKeyKHR>,
    pub get_pipeline_binary_data_khr: Option<vkGetPipelineBinaryDataKHR>,
    pub release_captured_pipeline_data_khr: Option<vkReleaseCapturedPipelineDataKHR>,
}

impl DeviceFnKhrPipelineBinary {
    pub const EMPTY: Self = Self {
        create_pipeline_binaries_khr: None,
        destroy_pipeline_binary_khr: None,
        get_pipeline_key_khr: None,
        get_pipeline_binary_data_khr: None,
        release_captured_pipeline_data_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrPipelineExecutableProperties {
    pub get_pipeline_executable_properties_khr: Option<vkGetPipelineExecutablePropertiesKHR>,
    pub get_pipeline_executable_statistics_khr: Option<vkGetPipelineExecutableStatisticsKHR>,
    pub get_pipeline_executable_internal_representations_khr:
        Option<vkGetPipelineExecutableInternalRepresentationsKHR>,
}

impl DeviceFnKhrPipelineExecutableProperties {
    pub const EMPTY: Self = Self {
        get_pipeline_executable_properties_khr: None,
        get_pipeline_executable_statistics_khr: None,
        get_pipeline_executable_internal_representations_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrPresentWait {
    pub wait_for_present_khr: Option<vkWaitForPresentKHR>,
}

impl DeviceFnKhrPresentWait {
    pub const EMPTY: Self = Self {
        wait_for_present_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrPresentWait2 {
    pub wait_for_present2_khr: Option<vkWaitForPresent2KHR>,
}

impl DeviceFnKhrPresentWait2 {
    pub const EMPTY: Self = Self {
        wait_for_present2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrRayTracingPipeline {
    pub get_ray_tracing_shader_group_handles_khr: Option<vkGetRayTracingShaderGroupHandlesKHR>,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr:
        Option<vkGetRayTracingCaptureReplayShaderGroupHandlesKHR>,
    pub create_ray_tracing_pipelines_khr: Option<vkCreateRayTracingPipelinesKHR>,
    pub get_ray_tracing_shader_group_stack_size_khr: Option<vkGetRayTracingShaderGroupStackSizeKHR>,
}

impl DeviceFnKhrRayTracingPipeline {
    pub const EMPTY: Self = Self {
        get_ray_tracing_shader_group_handles_khr: None,
        get_ray_tracing_capture_replay_shader_group_handles_khr: None,
        create_ray_tracing_pipelines_khr: None,
        get_ray_tracing_shader_group_stack_size_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrSharedPresentableImage {
    pub get_swapchain_status_khr: Option<vkGetSwapchainStatusKHR>,
}

impl DeviceFnKhrSharedPresentableImage {
    pub const EMPTY: Self = Self {
        get_swapchain_status_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrSwapchain {
    pub create_swapchain_khr: Option<vkCreateSwapchainKHR>,
    pub destroy_swapchain_khr: Option<vkDestroySwapchainKHR>,
    pub get_swapchain_images_khr: Option<vkGetSwapchainImagesKHR>,
    pub acquire_next_image_khr: Option<vkAcquireNextImageKHR>,
}

impl DeviceFnKhrSwapchain {
    pub const EMPTY: Self = Self {
        create_swapchain_khr: None,
        destroy_swapchain_khr: None,
        get_swapchain_images_khr: None,
        acquire_next_image_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrSwapchainMaintenance1 {
    pub release_swapchain_images_khr: Option<vkReleaseSwapchainImagesKHR>,
}

impl DeviceFnKhrSwapchainMaintenance1 {
    pub const EMPTY: Self = Self {
        release_swapchain_images_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrVideoEncodeQueue {
    pub get_encoded_video_session_parameters_khr: Option<vkGetEncodedVideoSessionParametersKHR>,
}

impl DeviceFnKhrVideoEncodeQueue {
    pub const EMPTY: Self = Self {
        get_encoded_video_session_parameters_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnKhrVideoQueue {
    pub create_video_session_khr: Option<vkCreateVideoSessionKHR>,
    pub destroy_video_session_khr: Option<vkDestroyVideoSessionKHR>,
    pub create_video_session_parameters_khr: Option<vkCreateVideoSessionParametersKHR>,
    pub update_video_session_parameters_khr: Option<vkUpdateVideoSessionParametersKHR>,
    pub destroy_video_session_parameters_khr: Option<vkDestroyVideoSessionParametersKHR>,
    pub get_video_session_memory_requirements_khr: Option<vkGetVideoSessionMemoryRequirementsKHR>,
    pub bind_video_session_memory_khr: Option<vkBindVideoSessionMemoryKHR>,
}

impl DeviceFnKhrVideoQueue {
    pub const EMPTY: Self = Self {
        create_video_session_khr: None,
        destroy_video_session_khr: None,
        create_video_session_parameters_khr: None,
        update_video_session_parameters_khr: None,
        destroy_video_session_parameters_khr: None,
        get_video_session_memory_requirements_khr: None,
        bind_video_session_memory_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvClusterAccelerationStructure {
    pub get_cluster_acceleration_structure_build_sizes_nv:
        Option<vkGetClusterAccelerationStructureBuildSizesNV>,
}

impl DeviceFnNvClusterAccelerationStructure {
    pub const EMPTY: Self = Self {
        get_cluster_acceleration_structure_build_sizes_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvCooperativeVector {
    pub convert_cooperative_vector_matrix_nv: Option<vkConvertCooperativeVectorMatrixNV>,
}

impl DeviceFnNvCooperativeVector {
    pub const EMPTY: Self = Self {
        convert_cooperative_vector_matrix_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvCudaKernelLaunch {
    pub create_cuda_module_nv: Option<vkCreateCudaModuleNV>,
    pub get_cuda_module_cache_nv: Option<vkGetCudaModuleCacheNV>,
    pub create_cuda_function_nv: Option<vkCreateCudaFunctionNV>,
    pub destroy_cuda_module_nv: Option<vkDestroyCudaModuleNV>,
    pub destroy_cuda_function_nv: Option<vkDestroyCudaFunctionNV>,
}

impl DeviceFnNvCudaKernelLaunch {
    pub const EMPTY: Self = Self {
        create_cuda_module_nv: None,
        get_cuda_module_cache_nv: None,
        create_cuda_function_nv: None,
        destroy_cuda_module_nv: None,
        destroy_cuda_function_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvDeviceGeneratedCommands {
    pub get_generated_commands_memory_requirements_nv:
        Option<vkGetGeneratedCommandsMemoryRequirementsNV>,
    pub create_indirect_commands_layout_nv: Option<vkCreateIndirectCommandsLayoutNV>,
    pub destroy_indirect_commands_layout_nv: Option<vkDestroyIndirectCommandsLayoutNV>,
}

impl DeviceFnNvDeviceGeneratedCommands {
    pub const EMPTY: Self = Self {
        get_generated_commands_memory_requirements_nv: None,
        create_indirect_commands_layout_nv: None,
        destroy_indirect_commands_layout_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvDeviceGeneratedCommandsCompute {
    pub get_pipeline_indirect_memory_requirements_nv:
        Option<vkGetPipelineIndirectMemoryRequirementsNV>,
    pub get_pipeline_indirect_device_address_nv: Option<vkGetPipelineIndirectDeviceAddressNV>,
}

impl DeviceFnNvDeviceGeneratedCommandsCompute {
    pub const EMPTY: Self = Self {
        get_pipeline_indirect_memory_requirements_nv: None,
        get_pipeline_indirect_device_address_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvExternalComputeQueue {
    pub create_external_compute_queue_nv: Option<vkCreateExternalComputeQueueNV>,
    pub destroy_external_compute_queue_nv: Option<vkDestroyExternalComputeQueueNV>,
}

impl DeviceFnNvExternalComputeQueue {
    pub const EMPTY: Self = Self {
        create_external_compute_queue_nv: None,
        destroy_external_compute_queue_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvExternalMemoryRdma {
    pub get_memory_remote_address_nv: Option<vkGetMemoryRemoteAddressNV>,
}

impl DeviceFnNvExternalMemoryRdma {
    pub const EMPTY: Self = Self {
        get_memory_remote_address_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvExternalMemorySciBuf {
    pub get_memory_sci_buf_nv: Option<vkGetMemorySciBufNV>,
}

impl DeviceFnNvExternalMemorySciBuf {
    pub const EMPTY: Self = Self {
        get_memory_sci_buf_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvExternalMemoryWin32 {
    pub get_memory_win32_handle_nv: Option<vkGetMemoryWin32HandleNV>,
}

impl DeviceFnNvExternalMemoryWin32 {
    pub const EMPTY: Self = Self {
        get_memory_win32_handle_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvExternalSciSync {
    pub get_semaphore_sci_sync_obj_nv: Option<vkGetSemaphoreSciSyncObjNV>,
    pub import_semaphore_sci_sync_obj_nv: Option<vkImportSemaphoreSciSyncObjNV>,
}

impl DeviceFnNvExternalSciSync {
    pub const EMPTY: Self = Self {
        get_semaphore_sci_sync_obj_nv: None,
        import_semaphore_sci_sync_obj_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvExternalSciSync2 {
    pub get_fence_sci_sync_fence_nv: Option<vkGetFenceSciSyncFenceNV>,
    pub get_fence_sci_sync_obj_nv: Option<vkGetFenceSciSyncObjNV>,
    pub import_fence_sci_sync_fence_nv: Option<vkImportFenceSciSyncFenceNV>,
    pub import_fence_sci_sync_obj_nv: Option<vkImportFenceSciSyncObjNV>,
    pub create_semaphore_sci_sync_pool_nv: Option<vkCreateSemaphoreSciSyncPoolNV>,
    pub destroy_semaphore_sci_sync_pool_nv: Option<vkDestroySemaphoreSciSyncPoolNV>,
}

impl DeviceFnNvExternalSciSync2 {
    pub const EMPTY: Self = Self {
        get_fence_sci_sync_fence_nv: None,
        get_fence_sci_sync_obj_nv: None,
        import_fence_sci_sync_fence_nv: None,
        import_fence_sci_sync_obj_nv: None,
        create_semaphore_sci_sync_pool_nv: None,
        destroy_semaphore_sci_sync_pool_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvLowLatency {
    pub set_latency_sleep_mode_legacy_nv: Option<vkSetLatencySleepModeLegacyNV>,
    pub latency_sleep_legacy_nv: Option<vkLatencySleepLegacyNV>,
    pub set_latency_marker_legacy_nv: Option<vkSetLatencyMarkerLegacyNV>,
    pub get_latency_timings_legacy_nv: Option<vkGetLatencyTimingsLegacyNV>,
    pub get_sleep_status_legacy_nv: Option<vkGetSleepStatusLegacyNV>,
    pub shutdown_latency_device_legacy_nv: Option<vkShutdownLatencyDeviceLegacyNV>,
}

impl DeviceFnNvLowLatency {
    pub const EMPTY: Self = Self {
        set_latency_sleep_mode_legacy_nv: None,
        latency_sleep_legacy_nv: None,
        set_latency_marker_legacy_nv: None,
        get_latency_timings_legacy_nv: None,
        get_sleep_status_legacy_nv: None,
        shutdown_latency_device_legacy_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvLowLatency2 {
    pub set_latency_sleep_mode_nv: Option<vkSetLatencySleepModeNV>,
    pub latency_sleep_nv: Option<vkLatencySleepNV>,
    pub set_latency_marker_nv: Option<vkSetLatencyMarkerNV>,
    pub get_latency_timings_nv: Option<vkGetLatencyTimingsNV>,
}

impl DeviceFnNvLowLatency2 {
    pub const EMPTY: Self = Self {
        set_latency_sleep_mode_nv: None,
        latency_sleep_nv: None,
        set_latency_marker_nv: None,
        get_latency_timings_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvOpticalFlow {
    pub create_optical_flow_session_nv: Option<vkCreateOpticalFlowSessionNV>,
    pub destroy_optical_flow_session_nv: Option<vkDestroyOpticalFlowSessionNV>,
    pub bind_optical_flow_session_image_nv: Option<vkBindOpticalFlowSessionImageNV>,
}

impl DeviceFnNvOpticalFlow {
    pub const EMPTY: Self = Self {
        create_optical_flow_session_nv: None,
        destroy_optical_flow_session_nv: None,
        bind_optical_flow_session_image_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvPartitionedAccelerationStructure {
    pub get_partitioned_acceleration_structures_build_sizes_nv:
        Option<vkGetPartitionedAccelerationStructuresBuildSizesNV>,
}

impl DeviceFnNvPartitionedAccelerationStructure {
    pub const EMPTY: Self = Self {
        get_partitioned_acceleration_structures_build_sizes_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvRayTracing {
    pub compile_deferred_nv: Option<vkCompileDeferredNV>,
    pub create_acceleration_structure_nv: Option<vkCreateAccelerationStructureNV>,
    pub destroy_acceleration_structure_nv: Option<vkDestroyAccelerationStructureNV>,
    pub get_acceleration_structure_memory_requirements_nv:
        Option<vkGetAccelerationStructureMemoryRequirementsNV>,
    pub bind_acceleration_structure_memory_nv: Option<vkBindAccelerationStructureMemoryNV>,
    pub get_acceleration_structure_handle_nv: Option<vkGetAccelerationStructureHandleNV>,
    pub create_ray_tracing_pipelines_nv: Option<vkCreateRayTracingPipelinesNV>,
}

impl DeviceFnNvRayTracing {
    pub const EMPTY: Self = Self {
        compile_deferred_nv: None,
        create_acceleration_structure_nv: None,
        destroy_acceleration_structure_nv: None,
        get_acceleration_structure_memory_requirements_nv: None,
        bind_acceleration_structure_memory_nv: None,
        get_acceleration_structure_handle_nv: None,
        create_ray_tracing_pipelines_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvxBinaryImport {
    pub create_cu_module_nvx: Option<vkCreateCuModuleNVX>,
    pub create_cu_function_nvx: Option<vkCreateCuFunctionNVX>,
    pub destroy_cu_module_nvx: Option<vkDestroyCuModuleNVX>,
    pub destroy_cu_function_nvx: Option<vkDestroyCuFunctionNVX>,
}

impl DeviceFnNvxBinaryImport {
    pub const EMPTY: Self = Self {
        create_cu_module_nvx: None,
        create_cu_function_nvx: None,
        destroy_cu_module_nvx: None,
        destroy_cu_function_nvx: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnNvxImageViewHandle {
    pub get_image_view_handle_nvx: Option<vkGetImageViewHandleNVX>,
    pub get_image_view_handle64_nvx: Option<vkGetImageViewHandle64NVX>,
    pub get_image_view_address_nvx: Option<vkGetImageViewAddressNVX>,
    pub get_device_combined_image_sampler_index_nvx:
        Option<vkGetDeviceCombinedImageSamplerIndexNVX>,
}

impl DeviceFnNvxImageViewHandle {
    pub const EMPTY: Self = Self {
        get_image_view_handle_nvx: None,
        get_image_view_handle64_nvx: None,
        get_image_view_address_nvx: None,
        get_device_combined_image_sampler_index_nvx: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnOhosExternalMemory {
    pub get_native_buffer_properties_ohos: Option<vkGetNativeBufferPropertiesOHOS>,
    pub get_memory_native_buffer_ohos: Option<vkGetMemoryNativeBufferOHOS>,
}

impl DeviceFnOhosExternalMemory {
    pub const EMPTY: Self = Self {
        get_native_buffer_properties_ohos: None,
        get_memory_native_buffer_ohos: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnQcomTileProperties {
    pub get_framebuffer_tile_properties_qcom: Option<vkGetFramebufferTilePropertiesQCOM>,
    pub get_dynamic_rendering_tile_properties_qcom: Option<vkGetDynamicRenderingTilePropertiesQCOM>,
}

impl DeviceFnQcomTileProperties {
    pub const EMPTY: Self = Self {
        get_framebuffer_tile_properties_qcom: None,
        get_dynamic_rendering_tile_properties_qcom: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnQnxExternalMemoryScreenBuffer {
    pub get_screen_buffer_properties_qnx: Option<vkGetScreenBufferPropertiesQNX>,
}

impl DeviceFnQnxExternalMemoryScreenBuffer {
    pub const EMPTY: Self = Self {
        get_screen_buffer_properties_qnx: None,
    };
}

#[derive(Clone, Default)]
pub struct DeviceFnValveDescriptorSetHostMapping {
    pub get_descriptor_set_layout_host_mapping_info_valve:
        Option<vkGetDescriptorSetLayoutHostMappingInfoVALVE>,
    pub get_descriptor_set_host_mapping_valve: Option<vkGetDescriptorSetHostMappingVALVE>,
}

impl DeviceFnValveDescriptorSetHostMapping {
    pub const EMPTY: Self = Self {
        get_descriptor_set_layout_host_mapping_info_valve: None,
        get_descriptor_set_host_mapping_valve: None,
    };
}

#[derive(Clone)]
pub struct QueueFn {
    pub v1_0: QueueFnv1_0,
    pub v1_3: QueueFnv1_3,
    pub ext_debug_utils: QueueFnExtDebugUtils,
    pub intel_performance_query: QueueFnIntelPerformanceQuery,
    pub khr_swapchain: QueueFnKhrSwapchain,
    pub nv_device_diagnostic_checkpoints: QueueFnNvDeviceDiagnosticCheckpoints,
    pub nv_low_latency: QueueFnNvLowLatency,
    pub nv_low_latency2: QueueFnNvLowLatency2,
    pub qcom_queue_perf_hint: QueueFnQcomQueuePerfHint,
}

impl QueueFn {
    /// A table with no functions loaded; every call through it panics.
    pub const EMPTY: Self = Self {
        v1_0: QueueFnv1_0::EMPTY,
        v1_3: QueueFnv1_3::EMPTY,
        ext_debug_utils: QueueFnExtDebugUtils::EMPTY,
        intel_performance_query: QueueFnIntelPerformanceQuery::EMPTY,
        khr_swapchain: QueueFnKhrSwapchain::EMPTY,
        nv_device_diagnostic_checkpoints: QueueFnNvDeviceDiagnosticCheckpoints::EMPTY,
        nv_low_latency: QueueFnNvLowLatency::EMPTY,
        nv_low_latency2: QueueFnNvLowLatency2::EMPTY,
        qcom_queue_perf_hint: QueueFnQcomQueuePerfHint::EMPTY,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        let mut out = Self {
            v1_0: QueueFnv1_0::load(&mut loader),
            v1_3: if api_version >= API_VERSION_1_3 {
                QueueFnv1_3::load(&mut loader)
            } else {
                QueueFnv1_3::EMPTY
            },
            ext_debug_utils: QueueFnExtDebugUtils::EMPTY,
            intel_performance_query: QueueFnIntelPerformanceQuery::EMPTY,
            khr_swapchain: QueueFnKhrSwapchain::EMPTY,
            nv_device_diagnostic_checkpoints: QueueFnNvDeviceDiagnosticCheckpoints::EMPTY,
            nv_low_latency: QueueFnNvLowLatency::EMPTY,
            nv_low_latency2: QueueFnNvLowLatency2::EMPTY,
            qcom_queue_perf_hint: QueueFnQcomQueuePerfHint::EMPTY,
        };
        out.ext_debug_utils.queue_begin_debug_utils_label_ext =
            to_option(loader(c"vkQueueBeginDebugUtilsLabelEXT"));
        out.ext_debug_utils.queue_end_debug_utils_label_ext =
            to_option(loader(c"vkQueueEndDebugUtilsLabelEXT"));
        out.ext_debug_utils.queue_insert_debug_utils_label_ext =
            to_option(loader(c"vkQueueInsertDebugUtilsLabelEXT"));
        for &ext in extensions {
            match unsafe { CStr::from_ptr(ext) }.to_bytes() {
                b"VK_KHR_swapchain" => {
                    out.khr_swapchain.queue_present_khr = to_option(loader(c"vkQueuePresentKHR"));
                }
                b"VK_NV_device_diagnostic_checkpoints" => {
                    out.nv_device_diagnostic_checkpoints
                        .get_queue_checkpoint_data_nv =
                        to_option(loader(c"vkGetQueueCheckpointDataNV"));
                    out.nv_device_diagnostic_checkpoints
                        .get_queue_checkpoint_data2_nv =
                        to_option(loader(c"vkGetQueueCheckpointData2NV"));
                }
                b"VK_INTEL_performance_query" => {
                    out.intel_performance_query
                        .queue_set_performance_configuration_intel =
                        to_option(loader(c"vkQueueSetPerformanceConfigurationINTEL"));
                }
                b"VK_QCOM_queue_perf_hint" => {
                    out.qcom_queue_perf_hint.queue_set_perf_hint_qcom =
                        to_option(loader(c"vkQueueSetPerfHintQCOM"));
                }
                b"VK_NV_low_latency" => {
                    out.nv_low_latency.queue_notify_out_of_band_legacy_nv =
                        to_option(loader(c"vkQueueNotifyOutOfBandLegacyNV"));
                }
                b"VK_KHR_synchronization2" => {
                    if out.v1_3.queue_submit2.is_none() {
                        out.v1_3.queue_submit2 = to_option(loader(c"vkQueueSubmit2KHR"));
                    }
                }
                b"VK_NV_low_latency2" => {
                    out.nv_low_latency2.queue_notify_out_of_band_nv =
                        to_option(loader(c"vkQueueNotifyOutOfBandNV"));
                }
                _ => (),
            }
        }
        out
    }
}

#[derive(Clone, Default)]
pub struct QueueFnv1_0 {
    pub queue_submit: Option<vkQueueSubmit>,
    pub queue_wait_idle: Option<vkQueueWaitIdle>,
    pub queue_bind_sparse: Option<vkQueueBindSparse>,
}

impl QueueFnv1_0 {
    pub const EMPTY: Self = Self {
        queue_submit: None,
        queue_wait_idle: None,
        queue_bind_sparse: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_submit: to_option(loader(c"vkQueueSubmit")),
            queue_wait_idle: to_option(loader(c"vkQueueWaitIdle")),
            queue_bind_sparse: to_option(loader(c"vkQueueBindSparse")),
        }
    }
}

#[derive(Clone, Default)]
pub struct QueueFnv1_3 {
    pub queue_submit2: Option<vkQueueSubmit2>,
}

impl QueueFnv1_3 {
    pub const EMPTY: Self = Self {
        queue_submit2: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_submit2: to_option(loader(c"vkQueueSubmit2")),
        }
    }
}

#[derive(Clone, Default)]
pub struct QueueFnExtDebugUtils {
    pub queue_begin_debug_utils_label_ext: Option<vkQueueBeginDebugUtilsLabelEXT>,
    pub queue_end_debug_utils_label_ext: Option<vkQueueEndDebugUtilsLabelEXT>,
    pub queue_insert_debug_utils_label_ext: Option<vkQueueInsertDebugUtilsLabelEXT>,
}

impl QueueFnExtDebugUtils {
    pub const EMPTY: Self = Self {
        queue_begin_debug_utils_label_ext: None,
        queue_end_debug_utils_label_ext: None,
        queue_insert_debug_utils_label_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct QueueFnIntelPerformanceQuery {
    pub queue_set_performance_configuration_intel: Option<vkQueueSetPerformanceConfigurationINTEL>,
}

impl QueueFnIntelPerformanceQuery {
    pub const EMPTY: Self = Self {
        queue_set_performance_configuration_intel: None,
    };
}

#[derive(Clone, Default)]
pub struct QueueFnKhrSwapchain {
    pub queue_present_khr: Option<vkQueuePresentKHR>,
}

impl QueueFnKhrSwapchain {
    pub const EMPTY: Self = Self {
        queue_present_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct QueueFnNvDeviceDiagnosticCheckpoints {
    pub get_queue_checkpoint_data_nv: Option<vkGetQueueCheckpointDataNV>,
    pub get_queue_checkpoint_data2_nv: Option<vkGetQueueCheckpointData2NV>,
}

impl QueueFnNvDeviceDiagnosticCheckpoints {
    pub const EMPTY: Self = Self {
        get_queue_checkpoint_data_nv: None,
        get_queue_checkpoint_data2_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct QueueFnNvLowLatency {
    pub queue_notify_out_of_band_legacy_nv: Option<vkQueueNotifyOutOfBandLegacyNV>,
}

impl QueueFnNvLowLatency {
    pub const EMPTY: Self = Self {
        queue_notify_out_of_band_legacy_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct QueueFnNvLowLatency2 {
    pub queue_notify_out_of_band_nv: Option<vkQueueNotifyOutOfBandNV>,
}

impl QueueFnNvLowLatency2 {
    pub const EMPTY: Self = Self {
        queue_notify_out_of_band_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct QueueFnQcomQueuePerfHint {
    pub queue_set_perf_hint_qcom: Option<vkQueueSetPerfHintQCOM>,
}

impl QueueFnQcomQueuePerfHint {
    pub const EMPTY: Self = Self {
        queue_set_perf_hint_qcom: None,
    };
}

#[derive(Clone)]
pub struct CommandBufferFn {
    pub v1_0: CommandBufferFnv1_0,
    pub v1_1: CommandBufferFnv1_1,
    pub v1_2: CommandBufferFnv1_2,
    pub v1_3: CommandBufferFnv1_3,
    pub v1_4: CommandBufferFnv1_4,
    pub amd_buffer_marker: CommandBufferFnAmdBufferMarker,
    pub amd_gpa_interface: CommandBufferFnAmdGpaInterface,
    pub amdx_shader_enqueue: CommandBufferFnAmdxShaderEnqueue,
    pub arm_data_graph: CommandBufferFnArmDataGraph,
    pub arm_scheduling_controls: CommandBufferFnArmSchedulingControls,
    pub arm_shader_instrumentation: CommandBufferFnArmShaderInstrumentation,
    pub arm_tensors: CommandBufferFnArmTensors,
    pub ext_attachment_feedback_loop_dynamic_state:
        CommandBufferFnExtAttachmentFeedbackLoopDynamicState,
    pub ext_color_write_enable: CommandBufferFnExtColorWriteEnable,
    pub ext_conditional_rendering: CommandBufferFnExtConditionalRendering,
    pub ext_custom_resolve: CommandBufferFnExtCustomResolve,
    pub ext_debug_marker: CommandBufferFnExtDebugMarker,
    pub ext_debug_utils: CommandBufferFnExtDebugUtils,
    pub ext_depth_bias_control: CommandBufferFnExtDepthBiasControl,
    pub ext_depth_clamp_control: CommandBufferFnExtDepthClampControl,
    pub ext_descriptor_buffer: CommandBufferFnExtDescriptorBuffer,
    pub ext_descriptor_heap: CommandBufferFnExtDescriptorHeap,
    pub ext_device_generated_commands: CommandBufferFnExtDeviceGeneratedCommands,
    pub ext_discard_rectangles: CommandBufferFnExtDiscardRectangles,
    pub ext_memory_decompression: CommandBufferFnExtMemoryDecompression,
    pub ext_mesh_shader: CommandBufferFnExtMeshShader,
    pub ext_multi_draw: CommandBufferFnExtMultiDraw,
    pub ext_opacity_micromap: CommandBufferFnExtOpacityMicromap,
    pub ext_primitive_restart_index: CommandBufferFnExtPrimitiveRestartIndex,
    pub ext_sample_locations: CommandBufferFnExtSampleLocations,
    pub ext_shader_object: CommandBufferFnExtShaderObject,
    pub ext_transform_feedback: CommandBufferFnExtTransformFeedback,
    pub huawei_cluster_culling_shader: CommandBufferFnHuaweiClusterCullingShader,
    pub huawei_invocation_mask: CommandBufferFnHuaweiInvocationMask,
    pub huawei_subpass_shading: CommandBufferFnHuaweiSubpassShading,
    pub intel_performance_query: CommandBufferFnIntelPerformanceQuery,
    pub khr_acceleration_structure: CommandBufferFnKhrAccelerationStructure,
    pub khr_copy_memory_indirect: CommandBufferFnKhrCopyMemoryIndirect,
    pub khr_device_address_commands: CommandBufferFnKhrDeviceAddressCommands,
    pub khr_fragment_shading_rate: CommandBufferFnKhrFragmentShadingRate,
    pub khr_maintenance10: CommandBufferFnKhrMaintenance10,
    pub khr_maintenance6: CommandBufferFnKhrMaintenance6,
    pub khr_object_refresh: CommandBufferFnKhrObjectRefresh,
    pub khr_ray_tracing_maintenance1: CommandBufferFnKhrRayTracingMaintenance1,
    pub khr_ray_tracing_pipeline: CommandBufferFnKhrRayTracingPipeline,
    pub khr_video_decode_queue: CommandBufferFnKhrVideoDecodeQueue,
    pub khr_video_encode_queue: CommandBufferFnKhrVideoEncodeQueue,
    pub khr_video_queue: CommandBufferFnKhrVideoQueue,
    pub nv_clip_space_w_scaling: CommandBufferFnNvClipSpaceWScaling,
    pub nv_cluster_acceleration_structure: CommandBufferFnNvClusterAccelerationStructure,
    pub nv_compute_occupancy_priority: CommandBufferFnNvComputeOccupancyPriority,
    pub nv_cooperative_vector: CommandBufferFnNvCooperativeVector,
    pub nv_copy_memory_indirect: CommandBufferFnNvCopyMemoryIndirect,
    pub nv_cuda_kernel_launch: CommandBufferFnNvCudaKernelLaunch,
    pub nv_device_diagnostic_checkpoints: CommandBufferFnNvDeviceDiagnosticCheckpoints,
    pub nv_device_generated_commands: CommandBufferFnNvDeviceGeneratedCommands,
    pub nv_device_generated_commands_compute: CommandBufferFnNvDeviceGeneratedCommandsCompute,
    pub nv_fragment_shading_rate_enums: CommandBufferFnNvFragmentShadingRateEnums,
    pub nv_memory_decompression: CommandBufferFnNvMemoryDecompression,
    pub nv_mesh_shader: CommandBufferFnNvMeshShader,
    pub nv_optical_flow: CommandBufferFnNvOpticalFlow,
    pub nv_partitioned_acceleration_structure: CommandBufferFnNvPartitionedAccelerationStructure,
    pub nv_ray_tracing: CommandBufferFnNvRayTracing,
    pub nv_scissor_exclusive: CommandBufferFnNvScissorExclusive,
    pub nv_shading_rate_image: CommandBufferFnNvShadingRateImage,
    pub nvx_binary_import: CommandBufferFnNvxBinaryImport,
    pub qcom_tile_memory_heap: CommandBufferFnQcomTileMemoryHeap,
    pub qcom_tile_shading: CommandBufferFnQcomTileShading,
}

impl CommandBufferFn {
    /// A table with no functions loaded; every call through it panics.
    pub const EMPTY: Self = Self {
        v1_0: CommandBufferFnv1_0::EMPTY,
        v1_1: CommandBufferFnv1_1::EMPTY,
        v1_2: CommandBufferFnv1_2::EMPTY,
        v1_3: CommandBufferFnv1_3::EMPTY,
        v1_4: CommandBufferFnv1_4::EMPTY,
        amd_buffer_marker: CommandBufferFnAmdBufferMarker::EMPTY,
        amd_gpa_interface: CommandBufferFnAmdGpaInterface::EMPTY,
        amdx_shader_enqueue: CommandBufferFnAmdxShaderEnqueue::EMPTY,
        arm_data_graph: CommandBufferFnArmDataGraph::EMPTY,
        arm_scheduling_controls: CommandBufferFnArmSchedulingControls::EMPTY,
        arm_shader_instrumentation: CommandBufferFnArmShaderInstrumentation::EMPTY,
        arm_tensors: CommandBufferFnArmTensors::EMPTY,
        ext_attachment_feedback_loop_dynamic_state:
            CommandBufferFnExtAttachmentFeedbackLoopDynamicState::EMPTY,
        ext_color_write_enable: CommandBufferFnExtColorWriteEnable::EMPTY,
        ext_conditional_rendering: CommandBufferFnExtConditionalRendering::EMPTY,
        ext_custom_resolve: CommandBufferFnExtCustomResolve::EMPTY,
        ext_debug_marker: CommandBufferFnExtDebugMarker::EMPTY,
        ext_debug_utils: CommandBufferFnExtDebugUtils::EMPTY,
        ext_depth_bias_control: CommandBufferFnExtDepthBiasControl::EMPTY,
        ext_depth_clamp_control: CommandBufferFnExtDepthClampControl::EMPTY,
        ext_descriptor_buffer: CommandBufferFnExtDescriptorBuffer::EMPTY,
        ext_descriptor_heap: CommandBufferFnExtDescriptorHeap::EMPTY,
        ext_device_generated_commands: CommandBufferFnExtDeviceGeneratedCommands::EMPTY,
        ext_discard_rectangles: CommandBufferFnExtDiscardRectangles::EMPTY,
        ext_memory_decompression: CommandBufferFnExtMemoryDecompression::EMPTY,
        ext_mesh_shader: CommandBufferFnExtMeshShader::EMPTY,
        ext_multi_draw: CommandBufferFnExtMultiDraw::EMPTY,
        ext_opacity_micromap: CommandBufferFnExtOpacityMicromap::EMPTY,
        ext_primitive_restart_index: CommandBufferFnExtPrimitiveRestartIndex::EMPTY,
        ext_sample_locations: CommandBufferFnExtSampleLocations::EMPTY,
        ext_shader_object: CommandBufferFnExtShaderObject::EMPTY,
        ext_transform_feedback: CommandBufferFnExtTransformFeedback::EMPTY,
        huawei_cluster_culling_shader: CommandBufferFnHuaweiClusterCullingShader::EMPTY,
        huawei_invocation_mask: CommandBufferFnHuaweiInvocationMask::EMPTY,
        huawei_subpass_shading: CommandBufferFnHuaweiSubpassShading::EMPTY,
        intel_performance_query: CommandBufferFnIntelPerformanceQuery::EMPTY,
        khr_acceleration_structure: CommandBufferFnKhrAccelerationStructure::EMPTY,
        khr_copy_memory_indirect: CommandBufferFnKhrCopyMemoryIndirect::EMPTY,
        khr_device_address_commands: CommandBufferFnKhrDeviceAddressCommands::EMPTY,
        khr_fragment_shading_rate: CommandBufferFnKhrFragmentShadingRate::EMPTY,
        khr_maintenance10: CommandBufferFnKhrMaintenance10::EMPTY,
        khr_maintenance6: CommandBufferFnKhrMaintenance6::EMPTY,
        khr_object_refresh: CommandBufferFnKhrObjectRefresh::EMPTY,
        khr_ray_tracing_maintenance1: CommandBufferFnKhrRayTracingMaintenance1::EMPTY,
        khr_ray_tracing_pipeline: CommandBufferFnKhrRayTracingPipeline::EMPTY,
        khr_video_decode_queue: CommandBufferFnKhrVideoDecodeQueue::EMPTY,
        khr_video_encode_queue: CommandBufferFnKhrVideoEncodeQueue::EMPTY,
        khr_video_queue: CommandBufferFnKhrVideoQueue::EMPTY,
        nv_clip_space_w_scaling: CommandBufferFnNvClipSpaceWScaling::EMPTY,
        nv_cluster_acceleration_structure: CommandBufferFnNvClusterAccelerationStructure::EMPTY,
        nv_compute_occupancy_priority: CommandBufferFnNvComputeOccupancyPriority::EMPTY,
        nv_cooperative_vector: CommandBufferFnNvCooperativeVector::EMPTY,
        nv_copy_memory_indirect: CommandBufferFnNvCopyMemoryIndirect::EMPTY,
        nv_cuda_kernel_launch: CommandBufferFnNvCudaKernelLaunch::EMPTY,
        nv_device_diagnostic_checkpoints: CommandBufferFnNvDeviceDiagnosticCheckpoints::EMPTY,
        nv_device_generated_commands: CommandBufferFnNvDeviceGeneratedCommands::EMPTY,
        nv_device_generated_commands_compute:
            CommandBufferFnNvDeviceGeneratedCommandsCompute::EMPTY,
        nv_fragment_shading_rate_enums: CommandBufferFnNvFragmentShadingRateEnums::EMPTY,
        nv_memory_decompression: CommandBufferFnNvMemoryDecompression::EMPTY,
        nv_mesh_shader: CommandBufferFnNvMeshShader::EMPTY,
        nv_optical_flow: CommandBufferFnNvOpticalFlow::EMPTY,
        nv_partitioned_acceleration_structure:
            CommandBufferFnNvPartitionedAccelerationStructure::EMPTY,
        nv_ray_tracing: CommandBufferFnNvRayTracing::EMPTY,
        nv_scissor_exclusive: CommandBufferFnNvScissorExclusive::EMPTY,
        nv_shading_rate_image: CommandBufferFnNvShadingRateImage::EMPTY,
        nvx_binary_import: CommandBufferFnNvxBinaryImport::EMPTY,
        qcom_tile_memory_heap: CommandBufferFnQcomTileMemoryHeap::EMPTY,
        qcom_tile_shading: CommandBufferFnQcomTileShading::EMPTY,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        let mut out = Self {
            v1_0: CommandBufferFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                CommandBufferFnv1_1::load(&mut loader)
            } else {
                CommandBufferFnv1_1::EMPTY
            },
            v1_2: if api_version >= API_VERSION_1_2 {
                CommandBufferFnv1_2::load(&mut loader)
            } else {
                CommandBufferFnv1_2::EMPTY
            },
            v1_3: if api_version >= API_VERSION_1_3 {
                CommandBufferFnv1_3::load(&mut loader)
            } else {
                CommandBufferFnv1_3::EMPTY
            },
            v1_4: if api_version >= API_VERSION_1_4 {
                CommandBufferFnv1_4::load(&mut loader)
            } else {
                CommandBufferFnv1_4::EMPTY
            },
            amd_buffer_marker: CommandBufferFnAmdBufferMarker::EMPTY,
            amd_gpa_interface: CommandBufferFnAmdGpaInterface::EMPTY,
            amdx_shader_enqueue: CommandBufferFnAmdxShaderEnqueue::EMPTY,
            arm_data_graph: CommandBufferFnArmDataGraph::EMPTY,
            arm_scheduling_controls: CommandBufferFnArmSchedulingControls::EMPTY,
            arm_shader_instrumentation: CommandBufferFnArmShaderInstrumentation::EMPTY,
            arm_tensors: CommandBufferFnArmTensors::EMPTY,
            ext_attachment_feedback_loop_dynamic_state:
                CommandBufferFnExtAttachmentFeedbackLoopDynamicState::EMPTY,
            ext_color_write_enable: CommandBufferFnExtColorWriteEnable::EMPTY,
            ext_conditional_rendering: CommandBufferFnExtConditionalRendering::EMPTY,
            ext_custom_resolve: CommandBufferFnExtCustomResolve::EMPTY,
            ext_debug_marker: CommandBufferFnExtDebugMarker::EMPTY,
            ext_debug_utils: CommandBufferFnExtDebugUtils::EMPTY,
            ext_depth_bias_control: CommandBufferFnExtDepthBiasControl::EMPTY,
            ext_depth_clamp_control: CommandBufferFnExtDepthClampControl::EMPTY,
            ext_descriptor_buffer: CommandBufferFnExtDescriptorBuffer::EMPTY,
            ext_descriptor_heap: CommandBufferFnExtDescriptorHeap::EMPTY,
            ext_device_generated_commands: CommandBufferFnExtDeviceGeneratedCommands::EMPTY,
            ext_discard_rectangles: CommandBufferFnExtDiscardRectangles::EMPTY,
            ext_memory_decompression: CommandBufferFnExtMemoryDecompression::EMPTY,
            ext_mesh_shader: CommandBufferFnExtMeshShader::EMPTY,
            ext_multi_draw: CommandBufferFnExtMultiDraw::EMPTY,
            ext_opacity_micromap: CommandBufferFnExtOpacityMicromap::EMPTY,
            ext_primitive_restart_index: CommandBufferFnExtPrimitiveRestartIndex::EMPTY,
            ext_sample_locations: CommandBufferFnExtSampleLocations::EMPTY,
            ext_shader_object: CommandBufferFnExtShaderObject::EMPTY,
            ext_transform_feedback: CommandBufferFnExtTransformFeedback::EMPTY,
            huawei_cluster_culling_shader: CommandBufferFnHuaweiClusterCullingShader::EMPTY,
            huawei_invocation_mask: CommandBufferFnHuaweiInvocationMask::EMPTY,
            huawei_subpass_shading: CommandBufferFnHuaweiSubpassShading::EMPTY,
            intel_performance_query: CommandBufferFnIntelPerformanceQuery::EMPTY,
            khr_acceleration_structure: CommandBufferFnKhrAccelerationStructure::EMPTY,
            khr_copy_memory_indirect: CommandBufferFnKhrCopyMemoryIndirect::EMPTY,
            khr_device_address_commands: CommandBufferFnKhrDeviceAddressCommands::EMPTY,
            khr_fragment_shading_rate: CommandBufferFnKhrFragmentShadingRate::EMPTY,
            khr_maintenance10: CommandBufferFnKhrMaintenance10::EMPTY,
            khr_maintenance6: CommandBufferFnKhrMaintenance6::EMPTY,
            khr_object_refresh: CommandBufferFnKhrObjectRefresh::EMPTY,
            khr_ray_tracing_maintenance1: CommandBufferFnKhrRayTracingMaintenance1::EMPTY,
            khr_ray_tracing_pipeline: CommandBufferFnKhrRayTracingPipeline::EMPTY,
            khr_video_decode_queue: CommandBufferFnKhrVideoDecodeQueue::EMPTY,
            khr_video_encode_queue: CommandBufferFnKhrVideoEncodeQueue::EMPTY,
            khr_video_queue: CommandBufferFnKhrVideoQueue::EMPTY,
            nv_clip_space_w_scaling: CommandBufferFnNvClipSpaceWScaling::EMPTY,
            nv_cluster_acceleration_structure: CommandBufferFnNvClusterAccelerationStructure::EMPTY,
            nv_compute_occupancy_priority: CommandBufferFnNvComputeOccupancyPriority::EMPTY,
            nv_cooperative_vector: CommandBufferFnNvCooperativeVector::EMPTY,
            nv_copy_memory_indirect: CommandBufferFnNvCopyMemoryIndirect::EMPTY,
            nv_cuda_kernel_launch: CommandBufferFnNvCudaKernelLaunch::EMPTY,
            nv_device_diagnostic_checkpoints: CommandBufferFnNvDeviceDiagnosticCheckpoints::EMPTY,
            nv_device_generated_commands: CommandBufferFnNvDeviceGeneratedCommands::EMPTY,
            nv_device_generated_commands_compute:
                CommandBufferFnNvDeviceGeneratedCommandsCompute::EMPTY,
            nv_fragment_shading_rate_enums: CommandBufferFnNvFragmentShadingRateEnums::EMPTY,
            nv_memory_decompression: CommandBufferFnNvMemoryDecompression::EMPTY,
            nv_mesh_shader: CommandBufferFnNvMeshShader::EMPTY,
            nv_optical_flow: CommandBufferFnNvOpticalFlow::EMPTY,
            nv_partitioned_acceleration_structure:
                CommandBufferFnNvPartitionedAccelerationStructure::EMPTY,
            nv_ray_tracing: CommandBufferFnNvRayTracing::EMPTY,
            nv_scissor_exclusive: CommandBufferFnNvScissorExclusive::EMPTY,
            nv_shading_rate_image: CommandBufferFnNvShadingRateImage::EMPTY,
            nvx_binary_import: CommandBufferFnNvxBinaryImport::EMPTY,
            qcom_tile_memory_heap: CommandBufferFnQcomTileMemoryHeap::EMPTY,
            qcom_tile_shading: CommandBufferFnQcomTileShading::EMPTY,
        };
        out.ext_debug_utils.begin_debug_utils_label_ext =
            to_option(loader(c"vkCmdBeginDebugUtilsLabelEXT"));
        out.ext_debug_utils.end_debug_utils_label_ext =
            to_option(loader(c"vkCmdEndDebugUtilsLabelEXT"));
        out.ext_debug_utils.insert_debug_utils_label_ext =
            to_option(loader(c"vkCmdInsertDebugUtilsLabelEXT"));
        for &ext in extensions {
            match unsafe { CStr::from_ptr(ext) }.to_bytes() {
                b"VK_EXT_debug_marker" => {
                    out.ext_debug_marker.debug_marker_begin_ext =
                        to_option(loader(c"vkCmdDebugMarkerBeginEXT"));
                    out.ext_debug_marker.debug_marker_end_ext =
                        to_option(loader(c"vkCmdDebugMarkerEndEXT"));
                    out.ext_debug_marker.debug_marker_insert_ext =
                        to_option(loader(c"vkCmdDebugMarkerInsertEXT"));
                }
                b"VK_KHR_video_queue" => {
                    out.khr_video_queue.begin_video_coding_khr =
                        to_option(loader(c"vkCmdBeginVideoCodingKHR"));
                    out.khr_video_queue.end_video_coding_khr =
                        to_option(loader(c"vkCmdEndVideoCodingKHR"));
                    out.khr_video_queue.control_video_coding_khr =
                        to_option(loader(c"vkCmdControlVideoCodingKHR"));
                }
                b"VK_KHR_video_decode_queue" => {
                    out.khr_video_decode_queue.decode_video_khr =
                        to_option(loader(c"vkCmdDecodeVideoKHR"));
                }
                b"VK_EXT_transform_feedback" => {
                    out.ext_transform_feedback
                        .bind_transform_feedback_buffers_ext =
                        to_option(loader(c"vkCmdBindTransformFeedbackBuffersEXT"));
                    out.ext_transform_feedback.begin_transform_feedback_ext =
                        to_option(loader(c"vkCmdBeginTransformFeedbackEXT"));
                    out.ext_transform_feedback.end_transform_feedback_ext =
                        to_option(loader(c"vkCmdEndTransformFeedbackEXT"));
                    out.ext_transform_feedback.begin_query_indexed_ext =
                        to_option(loader(c"vkCmdBeginQueryIndexedEXT"));
                    out.ext_transform_feedback.end_query_indexed_ext =
                        to_option(loader(c"vkCmdEndQueryIndexedEXT"));
                    out.ext_transform_feedback.draw_indirect_byte_count_ext =
                        to_option(loader(c"vkCmdDrawIndirectByteCountEXT"));
                }
                b"VK_NVX_binary_import" => {
                    out.nvx_binary_import.cu_launch_kernel_nvx =
                        to_option(loader(c"vkCmdCuLaunchKernelNVX"));
                }
                b"VK_AMD_draw_indirect_count" => {
                    if out.v1_2.draw_indirect_count.is_none() {
                        out.v1_2.draw_indirect_count =
                            to_option(loader(c"vkCmdDrawIndirectCountAMD"));
                    }
                    if out.v1_2.draw_indexed_indirect_count.is_none() {
                        out.v1_2.draw_indexed_indirect_count =
                            to_option(loader(c"vkCmdDrawIndexedIndirectCountAMD"));
                    }
                }
                b"VK_KHR_dynamic_rendering" => {
                    if out.v1_3.begin_rendering.is_none() {
                        out.v1_3.begin_rendering = to_option(loader(c"vkCmdBeginRenderingKHR"));
                    }
                    if out.v1_3.end_rendering.is_none() {
                        out.v1_3.end_rendering = to_option(loader(c"vkCmdEndRenderingKHR"));
                    }
                }
                b"VK_KHR_device_group" => {
                    if out.v1_1.set_device_mask.is_none() {
                        out.v1_1.set_device_mask = to_option(loader(c"vkCmdSetDeviceMaskKHR"));
                    }
                    if out.v1_1.dispatch_base.is_none() {
                        out.v1_1.dispatch_base = to_option(loader(c"vkCmdDispatchBaseKHR"));
                    }
                }
                b"VK_KHR_push_descriptor" => {
                    if out.v1_4.push_descriptor_set.is_none() {
                        out.v1_4.push_descriptor_set =
                            to_option(loader(c"vkCmdPushDescriptorSetKHR"));
                    }
                    if out.v1_4.push_descriptor_set_with_template.is_none() {
                        out.v1_4.push_descriptor_set_with_template =
                            to_option(loader(c"vkCmdPushDescriptorSetWithTemplateKHR"));
                    }
                }
                b"VK_EXT_conditional_rendering" => {
                    out.ext_conditional_rendering
                        .begin_conditional_rendering_ext =
                        to_option(loader(c"vkCmdBeginConditionalRenderingEXT"));
                    out.ext_conditional_rendering.end_conditional_rendering_ext =
                        to_option(loader(c"vkCmdEndConditionalRenderingEXT"));
                }
                b"VK_KHR_descriptor_update_template" => {
                    if out.v1_4.push_descriptor_set_with_template.is_none() {
                        out.v1_4.push_descriptor_set_with_template =
                            to_option(loader(c"vkCmdPushDescriptorSetWithTemplateKHR"));
                    }
                }
                b"VK_NV_clip_space_w_scaling" => {
                    out.nv_clip_space_w_scaling.set_viewport_w_scaling_nv =
                        to_option(loader(c"vkCmdSetViewportWScalingNV"));
                }
                b"VK_EXT_discard_rectangles" => {
                    out.ext_discard_rectangles.set_discard_rectangle_ext =
                        to_option(loader(c"vkCmdSetDiscardRectangleEXT"));
                    out.ext_discard_rectangles.set_discard_rectangle_enable_ext =
                        to_option(loader(c"vkCmdSetDiscardRectangleEnableEXT"));
                    out.ext_discard_rectangles.set_discard_rectangle_mode_ext =
                        to_option(loader(c"vkCmdSetDiscardRectangleModeEXT"));
                }
                b"VK_KHR_create_renderpass2" => {
                    if out.v1_2.begin_render_pass2.is_none() {
                        out.v1_2.begin_render_pass2 =
                            to_option(loader(c"vkCmdBeginRenderPass2KHR"));
                    }
                    if out.v1_2.next_subpass2.is_none() {
                        out.v1_2.next_subpass2 = to_option(loader(c"vkCmdNextSubpass2KHR"));
                    }
                    if out.v1_2.end_render_pass2.is_none() {
                        out.v1_2.end_render_pass2 = to_option(loader(c"vkCmdEndRenderPass2KHR"));
                    }
                }
                b"VK_AMD_gpa_interface" => {
                    out.amd_gpa_interface.begin_gpa_session_amd =
                        to_option(loader(c"vkCmdBeginGpaSessionAMD"));
                    out.amd_gpa_interface.end_gpa_session_amd =
                        to_option(loader(c"vkCmdEndGpaSessionAMD"));
                    out.amd_gpa_interface.begin_gpa_sample_amd =
                        to_option(loader(c"vkCmdBeginGpaSampleAMD"));
                    out.amd_gpa_interface.end_gpa_sample_amd =
                        to_option(loader(c"vkCmdEndGpaSampleAMD"));
                    out.amd_gpa_interface.copy_gpa_session_results_amd =
                        to_option(loader(c"vkCmdCopyGpaSessionResultsAMD"));
                }
                b"VK_AMDX_shader_enqueue" => {
                    out.amdx_shader_enqueue.initialize_graph_scratch_memory_amdx =
                        to_option(loader(c"vkCmdInitializeGraphScratchMemoryAMDX"));
                    out.amdx_shader_enqueue.dispatch_graph_amdx =
                        to_option(loader(c"vkCmdDispatchGraphAMDX"));
                    out.amdx_shader_enqueue.dispatch_graph_indirect_amdx =
                        to_option(loader(c"vkCmdDispatchGraphIndirectAMDX"));
                    out.amdx_shader_enqueue.dispatch_graph_indirect_count_amdx =
                        to_option(loader(c"vkCmdDispatchGraphIndirectCountAMDX"));
                }
                b"VK_EXT_descriptor_heap" => {
                    out.ext_descriptor_heap.bind_sampler_heap_ext =
                        to_option(loader(c"vkCmdBindSamplerHeapEXT"));
                    out.ext_descriptor_heap.bind_resource_heap_ext =
                        to_option(loader(c"vkCmdBindResourceHeapEXT"));
                    out.ext_descriptor_heap.push_data_ext = to_option(loader(c"vkCmdPushDataEXT"));
                }
                b"VK_EXT_sample_locations" => {
                    out.ext_sample_locations.set_sample_locations_ext =
                        to_option(loader(c"vkCmdSetSampleLocationsEXT"));
                }
                b"VK_KHR_acceleration_structure" => {
                    out.khr_acceleration_structure
                        .build_acceleration_structures_khr =
                        to_option(loader(c"vkCmdBuildAccelerationStructuresKHR"));
                    out.khr_acceleration_structure
                        .build_acceleration_structures_indirect_khr =
                        to_option(loader(c"vkCmdBuildAccelerationStructuresIndirectKHR"));
                    out.khr_acceleration_structure
                        .copy_acceleration_structure_khr =
                        to_option(loader(c"vkCmdCopyAccelerationStructureKHR"));
                    out.khr_acceleration_structure
                        .copy_acceleration_structure_to_memory_khr =
                        to_option(loader(c"vkCmdCopyAccelerationStructureToMemoryKHR"));
                    out.khr_acceleration_structure
                        .copy_memory_to_acceleration_structure_khr =
                        to_option(loader(c"vkCmdCopyMemoryToAccelerationStructureKHR"));
                    out.khr_acceleration_structure
                        .write_acceleration_structures_properties_khr =
                        to_option(loader(c"vkCmdWriteAccelerationStructuresPropertiesKHR"));
                }
                b"VK_KHR_ray_tracing_pipeline" => {
                    out.khr_ray_tracing_pipeline.trace_rays_khr =
                        to_option(loader(c"vkCmdTraceRaysKHR"));
                    out.khr_ray_tracing_pipeline.trace_rays_indirect_khr =
                        to_option(loader(c"vkCmdTraceRaysIndirectKHR"));
                    out.khr_ray_tracing_pipeline
                        .set_ray_tracing_pipeline_stack_size_khr =
                        to_option(loader(c"vkCmdSetRayTracingPipelineStackSizeKHR"));
                }
                b"VK_NV_shading_rate_image" => {
                    out.nv_shading_rate_image.bind_shading_rate_image_nv =
                        to_option(loader(c"vkCmdBindShadingRateImageNV"));
                    out.nv_shading_rate_image
                        .set_viewport_shading_rate_palette_nv =
                        to_option(loader(c"vkCmdSetViewportShadingRatePaletteNV"));
                    out.nv_shading_rate_image.set_coarse_sample_order_nv =
                        to_option(loader(c"vkCmdSetCoarseSampleOrderNV"));
                }
                b"VK_NV_ray_tracing" => {
                    out.nv_ray_tracing.build_acceleration_structure_nv =
                        to_option(loader(c"vkCmdBuildAccelerationStructureNV"));
                    out.nv_ray_tracing.copy_acceleration_structure_nv =
                        to_option(loader(c"vkCmdCopyAccelerationStructureNV"));
                    out.nv_ray_tracing.trace_rays_nv = to_option(loader(c"vkCmdTraceRaysNV"));
                    out.nv_ray_tracing
                        .write_acceleration_structures_properties_nv =
                        to_option(loader(c"vkCmdWriteAccelerationStructuresPropertiesNV"));
                }
                b"VK_KHR_draw_indirect_count" => {
                    if out.v1_2.draw_indirect_count.is_none() {
                        out.v1_2.draw_indirect_count =
                            to_option(loader(c"vkCmdDrawIndirectCountKHR"));
                    }
                    if out.v1_2.draw_indexed_indirect_count.is_none() {
                        out.v1_2.draw_indexed_indirect_count =
                            to_option(loader(c"vkCmdDrawIndexedIndirectCountKHR"));
                    }
                }
                b"VK_AMD_buffer_marker" => {
                    out.amd_buffer_marker.write_buffer_marker_amd =
                        to_option(loader(c"vkCmdWriteBufferMarkerAMD"));
                    out.amd_buffer_marker.write_buffer_marker2_amd =
                        to_option(loader(c"vkCmdWriteBufferMarker2AMD"));
                }
                b"VK_NV_mesh_shader" => {
                    out.nv_mesh_shader.draw_mesh_tasks_nv =
                        to_option(loader(c"vkCmdDrawMeshTasksNV"));
                    out.nv_mesh_shader.draw_mesh_tasks_indirect_nv =
                        to_option(loader(c"vkCmdDrawMeshTasksIndirectNV"));
                    out.nv_mesh_shader.draw_mesh_tasks_indirect_count_nv =
                        to_option(loader(c"vkCmdDrawMeshTasksIndirectCountNV"));
                }
                b"VK_NV_scissor_exclusive" => {
                    out.nv_scissor_exclusive.set_exclusive_scissor_enable_nv =
                        to_option(loader(c"vkCmdSetExclusiveScissorEnableNV"));
                    out.nv_scissor_exclusive.set_exclusive_scissor_nv =
                        to_option(loader(c"vkCmdSetExclusiveScissorNV"));
                }
                b"VK_NV_device_diagnostic_checkpoints" => {
                    out.nv_device_diagnostic_checkpoints.set_checkpoint_nv =
                        to_option(loader(c"vkCmdSetCheckpointNV"));
                }
                b"VK_INTEL_performance_query" => {
                    out.intel_performance_query.set_performance_marker_intel =
                        to_option(loader(c"vkCmdSetPerformanceMarkerINTEL"));
                    out.intel_performance_query
                        .set_performance_stream_marker_intel =
                        to_option(loader(c"vkCmdSetPerformanceStreamMarkerINTEL"));
                    out.intel_performance_query.set_performance_override_intel =
                        to_option(loader(c"vkCmdSetPerformanceOverrideINTEL"));
                }
                b"VK_KHR_fragment_shading_rate" => {
                    out.khr_fragment_shading_rate.set_fragment_shading_rate_khr =
                        to_option(loader(c"vkCmdSetFragmentShadingRateKHR"));
                }
                b"VK_KHR_dynamic_rendering_local_read" => {
                    if out.v1_4.set_rendering_attachment_locations.is_none() {
                        out.v1_4.set_rendering_attachment_locations =
                            to_option(loader(c"vkCmdSetRenderingAttachmentLocationsKHR"));
                    }
                    if out.v1_4.set_rendering_input_attachment_indices.is_none() {
                        out.v1_4.set_rendering_input_attachment_indices =
                            to_option(loader(c"vkCmdSetRenderingInputAttachmentIndicesKHR"));
                    }
                }
                b"VK_EXT_line_rasterization" => {
                    if out.v1_4.set_line_stipple.is_none() {
                        out.v1_4.set_line_stipple = to_option(loader(c"vkCmdSetLineStippleEXT"));
                    }
                }
                b"VK_EXT_extended_dynamic_state" => {
                    if out.v1_3.set_cull_mode.is_none() {
                        out.v1_3.set_cull_mode = to_option(loader(c"vkCmdSetCullModeEXT"));
                    }
                    if out.v1_3.set_front_face.is_none() {
                        out.v1_3.set_front_face = to_option(loader(c"vkCmdSetFrontFaceEXT"));
                    }
                    if out.v1_3.set_primitive_topology.is_none() {
                        out.v1_3.set_primitive_topology =
                            to_option(loader(c"vkCmdSetPrimitiveTopologyEXT"));
                    }
                    if out.v1_3.set_viewport_with_count.is_none() {
                        out.v1_3.set_viewport_with_count =
                            to_option(loader(c"vkCmdSetViewportWithCountEXT"));
                    }
                    if out.v1_3.set_scissor_with_count.is_none() {
                        out.v1_3.set_scissor_with_count =
                            to_option(loader(c"vkCmdSetScissorWithCountEXT"));
                    }
                    if out.v1_3.bind_vertex_buffers2.is_none() {
                        out.v1_3.bind_vertex_buffers2 =
                            to_option(loader(c"vkCmdBindVertexBuffers2EXT"));
                    }
                    if out.v1_3.set_depth_test_enable.is_none() {
                        out.v1_3.set_depth_test_enable =
                            to_option(loader(c"vkCmdSetDepthTestEnableEXT"));
                    }
                    if out.v1_3.set_depth_write_enable.is_none() {
                        out.v1_3.set_depth_write_enable =
                            to_option(loader(c"vkCmdSetDepthWriteEnableEXT"));
                    }
                    if out.v1_3.set_depth_compare_op.is_none() {
                        out.v1_3.set_depth_compare_op =
                            to_option(loader(c"vkCmdSetDepthCompareOpEXT"));
                    }
                    if out.v1_3.set_depth_bounds_test_enable.is_none() {
                        out.v1_3.set_depth_bounds_test_enable =
                            to_option(loader(c"vkCmdSetDepthBoundsTestEnableEXT"));
                    }
                    if out.v1_3.set_stencil_test_enable.is_none() {
                        out.v1_3.set_stencil_test_enable =
                            to_option(loader(c"vkCmdSetStencilTestEnableEXT"));
                    }
                    if out.v1_3.set_stencil_op.is_none() {
                        out.v1_3.set_stencil_op = to_option(loader(c"vkCmdSetStencilOpEXT"));
                    }
                }
                b"VK_NV_device_generated_commands" => {
                    out.nv_device_generated_commands
                        .preprocess_generated_commands_nv =
                        to_option(loader(c"vkCmdPreprocessGeneratedCommandsNV"));
                    out.nv_device_generated_commands
                        .execute_generated_commands_nv =
                        to_option(loader(c"vkCmdExecuteGeneratedCommandsNV"));
                    out.nv_device_generated_commands
                        .bind_pipeline_shader_group_nv =
                        to_option(loader(c"vkCmdBindPipelineShaderGroupNV"));
                }
                b"VK_EXT_depth_bias_control" => {
                    out.ext_depth_bias_control.set_depth_bias2_ext =
                        to_option(loader(c"vkCmdSetDepthBias2EXT"));
                }
                b"VK_KHR_video_encode_queue" => {
                    out.khr_video_encode_queue.encode_video_khr =
                        to_option(loader(c"vkCmdEncodeVideoKHR"));
                }
                b"VK_NV_cuda_kernel_launch" => {
                    out.nv_cuda_kernel_launch.cuda_launch_kernel_nv =
                        to_option(loader(c"vkCmdCudaLaunchKernelNV"));
                }
                b"VK_KHR_object_refresh" => {
                    out.khr_object_refresh.refresh_objects_khr =
                        to_option(loader(c"vkCmdRefreshObjectsKHR"));
                }
                b"VK_QCOM_tile_shading" => {
                    out.qcom_tile_shading.dispatch_tile_qcom =
                        to_option(loader(c"vkCmdDispatchTileQCOM"));
                    out.qcom_tile_shading.begin_per_tile_execution_qcom =
                        to_option(loader(c"vkCmdBeginPerTileExecutionQCOM"));
                    out.qcom_tile_shading.end_per_tile_execution_qcom =
                        to_option(loader(c"vkCmdEndPerTileExecutionQCOM"));
                }
                b"VK_KHR_synchronization2" => {
                    if out.v1_3.set_event2.is_none() {
                        out.v1_3.set_event2 = to_option(loader(c"vkCmdSetEvent2KHR"));
                    }
                    if out.v1_3.reset_event2.is_none() {
                        out.v1_3.reset_event2 = to_option(loader(c"vkCmdResetEvent2KHR"));
                    }
                    if out.v1_3.wait_events2.is_none() {
                        out.v1_3.wait_events2 = to_option(loader(c"vkCmdWaitEvents2KHR"));
                    }
                    if out.v1_3.pipeline_barrier2.is_none() {
                        out.v1_3.pipeline_barrier2 = to_option(loader(c"vkCmdPipelineBarrier2KHR"));
                    }
                    if out.v1_3.write_timestamp2.is_none() {
                        out.v1_3.write_timestamp2 = to_option(loader(c"vkCmdWriteTimestamp2KHR"));
                    }
                }
                b"VK_EXT_descriptor_buffer" => {
                    out.ext_descriptor_buffer.bind_descriptor_buffers_ext =
                        to_option(loader(c"vkCmdBindDescriptorBuffersEXT"));
                    out.ext_descriptor_buffer.set_descriptor_buffer_offsets_ext =
                        to_option(loader(c"vkCmdSetDescriptorBufferOffsetsEXT"));
                    out.ext_descriptor_buffer
                        .bind_descriptor_buffer_embedded_samplers_ext =
                        to_option(loader(c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT"));
                }
                b"VK_KHR_device_address_commands" => {
                    out.khr_device_address_commands.bind_index_buffer3_khr =
                        to_option(loader(c"vkCmdBindIndexBuffer3KHR"));
                    out.khr_device_address_commands.bind_vertex_buffers3_khr =
                        to_option(loader(c"vkCmdBindVertexBuffers3KHR"));
                    out.khr_device_address_commands.draw_indirect2_khr =
                        to_option(loader(c"vkCmdDrawIndirect2KHR"));
                    out.khr_device_address_commands.draw_indexed_indirect2_khr =
                        to_option(loader(c"vkCmdDrawIndexedIndirect2KHR"));
                    out.khr_device_address_commands.dispatch_indirect2_khr =
                        to_option(loader(c"vkCmdDispatchIndirect2KHR"));
                    out.khr_device_address_commands.copy_memory_khr =
                        to_option(loader(c"vkCmdCopyMemoryKHR"));
                    out.khr_device_address_commands.copy_memory_to_image_khr =
                        to_option(loader(c"vkCmdCopyMemoryToImageKHR"));
                    out.khr_device_address_commands.copy_image_to_memory_khr =
                        to_option(loader(c"vkCmdCopyImageToMemoryKHR"));
                    out.khr_device_address_commands.update_memory_khr =
                        to_option(loader(c"vkCmdUpdateMemoryKHR"));
                    out.khr_device_address_commands.fill_memory_khr =
                        to_option(loader(c"vkCmdFillMemoryKHR"));
                    out.khr_device_address_commands
                        .copy_query_pool_results_to_memory_khr =
                        to_option(loader(c"vkCmdCopyQueryPoolResultsToMemoryKHR"));
                    out.khr_device_address_commands.draw_indirect_count2_khr =
                        to_option(loader(c"vkCmdDrawIndirectCount2KHR"));
                    out.khr_device_address_commands
                        .draw_indexed_indirect_count2_khr =
                        to_option(loader(c"vkCmdDrawIndexedIndirectCount2KHR"));
                    out.khr_device_address_commands
                        .begin_conditional_rendering2_ext =
                        to_option(loader(c"vkCmdBeginConditionalRendering2EXT"));
                    out.khr_device_address_commands
                        .bind_transform_feedback_buffers2_ext =
                        to_option(loader(c"vkCmdBindTransformFeedbackBuffers2EXT"));
                    out.khr_device_address_commands
                        .begin_transform_feedback2_ext =
                        to_option(loader(c"vkCmdBeginTransformFeedback2EXT"));
                    out.khr_device_address_commands.end_transform_feedback2_ext =
                        to_option(loader(c"vkCmdEndTransformFeedback2EXT"));
                    out.khr_device_address_commands
                        .draw_indirect_byte_count2_ext =
                        to_option(loader(c"vkCmdDrawIndirectByteCount2EXT"));
                    out.khr_device_address_commands
                        .draw_mesh_tasks_indirect2_ext =
                        to_option(loader(c"vkCmdDrawMeshTasksIndirect2EXT"));
                    out.khr_device_address_commands
                        .draw_mesh_tasks_indirect_count2_ext =
                        to_option(loader(c"vkCmdDrawMeshTasksIndirectCount2EXT"));
                    out.khr_device_address_commands.write_marker_to_memory_amd =
                        to_option(loader(c"vkCmdWriteMarkerToMemoryAMD"));
                }
                b"VK_NV_fragment_shading_rate_enums" => {
                    out.nv_fragment_shading_rate_enums
                        .set_fragment_shading_rate_enum_nv =
                        to_option(loader(c"vkCmdSetFragmentShadingRateEnumNV"));
                }
                b"VK_EXT_mesh_shader" => {
                    out.ext_mesh_shader.draw_mesh_tasks_ext =
                        to_option(loader(c"vkCmdDrawMeshTasksEXT"));
                    out.ext_mesh_shader.draw_mesh_tasks_indirect_ext =
                        to_option(loader(c"vkCmdDrawMeshTasksIndirectEXT"));
                    out.ext_mesh_shader.draw_mesh_tasks_indirect_count_ext =
                        to_option(loader(c"vkCmdDrawMeshTasksIndirectCountEXT"));
                }
                b"VK_KHR_copy_commands2" => {
                    if out.v1_3.copy_buffer2.is_none() {
                        out.v1_3.copy_buffer2 = to_option(loader(c"vkCmdCopyBuffer2KHR"));
                    }
                    if out.v1_3.copy_image2.is_none() {
                        out.v1_3.copy_image2 = to_option(loader(c"vkCmdCopyImage2KHR"));
                    }
                    if out.v1_3.copy_buffer_to_image2.is_none() {
                        out.v1_3.copy_buffer_to_image2 =
                            to_option(loader(c"vkCmdCopyBufferToImage2KHR"));
                    }
                    if out.v1_3.copy_image_to_buffer2.is_none() {
                        out.v1_3.copy_image_to_buffer2 =
                            to_option(loader(c"vkCmdCopyImageToBuffer2KHR"));
                    }
                    if out.v1_3.blit_image2.is_none() {
                        out.v1_3.blit_image2 = to_option(loader(c"vkCmdBlitImage2KHR"));
                    }
                    if out.v1_3.resolve_image2.is_none() {
                        out.v1_3.resolve_image2 = to_option(loader(c"vkCmdResolveImage2KHR"));
                    }
                }
                b"VK_EXT_vertex_input_dynamic_state" => {
                    if out.ext_shader_object.set_vertex_input_ext.is_none() {
                        out.ext_shader_object.set_vertex_input_ext =
                            to_option(loader(c"vkCmdSetVertexInputEXT"));
                    }
                }
                b"VK_HUAWEI_subpass_shading" => {
                    out.huawei_subpass_shading.subpass_shading_huawei =
                        to_option(loader(c"vkCmdSubpassShadingHUAWEI"));
                }
                b"VK_HUAWEI_invocation_mask" => {
                    out.huawei_invocation_mask.bind_invocation_mask_huawei =
                        to_option(loader(c"vkCmdBindInvocationMaskHUAWEI"));
                }
                b"VK_EXT_extended_dynamic_state2" => {
                    if out.ext_shader_object.set_patch_control_points_ext.is_none() {
                        out.ext_shader_object.set_patch_control_points_ext =
                            to_option(loader(c"vkCmdSetPatchControlPointsEXT"));
                    }
                    if out.v1_3.set_rasterizer_discard_enable.is_none() {
                        out.v1_3.set_rasterizer_discard_enable =
                            to_option(loader(c"vkCmdSetRasterizerDiscardEnableEXT"));
                    }
                    if out.v1_3.set_depth_bias_enable.is_none() {
                        out.v1_3.set_depth_bias_enable =
                            to_option(loader(c"vkCmdSetDepthBiasEnableEXT"));
                    }
                    if out.ext_shader_object.set_logic_op_ext.is_none() {
                        out.ext_shader_object.set_logic_op_ext =
                            to_option(loader(c"vkCmdSetLogicOpEXT"));
                    }
                    if out.v1_3.set_primitive_restart_enable.is_none() {
                        out.v1_3.set_primitive_restart_enable =
                            to_option(loader(c"vkCmdSetPrimitiveRestartEnableEXT"));
                    }
                }
                b"VK_EXT_color_write_enable" => {
                    out.ext_color_write_enable.set_color_write_enable_ext =
                        to_option(loader(c"vkCmdSetColorWriteEnableEXT"));
                }
                b"VK_KHR_ray_tracing_maintenance1" => {
                    out.khr_ray_tracing_maintenance1.trace_rays_indirect2_khr =
                        to_option(loader(c"vkCmdTraceRaysIndirect2KHR"));
                }
                b"VK_EXT_multi_draw" => {
                    out.ext_multi_draw.draw_multi_ext = to_option(loader(c"vkCmdDrawMultiEXT"));
                    out.ext_multi_draw.draw_multi_indexed_ext =
                        to_option(loader(c"vkCmdDrawMultiIndexedEXT"));
                }
                b"VK_EXT_opacity_micromap" => {
                    out.ext_opacity_micromap.build_micromaps_ext =
                        to_option(loader(c"vkCmdBuildMicromapsEXT"));
                    out.ext_opacity_micromap.copy_micromap_ext =
                        to_option(loader(c"vkCmdCopyMicromapEXT"));
                    out.ext_opacity_micromap.copy_micromap_to_memory_ext =
                        to_option(loader(c"vkCmdCopyMicromapToMemoryEXT"));
                    out.ext_opacity_micromap.copy_memory_to_micromap_ext =
                        to_option(loader(c"vkCmdCopyMemoryToMicromapEXT"));
                    out.ext_opacity_micromap.write_micromaps_properties_ext =
                        to_option(loader(c"vkCmdWriteMicromapsPropertiesEXT"));
                }
                b"VK_HUAWEI_cluster_culling_shader" => {
                    out.huawei_cluster_culling_shader.draw_cluster_huawei =
                        to_option(loader(c"vkCmdDrawClusterHUAWEI"));
                    out.huawei_cluster_culling_shader
                        .draw_cluster_indirect_huawei =
                        to_option(loader(c"vkCmdDrawClusterIndirectHUAWEI"));
                }
                b"VK_ARM_scheduling_controls" => {
                    out.arm_scheduling_controls.set_dispatch_parameters_arm =
                        to_option(loader(c"vkCmdSetDispatchParametersARM"));
                }
                b"VK_NV_copy_memory_indirect" => {
                    out.nv_copy_memory_indirect.copy_memory_indirect_nv =
                        to_option(loader(c"vkCmdCopyMemoryIndirectNV"));
                    out.nv_copy_memory_indirect.copy_memory_to_image_indirect_nv =
                        to_option(loader(c"vkCmdCopyMemoryToImageIndirectNV"));
                }
                b"VK_NV_memory_decompression" => {
                    out.nv_memory_decompression.decompress_memory_nv =
                        to_option(loader(c"vkCmdDecompressMemoryNV"));
                    out.nv_memory_decompression
                        .decompress_memory_indirect_count_nv =
                        to_option(loader(c"vkCmdDecompressMemoryIndirectCountNV"));
                }
                b"VK_NV_device_generated_commands_compute" => {
                    out.nv_device_generated_commands_compute
                        .update_pipeline_indirect_buffer_nv =
                        to_option(loader(c"vkCmdUpdatePipelineIndirectBufferNV"));
                }
                b"VK_EXT_extended_dynamic_state3" => {
                    if out.ext_shader_object.set_depth_clamp_enable_ext.is_none() {
                        out.ext_shader_object.set_depth_clamp_enable_ext =
                            to_option(loader(c"vkCmdSetDepthClampEnableEXT"));
                    }
                    if out.ext_shader_object.set_polygon_mode_ext.is_none() {
                        out.ext_shader_object.set_polygon_mode_ext =
                            to_option(loader(c"vkCmdSetPolygonModeEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_rasterization_samples_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_rasterization_samples_ext =
                            to_option(loader(c"vkCmdSetRasterizationSamplesEXT"));
                    }
                    if out.ext_shader_object.set_sample_mask_ext.is_none() {
                        out.ext_shader_object.set_sample_mask_ext =
                            to_option(loader(c"vkCmdSetSampleMaskEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_alpha_to_coverage_enable_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_alpha_to_coverage_enable_ext =
                            to_option(loader(c"vkCmdSetAlphaToCoverageEnableEXT"));
                    }
                    if out.ext_shader_object.set_alpha_to_one_enable_ext.is_none() {
                        out.ext_shader_object.set_alpha_to_one_enable_ext =
                            to_option(loader(c"vkCmdSetAlphaToOneEnableEXT"));
                    }
                    if out.ext_shader_object.set_logic_op_enable_ext.is_none() {
                        out.ext_shader_object.set_logic_op_enable_ext =
                            to_option(loader(c"vkCmdSetLogicOpEnableEXT"));
                    }
                    if out.ext_shader_object.set_color_blend_enable_ext.is_none() {
                        out.ext_shader_object.set_color_blend_enable_ext =
                            to_option(loader(c"vkCmdSetColorBlendEnableEXT"));
                    }
                    if out.ext_shader_object.set_color_blend_equation_ext.is_none() {
                        out.ext_shader_object.set_color_blend_equation_ext =
                            to_option(loader(c"vkCmdSetColorBlendEquationEXT"));
                    }
                    if out.ext_shader_object.set_color_write_mask_ext.is_none() {
                        out.ext_shader_object.set_color_write_mask_ext =
                            to_option(loader(c"vkCmdSetColorWriteMaskEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_tessellation_domain_origin_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_tessellation_domain_origin_ext =
                            to_option(loader(c"vkCmdSetTessellationDomainOriginEXT"));
                    }
                    if out.ext_shader_object.set_rasterization_stream_ext.is_none() {
                        out.ext_shader_object.set_rasterization_stream_ext =
                            to_option(loader(c"vkCmdSetRasterizationStreamEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_conservative_rasterization_mode_ext
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_conservative_rasterization_mode_ext =
                            to_option(loader(c"vkCmdSetConservativeRasterizationModeEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_extra_primitive_overestimation_size_ext
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_extra_primitive_overestimation_size_ext =
                            to_option(loader(c"vkCmdSetExtraPrimitiveOverestimationSizeEXT"));
                    }
                    if out.ext_shader_object.set_depth_clip_enable_ext.is_none() {
                        out.ext_shader_object.set_depth_clip_enable_ext =
                            to_option(loader(c"vkCmdSetDepthClipEnableEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_sample_locations_enable_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_sample_locations_enable_ext =
                            to_option(loader(c"vkCmdSetSampleLocationsEnableEXT"));
                    }
                    if out.ext_shader_object.set_color_blend_advanced_ext.is_none() {
                        out.ext_shader_object.set_color_blend_advanced_ext =
                            to_option(loader(c"vkCmdSetColorBlendAdvancedEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_provoking_vertex_mode_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_provoking_vertex_mode_ext =
                            to_option(loader(c"vkCmdSetProvokingVertexModeEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_line_rasterization_mode_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_line_rasterization_mode_ext =
                            to_option(loader(c"vkCmdSetLineRasterizationModeEXT"));
                    }
                    if out.ext_shader_object.set_line_stipple_enable_ext.is_none() {
                        out.ext_shader_object.set_line_stipple_enable_ext =
                            to_option(loader(c"vkCmdSetLineStippleEnableEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_depth_clip_negative_one_to_one_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_depth_clip_negative_one_to_one_ext =
                            to_option(loader(c"vkCmdSetDepthClipNegativeOneToOneEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_viewport_w_scaling_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_viewport_w_scaling_enable_nv =
                            to_option(loader(c"vkCmdSetViewportWScalingEnableNV"));
                    }
                    if out.ext_shader_object.set_viewport_swizzle_nv.is_none() {
                        out.ext_shader_object.set_viewport_swizzle_nv =
                            to_option(loader(c"vkCmdSetViewportSwizzleNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_to_color_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_to_color_enable_nv =
                            to_option(loader(c"vkCmdSetCoverageToColorEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_to_color_location_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_to_color_location_nv =
                            to_option(loader(c"vkCmdSetCoverageToColorLocationNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_modulation_mode_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_modulation_mode_nv =
                            to_option(loader(c"vkCmdSetCoverageModulationModeNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_modulation_table_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_coverage_modulation_table_enable_nv =
                            to_option(loader(c"vkCmdSetCoverageModulationTableEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_modulation_table_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_modulation_table_nv =
                            to_option(loader(c"vkCmdSetCoverageModulationTableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_shading_rate_image_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_shading_rate_image_enable_nv =
                            to_option(loader(c"vkCmdSetShadingRateImageEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_representative_fragment_test_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_representative_fragment_test_enable_nv =
                            to_option(loader(c"vkCmdSetRepresentativeFragmentTestEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_reduction_mode_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_reduction_mode_nv =
                            to_option(loader(c"vkCmdSetCoverageReductionModeNV"));
                    }
                }
                b"VK_ARM_tensors" => {
                    out.arm_tensors.copy_tensor_arm = to_option(loader(c"vkCmdCopyTensorARM"));
                }
                b"VK_NV_optical_flow" => {
                    out.nv_optical_flow.optical_flow_execute_nv =
                        to_option(loader(c"vkCmdOpticalFlowExecuteNV"));
                }
                b"VK_KHR_maintenance5" => {
                    if out.v1_4.bind_index_buffer2.is_none() {
                        out.v1_4.bind_index_buffer2 =
                            to_option(loader(c"vkCmdBindIndexBuffer2KHR"));
                    }
                }
                b"VK_EXT_shader_object" => {
                    out.ext_shader_object.bind_shaders_ext =
                        to_option(loader(c"vkCmdBindShadersEXT"));
                    if out.v1_3.set_cull_mode.is_none() {
                        out.v1_3.set_cull_mode = to_option(loader(c"vkCmdSetCullModeEXT"));
                    }
                    if out.v1_3.set_front_face.is_none() {
                        out.v1_3.set_front_face = to_option(loader(c"vkCmdSetFrontFaceEXT"));
                    }
                    if out.v1_3.set_primitive_topology.is_none() {
                        out.v1_3.set_primitive_topology =
                            to_option(loader(c"vkCmdSetPrimitiveTopologyEXT"));
                    }
                    if out.v1_3.set_viewport_with_count.is_none() {
                        out.v1_3.set_viewport_with_count =
                            to_option(loader(c"vkCmdSetViewportWithCountEXT"));
                    }
                    if out.v1_3.set_scissor_with_count.is_none() {
                        out.v1_3.set_scissor_with_count =
                            to_option(loader(c"vkCmdSetScissorWithCountEXT"));
                    }
                    if out.v1_3.bind_vertex_buffers2.is_none() {
                        out.v1_3.bind_vertex_buffers2 =
                            to_option(loader(c"vkCmdBindVertexBuffers2EXT"));
                    }
                    if out.v1_3.set_depth_test_enable.is_none() {
                        out.v1_3.set_depth_test_enable =
                            to_option(loader(c"vkCmdSetDepthTestEnableEXT"));
                    }
                    if out.v1_3.set_depth_write_enable.is_none() {
                        out.v1_3.set_depth_write_enable =
                            to_option(loader(c"vkCmdSetDepthWriteEnableEXT"));
                    }
                    if out.v1_3.set_depth_compare_op.is_none() {
                        out.v1_3.set_depth_compare_op =
                            to_option(loader(c"vkCmdSetDepthCompareOpEXT"));
                    }
                    if out.v1_3.set_depth_bounds_test_enable.is_none() {
                        out.v1_3.set_depth_bounds_test_enable =
                            to_option(loader(c"vkCmdSetDepthBoundsTestEnableEXT"));
                    }
                    if out.v1_3.set_stencil_test_enable.is_none() {
                        out.v1_3.set_stencil_test_enable =
                            to_option(loader(c"vkCmdSetStencilTestEnableEXT"));
                    }
                    if out.v1_3.set_stencil_op.is_none() {
                        out.v1_3.set_stencil_op = to_option(loader(c"vkCmdSetStencilOpEXT"));
                    }
                    if out.ext_shader_object.set_vertex_input_ext.is_none() {
                        out.ext_shader_object.set_vertex_input_ext =
                            to_option(loader(c"vkCmdSetVertexInputEXT"));
                    }
                    if out.ext_shader_object.set_patch_control_points_ext.is_none() {
                        out.ext_shader_object.set_patch_control_points_ext =
                            to_option(loader(c"vkCmdSetPatchControlPointsEXT"));
                    }
                    if out.v1_3.set_rasterizer_discard_enable.is_none() {
                        out.v1_3.set_rasterizer_discard_enable =
                            to_option(loader(c"vkCmdSetRasterizerDiscardEnableEXT"));
                    }
                    if out.v1_3.set_depth_bias_enable.is_none() {
                        out.v1_3.set_depth_bias_enable =
                            to_option(loader(c"vkCmdSetDepthBiasEnableEXT"));
                    }
                    if out.ext_shader_object.set_logic_op_ext.is_none() {
                        out.ext_shader_object.set_logic_op_ext =
                            to_option(loader(c"vkCmdSetLogicOpEXT"));
                    }
                    if out.v1_3.set_primitive_restart_enable.is_none() {
                        out.v1_3.set_primitive_restart_enable =
                            to_option(loader(c"vkCmdSetPrimitiveRestartEnableEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_tessellation_domain_origin_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_tessellation_domain_origin_ext =
                            to_option(loader(c"vkCmdSetTessellationDomainOriginEXT"));
                    }
                    if out.ext_shader_object.set_depth_clamp_enable_ext.is_none() {
                        out.ext_shader_object.set_depth_clamp_enable_ext =
                            to_option(loader(c"vkCmdSetDepthClampEnableEXT"));
                    }
                    if out.ext_shader_object.set_polygon_mode_ext.is_none() {
                        out.ext_shader_object.set_polygon_mode_ext =
                            to_option(loader(c"vkCmdSetPolygonModeEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_rasterization_samples_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_rasterization_samples_ext =
                            to_option(loader(c"vkCmdSetRasterizationSamplesEXT"));
                    }
                    if out.ext_shader_object.set_sample_mask_ext.is_none() {
                        out.ext_shader_object.set_sample_mask_ext =
                            to_option(loader(c"vkCmdSetSampleMaskEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_alpha_to_coverage_enable_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_alpha_to_coverage_enable_ext =
                            to_option(loader(c"vkCmdSetAlphaToCoverageEnableEXT"));
                    }
                    if out.ext_shader_object.set_alpha_to_one_enable_ext.is_none() {
                        out.ext_shader_object.set_alpha_to_one_enable_ext =
                            to_option(loader(c"vkCmdSetAlphaToOneEnableEXT"));
                    }
                    if out.ext_shader_object.set_logic_op_enable_ext.is_none() {
                        out.ext_shader_object.set_logic_op_enable_ext =
                            to_option(loader(c"vkCmdSetLogicOpEnableEXT"));
                    }
                    if out.ext_shader_object.set_color_blend_enable_ext.is_none() {
                        out.ext_shader_object.set_color_blend_enable_ext =
                            to_option(loader(c"vkCmdSetColorBlendEnableEXT"));
                    }
                    if out.ext_shader_object.set_color_blend_equation_ext.is_none() {
                        out.ext_shader_object.set_color_blend_equation_ext =
                            to_option(loader(c"vkCmdSetColorBlendEquationEXT"));
                    }
                    if out.ext_shader_object.set_color_write_mask_ext.is_none() {
                        out.ext_shader_object.set_color_write_mask_ext =
                            to_option(loader(c"vkCmdSetColorWriteMaskEXT"));
                    }
                    if out.ext_shader_object.set_rasterization_stream_ext.is_none() {
                        out.ext_shader_object.set_rasterization_stream_ext =
                            to_option(loader(c"vkCmdSetRasterizationStreamEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_conservative_rasterization_mode_ext
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_conservative_rasterization_mode_ext =
                            to_option(loader(c"vkCmdSetConservativeRasterizationModeEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_extra_primitive_overestimation_size_ext
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_extra_primitive_overestimation_size_ext =
                            to_option(loader(c"vkCmdSetExtraPrimitiveOverestimationSizeEXT"));
                    }
                    if out.ext_shader_object.set_depth_clip_enable_ext.is_none() {
                        out.ext_shader_object.set_depth_clip_enable_ext =
                            to_option(loader(c"vkCmdSetDepthClipEnableEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_sample_locations_enable_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_sample_locations_enable_ext =
                            to_option(loader(c"vkCmdSetSampleLocationsEnableEXT"));
                    }
                    if out.ext_shader_object.set_color_blend_advanced_ext.is_none() {
                        out.ext_shader_object.set_color_blend_advanced_ext =
                            to_option(loader(c"vkCmdSetColorBlendAdvancedEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_provoking_vertex_mode_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_provoking_vertex_mode_ext =
                            to_option(loader(c"vkCmdSetProvokingVertexModeEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_line_rasterization_mode_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_line_rasterization_mode_ext =
                            to_option(loader(c"vkCmdSetLineRasterizationModeEXT"));
                    }
                    if out.ext_shader_object.set_line_stipple_enable_ext.is_none() {
                        out.ext_shader_object.set_line_stipple_enable_ext =
                            to_option(loader(c"vkCmdSetLineStippleEnableEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_depth_clip_negative_one_to_one_ext
                        .is_none()
                    {
                        out.ext_shader_object.set_depth_clip_negative_one_to_one_ext =
                            to_option(loader(c"vkCmdSetDepthClipNegativeOneToOneEXT"));
                    }
                    if out
                        .ext_shader_object
                        .set_viewport_w_scaling_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_viewport_w_scaling_enable_nv =
                            to_option(loader(c"vkCmdSetViewportWScalingEnableNV"));
                    }
                    if out.ext_shader_object.set_viewport_swizzle_nv.is_none() {
                        out.ext_shader_object.set_viewport_swizzle_nv =
                            to_option(loader(c"vkCmdSetViewportSwizzleNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_to_color_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_to_color_enable_nv =
                            to_option(loader(c"vkCmdSetCoverageToColorEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_to_color_location_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_to_color_location_nv =
                            to_option(loader(c"vkCmdSetCoverageToColorLocationNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_modulation_mode_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_modulation_mode_nv =
                            to_option(loader(c"vkCmdSetCoverageModulationModeNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_modulation_table_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_coverage_modulation_table_enable_nv =
                            to_option(loader(c"vkCmdSetCoverageModulationTableEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_modulation_table_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_modulation_table_nv =
                            to_option(loader(c"vkCmdSetCoverageModulationTableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_shading_rate_image_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_shading_rate_image_enable_nv =
                            to_option(loader(c"vkCmdSetShadingRateImageEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_representative_fragment_test_enable_nv
                        .is_none()
                    {
                        out.ext_shader_object
                            .set_representative_fragment_test_enable_nv =
                            to_option(loader(c"vkCmdSetRepresentativeFragmentTestEnableNV"));
                    }
                    if out
                        .ext_shader_object
                        .set_coverage_reduction_mode_nv
                        .is_none()
                    {
                        out.ext_shader_object.set_coverage_reduction_mode_nv =
                            to_option(loader(c"vkCmdSetCoverageReductionModeNV"));
                    }
                    if out
                        .ext_depth_clamp_control
                        .set_depth_clamp_range_ext
                        .is_none()
                    {
                        out.ext_depth_clamp_control.set_depth_clamp_range_ext =
                            to_option(loader(c"vkCmdSetDepthClampRangeEXT"));
                    }
                }
                b"VK_NV_cooperative_vector" => {
                    out.nv_cooperative_vector
                        .convert_cooperative_vector_matrix_nv =
                        to_option(loader(c"vkCmdConvertCooperativeVectorMatrixNV"));
                }
                b"VK_ARM_data_graph" => {
                    out.arm_data_graph.dispatch_data_graph_arm =
                        to_option(loader(c"vkCmdDispatchDataGraphARM"));
                }
                b"VK_EXT_attachment_feedback_loop_dynamic_state" => {
                    out.ext_attachment_feedback_loop_dynamic_state
                        .set_attachment_feedback_loop_enable_ext =
                        to_option(loader(c"vkCmdSetAttachmentFeedbackLoopEnableEXT"));
                }
                b"VK_KHR_line_rasterization" => {
                    if out.v1_4.set_line_stipple.is_none() {
                        out.v1_4.set_line_stipple = to_option(loader(c"vkCmdSetLineStippleKHR"));
                    }
                }
                b"VK_KHR_maintenance6" => {
                    if out.v1_4.bind_descriptor_sets2.is_none() {
                        out.v1_4.bind_descriptor_sets2 =
                            to_option(loader(c"vkCmdBindDescriptorSets2KHR"));
                    }
                    if out.v1_4.push_constants2.is_none() {
                        out.v1_4.push_constants2 = to_option(loader(c"vkCmdPushConstants2KHR"));
                    }
                    if out.v1_4.push_descriptor_set2.is_none() {
                        out.v1_4.push_descriptor_set2 =
                            to_option(loader(c"vkCmdPushDescriptorSet2KHR"));
                    }
                    if out.v1_4.push_descriptor_set_with_template2.is_none() {
                        out.v1_4.push_descriptor_set_with_template2 =
                            to_option(loader(c"vkCmdPushDescriptorSetWithTemplate2KHR"));
                    }
                    out.khr_maintenance6.set_descriptor_buffer_offsets2_ext =
                        to_option(loader(c"vkCmdSetDescriptorBufferOffsets2EXT"));
                    out.khr_maintenance6
                        .bind_descriptor_buffer_embedded_samplers2_ext =
                        to_option(loader(c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT"));
                }
                b"VK_QCOM_tile_memory_heap" => {
                    out.qcom_tile_memory_heap.bind_tile_memory_qcom =
                        to_option(loader(c"vkCmdBindTileMemoryQCOM"));
                }
                b"VK_KHR_copy_memory_indirect" => {
                    out.khr_copy_memory_indirect.copy_memory_indirect_khr =
                        to_option(loader(c"vkCmdCopyMemoryIndirectKHR"));
                    out.khr_copy_memory_indirect
                        .copy_memory_to_image_indirect_khr =
                        to_option(loader(c"vkCmdCopyMemoryToImageIndirectKHR"));
                }
                b"VK_EXT_memory_decompression" => {
                    out.ext_memory_decompression.decompress_memory_ext =
                        to_option(loader(c"vkCmdDecompressMemoryEXT"));
                    out.ext_memory_decompression
                        .decompress_memory_indirect_count_ext =
                        to_option(loader(c"vkCmdDecompressMemoryIndirectCountEXT"));
                }
                b"VK_NV_cluster_acceleration_structure" => {
                    out.nv_cluster_acceleration_structure
                        .build_cluster_acceleration_structure_indirect_nv =
                        to_option(loader(c"vkCmdBuildClusterAccelerationStructureIndirectNV"));
                }
                b"VK_NV_partitioned_acceleration_structure" => {
                    out.nv_partitioned_acceleration_structure
                        .build_partitioned_acceleration_structures_nv =
                        to_option(loader(c"vkCmdBuildPartitionedAccelerationStructuresNV"));
                }
                b"VK_EXT_device_generated_commands" => {
                    out.ext_device_generated_commands
                        .preprocess_generated_commands_ext =
                        to_option(loader(c"vkCmdPreprocessGeneratedCommandsEXT"));
                    out.ext_device_generated_commands
                        .execute_generated_commands_ext =
                        to_option(loader(c"vkCmdExecuteGeneratedCommandsEXT"));
                }
                b"VK_EXT_depth_clamp_control" => {
                    if out
                        .ext_depth_clamp_control
                        .set_depth_clamp_range_ext
                        .is_none()
                    {
                        out.ext_depth_clamp_control.set_depth_clamp_range_ext =
                            to_option(loader(c"vkCmdSetDepthClampRangeEXT"));
                    }
                }
                b"VK_ARM_shader_instrumentation" => {
                    out.arm_shader_instrumentation
                        .begin_shader_instrumentation_arm =
                        to_option(loader(c"vkCmdBeginShaderInstrumentationARM"));
                    out.arm_shader_instrumentation
                        .end_shader_instrumentation_arm =
                        to_option(loader(c"vkCmdEndShaderInstrumentationARM"));
                }
                b"VK_EXT_fragment_density_map_offset" => {
                    if out.khr_maintenance10.end_rendering2_khr.is_none() {
                        out.khr_maintenance10.end_rendering2_khr =
                            to_option(loader(c"vkCmdEndRendering2EXT"));
                    }
                }
                b"VK_EXT_custom_resolve" => {
                    out.ext_custom_resolve.begin_custom_resolve_ext =
                        to_option(loader(c"vkCmdBeginCustomResolveEXT"));
                }
                b"VK_KHR_maintenance10" => {
                    if out.khr_maintenance10.end_rendering2_khr.is_none() {
                        out.khr_maintenance10.end_rendering2_khr =
                            to_option(loader(c"vkCmdEndRendering2KHR"));
                    }
                }
                b"VK_NV_compute_occupancy_priority" => {
                    out.nv_compute_occupancy_priority
                        .set_compute_occupancy_priority_nv =
                        to_option(loader(c"vkCmdSetComputeOccupancyPriorityNV"));
                }
                b"VK_EXT_primitive_restart_index" => {
                    out.ext_primitive_restart_index
                        .set_primitive_restart_index_ext =
                        to_option(loader(c"vkCmdSetPrimitiveRestartIndexEXT"));
                }
                _ => (),
            }
        }
        out
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_0 {
    pub begin_command_buffer: Option<vkBeginCommandBuffer>,
    pub end_command_buffer: Option<vkEndCommandBuffer>,
    pub reset_command_buffer: Option<vkResetCommandBuffer>,
    pub bind_pipeline: Option<vkCmdBindPipeline>,
    pub set_viewport: Option<vkCmdSetViewport>,
    pub set_scissor: Option<vkCmdSetScissor>,
    pub set_line_width: Option<vkCmdSetLineWidth>,
    pub set_depth_bias: Option<vkCmdSetDepthBias>,
    pub set_blend_constants: Option<vkCmdSetBlendConstants>,
    pub set_depth_bounds: Option<vkCmdSetDepthBounds>,
    pub set_stencil_compare_mask: Option<vkCmdSetStencilCompareMask>,
    pub set_stencil_write_mask: Option<vkCmdSetStencilWriteMask>,
    pub set_stencil_reference: Option<vkCmdSetStencilReference>,
    pub bind_descriptor_sets: Option<vkCmdBindDescriptorSets>,
    pub bind_index_buffer: Option<vkCmdBindIndexBuffer>,
    pub bind_vertex_buffers: Option<vkCmdBindVertexBuffers>,
    pub draw: Option<vkCmdDraw>,
    pub draw_indexed: Option<vkCmdDrawIndexed>,
    pub draw_indirect: Option<vkCmdDrawIndirect>,
    pub draw_indexed_indirect: Option<vkCmdDrawIndexedIndirect>,
    pub dispatch: Option<vkCmdDispatch>,
    pub dispatch_indirect: Option<vkCmdDispatchIndirect>,
    pub copy_buffer: Option<vkCmdCopyBuffer>,
    pub copy_image: Option<vkCmdCopyImage>,
    pub blit_image: Option<vkCmdBlitImage>,
    pub copy_buffer_to_image: Option<vkCmdCopyBufferToImage>,
    pub copy_image_to_buffer: Option<vkCmdCopyImageToBuffer>,
    pub update_buffer: Option<vkCmdUpdateBuffer>,
    pub fill_buffer: Option<vkCmdFillBuffer>,
    pub clear_color_image: Option<vkCmdClearColorImage>,
    pub clear_depth_stencil_image: Option<vkCmdClearDepthStencilImage>,
    pub clear_attachments: Option<vkCmdClearAttachments>,
    pub resolve_image: Option<vkCmdResolveImage>,
    pub set_event: Option<vkCmdSetEvent>,
    pub reset_event: Option<vkCmdResetEvent>,
    pub wait_events: Option<vkCmdWaitEvents>,
    pub pipeline_barrier: Option<vkCmdPipelineBarrier>,
    pub begin_query: Option<vkCmdBeginQuery>,
    pub end_query: Option<vkCmdEndQuery>,
    pub reset_query_pool: Option<vkCmdResetQueryPool>,
    pub write_timestamp: Option<vkCmdWriteTimestamp>,
    pub copy_query_pool_results: Option<vkCmdCopyQueryPoolResults>,
    pub push_constants: Option<vkCmdPushConstants>,
    pub begin_render_pass: Option<vkCmdBeginRenderPass>,
    pub next_subpass: Option<vkCmdNextSubpass>,
    pub end_render_pass: Option<vkCmdEndRenderPass>,
    pub execute_commands: Option<vkCmdExecuteCommands>,
}

impl CommandBufferFnv1_0 {
    pub const EMPTY: Self = Self {
        begin_command_buffer: None,
        end_command_buffer: None,
        reset_command_buffer: None,
        bind_pipeline: None,
        set_viewport: None,
        set_scissor: None,
        set_line_width: None,
        set_depth_bias: None,
        set_blend_constants: None,
        set_depth_bounds: None,
        set_stencil_compare_mask: None,
        set_stencil_write_mask: None,
        set_stencil_reference: None,
        bind_descriptor_sets: None,
        bind_index_buffer: None,
        bind_vertex_buffers: None,
        draw: None,
        draw_indexed: None,
        draw_indirect: None,
        draw_indexed_indirect: None,
        dispatch: None,
        dispatch_indirect: None,
        copy_buffer: None,
        copy_image: None,
        blit_image: None,
        copy_buffer_to_image: None,
        copy_image_to_buffer: None,
        update_buffer: None,
        fill_buffer: None,
        clear_color_image: None,
        clear_depth_stencil_image: None,
        clear_attachments: None,
        resolve_image: None,
        set_event: None,
        reset_event: None,
        wait_events: None,
        pipeline_barrier: None,
        begin_query: None,
        end_query: None,
        reset_query_pool: None,
        write_timestamp: None,
        copy_query_pool_results: None,
        push_constants: None,
        begin_render_pass: None,
        next_subpass: None,
        end_render_pass: None,
        execute_commands: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_command_buffer: to_option(loader(c"vkBeginCommandBuffer")),
            end_command_buffer: to_option(loader(c"vkEndCommandBuffer")),
            reset_command_buffer: to_option(loader(c"vkResetCommandBuffer")),
            bind_pipeline: to_option(loader(c"vkCmdBindPipeline")),
            set_viewport: to_option(loader(c"vkCmdSetViewport")),
            set_scissor: to_option(loader(c"vkCmdSetScissor")),
            set_line_width: to_option(loader(c"vkCmdSetLineWidth")),
            set_depth_bias: to_option(loader(c"vkCmdSetDepthBias")),
            set_blend_constants: to_option(loader(c"vkCmdSetBlendConstants")),
            set_depth_bounds: to_option(loader(c"vkCmdSetDepthBounds")),
            set_stencil_compare_mask: to_option(loader(c"vkCmdSetStencilCompareMask")),
            set_stencil_write_mask: to_option(loader(c"vkCmdSetStencilWriteMask")),
            set_stencil_reference: to_option(loader(c"vkCmdSetStencilReference")),
            bind_descriptor_sets: to_option(loader(c"vkCmdBindDescriptorSets")),
            bind_index_buffer: to_option(loader(c"vkCmdBindIndexBuffer")),
            bind_vertex_buffers: to_option(loader(c"vkCmdBindVertexBuffers")),
            draw: to_option(loader(c"vkCmdDraw")),
            draw_indexed: to_option(loader(c"vkCmdDrawIndexed")),
            draw_indirect: to_option(loader(c"vkCmdDrawIndirect")),
            draw_indexed_indirect: to_option(loader(c"vkCmdDrawIndexedIndirect")),
            dispatch: to_option(loader(c"vkCmdDispatch")),
            dispatch_indirect: to_option(loader(c"vkCmdDispatchIndirect")),
            copy_buffer: to_option(loader(c"vkCmdCopyBuffer")),
            copy_image: to_option(loader(c"vkCmdCopyImage")),
            blit_image: to_option(loader(c"vkCmdBlitImage")),
            copy_buffer_to_image: to_option(loader(c"vkCmdCopyBufferToImage")),
            copy_image_to_buffer: to_option(loader(c"vkCmdCopyImageToBuffer")),
            update_buffer: to_option(loader(c"vkCmdUpdateBuffer")),
            fill_buffer: to_option(loader(c"vkCmdFillBuffer")),
            clear_color_image: to_option(loader(c"vkCmdClearColorImage")),
            clear_depth_stencil_image: to_option(loader(c"vkCmdClearDepthStencilImage")),
            clear_attachments: to_option(loader(c"vkCmdClearAttachments")),
            resolve_image: to_option(loader(c"vkCmdResolveImage")),
            set_event: to_option(loader(c"vkCmdSetEvent")),
            reset_event: to_option(loader(c"vkCmdResetEvent")),
            wait_events: to_option(loader(c"vkCmdWaitEvents")),
            pipeline_barrier: to_option(loader(c"vkCmdPipelineBarrier")),
            begin_query: to_option(loader(c"vkCmdBeginQuery")),
            end_query: to_option(loader(c"vkCmdEndQuery")),
            reset_query_pool: to_option(loader(c"vkCmdResetQueryPool")),
            write_timestamp: to_option(loader(c"vkCmdWriteTimestamp")),
            copy_query_pool_results: to_option(loader(c"vkCmdCopyQueryPoolResults")),
            push_constants: to_option(loader(c"vkCmdPushConstants")),
            begin_render_pass: to_option(loader(c"vkCmdBeginRenderPass")),
            next_subpass: to_option(loader(c"vkCmdNextSubpass")),
            end_render_pass: to_option(loader(c"vkCmdEndRenderPass")),
            execute_commands: to_option(loader(c"vkCmdExecuteCommands")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_1 {
    pub set_device_mask: Option<vkCmdSetDeviceMask>,
    pub dispatch_base: Option<vkCmdDispatchBase>,
}

impl CommandBufferFnv1_1 {
    pub const EMPTY: Self = Self {
        set_device_mask: None,
        dispatch_base: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_device_mask: to_option(loader(c"vkCmdSetDeviceMask")),
            dispatch_base: to_option(loader(c"vkCmdDispatchBase")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_2 {
    pub begin_render_pass2: Option<vkCmdBeginRenderPass2>,
    pub next_subpass2: Option<vkCmdNextSubpass2>,
    pub end_render_pass2: Option<vkCmdEndRenderPass2>,
    pub draw_indirect_count: Option<vkCmdDrawIndirectCount>,
    pub draw_indexed_indirect_count: Option<vkCmdDrawIndexedIndirectCount>,
}

impl CommandBufferFnv1_2 {
    pub const EMPTY: Self = Self {
        begin_render_pass2: None,
        next_subpass2: None,
        end_render_pass2: None,
        draw_indirect_count: None,
        draw_indexed_indirect_count: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_render_pass2: to_option(loader(c"vkCmdBeginRenderPass2")),
            next_subpass2: to_option(loader(c"vkCmdNextSubpass2")),
            end_render_pass2: to_option(loader(c"vkCmdEndRenderPass2")),
            draw_indirect_count: to_option(loader(c"vkCmdDrawIndirectCount")),
            draw_indexed_indirect_count: to_option(loader(c"vkCmdDrawIndexedIndirectCount")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_3 {
    pub set_cull_mode: Option<vkCmdSetCullMode>,
    pub set_front_face: Option<vkCmdSetFrontFace>,
    pub set_primitive_topology: Option<vkCmdSetPrimitiveTopology>,
    pub set_viewport_with_count: Option<vkCmdSetViewportWithCount>,
    pub set_scissor_with_count: Option<vkCmdSetScissorWithCount>,
    pub bind_vertex_buffers2: Option<vkCmdBindVertexBuffers2>,
    pub set_depth_test_enable: Option<vkCmdSetDepthTestEnable>,
    pub set_depth_write_enable: Option<vkCmdSetDepthWriteEnable>,
    pub set_depth_compare_op: Option<vkCmdSetDepthCompareOp>,
    pub set_depth_bounds_test_enable: Option<vkCmdSetDepthBoundsTestEnable>,
    pub set_stencil_test_enable: Option<vkCmdSetStencilTestEnable>,
    pub set_stencil_op: Option<vkCmdSetStencilOp>,
    pub set_rasterizer_discard_enable: Option<vkCmdSetRasterizerDiscardEnable>,
    pub set_depth_bias_enable: Option<vkCmdSetDepthBiasEnable>,
    pub set_primitive_restart_enable: Option<vkCmdSetPrimitiveRestartEnable>,
    pub copy_buffer2: Option<vkCmdCopyBuffer2>,
    pub copy_image2: Option<vkCmdCopyImage2>,
    pub blit_image2: Option<vkCmdBlitImage2>,
    pub copy_buffer_to_image2: Option<vkCmdCopyBufferToImage2>,
    pub copy_image_to_buffer2: Option<vkCmdCopyImageToBuffer2>,
    pub resolve_image2: Option<vkCmdResolveImage2>,
    pub set_event2: Option<vkCmdSetEvent2>,
    pub reset_event2: Option<vkCmdResetEvent2>,
    pub wait_events2: Option<vkCmdWaitEvents2>,
    pub pipeline_barrier2: Option<vkCmdPipelineBarrier2>,
    pub write_timestamp2: Option<vkCmdWriteTimestamp2>,
    pub begin_rendering: Option<vkCmdBeginRendering>,
    pub end_rendering: Option<vkCmdEndRendering>,
}

impl CommandBufferFnv1_3 {
    pub const EMPTY: Self = Self {
        set_cull_mode: None,
        set_front_face: None,
        set_primitive_topology: None,
        set_viewport_with_count: None,
        set_scissor_with_count: None,
        bind_vertex_buffers2: None,
        set_depth_test_enable: None,
        set_depth_write_enable: None,
        set_depth_compare_op: None,
        set_depth_bounds_test_enable: None,
        set_stencil_test_enable: None,
        set_stencil_op: None,
        set_rasterizer_discard_enable: None,
        set_depth_bias_enable: None,
        set_primitive_restart_enable: None,
        copy_buffer2: None,
        copy_image2: None,
        blit_image2: None,
        copy_buffer_to_image2: None,
        copy_image_to_buffer2: None,
        resolve_image2: None,
        set_event2: None,
        reset_event2: None,
        wait_events2: None,
        pipeline_barrier2: None,
        write_timestamp2: None,
        begin_rendering: None,
        end_rendering: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_cull_mode: to_option(loader(c"vkCmdSetCullMode")),
            set_front_face: to_option(loader(c"vkCmdSetFrontFace")),
            set_primitive_topology: to_option(loader(c"vkCmdSetPrimitiveTopology")),
            set_viewport_with_count: to_option(loader(c"vkCmdSetViewportWithCount")),
            set_scissor_with_count: to_option(loader(c"vkCmdSetScissorWithCount")),
            bind_vertex_buffers2: to_option(loader(c"vkCmdBindVertexBuffers2")),
            set_depth_test_enable: to_option(loader(c"vkCmdSetDepthTestEnable")),
            set_depth_write_enable: to_option(loader(c"vkCmdSetDepthWriteEnable")),
            set_depth_compare_op: to_option(loader(c"vkCmdSetDepthCompareOp")),
            set_depth_bounds_test_enable: to_option(loader(c"vkCmdSetDepthBoundsTestEnable")),
            set_stencil_test_enable: to_option(loader(c"vkCmdSetStencilTestEnable")),
            set_stencil_op: to_option(loader(c"vkCmdSetStencilOp")),
            set_rasterizer_discard_enable: to_option(loader(c"vkCmdSetRasterizerDiscardEnable")),
            set_depth_bias_enable: to_option(loader(c"vkCmdSetDepthBiasEnable")),
            set_primitive_restart_enable: to_option(loader(c"vkCmdSetPrimitiveRestartEnable")),
            copy_buffer2: to_option(loader(c"vkCmdCopyBuffer2")),
            copy_image2: to_option(loader(c"vkCmdCopyImage2")),
            blit_image2: to_option(loader(c"vkCmdBlitImage2")),
            copy_buffer_to_image2: to_option(loader(c"vkCmdCopyBufferToImage2")),
            copy_image_to_buffer2: to_option(loader(c"vkCmdCopyImageToBuffer2")),
            resolve_image2: to_option(loader(c"vkCmdResolveImage2")),
            set_event2: to_option(loader(c"vkCmdSetEvent2")),
            reset_event2: to_option(loader(c"vkCmdResetEvent2")),
            wait_events2: to_option(loader(c"vkCmdWaitEvents2")),
            pipeline_barrier2: to_option(loader(c"vkCmdPipelineBarrier2")),
            write_timestamp2: to_option(loader(c"vkCmdWriteTimestamp2")),
            begin_rendering: to_option(loader(c"vkCmdBeginRendering")),
            end_rendering: to_option(loader(c"vkCmdEndRendering")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_4 {
    pub push_descriptor_set: Option<vkCmdPushDescriptorSet>,
    pub push_descriptor_set_with_template: Option<vkCmdPushDescriptorSetWithTemplate>,
    pub set_line_stipple: Option<vkCmdSetLineStipple>,
    pub bind_index_buffer2: Option<vkCmdBindIndexBuffer2>,
    pub bind_descriptor_sets2: Option<vkCmdBindDescriptorSets2>,
    pub push_constants2: Option<vkCmdPushConstants2>,
    pub push_descriptor_set2: Option<vkCmdPushDescriptorSet2>,
    pub push_descriptor_set_with_template2: Option<vkCmdPushDescriptorSetWithTemplate2>,
    pub set_rendering_attachment_locations: Option<vkCmdSetRenderingAttachmentLocations>,
    pub set_rendering_input_attachment_indices: Option<vkCmdSetRenderingInputAttachmentIndices>,
}

impl CommandBufferFnv1_4 {
    pub const EMPTY: Self = Self {
        push_descriptor_set: None,
        push_descriptor_set_with_template: None,
        set_line_stipple: None,
        bind_index_buffer2: None,
        bind_descriptor_sets2: None,
        push_constants2: None,
        push_descriptor_set2: None,
        push_descriptor_set_with_template2: None,
        set_rendering_attachment_locations: None,
        set_rendering_input_attachment_indices: None,
    };

    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            push_descriptor_set: to_option(loader(c"vkCmdPushDescriptorSet")),
            push_descriptor_set_with_template: to_option(loader(
                c"vkCmdPushDescriptorSetWithTemplate",
            )),
            set_line_stipple: to_option(loader(c"vkCmdSetLineStipple")),
            bind_index_buffer2: to_option(loader(c"vkCmdBindIndexBuffer2")),
            bind_descriptor_sets2: to_option(loader(c"vkCmdBindDescriptorSets2")),
            push_constants2: to_option(loader(c"vkCmdPushConstants2")),
            push_descriptor_set2: to_option(loader(c"vkCmdPushDescriptorSet2")),
            push_descriptor_set_with_template2: to_option(loader(
                c"vkCmdPushDescriptorSetWithTemplate2",
            )),
            set_rendering_attachment_locations: to_option(loader(
                c"vkCmdSetRenderingAttachmentLocations",
            )),
            set_rendering_input_attachment_indices: to_option(loader(
                c"vkCmdSetRenderingInputAttachmentIndices",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnAmdBufferMarker {
    pub write_buffer_marker_amd: Option<vkCmdWriteBufferMarkerAMD>,
    pub write_buffer_marker2_amd: Option<vkCmdWriteBufferMarker2AMD>,
}

impl CommandBufferFnAmdBufferMarker {
    pub const EMPTY: Self = Self {
        write_buffer_marker_amd: None,
        write_buffer_marker2_amd: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnAmdGpaInterface {
    pub begin_gpa_session_amd: Option<vkCmdBeginGpaSessionAMD>,
    pub end_gpa_session_amd: Option<vkCmdEndGpaSessionAMD>,
    pub begin_gpa_sample_amd: Option<vkCmdBeginGpaSampleAMD>,
    pub end_gpa_sample_amd: Option<vkCmdEndGpaSampleAMD>,
    pub copy_gpa_session_results_amd: Option<vkCmdCopyGpaSessionResultsAMD>,
}

impl CommandBufferFnAmdGpaInterface {
    pub const EMPTY: Self = Self {
        begin_gpa_session_amd: None,
        end_gpa_session_amd: None,
        begin_gpa_sample_amd: None,
        end_gpa_sample_amd: None,
        copy_gpa_session_results_amd: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnAmdxShaderEnqueue {
    pub initialize_graph_scratch_memory_amdx: Option<vkCmdInitializeGraphScratchMemoryAMDX>,
    pub dispatch_graph_amdx: Option<vkCmdDispatchGraphAMDX>,
    pub dispatch_graph_indirect_amdx: Option<vkCmdDispatchGraphIndirectAMDX>,
    pub dispatch_graph_indirect_count_amdx: Option<vkCmdDispatchGraphIndirectCountAMDX>,
}

impl CommandBufferFnAmdxShaderEnqueue {
    pub const EMPTY: Self = Self {
        initialize_graph_scratch_memory_amdx: None,
        dispatch_graph_amdx: None,
        dispatch_graph_indirect_amdx: None,
        dispatch_graph_indirect_count_amdx: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnArmDataGraph {
    pub dispatch_data_graph_arm: Option<vkCmdDispatchDataGraphARM>,
}

impl CommandBufferFnArmDataGraph {
    pub const EMPTY: Self = Self {
        dispatch_data_graph_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnArmSchedulingControls {
    pub set_dispatch_parameters_arm: Option<vkCmdSetDispatchParametersARM>,
}

impl CommandBufferFnArmSchedulingControls {
    pub const EMPTY: Self = Self {
        set_dispatch_parameters_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnArmShaderInstrumentation {
    pub begin_shader_instrumentation_arm: Option<vkCmdBeginShaderInstrumentationARM>,
    pub end_shader_instrumentation_arm: Option<vkCmdEndShaderInstrumentationARM>,
}

impl CommandBufferFnArmShaderInstrumentation {
    pub const EMPTY: Self = Self {
        begin_shader_instrumentation_arm: None,
        end_shader_instrumentation_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnArmTensors {
    pub copy_tensor_arm: Option<vkCmdCopyTensorARM>,
}

impl CommandBufferFnArmTensors {
    pub const EMPTY: Self = Self {
        copy_tensor_arm: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtAttachmentFeedbackLoopDynamicState {
    pub set_attachment_feedback_loop_enable_ext: Option<vkCmdSetAttachmentFeedbackLoopEnableEXT>,
}

impl CommandBufferFnExtAttachmentFeedbackLoopDynamicState {
    pub const EMPTY: Self = Self {
        set_attachment_feedback_loop_enable_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtColorWriteEnable {
    pub set_color_write_enable_ext: Option<vkCmdSetColorWriteEnableEXT>,
}

impl CommandBufferFnExtColorWriteEnable {
    pub const EMPTY: Self = Self {
        set_color_write_enable_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtConditionalRendering {
    pub begin_conditional_rendering_ext: Option<vkCmdBeginConditionalRenderingEXT>,
    pub end_conditional_rendering_ext: Option<vkCmdEndConditionalRenderingEXT>,
}

impl CommandBufferFnExtConditionalRendering {
    pub const EMPTY: Self = Self {
        begin_conditional_rendering_ext: None,
        end_conditional_rendering_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtCustomResolve {
    pub begin_custom_resolve_ext: Option<vkCmdBeginCustomResolveEXT>,
}

impl CommandBufferFnExtCustomResolve {
    pub const EMPTY: Self = Self {
        begin_custom_resolve_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDebugMarker {
    pub debug_marker_begin_ext: Option<vkCmdDebugMarkerBeginEXT>,
    pub debug_marker_end_ext: Option<vkCmdDebugMarkerEndEXT>,
    pub debug_marker_insert_ext: Option<vkCmdDebugMarkerInsertEXT>,
}

impl CommandBufferFnExtDebugMarker {
    pub const EMPTY: Self = Self {
        debug_marker_begin_ext: None,
        debug_marker_end_ext: None,
        debug_marker_insert_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDebugUtils {
    pub begin_debug_utils_label_ext: Option<vkCmdBeginDebugUtilsLabelEXT>,
    pub end_debug_utils_label_ext: Option<vkCmdEndDebugUtilsLabelEXT>,
    pub insert_debug_utils_label_ext: Option<vkCmdInsertDebugUtilsLabelEXT>,
}

impl CommandBufferFnExtDebugUtils {
    pub const EMPTY: Self = Self {
        begin_debug_utils_label_ext: None,
        end_debug_utils_label_ext: None,
        insert_debug_utils_label_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDepthBiasControl {
    pub set_depth_bias2_ext: Option<vkCmdSetDepthBias2EXT>,
}

impl CommandBufferFnExtDepthBiasControl {
    pub const EMPTY: Self = Self {
        set_depth_bias2_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDepthClampControl {
    pub set_depth_clamp_range_ext: Option<vkCmdSetDepthClampRangeEXT>,
}

impl CommandBufferFnExtDepthClampControl {
    pub const EMPTY: Self = Self {
        set_depth_clamp_range_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDescriptorBuffer {
    pub bind_descriptor_buffers_ext: Option<vkCmdBindDescriptorBuffersEXT>,
    pub set_descriptor_buffer_offsets_ext: Option<vkCmdSetDescriptorBufferOffsetsEXT>,
    pub bind_descriptor_buffer_embedded_samplers_ext:
        Option<vkCmdBindDescriptorBufferEmbeddedSamplersEXT>,
}

impl CommandBufferFnExtDescriptorBuffer {
    pub const EMPTY: Self = Self {
        bind_descriptor_buffers_ext: None,
        set_descriptor_buffer_offsets_ext: None,
        bind_descriptor_buffer_embedded_samplers_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDescriptorHeap {
    pub bind_sampler_heap_ext: Option<vkCmdBindSamplerHeapEXT>,
    pub bind_resource_heap_ext: Option<vkCmdBindResourceHeapEXT>,
    pub push_data_ext: Option<vkCmdPushDataEXT>,
}

impl CommandBufferFnExtDescriptorHeap {
    pub const EMPTY: Self = Self {
        bind_sampler_heap_ext: None,
        bind_resource_heap_ext: None,
        push_data_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDeviceGeneratedCommands {
    pub execute_generated_commands_ext: Option<vkCmdExecuteGeneratedCommandsEXT>,
    pub preprocess_generated_commands_ext: Option<vkCmdPreprocessGeneratedCommandsEXT>,
}

impl CommandBufferFnExtDeviceGeneratedCommands {
    pub const EMPTY: Self = Self {
        execute_generated_commands_ext: None,
        preprocess_generated_commands_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtDiscardRectangles {
    pub set_discard_rectangle_ext: Option<vkCmdSetDiscardRectangleEXT>,
    pub set_discard_rectangle_enable_ext: Option<vkCmdSetDiscardRectangleEnableEXT>,
    pub set_discard_rectangle_mode_ext: Option<vkCmdSetDiscardRectangleModeEXT>,
}

impl CommandBufferFnExtDiscardRectangles {
    pub const EMPTY: Self = Self {
        set_discard_rectangle_ext: None,
        set_discard_rectangle_enable_ext: None,
        set_discard_rectangle_mode_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtMemoryDecompression {
    pub decompress_memory_ext: Option<vkCmdDecompressMemoryEXT>,
    pub decompress_memory_indirect_count_ext: Option<vkCmdDecompressMemoryIndirectCountEXT>,
}

impl CommandBufferFnExtMemoryDecompression {
    pub const EMPTY: Self = Self {
        decompress_memory_ext: None,
        decompress_memory_indirect_count_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtMeshShader {
    pub draw_mesh_tasks_ext: Option<vkCmdDrawMeshTasksEXT>,
    pub draw_mesh_tasks_indirect_ext: Option<vkCmdDrawMeshTasksIndirectEXT>,
    pub draw_mesh_tasks_indirect_count_ext: Option<vkCmdDrawMeshTasksIndirectCountEXT>,
}

impl CommandBufferFnExtMeshShader {
    pub const EMPTY: Self = Self {
        draw_mesh_tasks_ext: None,
        draw_mesh_tasks_indirect_ext: None,
        draw_mesh_tasks_indirect_count_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtMultiDraw {
    pub draw_multi_ext: Option<vkCmdDrawMultiEXT>,
    pub draw_multi_indexed_ext: Option<vkCmdDrawMultiIndexedEXT>,
}

impl CommandBufferFnExtMultiDraw {
    pub const EMPTY: Self = Self {
        draw_multi_ext: None,
        draw_multi_indexed_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtOpacityMicromap {
    pub build_micromaps_ext: Option<vkCmdBuildMicromapsEXT>,
    pub copy_micromap_ext: Option<vkCmdCopyMicromapEXT>,
    pub copy_micromap_to_memory_ext: Option<vkCmdCopyMicromapToMemoryEXT>,
    pub copy_memory_to_micromap_ext: Option<vkCmdCopyMemoryToMicromapEXT>,
    pub write_micromaps_properties_ext: Option<vkCmdWriteMicromapsPropertiesEXT>,
}

impl CommandBufferFnExtOpacityMicromap {
    pub const EMPTY: Self = Self {
        build_micromaps_ext: None,
        copy_micromap_ext: None,
        copy_micromap_to_memory_ext: None,
        copy_memory_to_micromap_ext: None,
        write_micromaps_properties_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtPrimitiveRestartIndex {
    pub set_primitive_restart_index_ext: Option<vkCmdSetPrimitiveRestartIndexEXT>,
}

impl CommandBufferFnExtPrimitiveRestartIndex {
    pub const EMPTY: Self = Self {
        set_primitive_restart_index_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtSampleLocations {
    pub set_sample_locations_ext: Option<vkCmdSetSampleLocationsEXT>,
}

impl CommandBufferFnExtSampleLocations {
    pub const EMPTY: Self = Self {
        set_sample_locations_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtShaderObject {
    pub set_patch_control_points_ext: Option<vkCmdSetPatchControlPointsEXT>,
    pub set_logic_op_ext: Option<vkCmdSetLogicOpEXT>,
    pub set_tessellation_domain_origin_ext: Option<vkCmdSetTessellationDomainOriginEXT>,
    pub set_depth_clamp_enable_ext: Option<vkCmdSetDepthClampEnableEXT>,
    pub set_polygon_mode_ext: Option<vkCmdSetPolygonModeEXT>,
    pub set_rasterization_samples_ext: Option<vkCmdSetRasterizationSamplesEXT>,
    pub set_sample_mask_ext: Option<vkCmdSetSampleMaskEXT>,
    pub set_alpha_to_coverage_enable_ext: Option<vkCmdSetAlphaToCoverageEnableEXT>,
    pub set_alpha_to_one_enable_ext: Option<vkCmdSetAlphaToOneEnableEXT>,
    pub set_logic_op_enable_ext: Option<vkCmdSetLogicOpEnableEXT>,
    pub set_color_blend_enable_ext: Option<vkCmdSetColorBlendEnableEXT>,
    pub set_color_blend_equation_ext: Option<vkCmdSetColorBlendEquationEXT>,
    pub set_color_write_mask_ext: Option<vkCmdSetColorWriteMaskEXT>,
    pub set_rasterization_stream_ext: Option<vkCmdSetRasterizationStreamEXT>,
    pub set_conservative_rasterization_mode_ext: Option<vkCmdSetConservativeRasterizationModeEXT>,
    pub set_extra_primitive_overestimation_size_ext:
        Option<vkCmdSetExtraPrimitiveOverestimationSizeEXT>,
    pub set_depth_clip_enable_ext: Option<vkCmdSetDepthClipEnableEXT>,
    pub set_sample_locations_enable_ext: Option<vkCmdSetSampleLocationsEnableEXT>,
    pub set_color_blend_advanced_ext: Option<vkCmdSetColorBlendAdvancedEXT>,
    pub set_provoking_vertex_mode_ext: Option<vkCmdSetProvokingVertexModeEXT>,
    pub set_line_rasterization_mode_ext: Option<vkCmdSetLineRasterizationModeEXT>,
    pub set_line_stipple_enable_ext: Option<vkCmdSetLineStippleEnableEXT>,
    pub set_depth_clip_negative_one_to_one_ext: Option<vkCmdSetDepthClipNegativeOneToOneEXT>,
    pub set_viewport_w_scaling_enable_nv: Option<vkCmdSetViewportWScalingEnableNV>,
    pub set_viewport_swizzle_nv: Option<vkCmdSetViewportSwizzleNV>,
    pub set_coverage_to_color_enable_nv: Option<vkCmdSetCoverageToColorEnableNV>,
    pub set_coverage_to_color_location_nv: Option<vkCmdSetCoverageToColorLocationNV>,
    pub set_coverage_modulation_mode_nv: Option<vkCmdSetCoverageModulationModeNV>,
    pub set_coverage_modulation_table_enable_nv: Option<vkCmdSetCoverageModulationTableEnableNV>,
    pub set_coverage_modulation_table_nv: Option<vkCmdSetCoverageModulationTableNV>,
    pub set_shading_rate_image_enable_nv: Option<vkCmdSetShadingRateImageEnableNV>,
    pub set_coverage_reduction_mode_nv: Option<vkCmdSetCoverageReductionModeNV>,
    pub set_representative_fragment_test_enable_nv:
        Option<vkCmdSetRepresentativeFragmentTestEnableNV>,
    pub set_vertex_input_ext: Option<vkCmdSetVertexInputEXT>,
    pub bind_shaders_ext: Option<vkCmdBindShadersEXT>,
}

impl CommandBufferFnExtShaderObject {
    pub const EMPTY: Self = Self {
        set_patch_control_points_ext: None,
        set_logic_op_ext: None,
        set_tessellation_domain_origin_ext: None,
        set_depth_clamp_enable_ext: None,
        set_polygon_mode_ext: None,
        set_rasterization_samples_ext: None,
        set_sample_mask_ext: None,
        set_alpha_to_coverage_enable_ext: None,
        set_alpha_to_one_enable_ext: None,
        set_logic_op_enable_ext: None,
        set_color_blend_enable_ext: None,
        set_color_blend_equation_ext: None,
        set_color_write_mask_ext: None,
        set_rasterization_stream_ext: None,
        set_conservative_rasterization_mode_ext: None,
        set_extra_primitive_overestimation_size_ext: None,
        set_depth_clip_enable_ext: None,
        set_sample_locations_enable_ext: None,
        set_color_blend_advanced_ext: None,
        set_provoking_vertex_mode_ext: None,
        set_line_rasterization_mode_ext: None,
        set_line_stipple_enable_ext: None,
        set_depth_clip_negative_one_to_one_ext: None,
        set_viewport_w_scaling_enable_nv: None,
        set_viewport_swizzle_nv: None,
        set_coverage_to_color_enable_nv: None,
        set_coverage_to_color_location_nv: None,
        set_coverage_modulation_mode_nv: None,
        set_coverage_modulation_table_enable_nv: None,
        set_coverage_modulation_table_nv: None,
        set_shading_rate_image_enable_nv: None,
        set_coverage_reduction_mode_nv: None,
        set_representative_fragment_test_enable_nv: None,
        set_vertex_input_ext: None,
        bind_shaders_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnExtTransformFeedback {
    pub bind_transform_feedback_buffers_ext: Option<vkCmdBindTransformFeedbackBuffersEXT>,
    pub begin_transform_feedback_ext: Option<vkCmdBeginTransformFeedbackEXT>,
    pub end_transform_feedback_ext: Option<vkCmdEndTransformFeedbackEXT>,
    pub begin_query_indexed_ext: Option<vkCmdBeginQueryIndexedEXT>,
    pub end_query_indexed_ext: Option<vkCmdEndQueryIndexedEXT>,
    pub draw_indirect_byte_count_ext: Option<vkCmdDrawIndirectByteCountEXT>,
}

impl CommandBufferFnExtTransformFeedback {
    pub const EMPTY: Self = Self {
        bind_transform_feedback_buffers_ext: None,
        begin_transform_feedback_ext: None,
        end_transform_feedback_ext: None,
        begin_query_indexed_ext: None,
        end_query_indexed_ext: None,
        draw_indirect_byte_count_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnHuaweiClusterCullingShader {
    pub draw_cluster_huawei: Option<vkCmdDrawClusterHUAWEI>,
    pub draw_cluster_indirect_huawei: Option<vkCmdDrawClusterIndirectHUAWEI>,
}

impl CommandBufferFnHuaweiClusterCullingShader {
    pub const EMPTY: Self = Self {
        draw_cluster_huawei: None,
        draw_cluster_indirect_huawei: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnHuaweiInvocationMask {
    pub bind_invocation_mask_huawei: Option<vkCmdBindInvocationMaskHUAWEI>,
}

impl CommandBufferFnHuaweiInvocationMask {
    pub const EMPTY: Self = Self {
        bind_invocation_mask_huawei: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnHuaweiSubpassShading {
    pub subpass_shading_huawei: Option<vkCmdSubpassShadingHUAWEI>,
}

impl CommandBufferFnHuaweiSubpassShading {
    pub const EMPTY: Self = Self {
        subpass_shading_huawei: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnIntelPerformanceQuery {
    pub set_performance_marker_intel: Option<vkCmdSetPerformanceMarkerINTEL>,
    pub set_performance_stream_marker_intel: Option<vkCmdSetPerformanceStreamMarkerINTEL>,
    pub set_performance_override_intel: Option<vkCmdSetPerformanceOverrideINTEL>,
}

impl CommandBufferFnIntelPerformanceQuery {
    pub const EMPTY: Self = Self {
        set_performance_marker_intel: None,
        set_performance_stream_marker_intel: None,
        set_performance_override_intel: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrAccelerationStructure {
    pub copy_acceleration_structure_khr: Option<vkCmdCopyAccelerationStructureKHR>,
    pub copy_acceleration_structure_to_memory_khr:
        Option<vkCmdCopyAccelerationStructureToMemoryKHR>,
    pub copy_memory_to_acceleration_structure_khr:
        Option<vkCmdCopyMemoryToAccelerationStructureKHR>,
    pub write_acceleration_structures_properties_khr:
        Option<vkCmdWriteAccelerationStructuresPropertiesKHR>,
    pub build_acceleration_structures_khr: Option<vkCmdBuildAccelerationStructuresKHR>,
    pub build_acceleration_structures_indirect_khr:
        Option<vkCmdBuildAccelerationStructuresIndirectKHR>,
}

impl CommandBufferFnKhrAccelerationStructure {
    pub const EMPTY: Self = Self {
        copy_acceleration_structure_khr: None,
        copy_acceleration_structure_to_memory_khr: None,
        copy_memory_to_acceleration_structure_khr: None,
        write_acceleration_structures_properties_khr: None,
        build_acceleration_structures_khr: None,
        build_acceleration_structures_indirect_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrCopyMemoryIndirect {
    pub copy_memory_indirect_khr: Option<vkCmdCopyMemoryIndirectKHR>,
    pub copy_memory_to_image_indirect_khr: Option<vkCmdCopyMemoryToImageIndirectKHR>,
}

impl CommandBufferFnKhrCopyMemoryIndirect {
    pub const EMPTY: Self = Self {
        copy_memory_indirect_khr: None,
        copy_memory_to_image_indirect_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrDeviceAddressCommands {
    pub copy_memory_khr: Option<vkCmdCopyMemoryKHR>,
    pub copy_memory_to_image_khr: Option<vkCmdCopyMemoryToImageKHR>,
    pub copy_image_to_memory_khr: Option<vkCmdCopyImageToMemoryKHR>,
    pub update_memory_khr: Option<vkCmdUpdateMemoryKHR>,
    pub fill_memory_khr: Option<vkCmdFillMemoryKHR>,
    pub copy_query_pool_results_to_memory_khr: Option<vkCmdCopyQueryPoolResultsToMemoryKHR>,
    pub begin_conditional_rendering2_ext: Option<vkCmdBeginConditionalRendering2EXT>,
    pub bind_transform_feedback_buffers2_ext: Option<vkCmdBindTransformFeedbackBuffers2EXT>,
    pub begin_transform_feedback2_ext: Option<vkCmdBeginTransformFeedback2EXT>,
    pub end_transform_feedback2_ext: Option<vkCmdEndTransformFeedback2EXT>,
    pub draw_indirect_byte_count2_ext: Option<vkCmdDrawIndirectByteCount2EXT>,
    pub write_marker_to_memory_amd: Option<vkCmdWriteMarkerToMemoryAMD>,
    pub bind_index_buffer3_khr: Option<vkCmdBindIndexBuffer3KHR>,
    pub bind_vertex_buffers3_khr: Option<vkCmdBindVertexBuffers3KHR>,
    pub draw_indirect2_khr: Option<vkCmdDrawIndirect2KHR>,
    pub draw_indexed_indirect2_khr: Option<vkCmdDrawIndexedIndirect2KHR>,
    pub draw_indirect_count2_khr: Option<vkCmdDrawIndirectCount2KHR>,
    pub draw_indexed_indirect_count2_khr: Option<vkCmdDrawIndexedIndirectCount2KHR>,
    pub draw_mesh_tasks_indirect2_ext: Option<vkCmdDrawMeshTasksIndirect2EXT>,
    pub draw_mesh_tasks_indirect_count2_ext: Option<vkCmdDrawMeshTasksIndirectCount2EXT>,
    pub dispatch_indirect2_khr: Option<vkCmdDispatchIndirect2KHR>,
}

impl CommandBufferFnKhrDeviceAddressCommands {
    pub const EMPTY: Self = Self {
        copy_memory_khr: None,
        copy_memory_to_image_khr: None,
        copy_image_to_memory_khr: None,
        update_memory_khr: None,
        fill_memory_khr: None,
        copy_query_pool_results_to_memory_khr: None,
        begin_conditional_rendering2_ext: None,
        bind_transform_feedback_buffers2_ext: None,
        begin_transform_feedback2_ext: None,
        end_transform_feedback2_ext: None,
        draw_indirect_byte_count2_ext: None,
        write_marker_to_memory_amd: None,
        bind_index_buffer3_khr: None,
        bind_vertex_buffers3_khr: None,
        draw_indirect2_khr: None,
        draw_indexed_indirect2_khr: None,
        draw_indirect_count2_khr: None,
        draw_indexed_indirect_count2_khr: None,
        draw_mesh_tasks_indirect2_ext: None,
        draw_mesh_tasks_indirect_count2_ext: None,
        dispatch_indirect2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrFragmentShadingRate {
    pub set_fragment_shading_rate_khr: Option<vkCmdSetFragmentShadingRateKHR>,
}

impl CommandBufferFnKhrFragmentShadingRate {
    pub const EMPTY: Self = Self {
        set_fragment_shading_rate_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrMaintenance10 {
    pub end_rendering2_khr: Option<vkCmdEndRendering2KHR>,
}

impl CommandBufferFnKhrMaintenance10 {
    pub const EMPTY: Self = Self {
        end_rendering2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrMaintenance6 {
    pub set_descriptor_buffer_offsets2_ext: Option<vkCmdSetDescriptorBufferOffsets2EXT>,
    pub bind_descriptor_buffer_embedded_samplers2_ext:
        Option<vkCmdBindDescriptorBufferEmbeddedSamplers2EXT>,
}

impl CommandBufferFnKhrMaintenance6 {
    pub const EMPTY: Self = Self {
        set_descriptor_buffer_offsets2_ext: None,
        bind_descriptor_buffer_embedded_samplers2_ext: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrObjectRefresh {
    pub refresh_objects_khr: Option<vkCmdRefreshObjectsKHR>,
}

impl CommandBufferFnKhrObjectRefresh {
    pub const EMPTY: Self = Self {
        refresh_objects_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrRayTracingMaintenance1 {
    pub trace_rays_indirect2_khr: Option<vkCmdTraceRaysIndirect2KHR>,
}

impl CommandBufferFnKhrRayTracingMaintenance1 {
    pub const EMPTY: Self = Self {
        trace_rays_indirect2_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrRayTracingPipeline {
    pub trace_rays_khr: Option<vkCmdTraceRaysKHR>,
    pub trace_rays_indirect_khr: Option<vkCmdTraceRaysIndirectKHR>,
    pub set_ray_tracing_pipeline_stack_size_khr: Option<vkCmdSetRayTracingPipelineStackSizeKHR>,
}

impl CommandBufferFnKhrRayTracingPipeline {
    pub const EMPTY: Self = Self {
        trace_rays_khr: None,
        trace_rays_indirect_khr: None,
        set_ray_tracing_pipeline_stack_size_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrVideoDecodeQueue {
    pub decode_video_khr: Option<vkCmdDecodeVideoKHR>,
}

impl CommandBufferFnKhrVideoDecodeQueue {
    pub const EMPTY: Self = Self {
        decode_video_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrVideoEncodeQueue {
    pub encode_video_khr: Option<vkCmdEncodeVideoKHR>,
}

impl CommandBufferFnKhrVideoEncodeQueue {
    pub const EMPTY: Self = Self {
        encode_video_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnKhrVideoQueue {
    pub begin_video_coding_khr: Option<vkCmdBeginVideoCodingKHR>,
    pub control_video_coding_khr: Option<vkCmdControlVideoCodingKHR>,
    pub end_video_coding_khr: Option<vkCmdEndVideoCodingKHR>,
}

impl CommandBufferFnKhrVideoQueue {
    pub const EMPTY: Self = Self {
        begin_video_coding_khr: None,
        control_video_coding_khr: None,
        end_video_coding_khr: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvClipSpaceWScaling {
    pub set_viewport_w_scaling_nv: Option<vkCmdSetViewportWScalingNV>,
}

impl CommandBufferFnNvClipSpaceWScaling {
    pub const EMPTY: Self = Self {
        set_viewport_w_scaling_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvClusterAccelerationStructure {
    pub build_cluster_acceleration_structure_indirect_nv:
        Option<vkCmdBuildClusterAccelerationStructureIndirectNV>,
}

impl CommandBufferFnNvClusterAccelerationStructure {
    pub const EMPTY: Self = Self {
        build_cluster_acceleration_structure_indirect_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvComputeOccupancyPriority {
    pub set_compute_occupancy_priority_nv: Option<vkCmdSetComputeOccupancyPriorityNV>,
}

impl CommandBufferFnNvComputeOccupancyPriority {
    pub const EMPTY: Self = Self {
        set_compute_occupancy_priority_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvCooperativeVector {
    pub convert_cooperative_vector_matrix_nv: Option<vkCmdConvertCooperativeVectorMatrixNV>,
}

impl CommandBufferFnNvCooperativeVector {
    pub const EMPTY: Self = Self {
        convert_cooperative_vector_matrix_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvCopyMemoryIndirect {
    pub copy_memory_indirect_nv: Option<vkCmdCopyMemoryIndirectNV>,
    pub copy_memory_to_image_indirect_nv: Option<vkCmdCopyMemoryToImageIndirectNV>,
}

impl CommandBufferFnNvCopyMemoryIndirect {
    pub const EMPTY: Self = Self {
        copy_memory_indirect_nv: None,
        copy_memory_to_image_indirect_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvCudaKernelLaunch {
    pub cuda_launch_kernel_nv: Option<vkCmdCudaLaunchKernelNV>,
}

impl CommandBufferFnNvCudaKernelLaunch {
    pub const EMPTY: Self = Self {
        cuda_launch_kernel_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvDeviceDiagnosticCheckpoints {
    pub set_checkpoint_nv: Option<vkCmdSetCheckpointNV>,
}

impl CommandBufferFnNvDeviceDiagnosticCheckpoints {
    pub const EMPTY: Self = Self {
        set_checkpoint_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvDeviceGeneratedCommands {
    pub execute_generated_commands_nv: Option<vkCmdExecuteGeneratedCommandsNV>,
    pub preprocess_generated_commands_nv: Option<vkCmdPreprocessGeneratedCommandsNV>,
    pub bind_pipeline_shader_group_nv: Option<vkCmdBindPipelineShaderGroupNV>,
}

impl CommandBufferFnNvDeviceGeneratedCommands {
    pub const EMPTY: Self = Self {
        execute_generated_commands_nv: None,
        preprocess_generated_commands_nv: None,
        bind_pipeline_shader_group_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvDeviceGeneratedCommandsCompute {
    pub update_pipeline_indirect_buffer_nv: Option<vkCmdUpdatePipelineIndirectBufferNV>,
}

impl CommandBufferFnNvDeviceGeneratedCommandsCompute {
    pub const EMPTY: Self = Self {
        update_pipeline_indirect_buffer_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvFragmentShadingRateEnums {
    pub set_fragment_shading_rate_enum_nv: Option<vkCmdSetFragmentShadingRateEnumNV>,
}

impl CommandBufferFnNvFragmentShadingRateEnums {
    pub const EMPTY: Self = Self {
        set_fragment_shading_rate_enum_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvMemoryDecompression {
    pub decompress_memory_nv: Option<vkCmdDecompressMemoryNV>,
    pub decompress_memory_indirect_count_nv: Option<vkCmdDecompressMemoryIndirectCountNV>,
}

impl CommandBufferFnNvMemoryDecompression {
    pub const EMPTY: Self = Self {
        decompress_memory_nv: None,
        decompress_memory_indirect_count_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvMeshShader {
    pub draw_mesh_tasks_nv: Option<vkCmdDrawMeshTasksNV>,
    pub draw_mesh_tasks_indirect_nv: Option<vkCmdDrawMeshTasksIndirectNV>,
    pub draw_mesh_tasks_indirect_count_nv: Option<vkCmdDrawMeshTasksIndirectCountNV>,
}

impl CommandBufferFnNvMeshShader {
    pub const EMPTY: Self = Self {
        draw_mesh_tasks_nv: None,
        draw_mesh_tasks_indirect_nv: None,
        draw_mesh_tasks_indirect_count_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvOpticalFlow {
    pub optical_flow_execute_nv: Option<vkCmdOpticalFlowExecuteNV>,
}

impl CommandBufferFnNvOpticalFlow {
    pub const EMPTY: Self = Self {
        optical_flow_execute_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvPartitionedAccelerationStructure {
    pub build_partitioned_acceleration_structures_nv:
        Option<vkCmdBuildPartitionedAccelerationStructuresNV>,
}

impl CommandBufferFnNvPartitionedAccelerationStructure {
    pub const EMPTY: Self = Self {
        build_partitioned_acceleration_structures_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvRayTracing {
    pub copy_acceleration_structure_nv: Option<vkCmdCopyAccelerationStructureNV>,
    pub write_acceleration_structures_properties_nv:
        Option<vkCmdWriteAccelerationStructuresPropertiesNV>,
    pub build_acceleration_structure_nv: Option<vkCmdBuildAccelerationStructureNV>,
    pub trace_rays_nv: Option<vkCmdTraceRaysNV>,
}

impl CommandBufferFnNvRayTracing {
    pub const EMPTY: Self = Self {
        copy_acceleration_structure_nv: None,
        write_acceleration_structures_properties_nv: None,
        build_acceleration_structure_nv: None,
        trace_rays_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvScissorExclusive {
    pub set_exclusive_scissor_nv: Option<vkCmdSetExclusiveScissorNV>,
    pub set_exclusive_scissor_enable_nv: Option<vkCmdSetExclusiveScissorEnableNV>,
}

impl CommandBufferFnNvScissorExclusive {
    pub const EMPTY: Self = Self {
        set_exclusive_scissor_nv: None,
        set_exclusive_scissor_enable_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvShadingRateImage {
    pub bind_shading_rate_image_nv: Option<vkCmdBindShadingRateImageNV>,
    pub set_viewport_shading_rate_palette_nv: Option<vkCmdSetViewportShadingRatePaletteNV>,
    pub set_coarse_sample_order_nv: Option<vkCmdSetCoarseSampleOrderNV>,
}

impl CommandBufferFnNvShadingRateImage {
    pub const EMPTY: Self = Self {
        bind_shading_rate_image_nv: None,
        set_viewport_shading_rate_palette_nv: None,
        set_coarse_sample_order_nv: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnNvxBinaryImport {
    pub cu_launch_kernel_nvx: Option<vkCmdCuLaunchKernelNVX>,
}

impl CommandBufferFnNvxBinaryImport {
    pub const EMPTY: Self = Self {
        cu_launch_kernel_nvx: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnQcomTileMemoryHeap {
    pub bind_tile_memory_qcom: Option<vkCmdBindTileMemoryQCOM>,
}

impl CommandBufferFnQcomTileMemoryHeap {
    pub const EMPTY: Self = Self {
        bind_tile_memory_qcom: None,
    };
}

#[derive(Clone, Default)]
pub struct CommandBufferFnQcomTileShading {
    pub dispatch_tile_qcom: Option<vkCmdDispatchTileQCOM>,
    pub begin_per_tile_execution_qcom: Option<vkCmdBeginPerTileExecutionQCOM>,
    pub end_per_tile_execution_qcom: Option<vkCmdEndPerTileExecutionQCOM>,
}

impl CommandBufferFnQcomTileShading {
    pub const EMPTY: Self = Self {
        dispatch_tile_qcom: None,
        begin_per_tile_execution_qcom: None,
        end_per_tile_execution_qcom: None,
    };
}

#[derive(Clone)]
pub struct InstanceVTable {
    pub instance: InstanceFn,
    pub physical_device: PhysicalDeviceFn,
}

impl InstanceVTable {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        debug_assert!(api_version >= API_VERSION_1_0);
        Self {
            instance: InstanceFn::load(&mut loader, api_version, extensions),
            physical_device: PhysicalDeviceFn::load(&mut loader, api_version, extensions),
        }
    }
}

#[derive(Clone)]
pub struct DeviceVTable {
    pub device: DeviceFn,
    pub queue: QueueFn,
    pub command_buffer: CommandBufferFn,
}

impl DeviceVTable {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const c_char],
    ) -> Self {
        debug_assert!(api_version >= API_VERSION_1_0);
        Self {
            device: DeviceFn::load(&mut loader, api_version, extensions),
            queue: QueueFn::load(&mut loader, api_version, extensions),
            command_buffer: CommandBufferFn::load(&mut loader, api_version, extensions),
        }
    }
}
