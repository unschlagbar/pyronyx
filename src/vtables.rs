// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!
// Auto generated from pyronyx-gen — vk/commands.rs
// Do not Edit! Execute `cargo run pyronyx-gen`
// !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!

use super::vk::*;
use core::ffi::{CStr, c_void};
use core::mem::transmute_copy;
pub fn to_option<T: Copy>(val: *const c_void) -> Option<T> {
    if val.is_null() {
        None
    } else {
        Some(unsafe { transmute_copy(&val) })
    }
}

fn to_panic<T: Copy>(val: *const c_void) -> T {
    if val.is_null() {
        panic!("Not supported fn!");
    } else {
        unsafe { transmute_copy(&val) }
    }
}
#[derive(Clone)]
pub struct InstanceFn {
    pub v1_0: InstanceFnv1_0,
    pub v1_1: InstanceFnv1_1,
    pub ext_debug_report: Option<InstanceFnExtDebugReport>,
    pub ext_debug_utils: Option<InstanceFnExtDebugUtils>,
    pub ext_directfb_surface: Option<InstanceFnExtDirectfbSurface>,
    pub ext_headless_surface: Option<InstanceFnExtHeadlessSurface>,
    pub ext_metal_surface: Option<InstanceFnExtMetalSurface>,
    pub fuchsia_imagepipe_surface: Option<InstanceFnFuchsiaImagepipeSurface>,
    pub ggp_stream_descriptor_surface: Option<InstanceFnGgpStreamDescriptorSurface>,
    pub khr_android_surface: Option<InstanceFnKhrAndroidSurface>,
    pub khr_display: Option<InstanceFnKhrDisplay>,
    pub khr_surface: Option<InstanceFnKhrSurface>,
    pub khr_wayland_surface: Option<InstanceFnKhrWaylandSurface>,
    pub khr_win32_surface: Option<InstanceFnKhrWin32Surface>,
    pub khr_xcb_surface: Option<InstanceFnKhrXcbSurface>,
    pub khr_xlib_surface: Option<InstanceFnKhrXlibSurface>,
    pub mvk_ios_surface: Option<InstanceFnMvkIosSurface>,
    pub mvk_macos_surface: Option<InstanceFnMvkMacosSurface>,
    pub nn_vi_surface: Option<InstanceFnNnViSurface>,
    pub ohos_surface: Option<InstanceFnOhosSurface>,
    pub qnx_screen_surface: Option<InstanceFnQnxScreenSurface>,
}

impl InstanceFn {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const i8],
    ) -> Self {
        let mut out = Self {
            v1_0: InstanceFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                InstanceFnv1_1::load(&mut loader)
            } else {
                InstanceFnv1_1::default()
            },
            ext_debug_report: None,
            ext_debug_utils: None,
            ext_directfb_surface: None,
            ext_headless_surface: None,
            ext_metal_surface: None,
            fuchsia_imagepipe_surface: None,
            ggp_stream_descriptor_surface: None,
            khr_android_surface: None,
            khr_display: None,
            khr_surface: None,
            khr_wayland_surface: None,
            khr_win32_surface: None,
            khr_xcb_surface: None,
            khr_xlib_surface: None,
            mvk_ios_surface: None,
            mvk_macos_surface: None,
            nn_vi_surface: None,
            ohos_surface: None,
            qnx_screen_surface: None,
        };
        for &ext in extensions {
            let ext = unsafe { CStr::from_ptr(ext).to_bytes() };
            match ext {
                b"VK_EXT_debug_report" => {
                    out.ext_debug_report = Some(InstanceFnExtDebugReport::load(&mut loader))
                }
                b"VK_EXT_debug_utils" => {
                    out.ext_debug_utils = Some(InstanceFnExtDebugUtils::load(&mut loader))
                }
                b"VK_EXT_directfb_surface" => {
                    out.ext_directfb_surface = Some(InstanceFnExtDirectfbSurface::load(&mut loader))
                }
                b"VK_EXT_headless_surface" => {
                    out.ext_headless_surface = Some(InstanceFnExtHeadlessSurface::load(&mut loader))
                }
                b"VK_EXT_metal_surface" => {
                    out.ext_metal_surface = Some(InstanceFnExtMetalSurface::load(&mut loader))
                }
                b"VK_FUCHSIA_imagepipe_surface" => {
                    out.fuchsia_imagepipe_surface =
                        Some(InstanceFnFuchsiaImagepipeSurface::load(&mut loader))
                }
                b"VK_GGP_stream_descriptor_surface" => {
                    out.ggp_stream_descriptor_surface =
                        Some(InstanceFnGgpStreamDescriptorSurface::load(&mut loader))
                }
                b"VK_KHR_android_surface" => {
                    out.khr_android_surface = Some(InstanceFnKhrAndroidSurface::load(&mut loader))
                }
                b"VK_KHR_display" => {
                    out.khr_display = Some(InstanceFnKhrDisplay::load(&mut loader))
                }
                b"VK_KHR_surface" => {
                    out.khr_surface = Some(InstanceFnKhrSurface::load(&mut loader))
                }
                b"VK_KHR_wayland_surface" => {
                    out.khr_wayland_surface = Some(InstanceFnKhrWaylandSurface::load(&mut loader))
                }
                b"VK_KHR_win32_surface" => {
                    out.khr_win32_surface = Some(InstanceFnKhrWin32Surface::load(&mut loader))
                }
                b"VK_KHR_xcb_surface" => {
                    out.khr_xcb_surface = Some(InstanceFnKhrXcbSurface::load(&mut loader))
                }
                b"VK_KHR_xlib_surface" => {
                    out.khr_xlib_surface = Some(InstanceFnKhrXlibSurface::load(&mut loader))
                }
                b"VK_MVK_ios_surface" => {
                    out.mvk_ios_surface = Some(InstanceFnMvkIosSurface::load(&mut loader))
                }
                b"VK_MVK_macos_surface" => {
                    out.mvk_macos_surface = Some(InstanceFnMvkMacosSurface::load(&mut loader))
                }
                b"VK_NN_vi_surface" => {
                    out.nn_vi_surface = Some(InstanceFnNnViSurface::load(&mut loader))
                }
                b"VK_OHOS_surface" => {
                    out.ohos_surface = Some(InstanceFnOhosSurface::load(&mut loader))
                }
                b"VK_QNX_screen_surface" => {
                    out.qnx_screen_surface = Some(InstanceFnQnxScreenSurface::load(&mut loader))
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
    pub enumerate_physical_device_groups_khr: Option<vkEnumeratePhysicalDeviceGroupsKHR>,
}

impl InstanceFnv1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            enumerate_physical_device_groups: to_option(loader(c"vkEnumeratePhysicalDeviceGroups")),
            enumerate_physical_device_groups_khr: to_option(loader(
                c"vkEnumeratePhysicalDeviceGroupsKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnExtDebugReport {
    pub create_debug_report_callback_ext: vkCreateDebugReportCallbackEXT,
    pub destroy_debug_report_callback_ext: vkDestroyDebugReportCallbackEXT,
    pub debug_report_message_ext: vkDebugReportMessageEXT,
}

impl InstanceFnExtDebugReport {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_debug_report_callback_ext: to_panic(loader(c"vkCreateDebugReportCallbackEXT")),
            destroy_debug_report_callback_ext: to_panic(loader(c"vkDestroyDebugReportCallbackEXT")),
            debug_report_message_ext: to_panic(loader(c"vkDebugReportMessageEXT")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnExtDebugUtils {
    pub create_debug_utils_messenger_ext: vkCreateDebugUtilsMessengerEXT,
    pub destroy_debug_utils_messenger_ext: vkDestroyDebugUtilsMessengerEXT,
    pub submit_debug_utils_message_ext: vkSubmitDebugUtilsMessageEXT,
}

impl InstanceFnExtDebugUtils {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_debug_utils_messenger_ext: to_panic(loader(c"vkCreateDebugUtilsMessengerEXT")),
            destroy_debug_utils_messenger_ext: to_panic(loader(c"vkDestroyDebugUtilsMessengerEXT")),
            submit_debug_utils_message_ext: to_panic(loader(c"vkSubmitDebugUtilsMessageEXT")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnExtDirectfbSurface {
    pub create_direct_fb_surface_ext: vkCreateDirectFBSurfaceEXT,
}

impl InstanceFnExtDirectfbSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_direct_fb_surface_ext: to_panic(loader(c"vkCreateDirectFBSurfaceEXT")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnExtHeadlessSurface {
    pub create_headless_surface_ext: vkCreateHeadlessSurfaceEXT,
}

impl InstanceFnExtHeadlessSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_headless_surface_ext: to_panic(loader(c"vkCreateHeadlessSurfaceEXT")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnExtMetalSurface {
    pub create_metal_surface_ext: vkCreateMetalSurfaceEXT,
}

impl InstanceFnExtMetalSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_metal_surface_ext: to_panic(loader(c"vkCreateMetalSurfaceEXT")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnFuchsiaImagepipeSurface {
    pub create_image_pipe_surface_fuchsia: vkCreateImagePipeSurfaceFUCHSIA,
}

impl InstanceFnFuchsiaImagepipeSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_image_pipe_surface_fuchsia: to_panic(loader(c"vkCreateImagePipeSurfaceFUCHSIA")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnGgpStreamDescriptorSurface {
    pub create_stream_descriptor_surface_ggp: vkCreateStreamDescriptorSurfaceGGP,
}

impl InstanceFnGgpStreamDescriptorSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_stream_descriptor_surface_ggp: to_panic(loader(
                c"vkCreateStreamDescriptorSurfaceGGP",
            )),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrAndroidSurface {
    pub create_android_surface_khr: vkCreateAndroidSurfaceKHR,
}

impl InstanceFnKhrAndroidSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_android_surface_khr: to_panic(loader(c"vkCreateAndroidSurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrDisplay {
    pub create_display_plane_surface_khr: vkCreateDisplayPlaneSurfaceKHR,
}

impl InstanceFnKhrDisplay {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_display_plane_surface_khr: to_panic(loader(c"vkCreateDisplayPlaneSurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrSurface {
    pub destroy_surface_khr: vkDestroySurfaceKHR,
}

impl InstanceFnKhrSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            destroy_surface_khr: to_panic(loader(c"vkDestroySurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrWaylandSurface {
    pub create_wayland_surface_khr: vkCreateWaylandSurfaceKHR,
}

impl InstanceFnKhrWaylandSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_wayland_surface_khr: to_panic(loader(c"vkCreateWaylandSurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrWin32Surface {
    pub create_win32_surface_khr: vkCreateWin32SurfaceKHR,
}

impl InstanceFnKhrWin32Surface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_win32_surface_khr: to_panic(loader(c"vkCreateWin32SurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrXcbSurface {
    pub create_xcb_surface_khr: vkCreateXcbSurfaceKHR,
}

impl InstanceFnKhrXcbSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_xcb_surface_khr: to_panic(loader(c"vkCreateXcbSurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnKhrXlibSurface {
    pub create_xlib_surface_khr: vkCreateXlibSurfaceKHR,
}

impl InstanceFnKhrXlibSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_xlib_surface_khr: to_panic(loader(c"vkCreateXlibSurfaceKHR")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnMvkIosSurface {
    pub create_ios_surface_mvk: vkCreateIOSSurfaceMVK,
}

impl InstanceFnMvkIosSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_ios_surface_mvk: to_panic(loader(c"vkCreateIOSSurfaceMVK")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnMvkMacosSurface {
    pub create_mac_os_surface_mvk: vkCreateMacOSSurfaceMVK,
}

impl InstanceFnMvkMacosSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_mac_os_surface_mvk: to_panic(loader(c"vkCreateMacOSSurfaceMVK")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnNnViSurface {
    pub create_vi_surface_nn: vkCreateViSurfaceNN,
}

impl InstanceFnNnViSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_vi_surface_nn: to_panic(loader(c"vkCreateViSurfaceNN")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnOhosSurface {
    pub create_surface_ohos: vkCreateSurfaceOHOS,
}

impl InstanceFnOhosSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_surface_ohos: to_panic(loader(c"vkCreateSurfaceOHOS")),
        }
    }
}

#[derive(Clone)]
pub struct InstanceFnQnxScreenSurface {
    pub create_screen_surface_qnx: vkCreateScreenSurfaceQNX,
}

impl InstanceFnQnxScreenSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_screen_surface_qnx: to_panic(loader(c"vkCreateScreenSurfaceQNX")),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFn {
    pub v1_0: PhysicalDeviceFnv1_0,
    pub v1_1: PhysicalDeviceFnv1_1,
    pub v1_3: PhysicalDeviceFnv1_3,
    pub arm_data_graph: Option<PhysicalDeviceFnArmDataGraph>,
    pub arm_performance_counters_by_region: Option<PhysicalDeviceFnArmPerformanceCountersByRegion>,
    pub arm_tensors: Option<PhysicalDeviceFnArmTensors>,
    pub ext_acquire_drm_display: Option<PhysicalDeviceFnExtAcquireDrmDisplay>,
    pub ext_acquire_xlib_display: Option<PhysicalDeviceFnExtAcquireXlibDisplay>,
    pub ext_descriptor_heap: Option<PhysicalDeviceFnExtDescriptorHeap>,
    pub ext_direct_mode_display: Option<PhysicalDeviceFnExtDirectModeDisplay>,
    pub ext_directfb_surface: Option<PhysicalDeviceFnExtDirectfbSurface>,
    pub ext_display_surface_counter: Option<PhysicalDeviceFnExtDisplaySurfaceCounter>,
    pub ext_full_screen_exclusive: Option<PhysicalDeviceFnExtFullScreenExclusive>,
    pub ext_sample_locations: Option<PhysicalDeviceFnExtSampleLocations>,
    pub khr_calibrated_timestamps: Option<PhysicalDeviceFnKhrCalibratedTimestamps>,
    pub khr_cooperative_matrix: Option<PhysicalDeviceFnKhrCooperativeMatrix>,
    pub khr_device_group: Option<PhysicalDeviceFnKhrDeviceGroup>,
    pub khr_display: Option<PhysicalDeviceFnKhrDisplay>,
    pub khr_fragment_shading_rate: Option<PhysicalDeviceFnKhrFragmentShadingRate>,
    pub khr_get_display_properties2: Option<PhysicalDeviceFnKhrGetDisplayProperties2>,
    pub khr_get_surface_capabilities2: Option<PhysicalDeviceFnKhrGetSurfaceCapabilities2>,
    pub khr_object_refresh: Option<PhysicalDeviceFnKhrObjectRefresh>,
    pub khr_performance_query: Option<PhysicalDeviceFnKhrPerformanceQuery>,
    pub khr_surface: Option<PhysicalDeviceFnKhrSurface>,
    pub khr_video_encode_queue: Option<PhysicalDeviceFnKhrVideoEncodeQueue>,
    pub khr_video_queue: Option<PhysicalDeviceFnKhrVideoQueue>,
    pub khr_wayland_surface: Option<PhysicalDeviceFnKhrWaylandSurface>,
    pub khr_win32_surface: Option<PhysicalDeviceFnKhrWin32Surface>,
    pub khr_xcb_surface: Option<PhysicalDeviceFnKhrXcbSurface>,
    pub khr_xlib_surface: Option<PhysicalDeviceFnKhrXlibSurface>,
    pub nv_acquire_winrt_display: Option<PhysicalDeviceFnNvAcquireWinrtDisplay>,
    pub nv_cooperative_matrix: Option<PhysicalDeviceFnNvCooperativeMatrix>,
    pub nv_cooperative_matrix2: Option<PhysicalDeviceFnNvCooperativeMatrix2>,
    pub nv_cooperative_vector: Option<PhysicalDeviceFnNvCooperativeVector>,
    pub nv_coverage_reduction_mode: Option<PhysicalDeviceFnNvCoverageReductionMode>,
    pub nv_external_memory_capabilities: Option<PhysicalDeviceFnNvExternalMemoryCapabilities>,
    pub nv_external_memory_sci_buf: Option<PhysicalDeviceFnNvExternalMemorySciBuf>,
    pub nv_external_sci_sync2: Option<PhysicalDeviceFnNvExternalSciSync2>,
    pub nv_optical_flow: Option<PhysicalDeviceFnNvOpticalFlow>,
    pub qnx_screen_surface: Option<PhysicalDeviceFnQnxScreenSurface>,
}

impl PhysicalDeviceFn {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const i8],
    ) -> Self {
        let mut out = Self {
            v1_0: PhysicalDeviceFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                PhysicalDeviceFnv1_1::load(&mut loader)
            } else {
                PhysicalDeviceFnv1_1::default()
            },
            v1_3: if api_version >= API_VERSION_1_3 {
                PhysicalDeviceFnv1_3::load(&mut loader)
            } else {
                PhysicalDeviceFnv1_3::default()
            },
            arm_data_graph: None,
            arm_performance_counters_by_region: None,
            arm_tensors: None,
            ext_acquire_drm_display: None,
            ext_acquire_xlib_display: None,
            ext_descriptor_heap: None,
            ext_direct_mode_display: None,
            ext_directfb_surface: None,
            ext_display_surface_counter: None,
            ext_full_screen_exclusive: None,
            ext_sample_locations: None,
            khr_calibrated_timestamps: None,
            khr_cooperative_matrix: None,
            khr_device_group: None,
            khr_display: None,
            khr_fragment_shading_rate: None,
            khr_get_display_properties2: None,
            khr_get_surface_capabilities2: None,
            khr_object_refresh: None,
            khr_performance_query: None,
            khr_surface: None,
            khr_video_encode_queue: None,
            khr_video_queue: None,
            khr_wayland_surface: None,
            khr_win32_surface: None,
            khr_xcb_surface: None,
            khr_xlib_surface: None,
            nv_acquire_winrt_display: None,
            nv_cooperative_matrix: None,
            nv_cooperative_matrix2: None,
            nv_cooperative_vector: None,
            nv_coverage_reduction_mode: None,
            nv_external_memory_capabilities: None,
            nv_external_memory_sci_buf: None,
            nv_external_sci_sync2: None,
            nv_optical_flow: None,
            qnx_screen_surface: None,
        };
        for &ext in extensions {
            let ext = unsafe { CStr::from_ptr(ext).to_bytes() };
            match ext {
                b"VK_ARM_data_graph" => {
                    out.arm_data_graph = Some(PhysicalDeviceFnArmDataGraph::load(&mut loader))
                }
                b"VK_ARM_performance_counters_by_region" => {
                    out.arm_performance_counters_by_region = Some(
                        PhysicalDeviceFnArmPerformanceCountersByRegion::load(&mut loader),
                    )
                }
                b"VK_ARM_tensors" => {
                    out.arm_tensors = Some(PhysicalDeviceFnArmTensors::load(&mut loader))
                }
                b"VK_EXT_acquire_drm_display" => {
                    out.ext_acquire_drm_display =
                        Some(PhysicalDeviceFnExtAcquireDrmDisplay::load(&mut loader))
                }
                b"VK_EXT_acquire_xlib_display" => {
                    out.ext_acquire_xlib_display =
                        Some(PhysicalDeviceFnExtAcquireXlibDisplay::load(&mut loader))
                }
                b"VK_EXT_descriptor_heap" => {
                    out.ext_descriptor_heap =
                        Some(PhysicalDeviceFnExtDescriptorHeap::load(&mut loader))
                }
                b"VK_EXT_direct_mode_display" => {
                    out.ext_direct_mode_display =
                        Some(PhysicalDeviceFnExtDirectModeDisplay::load(&mut loader))
                }
                b"VK_EXT_directfb_surface" => {
                    out.ext_directfb_surface =
                        Some(PhysicalDeviceFnExtDirectfbSurface::load(&mut loader))
                }
                b"VK_EXT_display_surface_counter" => {
                    out.ext_display_surface_counter =
                        Some(PhysicalDeviceFnExtDisplaySurfaceCounter::load(&mut loader))
                }
                b"VK_EXT_full_screen_exclusive" => {
                    out.ext_full_screen_exclusive =
                        Some(PhysicalDeviceFnExtFullScreenExclusive::load(&mut loader))
                }
                b"VK_EXT_sample_locations" => {
                    out.ext_sample_locations =
                        Some(PhysicalDeviceFnExtSampleLocations::load(&mut loader))
                }
                b"VK_KHR_calibrated_timestamps" => {
                    out.khr_calibrated_timestamps =
                        Some(PhysicalDeviceFnKhrCalibratedTimestamps::load(&mut loader))
                }
                b"VK_KHR_cooperative_matrix" => {
                    out.khr_cooperative_matrix =
                        Some(PhysicalDeviceFnKhrCooperativeMatrix::load(&mut loader))
                }
                b"VK_KHR_device_group" => {
                    out.khr_device_group = Some(PhysicalDeviceFnKhrDeviceGroup::load(&mut loader))
                }
                b"VK_KHR_display" => {
                    out.khr_display = Some(PhysicalDeviceFnKhrDisplay::load(&mut loader))
                }
                b"VK_KHR_fragment_shading_rate" => {
                    out.khr_fragment_shading_rate =
                        Some(PhysicalDeviceFnKhrFragmentShadingRate::load(&mut loader))
                }
                b"VK_KHR_get_display_properties2" => {
                    out.khr_get_display_properties2 =
                        Some(PhysicalDeviceFnKhrGetDisplayProperties2::load(&mut loader))
                }
                b"VK_KHR_get_surface_capabilities2" => {
                    out.khr_get_surface_capabilities2 = Some(
                        PhysicalDeviceFnKhrGetSurfaceCapabilities2::load(&mut loader),
                    )
                }
                b"VK_KHR_object_refresh" => {
                    out.khr_object_refresh =
                        Some(PhysicalDeviceFnKhrObjectRefresh::load(&mut loader))
                }
                b"VK_KHR_performance_query" => {
                    out.khr_performance_query =
                        Some(PhysicalDeviceFnKhrPerformanceQuery::load(&mut loader))
                }
                b"VK_KHR_surface" => {
                    out.khr_surface = Some(PhysicalDeviceFnKhrSurface::load(&mut loader))
                }
                b"VK_KHR_video_encode_queue" => {
                    out.khr_video_encode_queue =
                        Some(PhysicalDeviceFnKhrVideoEncodeQueue::load(&mut loader))
                }
                b"VK_KHR_video_queue" => {
                    out.khr_video_queue = Some(PhysicalDeviceFnKhrVideoQueue::load(&mut loader))
                }
                b"VK_KHR_wayland_surface" => {
                    out.khr_wayland_surface =
                        Some(PhysicalDeviceFnKhrWaylandSurface::load(&mut loader))
                }
                b"VK_KHR_win32_surface" => {
                    out.khr_win32_surface = Some(PhysicalDeviceFnKhrWin32Surface::load(&mut loader))
                }
                b"VK_KHR_xcb_surface" => {
                    out.khr_xcb_surface = Some(PhysicalDeviceFnKhrXcbSurface::load(&mut loader))
                }
                b"VK_KHR_xlib_surface" => {
                    out.khr_xlib_surface = Some(PhysicalDeviceFnKhrXlibSurface::load(&mut loader))
                }
                b"VK_NV_acquire_winrt_display" => {
                    out.nv_acquire_winrt_display =
                        Some(PhysicalDeviceFnNvAcquireWinrtDisplay::load(&mut loader))
                }
                b"VK_NV_cooperative_matrix" => {
                    out.nv_cooperative_matrix =
                        Some(PhysicalDeviceFnNvCooperativeMatrix::load(&mut loader))
                }
                b"VK_NV_cooperative_matrix2" => {
                    out.nv_cooperative_matrix2 =
                        Some(PhysicalDeviceFnNvCooperativeMatrix2::load(&mut loader))
                }
                b"VK_NV_cooperative_vector" => {
                    out.nv_cooperative_vector =
                        Some(PhysicalDeviceFnNvCooperativeVector::load(&mut loader))
                }
                b"VK_NV_coverage_reduction_mode" => {
                    out.nv_coverage_reduction_mode =
                        Some(PhysicalDeviceFnNvCoverageReductionMode::load(&mut loader))
                }
                b"VK_NV_external_memory_capabilities" => {
                    out.nv_external_memory_capabilities = Some(
                        PhysicalDeviceFnNvExternalMemoryCapabilities::load(&mut loader),
                    )
                }
                b"VK_NV_external_memory_sci_buf" => {
                    out.nv_external_memory_sci_buf =
                        Some(PhysicalDeviceFnNvExternalMemorySciBuf::load(&mut loader))
                }
                b"VK_NV_external_sci_sync2" => {
                    out.nv_external_sci_sync2 =
                        Some(PhysicalDeviceFnNvExternalSciSync2::load(&mut loader))
                }
                b"VK_NV_optical_flow" => {
                    out.nv_optical_flow = Some(PhysicalDeviceFnNvOpticalFlow::load(&mut loader))
                }
                b"VK_QNX_screen_surface" => {
                    out.qnx_screen_surface =
                        Some(PhysicalDeviceFnQnxScreenSurface::load(&mut loader))
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
    pub get_physical_device_features2_khr: Option<vkGetPhysicalDeviceFeatures2KHR>,
    pub get_physical_device_properties2: Option<vkGetPhysicalDeviceProperties2>,
    pub get_physical_device_properties2_khr: Option<vkGetPhysicalDeviceProperties2KHR>,
    pub get_physical_device_format_properties2: Option<vkGetPhysicalDeviceFormatProperties2>,
    pub get_physical_device_format_properties2_khr: Option<vkGetPhysicalDeviceFormatProperties2KHR>,
    pub get_physical_device_image_format_properties2:
        Option<vkGetPhysicalDeviceImageFormatProperties2>,
    pub get_physical_device_image_format_properties2_khr:
        Option<vkGetPhysicalDeviceImageFormatProperties2KHR>,
    pub get_physical_device_queue_family_properties2:
        Option<vkGetPhysicalDeviceQueueFamilyProperties2>,
    pub get_physical_device_queue_family_properties2_khr:
        Option<vkGetPhysicalDeviceQueueFamilyProperties2KHR>,
    pub get_physical_device_memory_properties2: Option<vkGetPhysicalDeviceMemoryProperties2>,
    pub get_physical_device_memory_properties2_khr: Option<vkGetPhysicalDeviceMemoryProperties2KHR>,
    pub get_physical_device_sparse_image_format_properties2:
        Option<vkGetPhysicalDeviceSparseImageFormatProperties2>,
    pub get_physical_device_sparse_image_format_properties2_khr:
        Option<vkGetPhysicalDeviceSparseImageFormatProperties2KHR>,
    pub get_physical_device_external_buffer_properties:
        Option<vkGetPhysicalDeviceExternalBufferProperties>,
    pub get_physical_device_external_buffer_properties_khr:
        Option<vkGetPhysicalDeviceExternalBufferPropertiesKHR>,
    pub get_physical_device_external_semaphore_properties:
        Option<vkGetPhysicalDeviceExternalSemaphoreProperties>,
    pub get_physical_device_external_semaphore_properties_khr:
        Option<vkGetPhysicalDeviceExternalSemaphorePropertiesKHR>,
    pub get_physical_device_external_fence_properties:
        Option<vkGetPhysicalDeviceExternalFenceProperties>,
    pub get_physical_device_external_fence_properties_khr:
        Option<vkGetPhysicalDeviceExternalFencePropertiesKHR>,
}

impl PhysicalDeviceFnv1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_features2: to_option(loader(c"vkGetPhysicalDeviceFeatures2")),
            get_physical_device_features2_khr: to_option(loader(
                c"vkGetPhysicalDeviceFeatures2KHR",
            )),
            get_physical_device_properties2: to_option(loader(c"vkGetPhysicalDeviceProperties2")),
            get_physical_device_properties2_khr: to_option(loader(
                c"vkGetPhysicalDeviceProperties2KHR",
            )),
            get_physical_device_format_properties2: to_option(loader(
                c"vkGetPhysicalDeviceFormatProperties2",
            )),
            get_physical_device_format_properties2_khr: to_option(loader(
                c"vkGetPhysicalDeviceFormatProperties2KHR",
            )),
            get_physical_device_image_format_properties2: to_option(loader(
                c"vkGetPhysicalDeviceImageFormatProperties2",
            )),
            get_physical_device_image_format_properties2_khr: to_option(loader(
                c"vkGetPhysicalDeviceImageFormatProperties2KHR",
            )),
            get_physical_device_queue_family_properties2: to_option(loader(
                c"vkGetPhysicalDeviceQueueFamilyProperties2",
            )),
            get_physical_device_queue_family_properties2_khr: to_option(loader(
                c"vkGetPhysicalDeviceQueueFamilyProperties2KHR",
            )),
            get_physical_device_memory_properties2: to_option(loader(
                c"vkGetPhysicalDeviceMemoryProperties2",
            )),
            get_physical_device_memory_properties2_khr: to_option(loader(
                c"vkGetPhysicalDeviceMemoryProperties2KHR",
            )),
            get_physical_device_sparse_image_format_properties2: to_option(loader(
                c"vkGetPhysicalDeviceSparseImageFormatProperties2",
            )),
            get_physical_device_sparse_image_format_properties2_khr: to_option(loader(
                c"vkGetPhysicalDeviceSparseImageFormatProperties2KHR",
            )),
            get_physical_device_external_buffer_properties: to_option(loader(
                c"vkGetPhysicalDeviceExternalBufferProperties",
            )),
            get_physical_device_external_buffer_properties_khr: to_option(loader(
                c"vkGetPhysicalDeviceExternalBufferPropertiesKHR",
            )),
            get_physical_device_external_semaphore_properties: to_option(loader(
                c"vkGetPhysicalDeviceExternalSemaphoreProperties",
            )),
            get_physical_device_external_semaphore_properties_khr: to_option(loader(
                c"vkGetPhysicalDeviceExternalSemaphorePropertiesKHR",
            )),
            get_physical_device_external_fence_properties: to_option(loader(
                c"vkGetPhysicalDeviceExternalFenceProperties",
            )),
            get_physical_device_external_fence_properties_khr: to_option(loader(
                c"vkGetPhysicalDeviceExternalFencePropertiesKHR",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct PhysicalDeviceFnv1_3 {
    pub get_physical_device_tool_properties: Option<vkGetPhysicalDeviceToolProperties>,
    pub get_physical_device_tool_properties_ext: Option<vkGetPhysicalDeviceToolPropertiesEXT>,
}

impl PhysicalDeviceFnv1_3 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_tool_properties: to_option(loader(
                c"vkGetPhysicalDeviceToolProperties",
            )),
            get_physical_device_tool_properties_ext: to_option(loader(
                c"vkGetPhysicalDeviceToolPropertiesEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnArmDataGraph {
    pub get_physical_device_queue_family_data_graph_properties_arm:
        vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM,
    pub get_physical_device_queue_family_data_graph_processing_engine_properties_arm:
        vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM,
}

impl PhysicalDeviceFnArmDataGraph {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_queue_family_data_graph_properties_arm: to_panic(loader(
                c"vkGetPhysicalDeviceQueueFamilyDataGraphPropertiesARM",
            )),
            get_physical_device_queue_family_data_graph_processing_engine_properties_arm: to_panic(
                loader(c"vkGetPhysicalDeviceQueueFamilyDataGraphProcessingEnginePropertiesARM"),
            ),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnArmPerformanceCountersByRegion {
    pub enumerate_physical_device_queue_family_performance_counters_by_region_arm:
        vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM,
}

impl PhysicalDeviceFnArmPerformanceCountersByRegion {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            enumerate_physical_device_queue_family_performance_counters_by_region_arm: to_panic(
                loader(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceCountersByRegionARM"),
            ),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnArmTensors {
    pub get_physical_device_external_tensor_properties_arm:
        vkGetPhysicalDeviceExternalTensorPropertiesARM,
}

impl PhysicalDeviceFnArmTensors {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_external_tensor_properties_arm: to_panic(loader(
                c"vkGetPhysicalDeviceExternalTensorPropertiesARM",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtAcquireDrmDisplay {
    pub acquire_drm_display_ext: vkAcquireDrmDisplayEXT,
    pub get_drm_display_ext: vkGetDrmDisplayEXT,
}

impl PhysicalDeviceFnExtAcquireDrmDisplay {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            acquire_drm_display_ext: to_panic(loader(c"vkAcquireDrmDisplayEXT")),
            get_drm_display_ext: to_panic(loader(c"vkGetDrmDisplayEXT")),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtAcquireXlibDisplay {
    pub acquire_xlib_display_ext: vkAcquireXlibDisplayEXT,
    pub get_rand_r_output_display_ext: vkGetRandROutputDisplayEXT,
}

impl PhysicalDeviceFnExtAcquireXlibDisplay {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            acquire_xlib_display_ext: to_panic(loader(c"vkAcquireXlibDisplayEXT")),
            get_rand_r_output_display_ext: to_panic(loader(c"vkGetRandROutputDisplayEXT")),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtDescriptorHeap {
    pub get_physical_device_descriptor_size_ext: vkGetPhysicalDeviceDescriptorSizeEXT,
}

impl PhysicalDeviceFnExtDescriptorHeap {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_descriptor_size_ext: to_panic(loader(
                c"vkGetPhysicalDeviceDescriptorSizeEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtDirectModeDisplay {
    pub release_display_ext: vkReleaseDisplayEXT,
}

impl PhysicalDeviceFnExtDirectModeDisplay {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            release_display_ext: to_panic(loader(c"vkReleaseDisplayEXT")),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtDirectfbSurface {
    pub get_physical_device_direct_fb_presentation_support_ext:
        vkGetPhysicalDeviceDirectFBPresentationSupportEXT,
}

impl PhysicalDeviceFnExtDirectfbSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_direct_fb_presentation_support_ext: to_panic(loader(
                c"vkGetPhysicalDeviceDirectFBPresentationSupportEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtDisplaySurfaceCounter {
    pub get_physical_device_surface_capabilities2_ext: vkGetPhysicalDeviceSurfaceCapabilities2EXT,
}

impl PhysicalDeviceFnExtDisplaySurfaceCounter {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_surface_capabilities2_ext: to_panic(loader(
                c"vkGetPhysicalDeviceSurfaceCapabilities2EXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtFullScreenExclusive {
    pub get_physical_device_surface_present_modes2_ext: vkGetPhysicalDeviceSurfacePresentModes2EXT,
}

impl PhysicalDeviceFnExtFullScreenExclusive {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_surface_present_modes2_ext: to_panic(loader(
                c"vkGetPhysicalDeviceSurfacePresentModes2EXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnExtSampleLocations {
    pub get_physical_device_multisample_properties_ext: vkGetPhysicalDeviceMultisamplePropertiesEXT,
}

impl PhysicalDeviceFnExtSampleLocations {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_multisample_properties_ext: to_panic(loader(
                c"vkGetPhysicalDeviceMultisamplePropertiesEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrCalibratedTimestamps {
    pub get_physical_device_calibrateable_time_domains_khr:
        vkGetPhysicalDeviceCalibrateableTimeDomainsKHR,
    pub get_physical_device_calibrateable_time_domains_ext:
        vkGetPhysicalDeviceCalibrateableTimeDomainsEXT,
}

impl PhysicalDeviceFnKhrCalibratedTimestamps {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_calibrateable_time_domains_khr: to_panic(loader(
                c"vkGetPhysicalDeviceCalibrateableTimeDomainsKHR",
            )),
            get_physical_device_calibrateable_time_domains_ext: to_panic(loader(
                c"vkGetPhysicalDeviceCalibrateableTimeDomainsEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrCooperativeMatrix {
    pub get_physical_device_cooperative_matrix_properties_khr:
        vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR,
}

impl PhysicalDeviceFnKhrCooperativeMatrix {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_cooperative_matrix_properties_khr: to_panic(loader(
                c"vkGetPhysicalDeviceCooperativeMatrixPropertiesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrDeviceGroup {
    pub get_physical_device_present_rectangles_khr: vkGetPhysicalDevicePresentRectanglesKHR,
}

impl PhysicalDeviceFnKhrDeviceGroup {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_present_rectangles_khr: to_panic(loader(
                c"vkGetPhysicalDevicePresentRectanglesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrDisplay {
    pub get_physical_device_display_properties_khr: vkGetPhysicalDeviceDisplayPropertiesKHR,
    pub get_physical_device_display_plane_properties_khr:
        vkGetPhysicalDeviceDisplayPlanePropertiesKHR,
    pub get_display_plane_supported_displays_khr: vkGetDisplayPlaneSupportedDisplaysKHR,
    pub get_display_mode_properties_khr: vkGetDisplayModePropertiesKHR,
    pub create_display_mode_khr: vkCreateDisplayModeKHR,
    pub get_display_plane_capabilities_khr: vkGetDisplayPlaneCapabilitiesKHR,
}

impl PhysicalDeviceFnKhrDisplay {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_display_properties_khr: to_panic(loader(
                c"vkGetPhysicalDeviceDisplayPropertiesKHR",
            )),
            get_physical_device_display_plane_properties_khr: to_panic(loader(
                c"vkGetPhysicalDeviceDisplayPlanePropertiesKHR",
            )),
            get_display_plane_supported_displays_khr: to_panic(loader(
                c"vkGetDisplayPlaneSupportedDisplaysKHR",
            )),
            get_display_mode_properties_khr: to_panic(loader(c"vkGetDisplayModePropertiesKHR")),
            create_display_mode_khr: to_panic(loader(c"vkCreateDisplayModeKHR")),
            get_display_plane_capabilities_khr: to_panic(loader(
                c"vkGetDisplayPlaneCapabilitiesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrFragmentShadingRate {
    pub get_physical_device_fragment_shading_rates_khr: vkGetPhysicalDeviceFragmentShadingRatesKHR,
}

impl PhysicalDeviceFnKhrFragmentShadingRate {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_fragment_shading_rates_khr: to_panic(loader(
                c"vkGetPhysicalDeviceFragmentShadingRatesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrGetDisplayProperties2 {
    pub get_physical_device_display_properties2_khr: vkGetPhysicalDeviceDisplayProperties2KHR,
    pub get_physical_device_display_plane_properties2_khr:
        vkGetPhysicalDeviceDisplayPlaneProperties2KHR,
    pub get_display_mode_properties2_khr: vkGetDisplayModeProperties2KHR,
    pub get_display_plane_capabilities2_khr: vkGetDisplayPlaneCapabilities2KHR,
}

impl PhysicalDeviceFnKhrGetDisplayProperties2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_display_properties2_khr: to_panic(loader(
                c"vkGetPhysicalDeviceDisplayProperties2KHR",
            )),
            get_physical_device_display_plane_properties2_khr: to_panic(loader(
                c"vkGetPhysicalDeviceDisplayPlaneProperties2KHR",
            )),
            get_display_mode_properties2_khr: to_panic(loader(c"vkGetDisplayModeProperties2KHR")),
            get_display_plane_capabilities2_khr: to_panic(loader(
                c"vkGetDisplayPlaneCapabilities2KHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrGetSurfaceCapabilities2 {
    pub get_physical_device_surface_capabilities2_khr: vkGetPhysicalDeviceSurfaceCapabilities2KHR,
    pub get_physical_device_surface_formats2_khr: vkGetPhysicalDeviceSurfaceFormats2KHR,
}

impl PhysicalDeviceFnKhrGetSurfaceCapabilities2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_surface_capabilities2_khr: to_panic(loader(
                c"vkGetPhysicalDeviceSurfaceCapabilities2KHR",
            )),
            get_physical_device_surface_formats2_khr: to_panic(loader(
                c"vkGetPhysicalDeviceSurfaceFormats2KHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrObjectRefresh {
    pub get_physical_device_refreshable_object_types_khr:
        vkGetPhysicalDeviceRefreshableObjectTypesKHR,
}

impl PhysicalDeviceFnKhrObjectRefresh {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_refreshable_object_types_khr: to_panic(loader(
                c"vkGetPhysicalDeviceRefreshableObjectTypesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrPerformanceQuery {
    pub enumerate_physical_device_queue_family_performance_query_counters_khr:
        vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR,
    pub get_physical_device_queue_family_performance_query_passes_khr:
        vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR,
}

impl PhysicalDeviceFnKhrPerformanceQuery {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            enumerate_physical_device_queue_family_performance_query_counters_khr: to_panic(
                loader(c"vkEnumeratePhysicalDeviceQueueFamilyPerformanceQueryCountersKHR"),
            ),
            get_physical_device_queue_family_performance_query_passes_khr: to_panic(loader(
                c"vkGetPhysicalDeviceQueueFamilyPerformanceQueryPassesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrSurface {
    pub get_physical_device_surface_support_khr: vkGetPhysicalDeviceSurfaceSupportKHR,
    pub get_physical_device_surface_capabilities_khr: vkGetPhysicalDeviceSurfaceCapabilitiesKHR,
    pub get_physical_device_surface_formats_khr: vkGetPhysicalDeviceSurfaceFormatsKHR,
    pub get_physical_device_surface_present_modes_khr: vkGetPhysicalDeviceSurfacePresentModesKHR,
}

impl PhysicalDeviceFnKhrSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_surface_support_khr: to_panic(loader(
                c"vkGetPhysicalDeviceSurfaceSupportKHR",
            )),
            get_physical_device_surface_capabilities_khr: to_panic(loader(
                c"vkGetPhysicalDeviceSurfaceCapabilitiesKHR",
            )),
            get_physical_device_surface_formats_khr: to_panic(loader(
                c"vkGetPhysicalDeviceSurfaceFormatsKHR",
            )),
            get_physical_device_surface_present_modes_khr: to_panic(loader(
                c"vkGetPhysicalDeviceSurfacePresentModesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrVideoEncodeQueue {
    pub get_physical_device_video_encode_quality_level_properties_khr:
        vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR,
}

impl PhysicalDeviceFnKhrVideoEncodeQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_video_encode_quality_level_properties_khr: to_panic(loader(
                c"vkGetPhysicalDeviceVideoEncodeQualityLevelPropertiesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrVideoQueue {
    pub get_physical_device_video_capabilities_khr: vkGetPhysicalDeviceVideoCapabilitiesKHR,
    pub get_physical_device_video_format_properties_khr:
        vkGetPhysicalDeviceVideoFormatPropertiesKHR,
}

impl PhysicalDeviceFnKhrVideoQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_video_capabilities_khr: to_panic(loader(
                c"vkGetPhysicalDeviceVideoCapabilitiesKHR",
            )),
            get_physical_device_video_format_properties_khr: to_panic(loader(
                c"vkGetPhysicalDeviceVideoFormatPropertiesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrWaylandSurface {
    pub get_physical_device_wayland_presentation_support_khr:
        vkGetPhysicalDeviceWaylandPresentationSupportKHR,
}

impl PhysicalDeviceFnKhrWaylandSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_wayland_presentation_support_khr: to_panic(loader(
                c"vkGetPhysicalDeviceWaylandPresentationSupportKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrWin32Surface {
    pub get_physical_device_win32_presentation_support_khr:
        vkGetPhysicalDeviceWin32PresentationSupportKHR,
}

impl PhysicalDeviceFnKhrWin32Surface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_win32_presentation_support_khr: to_panic(loader(
                c"vkGetPhysicalDeviceWin32PresentationSupportKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrXcbSurface {
    pub get_physical_device_xcb_presentation_support_khr:
        vkGetPhysicalDeviceXcbPresentationSupportKHR,
}

impl PhysicalDeviceFnKhrXcbSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_xcb_presentation_support_khr: to_panic(loader(
                c"vkGetPhysicalDeviceXcbPresentationSupportKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnKhrXlibSurface {
    pub get_physical_device_xlib_presentation_support_khr:
        vkGetPhysicalDeviceXlibPresentationSupportKHR,
}

impl PhysicalDeviceFnKhrXlibSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_xlib_presentation_support_khr: to_panic(loader(
                c"vkGetPhysicalDeviceXlibPresentationSupportKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvAcquireWinrtDisplay {
    pub acquire_winrt_display_nv: vkAcquireWinrtDisplayNV,
    pub get_winrt_display_nv: vkGetWinrtDisplayNV,
}

impl PhysicalDeviceFnNvAcquireWinrtDisplay {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            acquire_winrt_display_nv: to_panic(loader(c"vkAcquireWinrtDisplayNV")),
            get_winrt_display_nv: to_panic(loader(c"vkGetWinrtDisplayNV")),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvCooperativeMatrix {
    pub get_physical_device_cooperative_matrix_properties_nv:
        vkGetPhysicalDeviceCooperativeMatrixPropertiesNV,
}

impl PhysicalDeviceFnNvCooperativeMatrix {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_cooperative_matrix_properties_nv: to_panic(loader(
                c"vkGetPhysicalDeviceCooperativeMatrixPropertiesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvCooperativeMatrix2 {
    pub get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv:
        vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV,
}

impl PhysicalDeviceFnNvCooperativeMatrix2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_cooperative_matrix_flexible_dimensions_properties_nv: to_panic(
                loader(c"vkGetPhysicalDeviceCooperativeMatrixFlexibleDimensionsPropertiesNV"),
            ),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvCooperativeVector {
    pub get_physical_device_cooperative_vector_properties_nv:
        vkGetPhysicalDeviceCooperativeVectorPropertiesNV,
}

impl PhysicalDeviceFnNvCooperativeVector {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_cooperative_vector_properties_nv: to_panic(loader(
                c"vkGetPhysicalDeviceCooperativeVectorPropertiesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvCoverageReductionMode {
    pub get_physical_device_supported_framebuffer_mixed_samples_combinations_nv:
        vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV,
}

impl PhysicalDeviceFnNvCoverageReductionMode {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_supported_framebuffer_mixed_samples_combinations_nv: to_panic(
                loader(c"vkGetPhysicalDeviceSupportedFramebufferMixedSamplesCombinationsNV"),
            ),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvExternalMemoryCapabilities {
    pub get_physical_device_external_image_format_properties_nv:
        vkGetPhysicalDeviceExternalImageFormatPropertiesNV,
}

impl PhysicalDeviceFnNvExternalMemoryCapabilities {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_external_image_format_properties_nv: to_panic(loader(
                c"vkGetPhysicalDeviceExternalImageFormatPropertiesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvExternalMemorySciBuf {
    pub get_physical_device_external_memory_sci_buf_properties_nv:
        vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV,
    pub get_physical_device_sci_buf_attributes_nv: vkGetPhysicalDeviceSciBufAttributesNV,
}

impl PhysicalDeviceFnNvExternalMemorySciBuf {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_external_memory_sci_buf_properties_nv: to_panic(loader(
                c"vkGetPhysicalDeviceExternalMemorySciBufPropertiesNV",
            )),
            get_physical_device_sci_buf_attributes_nv: to_panic(loader(
                c"vkGetPhysicalDeviceSciBufAttributesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvExternalSciSync2 {
    pub get_physical_device_sci_sync_attributes_nv: vkGetPhysicalDeviceSciSyncAttributesNV,
}

impl PhysicalDeviceFnNvExternalSciSync2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_sci_sync_attributes_nv: to_panic(loader(
                c"vkGetPhysicalDeviceSciSyncAttributesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnNvOpticalFlow {
    pub get_physical_device_optical_flow_image_formats_nv:
        vkGetPhysicalDeviceOpticalFlowImageFormatsNV,
}

impl PhysicalDeviceFnNvOpticalFlow {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_optical_flow_image_formats_nv: to_panic(loader(
                c"vkGetPhysicalDeviceOpticalFlowImageFormatsNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct PhysicalDeviceFnQnxScreenSurface {
    pub get_physical_device_screen_presentation_support_qnx:
        vkGetPhysicalDeviceScreenPresentationSupportQNX,
}

impl PhysicalDeviceFnQnxScreenSurface {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_physical_device_screen_presentation_support_qnx: to_panic(loader(
                c"vkGetPhysicalDeviceScreenPresentationSupportQNX",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFn {
    pub v1_0: DeviceFnv1_0,
    pub v1_1: DeviceFnv1_1,
    pub v1_2: DeviceFnv1_2,
    pub v1_3: DeviceFnv1_3,
    pub v1_4: DeviceFnv1_4,
    pub amd_anti_lag: Option<DeviceFnAmdAntiLag>,
    pub amd_display_native_hdr: Option<DeviceFnAmdDisplayNativeHdr>,
    pub amd_shader_info: Option<DeviceFnAmdShaderInfo>,
    pub amdx_shader_enqueue: Option<DeviceFnAmdxShaderEnqueue>,
    pub android_external_memory_android_hardware_buffer:
        Option<DeviceFnAndroidExternalMemoryAndroidHardwareBuffer>,
    pub arm_data_graph: Option<DeviceFnArmDataGraph>,
    pub arm_tensors: Option<DeviceFnArmTensors>,
    pub ext_debug_marker: Option<DeviceFnExtDebugMarker>,
    pub ext_debug_utils: Option<DeviceFnExtDebugUtils>,
    pub ext_descriptor_buffer: Option<DeviceFnExtDescriptorBuffer>,
    pub ext_descriptor_heap: Option<DeviceFnExtDescriptorHeap>,
    pub ext_device_fault: Option<DeviceFnExtDeviceFault>,
    pub ext_device_generated_commands: Option<DeviceFnExtDeviceGeneratedCommands>,
    pub ext_display_control: Option<DeviceFnExtDisplayControl>,
    pub ext_external_memory_host: Option<DeviceFnExtExternalMemoryHost>,
    pub ext_external_memory_metal: Option<DeviceFnExtExternalMemoryMetal>,
    pub ext_full_screen_exclusive: Option<DeviceFnExtFullScreenExclusive>,
    pub ext_hdr_metadata: Option<DeviceFnExtHdrMetadata>,
    pub ext_image_drm_format_modifier: Option<DeviceFnExtImageDrmFormatModifier>,
    pub ext_metal_objects: Option<DeviceFnExtMetalObjects>,
    pub ext_opacity_micromap: Option<DeviceFnExtOpacityMicromap>,
    pub ext_pageable_device_local_memory: Option<DeviceFnExtPageableDeviceLocalMemory>,
    pub ext_pipeline_properties: Option<DeviceFnExtPipelineProperties>,
    pub ext_present_timing: Option<DeviceFnExtPresentTiming>,
    pub ext_shader_module_identifier: Option<DeviceFnExtShaderModuleIdentifier>,
    pub ext_shader_object: Option<DeviceFnExtShaderObject>,
    pub ext_validation_cache: Option<DeviceFnExtValidationCache>,
    pub fuchsia_buffer_collection: Option<DeviceFnFuchsiaBufferCollection>,
    pub fuchsia_external_memory: Option<DeviceFnFuchsiaExternalMemory>,
    pub fuchsia_external_semaphore: Option<DeviceFnFuchsiaExternalSemaphore>,
    pub google_display_timing: Option<DeviceFnGoogleDisplayTiming>,
    pub huawei_subpass_shading: Option<DeviceFnHuaweiSubpassShading>,
    pub intel_performance_query: Option<DeviceFnIntelPerformanceQuery>,
    pub khr_acceleration_structure: Option<DeviceFnKhrAccelerationStructure>,
    pub khr_calibrated_timestamps: Option<DeviceFnKhrCalibratedTimestamps>,
    pub khr_deferred_host_operations: Option<DeviceFnKhrDeferredHostOperations>,
    pub khr_device_group: Option<DeviceFnKhrDeviceGroup>,
    pub khr_display_swapchain: Option<DeviceFnKhrDisplaySwapchain>,
    pub khr_external_fence_fd: Option<DeviceFnKhrExternalFenceFd>,
    pub khr_external_fence_win32: Option<DeviceFnKhrExternalFenceWin32>,
    pub khr_external_memory_fd: Option<DeviceFnKhrExternalMemoryFd>,
    pub khr_external_memory_win32: Option<DeviceFnKhrExternalMemoryWin32>,
    pub khr_external_semaphore_fd: Option<DeviceFnKhrExternalSemaphoreFd>,
    pub khr_external_semaphore_win32: Option<DeviceFnKhrExternalSemaphoreWin32>,
    pub khr_performance_query: Option<DeviceFnKhrPerformanceQuery>,
    pub khr_pipeline_binary: Option<DeviceFnKhrPipelineBinary>,
    pub khr_pipeline_executable_properties: Option<DeviceFnKhrPipelineExecutableProperties>,
    pub khr_present_wait: Option<DeviceFnKhrPresentWait>,
    pub khr_present_wait2: Option<DeviceFnKhrPresentWait2>,
    pub khr_ray_tracing_pipeline: Option<DeviceFnKhrRayTracingPipeline>,
    pub khr_shared_presentable_image: Option<DeviceFnKhrSharedPresentableImage>,
    pub khr_swapchain: Option<DeviceFnKhrSwapchain>,
    pub khr_swapchain_maintenance1: Option<DeviceFnKhrSwapchainMaintenance1>,
    pub khr_video_encode_queue: Option<DeviceFnKhrVideoEncodeQueue>,
    pub khr_video_queue: Option<DeviceFnKhrVideoQueue>,
    pub nv_cluster_acceleration_structure: Option<DeviceFnNvClusterAccelerationStructure>,
    pub nv_cooperative_vector: Option<DeviceFnNvCooperativeVector>,
    pub nv_cuda_kernel_launch: Option<DeviceFnNvCudaKernelLaunch>,
    pub nv_device_generated_commands: Option<DeviceFnNvDeviceGeneratedCommands>,
    pub nv_device_generated_commands_compute: Option<DeviceFnNvDeviceGeneratedCommandsCompute>,
    pub nv_external_compute_queue: Option<DeviceFnNvExternalComputeQueue>,
    pub nv_external_memory_rdma: Option<DeviceFnNvExternalMemoryRdma>,
    pub nv_external_memory_sci_buf: Option<DeviceFnNvExternalMemorySciBuf>,
    pub nv_external_memory_win32: Option<DeviceFnNvExternalMemoryWin32>,
    pub nv_external_sci_sync: Option<DeviceFnNvExternalSciSync>,
    pub nv_external_sci_sync2: Option<DeviceFnNvExternalSciSync2>,
    pub nv_low_latency2: Option<DeviceFnNvLowLatency2>,
    pub nv_optical_flow: Option<DeviceFnNvOpticalFlow>,
    pub nv_partitioned_acceleration_structure: Option<DeviceFnNvPartitionedAccelerationStructure>,
    pub nv_ray_tracing: Option<DeviceFnNvRayTracing>,
    pub nvx_binary_import: Option<DeviceFnNvxBinaryImport>,
    pub nvx_image_view_handle: Option<DeviceFnNvxImageViewHandle>,
    pub ohos_external_memory: Option<DeviceFnOhosExternalMemory>,
    pub qcom_tile_properties: Option<DeviceFnQcomTileProperties>,
    pub qnx_external_memory_screen_buffer: Option<DeviceFnQnxExternalMemoryScreenBuffer>,
    pub valve_descriptor_set_host_mapping: Option<DeviceFnValveDescriptorSetHostMapping>,
}

impl DeviceFn {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const i8],
    ) -> Self {
        let mut out = Self {
            v1_0: DeviceFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                DeviceFnv1_1::load(&mut loader)
            } else {
                DeviceFnv1_1::default()
            },
            v1_2: if api_version >= API_VERSION_1_2 {
                DeviceFnv1_2::load(&mut loader)
            } else {
                DeviceFnv1_2::default()
            },
            v1_3: if api_version >= API_VERSION_1_3 {
                DeviceFnv1_3::load(&mut loader)
            } else {
                DeviceFnv1_3::default()
            },
            v1_4: if api_version >= API_VERSION_1_4 {
                DeviceFnv1_4::load(&mut loader)
            } else {
                DeviceFnv1_4::default()
            },
            amd_anti_lag: None,
            amd_display_native_hdr: None,
            amd_shader_info: None,
            amdx_shader_enqueue: None,
            android_external_memory_android_hardware_buffer: None,
            arm_data_graph: None,
            arm_tensors: None,
            ext_debug_marker: None,
            ext_debug_utils: None,
            ext_descriptor_buffer: None,
            ext_descriptor_heap: None,
            ext_device_fault: None,
            ext_device_generated_commands: None,
            ext_display_control: None,
            ext_external_memory_host: None,
            ext_external_memory_metal: None,
            ext_full_screen_exclusive: None,
            ext_hdr_metadata: None,
            ext_image_drm_format_modifier: None,
            ext_metal_objects: None,
            ext_opacity_micromap: None,
            ext_pageable_device_local_memory: None,
            ext_pipeline_properties: None,
            ext_present_timing: None,
            ext_shader_module_identifier: None,
            ext_shader_object: None,
            ext_validation_cache: None,
            fuchsia_buffer_collection: None,
            fuchsia_external_memory: None,
            fuchsia_external_semaphore: None,
            google_display_timing: None,
            huawei_subpass_shading: None,
            intel_performance_query: None,
            khr_acceleration_structure: None,
            khr_calibrated_timestamps: None,
            khr_deferred_host_operations: None,
            khr_device_group: None,
            khr_display_swapchain: None,
            khr_external_fence_fd: None,
            khr_external_fence_win32: None,
            khr_external_memory_fd: None,
            khr_external_memory_win32: None,
            khr_external_semaphore_fd: None,
            khr_external_semaphore_win32: None,
            khr_performance_query: None,
            khr_pipeline_binary: None,
            khr_pipeline_executable_properties: None,
            khr_present_wait: None,
            khr_present_wait2: None,
            khr_ray_tracing_pipeline: None,
            khr_shared_presentable_image: None,
            khr_swapchain: None,
            khr_swapchain_maintenance1: None,
            khr_video_encode_queue: None,
            khr_video_queue: None,
            nv_cluster_acceleration_structure: None,
            nv_cooperative_vector: None,
            nv_cuda_kernel_launch: None,
            nv_device_generated_commands: None,
            nv_device_generated_commands_compute: None,
            nv_external_compute_queue: None,
            nv_external_memory_rdma: None,
            nv_external_memory_sci_buf: None,
            nv_external_memory_win32: None,
            nv_external_sci_sync: None,
            nv_external_sci_sync2: None,
            nv_low_latency2: None,
            nv_optical_flow: None,
            nv_partitioned_acceleration_structure: None,
            nv_ray_tracing: None,
            nvx_binary_import: None,
            nvx_image_view_handle: None,
            ohos_external_memory: None,
            qcom_tile_properties: None,
            qnx_external_memory_screen_buffer: None,
            valve_descriptor_set_host_mapping: None,
        };
        for &ext in extensions {
            let ext = unsafe { CStr::from_ptr(ext).to_bytes() };
            match ext {
                b"VK_AMD_anti_lag" => {
                    out.amd_anti_lag = Some(DeviceFnAmdAntiLag::load(&mut loader))
                }
                b"VK_AMD_display_native_hdr" => {
                    out.amd_display_native_hdr =
                        Some(DeviceFnAmdDisplayNativeHdr::load(&mut loader))
                }
                b"VK_AMD_shader_info" => {
                    out.amd_shader_info = Some(DeviceFnAmdShaderInfo::load(&mut loader))
                }
                b"VK_AMDX_shader_enqueue" => {
                    out.amdx_shader_enqueue = Some(DeviceFnAmdxShaderEnqueue::load(&mut loader))
                }
                b"VK_ANDROID_external_memory_android_hardware_buffer" => {
                    out.android_external_memory_android_hardware_buffer = Some(
                        DeviceFnAndroidExternalMemoryAndroidHardwareBuffer::load(&mut loader),
                    )
                }
                b"VK_ARM_data_graph" => {
                    out.arm_data_graph = Some(DeviceFnArmDataGraph::load(&mut loader))
                }
                b"VK_ARM_tensors" => out.arm_tensors = Some(DeviceFnArmTensors::load(&mut loader)),
                b"VK_EXT_debug_marker" => {
                    out.ext_debug_marker = Some(DeviceFnExtDebugMarker::load(&mut loader))
                }
                b"VK_EXT_debug_utils" => {
                    out.ext_debug_utils = Some(DeviceFnExtDebugUtils::load(&mut loader))
                }
                b"VK_EXT_descriptor_buffer" => {
                    out.ext_descriptor_buffer = Some(DeviceFnExtDescriptorBuffer::load(&mut loader))
                }
                b"VK_EXT_descriptor_heap" => {
                    out.ext_descriptor_heap = Some(DeviceFnExtDescriptorHeap::load(&mut loader))
                }
                b"VK_EXT_device_fault" => {
                    out.ext_device_fault = Some(DeviceFnExtDeviceFault::load(&mut loader))
                }
                b"VK_EXT_device_generated_commands" => {
                    out.ext_device_generated_commands =
                        Some(DeviceFnExtDeviceGeneratedCommands::load(&mut loader))
                }
                b"VK_EXT_display_control" => {
                    out.ext_display_control = Some(DeviceFnExtDisplayControl::load(&mut loader))
                }
                b"VK_EXT_external_memory_host" => {
                    out.ext_external_memory_host =
                        Some(DeviceFnExtExternalMemoryHost::load(&mut loader))
                }
                b"VK_EXT_external_memory_metal" => {
                    out.ext_external_memory_metal =
                        Some(DeviceFnExtExternalMemoryMetal::load(&mut loader))
                }
                b"VK_EXT_full_screen_exclusive" => {
                    out.ext_full_screen_exclusive =
                        Some(DeviceFnExtFullScreenExclusive::load(&mut loader))
                }
                b"VK_EXT_hdr_metadata" => {
                    out.ext_hdr_metadata = Some(DeviceFnExtHdrMetadata::load(&mut loader))
                }
                b"VK_EXT_image_drm_format_modifier" => {
                    out.ext_image_drm_format_modifier =
                        Some(DeviceFnExtImageDrmFormatModifier::load(&mut loader))
                }
                b"VK_EXT_metal_objects" => {
                    out.ext_metal_objects = Some(DeviceFnExtMetalObjects::load(&mut loader))
                }
                b"VK_EXT_opacity_micromap" => {
                    out.ext_opacity_micromap = Some(DeviceFnExtOpacityMicromap::load(&mut loader))
                }
                b"VK_EXT_pageable_device_local_memory" => {
                    out.ext_pageable_device_local_memory =
                        Some(DeviceFnExtPageableDeviceLocalMemory::load(&mut loader))
                }
                b"VK_EXT_pipeline_properties" => {
                    out.ext_pipeline_properties =
                        Some(DeviceFnExtPipelineProperties::load(&mut loader))
                }
                b"VK_EXT_present_timing" => {
                    out.ext_present_timing = Some(DeviceFnExtPresentTiming::load(&mut loader))
                }
                b"VK_EXT_shader_module_identifier" => {
                    out.ext_shader_module_identifier =
                        Some(DeviceFnExtShaderModuleIdentifier::load(&mut loader))
                }
                b"VK_EXT_shader_object" => {
                    out.ext_shader_object = Some(DeviceFnExtShaderObject::load(&mut loader))
                }
                b"VK_EXT_validation_cache" => {
                    out.ext_validation_cache = Some(DeviceFnExtValidationCache::load(&mut loader))
                }
                b"VK_FUCHSIA_buffer_collection" => {
                    out.fuchsia_buffer_collection =
                        Some(DeviceFnFuchsiaBufferCollection::load(&mut loader))
                }
                b"VK_FUCHSIA_external_memory" => {
                    out.fuchsia_external_memory =
                        Some(DeviceFnFuchsiaExternalMemory::load(&mut loader))
                }
                b"VK_FUCHSIA_external_semaphore" => {
                    out.fuchsia_external_semaphore =
                        Some(DeviceFnFuchsiaExternalSemaphore::load(&mut loader))
                }
                b"VK_GOOGLE_display_timing" => {
                    out.google_display_timing = Some(DeviceFnGoogleDisplayTiming::load(&mut loader))
                }
                b"VK_HUAWEI_subpass_shading" => {
                    out.huawei_subpass_shading =
                        Some(DeviceFnHuaweiSubpassShading::load(&mut loader))
                }
                b"VK_INTEL_performance_query" => {
                    out.intel_performance_query =
                        Some(DeviceFnIntelPerformanceQuery::load(&mut loader))
                }
                b"VK_KHR_acceleration_structure" => {
                    out.khr_acceleration_structure =
                        Some(DeviceFnKhrAccelerationStructure::load(&mut loader))
                }
                b"VK_KHR_calibrated_timestamps" => {
                    out.khr_calibrated_timestamps =
                        Some(DeviceFnKhrCalibratedTimestamps::load(&mut loader))
                }
                b"VK_KHR_deferred_host_operations" => {
                    out.khr_deferred_host_operations =
                        Some(DeviceFnKhrDeferredHostOperations::load(&mut loader))
                }
                b"VK_KHR_device_group" => {
                    out.khr_device_group = Some(DeviceFnKhrDeviceGroup::load(&mut loader))
                }
                b"VK_KHR_display_swapchain" => {
                    out.khr_display_swapchain = Some(DeviceFnKhrDisplaySwapchain::load(&mut loader))
                }
                b"VK_KHR_external_fence_fd" => {
                    out.khr_external_fence_fd = Some(DeviceFnKhrExternalFenceFd::load(&mut loader))
                }
                b"VK_KHR_external_fence_win32" => {
                    out.khr_external_fence_win32 =
                        Some(DeviceFnKhrExternalFenceWin32::load(&mut loader))
                }
                b"VK_KHR_external_memory_fd" => {
                    out.khr_external_memory_fd =
                        Some(DeviceFnKhrExternalMemoryFd::load(&mut loader))
                }
                b"VK_KHR_external_memory_win32" => {
                    out.khr_external_memory_win32 =
                        Some(DeviceFnKhrExternalMemoryWin32::load(&mut loader))
                }
                b"VK_KHR_external_semaphore_fd" => {
                    out.khr_external_semaphore_fd =
                        Some(DeviceFnKhrExternalSemaphoreFd::load(&mut loader))
                }
                b"VK_KHR_external_semaphore_win32" => {
                    out.khr_external_semaphore_win32 =
                        Some(DeviceFnKhrExternalSemaphoreWin32::load(&mut loader))
                }
                b"VK_KHR_performance_query" => {
                    out.khr_performance_query = Some(DeviceFnKhrPerformanceQuery::load(&mut loader))
                }
                b"VK_KHR_pipeline_binary" => {
                    out.khr_pipeline_binary = Some(DeviceFnKhrPipelineBinary::load(&mut loader))
                }
                b"VK_KHR_pipeline_executable_properties" => {
                    out.khr_pipeline_executable_properties =
                        Some(DeviceFnKhrPipelineExecutableProperties::load(&mut loader))
                }
                b"VK_KHR_present_wait" => {
                    out.khr_present_wait = Some(DeviceFnKhrPresentWait::load(&mut loader))
                }
                b"VK_KHR_present_wait2" => {
                    out.khr_present_wait2 = Some(DeviceFnKhrPresentWait2::load(&mut loader))
                }
                b"VK_KHR_ray_tracing_pipeline" => {
                    out.khr_ray_tracing_pipeline =
                        Some(DeviceFnKhrRayTracingPipeline::load(&mut loader))
                }
                b"VK_KHR_shared_presentable_image" => {
                    out.khr_shared_presentable_image =
                        Some(DeviceFnKhrSharedPresentableImage::load(&mut loader))
                }
                b"VK_KHR_swapchain" => {
                    out.khr_swapchain = Some(DeviceFnKhrSwapchain::load(&mut loader))
                }
                b"VK_KHR_swapchain_maintenance1" => {
                    out.khr_swapchain_maintenance1 =
                        Some(DeviceFnKhrSwapchainMaintenance1::load(&mut loader))
                }
                b"VK_KHR_video_encode_queue" => {
                    out.khr_video_encode_queue =
                        Some(DeviceFnKhrVideoEncodeQueue::load(&mut loader))
                }
                b"VK_KHR_video_queue" => {
                    out.khr_video_queue = Some(DeviceFnKhrVideoQueue::load(&mut loader))
                }
                b"VK_NV_cluster_acceleration_structure" => {
                    out.nv_cluster_acceleration_structure =
                        Some(DeviceFnNvClusterAccelerationStructure::load(&mut loader))
                }
                b"VK_NV_cooperative_vector" => {
                    out.nv_cooperative_vector = Some(DeviceFnNvCooperativeVector::load(&mut loader))
                }
                b"VK_NV_cuda_kernel_launch" => {
                    out.nv_cuda_kernel_launch = Some(DeviceFnNvCudaKernelLaunch::load(&mut loader))
                }
                b"VK_NV_device_generated_commands" => {
                    out.nv_device_generated_commands =
                        Some(DeviceFnNvDeviceGeneratedCommands::load(&mut loader))
                }
                b"VK_NV_device_generated_commands_compute" => {
                    out.nv_device_generated_commands_compute =
                        Some(DeviceFnNvDeviceGeneratedCommandsCompute::load(&mut loader))
                }
                b"VK_NV_external_compute_queue" => {
                    out.nv_external_compute_queue =
                        Some(DeviceFnNvExternalComputeQueue::load(&mut loader))
                }
                b"VK_NV_external_memory_rdma" => {
                    out.nv_external_memory_rdma =
                        Some(DeviceFnNvExternalMemoryRdma::load(&mut loader))
                }
                b"VK_NV_external_memory_sci_buf" => {
                    out.nv_external_memory_sci_buf =
                        Some(DeviceFnNvExternalMemorySciBuf::load(&mut loader))
                }
                b"VK_NV_external_memory_win32" => {
                    out.nv_external_memory_win32 =
                        Some(DeviceFnNvExternalMemoryWin32::load(&mut loader))
                }
                b"VK_NV_external_sci_sync" => {
                    out.nv_external_sci_sync = Some(DeviceFnNvExternalSciSync::load(&mut loader))
                }
                b"VK_NV_external_sci_sync2" => {
                    out.nv_external_sci_sync2 = Some(DeviceFnNvExternalSciSync2::load(&mut loader))
                }
                b"VK_NV_low_latency2" => {
                    out.nv_low_latency2 = Some(DeviceFnNvLowLatency2::load(&mut loader))
                }
                b"VK_NV_optical_flow" => {
                    out.nv_optical_flow = Some(DeviceFnNvOpticalFlow::load(&mut loader))
                }
                b"VK_NV_partitioned_acceleration_structure" => {
                    out.nv_partitioned_acceleration_structure = Some(
                        DeviceFnNvPartitionedAccelerationStructure::load(&mut loader),
                    )
                }
                b"VK_NV_ray_tracing" => {
                    out.nv_ray_tracing = Some(DeviceFnNvRayTracing::load(&mut loader))
                }
                b"VK_NVX_binary_import" => {
                    out.nvx_binary_import = Some(DeviceFnNvxBinaryImport::load(&mut loader))
                }
                b"VK_NVX_image_view_handle" => {
                    out.nvx_image_view_handle = Some(DeviceFnNvxImageViewHandle::load(&mut loader))
                }
                b"VK_OHOS_external_memory" => {
                    out.ohos_external_memory = Some(DeviceFnOhosExternalMemory::load(&mut loader))
                }
                b"VK_QCOM_tile_properties" => {
                    out.qcom_tile_properties = Some(DeviceFnQcomTileProperties::load(&mut loader))
                }
                b"VK_QNX_external_memory_screen_buffer" => {
                    out.qnx_external_memory_screen_buffer =
                        Some(DeviceFnQnxExternalMemoryScreenBuffer::load(&mut loader))
                }
                b"VK_VALVE_descriptor_set_host_mapping" => {
                    out.valve_descriptor_set_host_mapping =
                        Some(DeviceFnValveDescriptorSetHostMapping::load(&mut loader))
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
    pub trim_command_pool_khr: Option<vkTrimCommandPoolKHR>,
    pub get_device_group_peer_memory_features: Option<vkGetDeviceGroupPeerMemoryFeatures>,
    pub get_device_group_peer_memory_features_khr: Option<vkGetDeviceGroupPeerMemoryFeaturesKHR>,
    pub bind_buffer_memory2: Option<vkBindBufferMemory2>,
    pub bind_buffer_memory2_khr: Option<vkBindBufferMemory2KHR>,
    pub bind_image_memory2: Option<vkBindImageMemory2>,
    pub bind_image_memory2_khr: Option<vkBindImageMemory2KHR>,
    pub create_descriptor_update_template: Option<vkCreateDescriptorUpdateTemplate>,
    pub create_descriptor_update_template_khr: Option<vkCreateDescriptorUpdateTemplateKHR>,
    pub destroy_descriptor_update_template: Option<vkDestroyDescriptorUpdateTemplate>,
    pub destroy_descriptor_update_template_khr: Option<vkDestroyDescriptorUpdateTemplateKHR>,
    pub update_descriptor_set_with_template: Option<vkUpdateDescriptorSetWithTemplate>,
    pub update_descriptor_set_with_template_khr: Option<vkUpdateDescriptorSetWithTemplateKHR>,
    pub get_buffer_memory_requirements2: Option<vkGetBufferMemoryRequirements2>,
    pub get_buffer_memory_requirements2_khr: Option<vkGetBufferMemoryRequirements2KHR>,
    pub get_image_memory_requirements2: Option<vkGetImageMemoryRequirements2>,
    pub get_image_memory_requirements2_khr: Option<vkGetImageMemoryRequirements2KHR>,
    pub get_image_sparse_memory_requirements2: Option<vkGetImageSparseMemoryRequirements2>,
    pub get_image_sparse_memory_requirements2_khr: Option<vkGetImageSparseMemoryRequirements2KHR>,
    pub create_sampler_ycbcr_conversion: Option<vkCreateSamplerYcbcrConversion>,
    pub create_sampler_ycbcr_conversion_khr: Option<vkCreateSamplerYcbcrConversionKHR>,
    pub destroy_sampler_ycbcr_conversion: Option<vkDestroySamplerYcbcrConversion>,
    pub destroy_sampler_ycbcr_conversion_khr: Option<vkDestroySamplerYcbcrConversionKHR>,
    pub get_device_queue2: Option<vkGetDeviceQueue2>,
    pub get_descriptor_set_layout_support: Option<vkGetDescriptorSetLayoutSupport>,
    pub get_descriptor_set_layout_support_khr: Option<vkGetDescriptorSetLayoutSupportKHR>,
}

impl DeviceFnv1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            trim_command_pool: to_option(loader(c"vkTrimCommandPool")),
            trim_command_pool_khr: to_option(loader(c"vkTrimCommandPoolKHR")),
            get_device_group_peer_memory_features: to_option(loader(
                c"vkGetDeviceGroupPeerMemoryFeatures",
            )),
            get_device_group_peer_memory_features_khr: to_option(loader(
                c"vkGetDeviceGroupPeerMemoryFeaturesKHR",
            )),
            bind_buffer_memory2: to_option(loader(c"vkBindBufferMemory2")),
            bind_buffer_memory2_khr: to_option(loader(c"vkBindBufferMemory2KHR")),
            bind_image_memory2: to_option(loader(c"vkBindImageMemory2")),
            bind_image_memory2_khr: to_option(loader(c"vkBindImageMemory2KHR")),
            create_descriptor_update_template: to_option(loader(
                c"vkCreateDescriptorUpdateTemplate",
            )),
            create_descriptor_update_template_khr: to_option(loader(
                c"vkCreateDescriptorUpdateTemplateKHR",
            )),
            destroy_descriptor_update_template: to_option(loader(
                c"vkDestroyDescriptorUpdateTemplate",
            )),
            destroy_descriptor_update_template_khr: to_option(loader(
                c"vkDestroyDescriptorUpdateTemplateKHR",
            )),
            update_descriptor_set_with_template: to_option(loader(
                c"vkUpdateDescriptorSetWithTemplate",
            )),
            update_descriptor_set_with_template_khr: to_option(loader(
                c"vkUpdateDescriptorSetWithTemplateKHR",
            )),
            get_buffer_memory_requirements2: to_option(loader(c"vkGetBufferMemoryRequirements2")),
            get_buffer_memory_requirements2_khr: to_option(loader(
                c"vkGetBufferMemoryRequirements2KHR",
            )),
            get_image_memory_requirements2: to_option(loader(c"vkGetImageMemoryRequirements2")),
            get_image_memory_requirements2_khr: to_option(loader(
                c"vkGetImageMemoryRequirements2KHR",
            )),
            get_image_sparse_memory_requirements2: to_option(loader(
                c"vkGetImageSparseMemoryRequirements2",
            )),
            get_image_sparse_memory_requirements2_khr: to_option(loader(
                c"vkGetImageSparseMemoryRequirements2KHR",
            )),
            create_sampler_ycbcr_conversion: to_option(loader(c"vkCreateSamplerYcbcrConversion")),
            create_sampler_ycbcr_conversion_khr: to_option(loader(
                c"vkCreateSamplerYcbcrConversionKHR",
            )),
            destroy_sampler_ycbcr_conversion: to_option(loader(c"vkDestroySamplerYcbcrConversion")),
            destroy_sampler_ycbcr_conversion_khr: to_option(loader(
                c"vkDestroySamplerYcbcrConversionKHR",
            )),
            get_device_queue2: to_option(loader(c"vkGetDeviceQueue2")),
            get_descriptor_set_layout_support: to_option(loader(
                c"vkGetDescriptorSetLayoutSupport",
            )),
            get_descriptor_set_layout_support_khr: to_option(loader(
                c"vkGetDescriptorSetLayoutSupportKHR",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_2 {
    pub reset_query_pool: Option<vkResetQueryPool>,
    pub reset_query_pool_ext: Option<vkResetQueryPoolEXT>,
    pub create_render_pass2: Option<vkCreateRenderPass2>,
    pub create_render_pass2_khr: Option<vkCreateRenderPass2KHR>,
    pub get_semaphore_counter_value: Option<vkGetSemaphoreCounterValue>,
    pub get_semaphore_counter_value_khr: Option<vkGetSemaphoreCounterValueKHR>,
    pub wait_semaphores: Option<vkWaitSemaphores>,
    pub wait_semaphores_khr: Option<vkWaitSemaphoresKHR>,
    pub signal_semaphore: Option<vkSignalSemaphore>,
    pub signal_semaphore_khr: Option<vkSignalSemaphoreKHR>,
    pub get_buffer_opaque_capture_address: Option<vkGetBufferOpaqueCaptureAddress>,
    pub get_buffer_opaque_capture_address_khr: Option<vkGetBufferOpaqueCaptureAddressKHR>,
    pub get_buffer_device_address: Option<vkGetBufferDeviceAddress>,
    pub get_buffer_device_address_khr: Option<vkGetBufferDeviceAddressKHR>,
    pub get_buffer_device_address_ext: Option<vkGetBufferDeviceAddressEXT>,
    pub get_device_memory_opaque_capture_address: Option<vkGetDeviceMemoryOpaqueCaptureAddress>,
    pub get_device_memory_opaque_capture_address_khr:
        Option<vkGetDeviceMemoryOpaqueCaptureAddressKHR>,
}

impl DeviceFnv1_2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            reset_query_pool: to_option(loader(c"vkResetQueryPool")),
            reset_query_pool_ext: to_option(loader(c"vkResetQueryPoolEXT")),
            create_render_pass2: to_option(loader(c"vkCreateRenderPass2")),
            create_render_pass2_khr: to_option(loader(c"vkCreateRenderPass2KHR")),
            get_semaphore_counter_value: to_option(loader(c"vkGetSemaphoreCounterValue")),
            get_semaphore_counter_value_khr: to_option(loader(c"vkGetSemaphoreCounterValueKHR")),
            wait_semaphores: to_option(loader(c"vkWaitSemaphores")),
            wait_semaphores_khr: to_option(loader(c"vkWaitSemaphoresKHR")),
            signal_semaphore: to_option(loader(c"vkSignalSemaphore")),
            signal_semaphore_khr: to_option(loader(c"vkSignalSemaphoreKHR")),
            get_buffer_opaque_capture_address: to_option(loader(
                c"vkGetBufferOpaqueCaptureAddress",
            )),
            get_buffer_opaque_capture_address_khr: to_option(loader(
                c"vkGetBufferOpaqueCaptureAddressKHR",
            )),
            get_buffer_device_address: to_option(loader(c"vkGetBufferDeviceAddress")),
            get_buffer_device_address_khr: to_option(loader(c"vkGetBufferDeviceAddressKHR")),
            get_buffer_device_address_ext: to_option(loader(c"vkGetBufferDeviceAddressEXT")),
            get_device_memory_opaque_capture_address: to_option(loader(
                c"vkGetDeviceMemoryOpaqueCaptureAddress",
            )),
            get_device_memory_opaque_capture_address_khr: to_option(loader(
                c"vkGetDeviceMemoryOpaqueCaptureAddressKHR",
            )),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_3 {
    pub get_device_buffer_memory_requirements: Option<vkGetDeviceBufferMemoryRequirements>,
    pub get_device_buffer_memory_requirements_khr: Option<vkGetDeviceBufferMemoryRequirementsKHR>,
    pub get_device_image_memory_requirements: Option<vkGetDeviceImageMemoryRequirements>,
    pub get_device_image_memory_requirements_khr: Option<vkGetDeviceImageMemoryRequirementsKHR>,
    pub get_device_image_sparse_memory_requirements:
        Option<vkGetDeviceImageSparseMemoryRequirements>,
    pub get_device_image_sparse_memory_requirements_khr:
        Option<vkGetDeviceImageSparseMemoryRequirementsKHR>,
    pub create_private_data_slot: Option<vkCreatePrivateDataSlot>,
    pub create_private_data_slot_ext: Option<vkCreatePrivateDataSlotEXT>,
    pub destroy_private_data_slot: Option<vkDestroyPrivateDataSlot>,
    pub destroy_private_data_slot_ext: Option<vkDestroyPrivateDataSlotEXT>,
    pub set_private_data: Option<vkSetPrivateData>,
    pub set_private_data_ext: Option<vkSetPrivateDataEXT>,
    pub get_private_data: Option<vkGetPrivateData>,
    pub get_private_data_ext: Option<vkGetPrivateDataEXT>,
}

impl DeviceFnv1_3 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_device_buffer_memory_requirements: to_option(loader(
                c"vkGetDeviceBufferMemoryRequirements",
            )),
            get_device_buffer_memory_requirements_khr: to_option(loader(
                c"vkGetDeviceBufferMemoryRequirementsKHR",
            )),
            get_device_image_memory_requirements: to_option(loader(
                c"vkGetDeviceImageMemoryRequirements",
            )),
            get_device_image_memory_requirements_khr: to_option(loader(
                c"vkGetDeviceImageMemoryRequirementsKHR",
            )),
            get_device_image_sparse_memory_requirements: to_option(loader(
                c"vkGetDeviceImageSparseMemoryRequirements",
            )),
            get_device_image_sparse_memory_requirements_khr: to_option(loader(
                c"vkGetDeviceImageSparseMemoryRequirementsKHR",
            )),
            create_private_data_slot: to_option(loader(c"vkCreatePrivateDataSlot")),
            create_private_data_slot_ext: to_option(loader(c"vkCreatePrivateDataSlotEXT")),
            destroy_private_data_slot: to_option(loader(c"vkDestroyPrivateDataSlot")),
            destroy_private_data_slot_ext: to_option(loader(c"vkDestroyPrivateDataSlotEXT")),
            set_private_data: to_option(loader(c"vkSetPrivateData")),
            set_private_data_ext: to_option(loader(c"vkSetPrivateDataEXT")),
            get_private_data: to_option(loader(c"vkGetPrivateData")),
            get_private_data_ext: to_option(loader(c"vkGetPrivateDataEXT")),
        }
    }
}

#[derive(Clone, Default)]
pub struct DeviceFnv1_4 {
    pub get_rendering_area_granularity: Option<vkGetRenderingAreaGranularity>,
    pub get_rendering_area_granularity_khr: Option<vkGetRenderingAreaGranularityKHR>,
    pub copy_memory_to_image: Option<vkCopyMemoryToImage>,
    pub copy_memory_to_image_ext: Option<vkCopyMemoryToImageEXT>,
    pub copy_image_to_memory: Option<vkCopyImageToMemory>,
    pub copy_image_to_memory_ext: Option<vkCopyImageToMemoryEXT>,
    pub copy_image_to_image: Option<vkCopyImageToImage>,
    pub copy_image_to_image_ext: Option<vkCopyImageToImageEXT>,
    pub transition_image_layout: Option<vkTransitionImageLayout>,
    pub transition_image_layout_ext: Option<vkTransitionImageLayoutEXT>,
    pub get_image_subresource_layout2: Option<vkGetImageSubresourceLayout2>,
    pub get_image_subresource_layout2_khr: Option<vkGetImageSubresourceLayout2KHR>,
    pub get_image_subresource_layout2_ext: Option<vkGetImageSubresourceLayout2EXT>,
    pub get_device_image_subresource_layout: Option<vkGetDeviceImageSubresourceLayout>,
    pub get_device_image_subresource_layout_khr: Option<vkGetDeviceImageSubresourceLayoutKHR>,
    pub map_memory2: Option<vkMapMemory2>,
    pub map_memory2_khr: Option<vkMapMemory2KHR>,
    pub unmap_memory2: Option<vkUnmapMemory2>,
    pub unmap_memory2_khr: Option<vkUnmapMemory2KHR>,
}

impl DeviceFnv1_4 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_rendering_area_granularity: to_option(loader(c"vkGetRenderingAreaGranularity")),
            get_rendering_area_granularity_khr: to_option(loader(
                c"vkGetRenderingAreaGranularityKHR",
            )),
            copy_memory_to_image: to_option(loader(c"vkCopyMemoryToImage")),
            copy_memory_to_image_ext: to_option(loader(c"vkCopyMemoryToImageEXT")),
            copy_image_to_memory: to_option(loader(c"vkCopyImageToMemory")),
            copy_image_to_memory_ext: to_option(loader(c"vkCopyImageToMemoryEXT")),
            copy_image_to_image: to_option(loader(c"vkCopyImageToImage")),
            copy_image_to_image_ext: to_option(loader(c"vkCopyImageToImageEXT")),
            transition_image_layout: to_option(loader(c"vkTransitionImageLayout")),
            transition_image_layout_ext: to_option(loader(c"vkTransitionImageLayoutEXT")),
            get_image_subresource_layout2: to_option(loader(c"vkGetImageSubresourceLayout2")),
            get_image_subresource_layout2_khr: to_option(loader(
                c"vkGetImageSubresourceLayout2KHR",
            )),
            get_image_subresource_layout2_ext: to_option(loader(
                c"vkGetImageSubresourceLayout2EXT",
            )),
            get_device_image_subresource_layout: to_option(loader(
                c"vkGetDeviceImageSubresourceLayout",
            )),
            get_device_image_subresource_layout_khr: to_option(loader(
                c"vkGetDeviceImageSubresourceLayoutKHR",
            )),
            map_memory2: to_option(loader(c"vkMapMemory2")),
            map_memory2_khr: to_option(loader(c"vkMapMemory2KHR")),
            unmap_memory2: to_option(loader(c"vkUnmapMemory2")),
            unmap_memory2_khr: to_option(loader(c"vkUnmapMemory2KHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnAmdAntiLag {
    pub anti_lag_update_amd: vkAntiLagUpdateAMD,
}

impl DeviceFnAmdAntiLag {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            anti_lag_update_amd: to_panic(loader(c"vkAntiLagUpdateAMD")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnAmdDisplayNativeHdr {
    pub set_local_dimming_amd: vkSetLocalDimmingAMD,
}

impl DeviceFnAmdDisplayNativeHdr {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_local_dimming_amd: to_panic(loader(c"vkSetLocalDimmingAMD")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnAmdShaderInfo {
    pub get_shader_info_amd: vkGetShaderInfoAMD,
}

impl DeviceFnAmdShaderInfo {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_shader_info_amd: to_panic(loader(c"vkGetShaderInfoAMD")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnAmdxShaderEnqueue {
    pub get_execution_graph_pipeline_scratch_size_amdx: vkGetExecutionGraphPipelineScratchSizeAMDX,
    pub get_execution_graph_pipeline_node_index_amdx: vkGetExecutionGraphPipelineNodeIndexAMDX,
    pub create_execution_graph_pipelines_amdx: vkCreateExecutionGraphPipelinesAMDX,
}

impl DeviceFnAmdxShaderEnqueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_execution_graph_pipeline_scratch_size_amdx: to_panic(loader(
                c"vkGetExecutionGraphPipelineScratchSizeAMDX",
            )),
            get_execution_graph_pipeline_node_index_amdx: to_panic(loader(
                c"vkGetExecutionGraphPipelineNodeIndexAMDX",
            )),
            create_execution_graph_pipelines_amdx: to_panic(loader(
                c"vkCreateExecutionGraphPipelinesAMDX",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnAndroidExternalMemoryAndroidHardwareBuffer {
    pub get_android_hardware_buffer_properties_android: vkGetAndroidHardwareBufferPropertiesANDROID,
    pub get_memory_android_hardware_buffer_android: vkGetMemoryAndroidHardwareBufferANDROID,
}

impl DeviceFnAndroidExternalMemoryAndroidHardwareBuffer {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_android_hardware_buffer_properties_android: to_panic(loader(
                c"vkGetAndroidHardwareBufferPropertiesANDROID",
            )),
            get_memory_android_hardware_buffer_android: to_panic(loader(
                c"vkGetMemoryAndroidHardwareBufferANDROID",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnArmDataGraph {
    pub create_data_graph_pipelines_arm: vkCreateDataGraphPipelinesARM,
    pub create_data_graph_pipeline_session_arm: vkCreateDataGraphPipelineSessionARM,
    pub get_data_graph_pipeline_session_bind_point_requirements_arm:
        vkGetDataGraphPipelineSessionBindPointRequirementsARM,
    pub get_data_graph_pipeline_session_memory_requirements_arm:
        vkGetDataGraphPipelineSessionMemoryRequirementsARM,
    pub bind_data_graph_pipeline_session_memory_arm: vkBindDataGraphPipelineSessionMemoryARM,
    pub destroy_data_graph_pipeline_session_arm: vkDestroyDataGraphPipelineSessionARM,
    pub get_data_graph_pipeline_available_properties_arm:
        vkGetDataGraphPipelineAvailablePropertiesARM,
    pub get_data_graph_pipeline_properties_arm: vkGetDataGraphPipelinePropertiesARM,
}

impl DeviceFnArmDataGraph {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_data_graph_pipelines_arm: to_panic(loader(c"vkCreateDataGraphPipelinesARM")),
            create_data_graph_pipeline_session_arm: to_panic(loader(
                c"vkCreateDataGraphPipelineSessionARM",
            )),
            get_data_graph_pipeline_session_bind_point_requirements_arm: to_panic(loader(
                c"vkGetDataGraphPipelineSessionBindPointRequirementsARM",
            )),
            get_data_graph_pipeline_session_memory_requirements_arm: to_panic(loader(
                c"vkGetDataGraphPipelineSessionMemoryRequirementsARM",
            )),
            bind_data_graph_pipeline_session_memory_arm: to_panic(loader(
                c"vkBindDataGraphPipelineSessionMemoryARM",
            )),
            destroy_data_graph_pipeline_session_arm: to_panic(loader(
                c"vkDestroyDataGraphPipelineSessionARM",
            )),
            get_data_graph_pipeline_available_properties_arm: to_panic(loader(
                c"vkGetDataGraphPipelineAvailablePropertiesARM",
            )),
            get_data_graph_pipeline_properties_arm: to_panic(loader(
                c"vkGetDataGraphPipelinePropertiesARM",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnArmTensors {
    pub create_tensor_arm: vkCreateTensorARM,
    pub destroy_tensor_arm: vkDestroyTensorARM,
    pub create_tensor_view_arm: vkCreateTensorViewARM,
    pub destroy_tensor_view_arm: vkDestroyTensorViewARM,
    pub get_tensor_memory_requirements_arm: vkGetTensorMemoryRequirementsARM,
    pub bind_tensor_memory_arm: vkBindTensorMemoryARM,
    pub get_device_tensor_memory_requirements_arm: vkGetDeviceTensorMemoryRequirementsARM,
    pub get_tensor_opaque_capture_descriptor_data_arm: vkGetTensorOpaqueCaptureDescriptorDataARM,
    pub get_tensor_view_opaque_capture_descriptor_data_arm:
        vkGetTensorViewOpaqueCaptureDescriptorDataARM,
}

impl DeviceFnArmTensors {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_tensor_arm: to_panic(loader(c"vkCreateTensorARM")),
            destroy_tensor_arm: to_panic(loader(c"vkDestroyTensorARM")),
            create_tensor_view_arm: to_panic(loader(c"vkCreateTensorViewARM")),
            destroy_tensor_view_arm: to_panic(loader(c"vkDestroyTensorViewARM")),
            get_tensor_memory_requirements_arm: to_panic(loader(
                c"vkGetTensorMemoryRequirementsARM",
            )),
            bind_tensor_memory_arm: to_panic(loader(c"vkBindTensorMemoryARM")),
            get_device_tensor_memory_requirements_arm: to_panic(loader(
                c"vkGetDeviceTensorMemoryRequirementsARM",
            )),
            get_tensor_opaque_capture_descriptor_data_arm: to_panic(loader(
                c"vkGetTensorOpaqueCaptureDescriptorDataARM",
            )),
            get_tensor_view_opaque_capture_descriptor_data_arm: to_panic(loader(
                c"vkGetTensorViewOpaqueCaptureDescriptorDataARM",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDebugMarker {
    pub debug_marker_set_object_name_ext: vkDebugMarkerSetObjectNameEXT,
    pub debug_marker_set_object_tag_ext: vkDebugMarkerSetObjectTagEXT,
}

impl DeviceFnExtDebugMarker {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            debug_marker_set_object_name_ext: to_panic(loader(c"vkDebugMarkerSetObjectNameEXT")),
            debug_marker_set_object_tag_ext: to_panic(loader(c"vkDebugMarkerSetObjectTagEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDebugUtils {
    pub set_debug_utils_object_name_ext: vkSetDebugUtilsObjectNameEXT,
    pub set_debug_utils_object_tag_ext: vkSetDebugUtilsObjectTagEXT,
}

impl DeviceFnExtDebugUtils {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_debug_utils_object_name_ext: to_panic(loader(c"vkSetDebugUtilsObjectNameEXT")),
            set_debug_utils_object_tag_ext: to_panic(loader(c"vkSetDebugUtilsObjectTagEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDescriptorBuffer {
    pub get_descriptor_set_layout_size_ext: vkGetDescriptorSetLayoutSizeEXT,
    pub get_descriptor_set_layout_binding_offset_ext: vkGetDescriptorSetLayoutBindingOffsetEXT,
    pub get_descriptor_ext: vkGetDescriptorEXT,
    pub get_buffer_opaque_capture_descriptor_data_ext: vkGetBufferOpaqueCaptureDescriptorDataEXT,
    pub get_image_opaque_capture_descriptor_data_ext: vkGetImageOpaqueCaptureDescriptorDataEXT,
    pub get_image_view_opaque_capture_descriptor_data_ext:
        vkGetImageViewOpaqueCaptureDescriptorDataEXT,
    pub get_sampler_opaque_capture_descriptor_data_ext: vkGetSamplerOpaqueCaptureDescriptorDataEXT,
    pub get_acceleration_structure_opaque_capture_descriptor_data_ext:
        vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT,
}

impl DeviceFnExtDescriptorBuffer {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_descriptor_set_layout_size_ext: to_panic(loader(
                c"vkGetDescriptorSetLayoutSizeEXT",
            )),
            get_descriptor_set_layout_binding_offset_ext: to_panic(loader(
                c"vkGetDescriptorSetLayoutBindingOffsetEXT",
            )),
            get_descriptor_ext: to_panic(loader(c"vkGetDescriptorEXT")),
            get_buffer_opaque_capture_descriptor_data_ext: to_panic(loader(
                c"vkGetBufferOpaqueCaptureDescriptorDataEXT",
            )),
            get_image_opaque_capture_descriptor_data_ext: to_panic(loader(
                c"vkGetImageOpaqueCaptureDescriptorDataEXT",
            )),
            get_image_view_opaque_capture_descriptor_data_ext: to_panic(loader(
                c"vkGetImageViewOpaqueCaptureDescriptorDataEXT",
            )),
            get_sampler_opaque_capture_descriptor_data_ext: to_panic(loader(
                c"vkGetSamplerOpaqueCaptureDescriptorDataEXT",
            )),
            get_acceleration_structure_opaque_capture_descriptor_data_ext: to_panic(loader(
                c"vkGetAccelerationStructureOpaqueCaptureDescriptorDataEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDescriptorHeap {
    pub write_sampler_descriptors_ext: vkWriteSamplerDescriptorsEXT,
    pub write_resource_descriptors_ext: vkWriteResourceDescriptorsEXT,
    pub register_custom_border_color_ext: vkRegisterCustomBorderColorEXT,
    pub unregister_custom_border_color_ext: vkUnregisterCustomBorderColorEXT,
    pub get_image_opaque_capture_data_ext: vkGetImageOpaqueCaptureDataEXT,
    pub get_tensor_opaque_capture_data_arm: vkGetTensorOpaqueCaptureDataARM,
}

impl DeviceFnExtDescriptorHeap {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            write_sampler_descriptors_ext: to_panic(loader(c"vkWriteSamplerDescriptorsEXT")),
            write_resource_descriptors_ext: to_panic(loader(c"vkWriteResourceDescriptorsEXT")),
            register_custom_border_color_ext: to_panic(loader(c"vkRegisterCustomBorderColorEXT")),
            unregister_custom_border_color_ext: to_panic(loader(
                c"vkUnregisterCustomBorderColorEXT",
            )),
            get_image_opaque_capture_data_ext: to_panic(loader(c"vkGetImageOpaqueCaptureDataEXT")),
            get_tensor_opaque_capture_data_arm: to_panic(loader(
                c"vkGetTensorOpaqueCaptureDataARM",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDeviceFault {
    pub get_device_fault_info_ext: vkGetDeviceFaultInfoEXT,
}

impl DeviceFnExtDeviceFault {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_device_fault_info_ext: to_panic(loader(c"vkGetDeviceFaultInfoEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDeviceGeneratedCommands {
    pub get_generated_commands_memory_requirements_ext: vkGetGeneratedCommandsMemoryRequirementsEXT,
    pub create_indirect_commands_layout_ext: vkCreateIndirectCommandsLayoutEXT,
    pub destroy_indirect_commands_layout_ext: vkDestroyIndirectCommandsLayoutEXT,
    pub create_indirect_execution_set_ext: vkCreateIndirectExecutionSetEXT,
    pub destroy_indirect_execution_set_ext: vkDestroyIndirectExecutionSetEXT,
    pub update_indirect_execution_set_pipeline_ext: vkUpdateIndirectExecutionSetPipelineEXT,
    pub update_indirect_execution_set_shader_ext: vkUpdateIndirectExecutionSetShaderEXT,
}

impl DeviceFnExtDeviceGeneratedCommands {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_generated_commands_memory_requirements_ext: to_panic(loader(
                c"vkGetGeneratedCommandsMemoryRequirementsEXT",
            )),
            create_indirect_commands_layout_ext: to_panic(loader(
                c"vkCreateIndirectCommandsLayoutEXT",
            )),
            destroy_indirect_commands_layout_ext: to_panic(loader(
                c"vkDestroyIndirectCommandsLayoutEXT",
            )),
            create_indirect_execution_set_ext: to_panic(loader(c"vkCreateIndirectExecutionSetEXT")),
            destroy_indirect_execution_set_ext: to_panic(loader(
                c"vkDestroyIndirectExecutionSetEXT",
            )),
            update_indirect_execution_set_pipeline_ext: to_panic(loader(
                c"vkUpdateIndirectExecutionSetPipelineEXT",
            )),
            update_indirect_execution_set_shader_ext: to_panic(loader(
                c"vkUpdateIndirectExecutionSetShaderEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtDisplayControl {
    pub display_power_control_ext: vkDisplayPowerControlEXT,
    pub register_device_event_ext: vkRegisterDeviceEventEXT,
    pub register_display_event_ext: vkRegisterDisplayEventEXT,
    pub get_swapchain_counter_ext: vkGetSwapchainCounterEXT,
}

impl DeviceFnExtDisplayControl {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            display_power_control_ext: to_panic(loader(c"vkDisplayPowerControlEXT")),
            register_device_event_ext: to_panic(loader(c"vkRegisterDeviceEventEXT")),
            register_display_event_ext: to_panic(loader(c"vkRegisterDisplayEventEXT")),
            get_swapchain_counter_ext: to_panic(loader(c"vkGetSwapchainCounterEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtExternalMemoryHost {
    pub get_memory_host_pointer_properties_ext: vkGetMemoryHostPointerPropertiesEXT,
}

impl DeviceFnExtExternalMemoryHost {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_host_pointer_properties_ext: to_panic(loader(
                c"vkGetMemoryHostPointerPropertiesEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtExternalMemoryMetal {
    pub get_memory_metal_handle_ext: vkGetMemoryMetalHandleEXT,
    pub get_memory_metal_handle_properties_ext: vkGetMemoryMetalHandlePropertiesEXT,
}

impl DeviceFnExtExternalMemoryMetal {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_metal_handle_ext: to_panic(loader(c"vkGetMemoryMetalHandleEXT")),
            get_memory_metal_handle_properties_ext: to_panic(loader(
                c"vkGetMemoryMetalHandlePropertiesEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtFullScreenExclusive {
    pub get_device_group_surface_present_modes2_ext: vkGetDeviceGroupSurfacePresentModes2EXT,
    pub acquire_full_screen_exclusive_mode_ext: vkAcquireFullScreenExclusiveModeEXT,
    pub release_full_screen_exclusive_mode_ext: vkReleaseFullScreenExclusiveModeEXT,
}

impl DeviceFnExtFullScreenExclusive {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_device_group_surface_present_modes2_ext: to_panic(loader(
                c"vkGetDeviceGroupSurfacePresentModes2EXT",
            )),
            acquire_full_screen_exclusive_mode_ext: to_panic(loader(
                c"vkAcquireFullScreenExclusiveModeEXT",
            )),
            release_full_screen_exclusive_mode_ext: to_panic(loader(
                c"vkReleaseFullScreenExclusiveModeEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtHdrMetadata {
    pub set_hdr_metadata_ext: vkSetHdrMetadataEXT,
}

impl DeviceFnExtHdrMetadata {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_hdr_metadata_ext: to_panic(loader(c"vkSetHdrMetadataEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtImageDrmFormatModifier {
    pub get_image_drm_format_modifier_properties_ext: vkGetImageDrmFormatModifierPropertiesEXT,
}

impl DeviceFnExtImageDrmFormatModifier {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_image_drm_format_modifier_properties_ext: to_panic(loader(
                c"vkGetImageDrmFormatModifierPropertiesEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtMetalObjects {
    pub export_metal_objects_ext: vkExportMetalObjectsEXT,
}

impl DeviceFnExtMetalObjects {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            export_metal_objects_ext: to_panic(loader(c"vkExportMetalObjectsEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtOpacityMicromap {
    pub create_micromap_ext: vkCreateMicromapEXT,
    pub build_micromaps_ext: vkBuildMicromapsEXT,
    pub destroy_micromap_ext: vkDestroyMicromapEXT,
    pub copy_micromap_ext: vkCopyMicromapEXT,
    pub copy_micromap_to_memory_ext: vkCopyMicromapToMemoryEXT,
    pub copy_memory_to_micromap_ext: vkCopyMemoryToMicromapEXT,
    pub write_micromaps_properties_ext: vkWriteMicromapsPropertiesEXT,
    pub get_device_micromap_compatibility_ext: vkGetDeviceMicromapCompatibilityEXT,
    pub get_micromap_build_sizes_ext: vkGetMicromapBuildSizesEXT,
}

impl DeviceFnExtOpacityMicromap {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_micromap_ext: to_panic(loader(c"vkCreateMicromapEXT")),
            build_micromaps_ext: to_panic(loader(c"vkBuildMicromapsEXT")),
            destroy_micromap_ext: to_panic(loader(c"vkDestroyMicromapEXT")),
            copy_micromap_ext: to_panic(loader(c"vkCopyMicromapEXT")),
            copy_micromap_to_memory_ext: to_panic(loader(c"vkCopyMicromapToMemoryEXT")),
            copy_memory_to_micromap_ext: to_panic(loader(c"vkCopyMemoryToMicromapEXT")),
            write_micromaps_properties_ext: to_panic(loader(c"vkWriteMicromapsPropertiesEXT")),
            get_device_micromap_compatibility_ext: to_panic(loader(
                c"vkGetDeviceMicromapCompatibilityEXT",
            )),
            get_micromap_build_sizes_ext: to_panic(loader(c"vkGetMicromapBuildSizesEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtPageableDeviceLocalMemory {
    pub set_device_memory_priority_ext: vkSetDeviceMemoryPriorityEXT,
}

impl DeviceFnExtPageableDeviceLocalMemory {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_device_memory_priority_ext: to_panic(loader(c"vkSetDeviceMemoryPriorityEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtPipelineProperties {
    pub get_pipeline_properties_ext: vkGetPipelinePropertiesEXT,
}

impl DeviceFnExtPipelineProperties {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_pipeline_properties_ext: to_panic(loader(c"vkGetPipelinePropertiesEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtPresentTiming {
    pub set_swapchain_present_timing_queue_size_ext: vkSetSwapchainPresentTimingQueueSizeEXT,
    pub get_swapchain_timing_properties_ext: vkGetSwapchainTimingPropertiesEXT,
    pub get_swapchain_time_domain_properties_ext: vkGetSwapchainTimeDomainPropertiesEXT,
    pub get_past_presentation_timing_ext: vkGetPastPresentationTimingEXT,
}

impl DeviceFnExtPresentTiming {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_swapchain_present_timing_queue_size_ext: to_panic(loader(
                c"vkSetSwapchainPresentTimingQueueSizeEXT",
            )),
            get_swapchain_timing_properties_ext: to_panic(loader(
                c"vkGetSwapchainTimingPropertiesEXT",
            )),
            get_swapchain_time_domain_properties_ext: to_panic(loader(
                c"vkGetSwapchainTimeDomainPropertiesEXT",
            )),
            get_past_presentation_timing_ext: to_panic(loader(c"vkGetPastPresentationTimingEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtShaderModuleIdentifier {
    pub get_shader_module_identifier_ext: vkGetShaderModuleIdentifierEXT,
    pub get_shader_module_create_info_identifier_ext: vkGetShaderModuleCreateInfoIdentifierEXT,
}

impl DeviceFnExtShaderModuleIdentifier {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_shader_module_identifier_ext: to_panic(loader(c"vkGetShaderModuleIdentifierEXT")),
            get_shader_module_create_info_identifier_ext: to_panic(loader(
                c"vkGetShaderModuleCreateInfoIdentifierEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtShaderObject {
    pub create_shaders_ext: vkCreateShadersEXT,
    pub destroy_shader_ext: vkDestroyShaderEXT,
    pub get_shader_binary_data_ext: vkGetShaderBinaryDataEXT,
}

impl DeviceFnExtShaderObject {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_shaders_ext: to_panic(loader(c"vkCreateShadersEXT")),
            destroy_shader_ext: to_panic(loader(c"vkDestroyShaderEXT")),
            get_shader_binary_data_ext: to_panic(loader(c"vkGetShaderBinaryDataEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnExtValidationCache {
    pub create_validation_cache_ext: vkCreateValidationCacheEXT,
    pub destroy_validation_cache_ext: vkDestroyValidationCacheEXT,
    pub get_validation_cache_data_ext: vkGetValidationCacheDataEXT,
    pub merge_validation_caches_ext: vkMergeValidationCachesEXT,
}

impl DeviceFnExtValidationCache {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_validation_cache_ext: to_panic(loader(c"vkCreateValidationCacheEXT")),
            destroy_validation_cache_ext: to_panic(loader(c"vkDestroyValidationCacheEXT")),
            get_validation_cache_data_ext: to_panic(loader(c"vkGetValidationCacheDataEXT")),
            merge_validation_caches_ext: to_panic(loader(c"vkMergeValidationCachesEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnFuchsiaBufferCollection {
    pub create_buffer_collection_fuchsia: vkCreateBufferCollectionFUCHSIA,
    pub set_buffer_collection_buffer_constraints_fuchsia:
        vkSetBufferCollectionBufferConstraintsFUCHSIA,
    pub set_buffer_collection_image_constraints_fuchsia:
        vkSetBufferCollectionImageConstraintsFUCHSIA,
    pub destroy_buffer_collection_fuchsia: vkDestroyBufferCollectionFUCHSIA,
    pub get_buffer_collection_properties_fuchsia: vkGetBufferCollectionPropertiesFUCHSIA,
}

impl DeviceFnFuchsiaBufferCollection {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_buffer_collection_fuchsia: to_panic(loader(c"vkCreateBufferCollectionFUCHSIA")),
            set_buffer_collection_buffer_constraints_fuchsia: to_panic(loader(
                c"vkSetBufferCollectionBufferConstraintsFUCHSIA",
            )),
            set_buffer_collection_image_constraints_fuchsia: to_panic(loader(
                c"vkSetBufferCollectionImageConstraintsFUCHSIA",
            )),
            destroy_buffer_collection_fuchsia: to_panic(loader(
                c"vkDestroyBufferCollectionFUCHSIA",
            )),
            get_buffer_collection_properties_fuchsia: to_panic(loader(
                c"vkGetBufferCollectionPropertiesFUCHSIA",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnFuchsiaExternalMemory {
    pub get_memory_zircon_handle_fuchsia: vkGetMemoryZirconHandleFUCHSIA,
    pub get_memory_zircon_handle_properties_fuchsia: vkGetMemoryZirconHandlePropertiesFUCHSIA,
}

impl DeviceFnFuchsiaExternalMemory {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_zircon_handle_fuchsia: to_panic(loader(c"vkGetMemoryZirconHandleFUCHSIA")),
            get_memory_zircon_handle_properties_fuchsia: to_panic(loader(
                c"vkGetMemoryZirconHandlePropertiesFUCHSIA",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnFuchsiaExternalSemaphore {
    pub get_semaphore_zircon_handle_fuchsia: vkGetSemaphoreZirconHandleFUCHSIA,
    pub import_semaphore_zircon_handle_fuchsia: vkImportSemaphoreZirconHandleFUCHSIA,
}

impl DeviceFnFuchsiaExternalSemaphore {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_semaphore_zircon_handle_fuchsia: to_panic(loader(
                c"vkGetSemaphoreZirconHandleFUCHSIA",
            )),
            import_semaphore_zircon_handle_fuchsia: to_panic(loader(
                c"vkImportSemaphoreZirconHandleFUCHSIA",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnGoogleDisplayTiming {
    pub get_refresh_cycle_duration_google: vkGetRefreshCycleDurationGOOGLE,
    pub get_past_presentation_timing_google: vkGetPastPresentationTimingGOOGLE,
}

impl DeviceFnGoogleDisplayTiming {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_refresh_cycle_duration_google: to_panic(loader(c"vkGetRefreshCycleDurationGOOGLE")),
            get_past_presentation_timing_google: to_panic(loader(
                c"vkGetPastPresentationTimingGOOGLE",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnHuaweiSubpassShading {
    pub get_device_subpass_shading_max_workgroup_size_huawei:
        vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI,
}

impl DeviceFnHuaweiSubpassShading {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_device_subpass_shading_max_workgroup_size_huawei: to_panic(loader(
                c"vkGetDeviceSubpassShadingMaxWorkgroupSizeHUAWEI",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnIntelPerformanceQuery {
    pub initialize_performance_api_intel: vkInitializePerformanceApiINTEL,
    pub uninitialize_performance_api_intel: vkUninitializePerformanceApiINTEL,
    pub acquire_performance_configuration_intel: vkAcquirePerformanceConfigurationINTEL,
    pub release_performance_configuration_intel: vkReleasePerformanceConfigurationINTEL,
    pub get_performance_parameter_intel: vkGetPerformanceParameterINTEL,
}

impl DeviceFnIntelPerformanceQuery {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            initialize_performance_api_intel: to_panic(loader(c"vkInitializePerformanceApiINTEL")),
            uninitialize_performance_api_intel: to_panic(loader(
                c"vkUninitializePerformanceApiINTEL",
            )),
            acquire_performance_configuration_intel: to_panic(loader(
                c"vkAcquirePerformanceConfigurationINTEL",
            )),
            release_performance_configuration_intel: to_panic(loader(
                c"vkReleasePerformanceConfigurationINTEL",
            )),
            get_performance_parameter_intel: to_panic(loader(c"vkGetPerformanceParameterINTEL")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrAccelerationStructure {
    pub destroy_acceleration_structure_khr: vkDestroyAccelerationStructureKHR,
    pub copy_acceleration_structure_khr: vkCopyAccelerationStructureKHR,
    pub copy_acceleration_structure_to_memory_khr: vkCopyAccelerationStructureToMemoryKHR,
    pub copy_memory_to_acceleration_structure_khr: vkCopyMemoryToAccelerationStructureKHR,
    pub write_acceleration_structures_properties_khr: vkWriteAccelerationStructuresPropertiesKHR,
    pub get_device_acceleration_structure_compatibility_khr:
        vkGetDeviceAccelerationStructureCompatibilityKHR,
    pub create_acceleration_structure_khr: vkCreateAccelerationStructureKHR,
    pub build_acceleration_structures_khr: vkBuildAccelerationStructuresKHR,
    pub get_acceleration_structure_device_address_khr: vkGetAccelerationStructureDeviceAddressKHR,
    pub get_acceleration_structure_build_sizes_khr: vkGetAccelerationStructureBuildSizesKHR,
}

impl DeviceFnKhrAccelerationStructure {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            destroy_acceleration_structure_khr: to_panic(loader(
                c"vkDestroyAccelerationStructureKHR",
            )),
            copy_acceleration_structure_khr: to_panic(loader(c"vkCopyAccelerationStructureKHR")),
            copy_acceleration_structure_to_memory_khr: to_panic(loader(
                c"vkCopyAccelerationStructureToMemoryKHR",
            )),
            copy_memory_to_acceleration_structure_khr: to_panic(loader(
                c"vkCopyMemoryToAccelerationStructureKHR",
            )),
            write_acceleration_structures_properties_khr: to_panic(loader(
                c"vkWriteAccelerationStructuresPropertiesKHR",
            )),
            get_device_acceleration_structure_compatibility_khr: to_panic(loader(
                c"vkGetDeviceAccelerationStructureCompatibilityKHR",
            )),
            create_acceleration_structure_khr: to_panic(loader(
                c"vkCreateAccelerationStructureKHR",
            )),
            build_acceleration_structures_khr: to_panic(loader(
                c"vkBuildAccelerationStructuresKHR",
            )),
            get_acceleration_structure_device_address_khr: to_panic(loader(
                c"vkGetAccelerationStructureDeviceAddressKHR",
            )),
            get_acceleration_structure_build_sizes_khr: to_panic(loader(
                c"vkGetAccelerationStructureBuildSizesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrCalibratedTimestamps {
    pub get_calibrated_timestamps_khr: vkGetCalibratedTimestampsKHR,
    pub get_calibrated_timestamps_ext: vkGetCalibratedTimestampsEXT,
}

impl DeviceFnKhrCalibratedTimestamps {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_calibrated_timestamps_khr: to_panic(loader(c"vkGetCalibratedTimestampsKHR")),
            get_calibrated_timestamps_ext: to_panic(loader(c"vkGetCalibratedTimestampsEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrDeferredHostOperations {
    pub create_deferred_operation_khr: vkCreateDeferredOperationKHR,
    pub destroy_deferred_operation_khr: vkDestroyDeferredOperationKHR,
    pub get_deferred_operation_max_concurrency_khr: vkGetDeferredOperationMaxConcurrencyKHR,
    pub get_deferred_operation_result_khr: vkGetDeferredOperationResultKHR,
    pub deferred_operation_join_khr: vkDeferredOperationJoinKHR,
}

impl DeviceFnKhrDeferredHostOperations {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_deferred_operation_khr: to_panic(loader(c"vkCreateDeferredOperationKHR")),
            destroy_deferred_operation_khr: to_panic(loader(c"vkDestroyDeferredOperationKHR")),
            get_deferred_operation_max_concurrency_khr: to_panic(loader(
                c"vkGetDeferredOperationMaxConcurrencyKHR",
            )),
            get_deferred_operation_result_khr: to_panic(loader(c"vkGetDeferredOperationResultKHR")),
            deferred_operation_join_khr: to_panic(loader(c"vkDeferredOperationJoinKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrDeviceGroup {
    pub get_device_group_present_capabilities_khr: vkGetDeviceGroupPresentCapabilitiesKHR,
    pub get_device_group_surface_present_modes_khr: vkGetDeviceGroupSurfacePresentModesKHR,
    pub acquire_next_image2_khr: vkAcquireNextImage2KHR,
}

impl DeviceFnKhrDeviceGroup {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_device_group_present_capabilities_khr: to_panic(loader(
                c"vkGetDeviceGroupPresentCapabilitiesKHR",
            )),
            get_device_group_surface_present_modes_khr: to_panic(loader(
                c"vkGetDeviceGroupSurfacePresentModesKHR",
            )),
            acquire_next_image2_khr: to_panic(loader(c"vkAcquireNextImage2KHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrDisplaySwapchain {
    pub create_shared_swapchains_khr: vkCreateSharedSwapchainsKHR,
}

impl DeviceFnKhrDisplaySwapchain {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_shared_swapchains_khr: to_panic(loader(c"vkCreateSharedSwapchainsKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrExternalFenceFd {
    pub get_fence_fd_khr: vkGetFenceFdKHR,
    pub import_fence_fd_khr: vkImportFenceFdKHR,
}

impl DeviceFnKhrExternalFenceFd {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_fence_fd_khr: to_panic(loader(c"vkGetFenceFdKHR")),
            import_fence_fd_khr: to_panic(loader(c"vkImportFenceFdKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrExternalFenceWin32 {
    pub get_fence_win32_handle_khr: vkGetFenceWin32HandleKHR,
    pub import_fence_win32_handle_khr: vkImportFenceWin32HandleKHR,
}

impl DeviceFnKhrExternalFenceWin32 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_fence_win32_handle_khr: to_panic(loader(c"vkGetFenceWin32HandleKHR")),
            import_fence_win32_handle_khr: to_panic(loader(c"vkImportFenceWin32HandleKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrExternalMemoryFd {
    pub get_memory_fd_khr: vkGetMemoryFdKHR,
    pub get_memory_fd_properties_khr: vkGetMemoryFdPropertiesKHR,
}

impl DeviceFnKhrExternalMemoryFd {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_fd_khr: to_panic(loader(c"vkGetMemoryFdKHR")),
            get_memory_fd_properties_khr: to_panic(loader(c"vkGetMemoryFdPropertiesKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrExternalMemoryWin32 {
    pub get_memory_win32_handle_khr: vkGetMemoryWin32HandleKHR,
    pub get_memory_win32_handle_properties_khr: vkGetMemoryWin32HandlePropertiesKHR,
}

impl DeviceFnKhrExternalMemoryWin32 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_win32_handle_khr: to_panic(loader(c"vkGetMemoryWin32HandleKHR")),
            get_memory_win32_handle_properties_khr: to_panic(loader(
                c"vkGetMemoryWin32HandlePropertiesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrExternalSemaphoreFd {
    pub get_semaphore_fd_khr: vkGetSemaphoreFdKHR,
    pub import_semaphore_fd_khr: vkImportSemaphoreFdKHR,
}

impl DeviceFnKhrExternalSemaphoreFd {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_semaphore_fd_khr: to_panic(loader(c"vkGetSemaphoreFdKHR")),
            import_semaphore_fd_khr: to_panic(loader(c"vkImportSemaphoreFdKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrExternalSemaphoreWin32 {
    pub get_semaphore_win32_handle_khr: vkGetSemaphoreWin32HandleKHR,
    pub import_semaphore_win32_handle_khr: vkImportSemaphoreWin32HandleKHR,
}

impl DeviceFnKhrExternalSemaphoreWin32 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_semaphore_win32_handle_khr: to_panic(loader(c"vkGetSemaphoreWin32HandleKHR")),
            import_semaphore_win32_handle_khr: to_panic(loader(c"vkImportSemaphoreWin32HandleKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrPerformanceQuery {
    pub acquire_profiling_lock_khr: vkAcquireProfilingLockKHR,
    pub release_profiling_lock_khr: vkReleaseProfilingLockKHR,
}

impl DeviceFnKhrPerformanceQuery {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            acquire_profiling_lock_khr: to_panic(loader(c"vkAcquireProfilingLockKHR")),
            release_profiling_lock_khr: to_panic(loader(c"vkReleaseProfilingLockKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrPipelineBinary {
    pub create_pipeline_binaries_khr: vkCreatePipelineBinariesKHR,
    pub destroy_pipeline_binary_khr: vkDestroyPipelineBinaryKHR,
    pub get_pipeline_key_khr: vkGetPipelineKeyKHR,
    pub get_pipeline_binary_data_khr: vkGetPipelineBinaryDataKHR,
    pub release_captured_pipeline_data_khr: vkReleaseCapturedPipelineDataKHR,
}

impl DeviceFnKhrPipelineBinary {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_pipeline_binaries_khr: to_panic(loader(c"vkCreatePipelineBinariesKHR")),
            destroy_pipeline_binary_khr: to_panic(loader(c"vkDestroyPipelineBinaryKHR")),
            get_pipeline_key_khr: to_panic(loader(c"vkGetPipelineKeyKHR")),
            get_pipeline_binary_data_khr: to_panic(loader(c"vkGetPipelineBinaryDataKHR")),
            release_captured_pipeline_data_khr: to_panic(loader(
                c"vkReleaseCapturedPipelineDataKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrPipelineExecutableProperties {
    pub get_pipeline_executable_properties_khr: vkGetPipelineExecutablePropertiesKHR,
    pub get_pipeline_executable_statistics_khr: vkGetPipelineExecutableStatisticsKHR,
    pub get_pipeline_executable_internal_representations_khr:
        vkGetPipelineExecutableInternalRepresentationsKHR,
}

impl DeviceFnKhrPipelineExecutableProperties {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_pipeline_executable_properties_khr: to_panic(loader(
                c"vkGetPipelineExecutablePropertiesKHR",
            )),
            get_pipeline_executable_statistics_khr: to_panic(loader(
                c"vkGetPipelineExecutableStatisticsKHR",
            )),
            get_pipeline_executable_internal_representations_khr: to_panic(loader(
                c"vkGetPipelineExecutableInternalRepresentationsKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrPresentWait {
    pub wait_for_present_khr: vkWaitForPresentKHR,
}

impl DeviceFnKhrPresentWait {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            wait_for_present_khr: to_panic(loader(c"vkWaitForPresentKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrPresentWait2 {
    pub wait_for_present2_khr: vkWaitForPresent2KHR,
}

impl DeviceFnKhrPresentWait2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            wait_for_present2_khr: to_panic(loader(c"vkWaitForPresent2KHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrRayTracingPipeline {
    pub get_ray_tracing_shader_group_handles_khr: vkGetRayTracingShaderGroupHandlesKHR,
    pub get_ray_tracing_shader_group_handles_nv: vkGetRayTracingShaderGroupHandlesNV,
    pub get_ray_tracing_capture_replay_shader_group_handles_khr:
        vkGetRayTracingCaptureReplayShaderGroupHandlesKHR,
    pub create_ray_tracing_pipelines_khr: vkCreateRayTracingPipelinesKHR,
    pub get_ray_tracing_shader_group_stack_size_khr: vkGetRayTracingShaderGroupStackSizeKHR,
}

impl DeviceFnKhrRayTracingPipeline {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_ray_tracing_shader_group_handles_khr: to_panic(loader(
                c"vkGetRayTracingShaderGroupHandlesKHR",
            )),
            get_ray_tracing_shader_group_handles_nv: to_panic(loader(
                c"vkGetRayTracingShaderGroupHandlesNV",
            )),
            get_ray_tracing_capture_replay_shader_group_handles_khr: to_panic(loader(
                c"vkGetRayTracingCaptureReplayShaderGroupHandlesKHR",
            )),
            create_ray_tracing_pipelines_khr: to_panic(loader(c"vkCreateRayTracingPipelinesKHR")),
            get_ray_tracing_shader_group_stack_size_khr: to_panic(loader(
                c"vkGetRayTracingShaderGroupStackSizeKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrSharedPresentableImage {
    pub get_swapchain_status_khr: vkGetSwapchainStatusKHR,
}

impl DeviceFnKhrSharedPresentableImage {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_swapchain_status_khr: to_panic(loader(c"vkGetSwapchainStatusKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrSwapchain {
    pub create_swapchain_khr: vkCreateSwapchainKHR,
    pub destroy_swapchain_khr: vkDestroySwapchainKHR,
    pub get_swapchain_images_khr: vkGetSwapchainImagesKHR,
    pub acquire_next_image_khr: vkAcquireNextImageKHR,
}

impl DeviceFnKhrSwapchain {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_swapchain_khr: to_panic(loader(c"vkCreateSwapchainKHR")),
            destroy_swapchain_khr: to_panic(loader(c"vkDestroySwapchainKHR")),
            get_swapchain_images_khr: to_panic(loader(c"vkGetSwapchainImagesKHR")),
            acquire_next_image_khr: to_panic(loader(c"vkAcquireNextImageKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrSwapchainMaintenance1 {
    pub release_swapchain_images_khr: vkReleaseSwapchainImagesKHR,
    pub release_swapchain_images_ext: vkReleaseSwapchainImagesEXT,
}

impl DeviceFnKhrSwapchainMaintenance1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            release_swapchain_images_khr: to_panic(loader(c"vkReleaseSwapchainImagesKHR")),
            release_swapchain_images_ext: to_panic(loader(c"vkReleaseSwapchainImagesEXT")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrVideoEncodeQueue {
    pub get_encoded_video_session_parameters_khr: vkGetEncodedVideoSessionParametersKHR,
}

impl DeviceFnKhrVideoEncodeQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_encoded_video_session_parameters_khr: to_panic(loader(
                c"vkGetEncodedVideoSessionParametersKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnKhrVideoQueue {
    pub create_video_session_khr: vkCreateVideoSessionKHR,
    pub destroy_video_session_khr: vkDestroyVideoSessionKHR,
    pub create_video_session_parameters_khr: vkCreateVideoSessionParametersKHR,
    pub update_video_session_parameters_khr: vkUpdateVideoSessionParametersKHR,
    pub destroy_video_session_parameters_khr: vkDestroyVideoSessionParametersKHR,
    pub get_video_session_memory_requirements_khr: vkGetVideoSessionMemoryRequirementsKHR,
    pub bind_video_session_memory_khr: vkBindVideoSessionMemoryKHR,
}

impl DeviceFnKhrVideoQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_video_session_khr: to_panic(loader(c"vkCreateVideoSessionKHR")),
            destroy_video_session_khr: to_panic(loader(c"vkDestroyVideoSessionKHR")),
            create_video_session_parameters_khr: to_panic(loader(
                c"vkCreateVideoSessionParametersKHR",
            )),
            update_video_session_parameters_khr: to_panic(loader(
                c"vkUpdateVideoSessionParametersKHR",
            )),
            destroy_video_session_parameters_khr: to_panic(loader(
                c"vkDestroyVideoSessionParametersKHR",
            )),
            get_video_session_memory_requirements_khr: to_panic(loader(
                c"vkGetVideoSessionMemoryRequirementsKHR",
            )),
            bind_video_session_memory_khr: to_panic(loader(c"vkBindVideoSessionMemoryKHR")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvClusterAccelerationStructure {
    pub get_cluster_acceleration_structure_build_sizes_nv:
        vkGetClusterAccelerationStructureBuildSizesNV,
}

impl DeviceFnNvClusterAccelerationStructure {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_cluster_acceleration_structure_build_sizes_nv: to_panic(loader(
                c"vkGetClusterAccelerationStructureBuildSizesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvCooperativeVector {
    pub convert_cooperative_vector_matrix_nv: vkConvertCooperativeVectorMatrixNV,
}

impl DeviceFnNvCooperativeVector {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            convert_cooperative_vector_matrix_nv: to_panic(loader(
                c"vkConvertCooperativeVectorMatrixNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvCudaKernelLaunch {
    pub create_cuda_module_nv: vkCreateCudaModuleNV,
    pub get_cuda_module_cache_nv: vkGetCudaModuleCacheNV,
    pub create_cuda_function_nv: vkCreateCudaFunctionNV,
    pub destroy_cuda_module_nv: vkDestroyCudaModuleNV,
    pub destroy_cuda_function_nv: vkDestroyCudaFunctionNV,
}

impl DeviceFnNvCudaKernelLaunch {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_cuda_module_nv: to_panic(loader(c"vkCreateCudaModuleNV")),
            get_cuda_module_cache_nv: to_panic(loader(c"vkGetCudaModuleCacheNV")),
            create_cuda_function_nv: to_panic(loader(c"vkCreateCudaFunctionNV")),
            destroy_cuda_module_nv: to_panic(loader(c"vkDestroyCudaModuleNV")),
            destroy_cuda_function_nv: to_panic(loader(c"vkDestroyCudaFunctionNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvDeviceGeneratedCommands {
    pub get_generated_commands_memory_requirements_nv: vkGetGeneratedCommandsMemoryRequirementsNV,
    pub create_indirect_commands_layout_nv: vkCreateIndirectCommandsLayoutNV,
    pub destroy_indirect_commands_layout_nv: vkDestroyIndirectCommandsLayoutNV,
}

impl DeviceFnNvDeviceGeneratedCommands {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_generated_commands_memory_requirements_nv: to_panic(loader(
                c"vkGetGeneratedCommandsMemoryRequirementsNV",
            )),
            create_indirect_commands_layout_nv: to_panic(loader(
                c"vkCreateIndirectCommandsLayoutNV",
            )),
            destroy_indirect_commands_layout_nv: to_panic(loader(
                c"vkDestroyIndirectCommandsLayoutNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvDeviceGeneratedCommandsCompute {
    pub get_pipeline_indirect_memory_requirements_nv: vkGetPipelineIndirectMemoryRequirementsNV,
    pub get_pipeline_indirect_device_address_nv: vkGetPipelineIndirectDeviceAddressNV,
}

impl DeviceFnNvDeviceGeneratedCommandsCompute {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_pipeline_indirect_memory_requirements_nv: to_panic(loader(
                c"vkGetPipelineIndirectMemoryRequirementsNV",
            )),
            get_pipeline_indirect_device_address_nv: to_panic(loader(
                c"vkGetPipelineIndirectDeviceAddressNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvExternalComputeQueue {
    pub create_external_compute_queue_nv: vkCreateExternalComputeQueueNV,
    pub destroy_external_compute_queue_nv: vkDestroyExternalComputeQueueNV,
}

impl DeviceFnNvExternalComputeQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_external_compute_queue_nv: to_panic(loader(c"vkCreateExternalComputeQueueNV")),
            destroy_external_compute_queue_nv: to_panic(loader(c"vkDestroyExternalComputeQueueNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvExternalMemoryRdma {
    pub get_memory_remote_address_nv: vkGetMemoryRemoteAddressNV,
}

impl DeviceFnNvExternalMemoryRdma {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_remote_address_nv: to_panic(loader(c"vkGetMemoryRemoteAddressNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvExternalMemorySciBuf {
    pub get_memory_sci_buf_nv: vkGetMemorySciBufNV,
}

impl DeviceFnNvExternalMemorySciBuf {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_sci_buf_nv: to_panic(loader(c"vkGetMemorySciBufNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvExternalMemoryWin32 {
    pub get_memory_win32_handle_nv: vkGetMemoryWin32HandleNV,
}

impl DeviceFnNvExternalMemoryWin32 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_memory_win32_handle_nv: to_panic(loader(c"vkGetMemoryWin32HandleNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvExternalSciSync {
    pub get_semaphore_sci_sync_obj_nv: vkGetSemaphoreSciSyncObjNV,
    pub import_semaphore_sci_sync_obj_nv: vkImportSemaphoreSciSyncObjNV,
}

impl DeviceFnNvExternalSciSync {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_semaphore_sci_sync_obj_nv: to_panic(loader(c"vkGetSemaphoreSciSyncObjNV")),
            import_semaphore_sci_sync_obj_nv: to_panic(loader(c"vkImportSemaphoreSciSyncObjNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvExternalSciSync2 {
    pub get_fence_sci_sync_fence_nv: vkGetFenceSciSyncFenceNV,
    pub get_fence_sci_sync_obj_nv: vkGetFenceSciSyncObjNV,
    pub import_fence_sci_sync_fence_nv: vkImportFenceSciSyncFenceNV,
    pub import_fence_sci_sync_obj_nv: vkImportFenceSciSyncObjNV,
    pub create_semaphore_sci_sync_pool_nv: vkCreateSemaphoreSciSyncPoolNV,
    pub destroy_semaphore_sci_sync_pool_nv: vkDestroySemaphoreSciSyncPoolNV,
}

impl DeviceFnNvExternalSciSync2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_fence_sci_sync_fence_nv: to_panic(loader(c"vkGetFenceSciSyncFenceNV")),
            get_fence_sci_sync_obj_nv: to_panic(loader(c"vkGetFenceSciSyncObjNV")),
            import_fence_sci_sync_fence_nv: to_panic(loader(c"vkImportFenceSciSyncFenceNV")),
            import_fence_sci_sync_obj_nv: to_panic(loader(c"vkImportFenceSciSyncObjNV")),
            create_semaphore_sci_sync_pool_nv: to_panic(loader(c"vkCreateSemaphoreSciSyncPoolNV")),
            destroy_semaphore_sci_sync_pool_nv: to_panic(loader(
                c"vkDestroySemaphoreSciSyncPoolNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvLowLatency2 {
    pub set_latency_sleep_mode_nv: vkSetLatencySleepModeNV,
    pub latency_sleep_nv: vkLatencySleepNV,
    pub set_latency_marker_nv: vkSetLatencyMarkerNV,
    pub get_latency_timings_nv: vkGetLatencyTimingsNV,
}

impl DeviceFnNvLowLatency2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_latency_sleep_mode_nv: to_panic(loader(c"vkSetLatencySleepModeNV")),
            latency_sleep_nv: to_panic(loader(c"vkLatencySleepNV")),
            set_latency_marker_nv: to_panic(loader(c"vkSetLatencyMarkerNV")),
            get_latency_timings_nv: to_panic(loader(c"vkGetLatencyTimingsNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvOpticalFlow {
    pub create_optical_flow_session_nv: vkCreateOpticalFlowSessionNV,
    pub destroy_optical_flow_session_nv: vkDestroyOpticalFlowSessionNV,
    pub bind_optical_flow_session_image_nv: vkBindOpticalFlowSessionImageNV,
}

impl DeviceFnNvOpticalFlow {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_optical_flow_session_nv: to_panic(loader(c"vkCreateOpticalFlowSessionNV")),
            destroy_optical_flow_session_nv: to_panic(loader(c"vkDestroyOpticalFlowSessionNV")),
            bind_optical_flow_session_image_nv: to_panic(loader(
                c"vkBindOpticalFlowSessionImageNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvPartitionedAccelerationStructure {
    pub get_partitioned_acceleration_structures_build_sizes_nv:
        vkGetPartitionedAccelerationStructuresBuildSizesNV,
}

impl DeviceFnNvPartitionedAccelerationStructure {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_partitioned_acceleration_structures_build_sizes_nv: to_panic(loader(
                c"vkGetPartitionedAccelerationStructuresBuildSizesNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvRayTracing {
    pub compile_deferred_nv: vkCompileDeferredNV,
    pub create_acceleration_structure_nv: vkCreateAccelerationStructureNV,
    pub destroy_acceleration_structure_nv: vkDestroyAccelerationStructureNV,
    pub get_acceleration_structure_memory_requirements_nv:
        vkGetAccelerationStructureMemoryRequirementsNV,
    pub bind_acceleration_structure_memory_nv: vkBindAccelerationStructureMemoryNV,
    pub get_acceleration_structure_handle_nv: vkGetAccelerationStructureHandleNV,
    pub create_ray_tracing_pipelines_nv: vkCreateRayTracingPipelinesNV,
}

impl DeviceFnNvRayTracing {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            compile_deferred_nv: to_panic(loader(c"vkCompileDeferredNV")),
            create_acceleration_structure_nv: to_panic(loader(c"vkCreateAccelerationStructureNV")),
            destroy_acceleration_structure_nv: to_panic(loader(
                c"vkDestroyAccelerationStructureNV",
            )),
            get_acceleration_structure_memory_requirements_nv: to_panic(loader(
                c"vkGetAccelerationStructureMemoryRequirementsNV",
            )),
            bind_acceleration_structure_memory_nv: to_panic(loader(
                c"vkBindAccelerationStructureMemoryNV",
            )),
            get_acceleration_structure_handle_nv: to_panic(loader(
                c"vkGetAccelerationStructureHandleNV",
            )),
            create_ray_tracing_pipelines_nv: to_panic(loader(c"vkCreateRayTracingPipelinesNV")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvxBinaryImport {
    pub create_cu_module_nvx: vkCreateCuModuleNVX,
    pub create_cu_function_nvx: vkCreateCuFunctionNVX,
    pub destroy_cu_module_nvx: vkDestroyCuModuleNVX,
    pub destroy_cu_function_nvx: vkDestroyCuFunctionNVX,
}

impl DeviceFnNvxBinaryImport {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            create_cu_module_nvx: to_panic(loader(c"vkCreateCuModuleNVX")),
            create_cu_function_nvx: to_panic(loader(c"vkCreateCuFunctionNVX")),
            destroy_cu_module_nvx: to_panic(loader(c"vkDestroyCuModuleNVX")),
            destroy_cu_function_nvx: to_panic(loader(c"vkDestroyCuFunctionNVX")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnNvxImageViewHandle {
    pub get_image_view_handle_nvx: vkGetImageViewHandleNVX,
    pub get_image_view_handle64_nvx: vkGetImageViewHandle64NVX,
    pub get_image_view_address_nvx: vkGetImageViewAddressNVX,
    pub get_device_combined_image_sampler_index_nvx: vkGetDeviceCombinedImageSamplerIndexNVX,
}

impl DeviceFnNvxImageViewHandle {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_image_view_handle_nvx: to_panic(loader(c"vkGetImageViewHandleNVX")),
            get_image_view_handle64_nvx: to_panic(loader(c"vkGetImageViewHandle64NVX")),
            get_image_view_address_nvx: to_panic(loader(c"vkGetImageViewAddressNVX")),
            get_device_combined_image_sampler_index_nvx: to_panic(loader(
                c"vkGetDeviceCombinedImageSamplerIndexNVX",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnOhosExternalMemory {
    pub get_native_buffer_properties_ohos: vkGetNativeBufferPropertiesOHOS,
    pub get_memory_native_buffer_ohos: vkGetMemoryNativeBufferOHOS,
}

impl DeviceFnOhosExternalMemory {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_native_buffer_properties_ohos: to_panic(loader(c"vkGetNativeBufferPropertiesOHOS")),
            get_memory_native_buffer_ohos: to_panic(loader(c"vkGetMemoryNativeBufferOHOS")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnQcomTileProperties {
    pub get_framebuffer_tile_properties_qcom: vkGetFramebufferTilePropertiesQCOM,
    pub get_dynamic_rendering_tile_properties_qcom: vkGetDynamicRenderingTilePropertiesQCOM,
}

impl DeviceFnQcomTileProperties {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_framebuffer_tile_properties_qcom: to_panic(loader(
                c"vkGetFramebufferTilePropertiesQCOM",
            )),
            get_dynamic_rendering_tile_properties_qcom: to_panic(loader(
                c"vkGetDynamicRenderingTilePropertiesQCOM",
            )),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnQnxExternalMemoryScreenBuffer {
    pub get_screen_buffer_properties_qnx: vkGetScreenBufferPropertiesQNX,
}

impl DeviceFnQnxExternalMemoryScreenBuffer {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_screen_buffer_properties_qnx: to_panic(loader(c"vkGetScreenBufferPropertiesQNX")),
        }
    }
}

#[derive(Clone)]
pub struct DeviceFnValveDescriptorSetHostMapping {
    pub get_descriptor_set_layout_host_mapping_info_valve:
        vkGetDescriptorSetLayoutHostMappingInfoVALVE,
    pub get_descriptor_set_host_mapping_valve: vkGetDescriptorSetHostMappingVALVE,
}

impl DeviceFnValveDescriptorSetHostMapping {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_descriptor_set_layout_host_mapping_info_valve: to_panic(loader(
                c"vkGetDescriptorSetLayoutHostMappingInfoVALVE",
            )),
            get_descriptor_set_host_mapping_valve: to_panic(loader(
                c"vkGetDescriptorSetHostMappingVALVE",
            )),
        }
    }
}

#[derive(Clone)]
pub struct QueueFn {
    pub v1_0: QueueFnv1_0,
    pub v1_3: QueueFnv1_3,
    pub ext_debug_utils: Option<QueueFnExtDebugUtils>,
    pub intel_performance_query: Option<QueueFnIntelPerformanceQuery>,
    pub khr_swapchain: Option<QueueFnKhrSwapchain>,
    pub nv_device_diagnostic_checkpoints: Option<QueueFnNvDeviceDiagnosticCheckpoints>,
    pub nv_low_latency2: Option<QueueFnNvLowLatency2>,
}

impl QueueFn {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const i8],
    ) -> Self {
        let mut out = Self {
            v1_0: QueueFnv1_0::load(&mut loader),
            v1_3: if api_version >= API_VERSION_1_3 {
                QueueFnv1_3::load(&mut loader)
            } else {
                QueueFnv1_3::default()
            },
            ext_debug_utils: None,
            intel_performance_query: None,
            khr_swapchain: None,
            nv_device_diagnostic_checkpoints: None,
            nv_low_latency2: None,
        };
        for &ext in extensions {
            let ext = unsafe { CStr::from_ptr(ext).to_bytes() };
            match ext {
                b"VK_EXT_debug_utils" => {
                    out.ext_debug_utils = Some(QueueFnExtDebugUtils::load(&mut loader))
                }
                b"VK_INTEL_performance_query" => {
                    out.intel_performance_query =
                        Some(QueueFnIntelPerformanceQuery::load(&mut loader))
                }
                b"VK_KHR_swapchain" => {
                    out.khr_swapchain = Some(QueueFnKhrSwapchain::load(&mut loader))
                }
                b"VK_NV_device_diagnostic_checkpoints" => {
                    out.nv_device_diagnostic_checkpoints =
                        Some(QueueFnNvDeviceDiagnosticCheckpoints::load(&mut loader))
                }
                b"VK_NV_low_latency2" => {
                    out.nv_low_latency2 = Some(QueueFnNvLowLatency2::load(&mut loader))
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
    pub queue_submit2_khr: Option<vkQueueSubmit2KHR>,
}

impl QueueFnv1_3 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_submit2: to_option(loader(c"vkQueueSubmit2")),
            queue_submit2_khr: to_option(loader(c"vkQueueSubmit2KHR")),
        }
    }
}

#[derive(Clone)]
pub struct QueueFnExtDebugUtils {
    pub queue_begin_debug_utils_label_ext: vkQueueBeginDebugUtilsLabelEXT,
    pub queue_end_debug_utils_label_ext: vkQueueEndDebugUtilsLabelEXT,
    pub queue_insert_debug_utils_label_ext: vkQueueInsertDebugUtilsLabelEXT,
}

impl QueueFnExtDebugUtils {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_begin_debug_utils_label_ext: to_panic(loader(c"vkQueueBeginDebugUtilsLabelEXT")),
            queue_end_debug_utils_label_ext: to_panic(loader(c"vkQueueEndDebugUtilsLabelEXT")),
            queue_insert_debug_utils_label_ext: to_panic(loader(
                c"vkQueueInsertDebugUtilsLabelEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct QueueFnIntelPerformanceQuery {
    pub queue_set_performance_configuration_intel: vkQueueSetPerformanceConfigurationINTEL,
}

impl QueueFnIntelPerformanceQuery {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_set_performance_configuration_intel: to_panic(loader(
                c"vkQueueSetPerformanceConfigurationINTEL",
            )),
        }
    }
}

#[derive(Clone)]
pub struct QueueFnKhrSwapchain {
    pub queue_present_khr: vkQueuePresentKHR,
}

impl QueueFnKhrSwapchain {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_present_khr: to_panic(loader(c"vkQueuePresentKHR")),
        }
    }
}

#[derive(Clone)]
pub struct QueueFnNvDeviceDiagnosticCheckpoints {
    pub get_queue_checkpoint_data_nv: vkGetQueueCheckpointDataNV,
    pub get_queue_checkpoint_data2_nv: vkGetQueueCheckpointData2NV,
}

impl QueueFnNvDeviceDiagnosticCheckpoints {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            get_queue_checkpoint_data_nv: to_panic(loader(c"vkGetQueueCheckpointDataNV")),
            get_queue_checkpoint_data2_nv: to_panic(loader(c"vkGetQueueCheckpointData2NV")),
        }
    }
}

#[derive(Clone)]
pub struct QueueFnNvLowLatency2 {
    pub queue_notify_out_of_band_nv: vkQueueNotifyOutOfBandNV,
}

impl QueueFnNvLowLatency2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            queue_notify_out_of_band_nv: to_panic(loader(c"vkQueueNotifyOutOfBandNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFn {
    pub v1_0: CommandBufferFnv1_0,
    pub v1_1: CommandBufferFnv1_1,
    pub v1_2: CommandBufferFnv1_2,
    pub v1_3: CommandBufferFnv1_3,
    pub v1_4: CommandBufferFnv1_4,
    pub amd_buffer_marker: Option<CommandBufferFnAmdBufferMarker>,
    pub amdx_shader_enqueue: Option<CommandBufferFnAmdxShaderEnqueue>,
    pub arm_data_graph: Option<CommandBufferFnArmDataGraph>,
    pub arm_tensors: Option<CommandBufferFnArmTensors>,
    pub ext_attachment_feedback_loop_dynamic_state:
        Option<CommandBufferFnExtAttachmentFeedbackLoopDynamicState>,
    pub ext_color_write_enable: Option<CommandBufferFnExtColorWriteEnable>,
    pub ext_conditional_rendering: Option<CommandBufferFnExtConditionalRendering>,
    pub ext_custom_resolve: Option<CommandBufferFnExtCustomResolve>,
    pub ext_debug_marker: Option<CommandBufferFnExtDebugMarker>,
    pub ext_debug_utils: Option<CommandBufferFnExtDebugUtils>,
    pub ext_depth_bias_control: Option<CommandBufferFnExtDepthBiasControl>,
    pub ext_depth_clamp_control: Option<CommandBufferFnExtDepthClampControl>,
    pub ext_descriptor_buffer: Option<CommandBufferFnExtDescriptorBuffer>,
    pub ext_descriptor_heap: Option<CommandBufferFnExtDescriptorHeap>,
    pub ext_device_generated_commands: Option<CommandBufferFnExtDeviceGeneratedCommands>,
    pub ext_discard_rectangles: Option<CommandBufferFnExtDiscardRectangles>,
    pub ext_memory_decompression: Option<CommandBufferFnExtMemoryDecompression>,
    pub ext_mesh_shader: Option<CommandBufferFnExtMeshShader>,
    pub ext_multi_draw: Option<CommandBufferFnExtMultiDraw>,
    pub ext_opacity_micromap: Option<CommandBufferFnExtOpacityMicromap>,
    pub ext_sample_locations: Option<CommandBufferFnExtSampleLocations>,
    pub ext_shader_object: Option<CommandBufferFnExtShaderObject>,
    pub ext_transform_feedback: Option<CommandBufferFnExtTransformFeedback>,
    pub huawei_cluster_culling_shader: Option<CommandBufferFnHuaweiClusterCullingShader>,
    pub huawei_invocation_mask: Option<CommandBufferFnHuaweiInvocationMask>,
    pub huawei_subpass_shading: Option<CommandBufferFnHuaweiSubpassShading>,
    pub intel_performance_query: Option<CommandBufferFnIntelPerformanceQuery>,
    pub khr_acceleration_structure: Option<CommandBufferFnKhrAccelerationStructure>,
    pub khr_copy_memory_indirect: Option<CommandBufferFnKhrCopyMemoryIndirect>,
    pub khr_fragment_shading_rate: Option<CommandBufferFnKhrFragmentShadingRate>,
    pub khr_maintenance10: Option<CommandBufferFnKhrMaintenance10>,
    pub khr_maintenance6: Option<CommandBufferFnKhrMaintenance6>,
    pub khr_object_refresh: Option<CommandBufferFnKhrObjectRefresh>,
    pub khr_ray_tracing_maintenance1: Option<CommandBufferFnKhrRayTracingMaintenance1>,
    pub khr_ray_tracing_pipeline: Option<CommandBufferFnKhrRayTracingPipeline>,
    pub khr_video_decode_queue: Option<CommandBufferFnKhrVideoDecodeQueue>,
    pub khr_video_encode_queue: Option<CommandBufferFnKhrVideoEncodeQueue>,
    pub khr_video_queue: Option<CommandBufferFnKhrVideoQueue>,
    pub nv_clip_space_w_scaling: Option<CommandBufferFnNvClipSpaceWScaling>,
    pub nv_cluster_acceleration_structure: Option<CommandBufferFnNvClusterAccelerationStructure>,
    pub nv_compute_occupancy_priority: Option<CommandBufferFnNvComputeOccupancyPriority>,
    pub nv_cooperative_vector: Option<CommandBufferFnNvCooperativeVector>,
    pub nv_copy_memory_indirect: Option<CommandBufferFnNvCopyMemoryIndirect>,
    pub nv_cuda_kernel_launch: Option<CommandBufferFnNvCudaKernelLaunch>,
    pub nv_device_diagnostic_checkpoints: Option<CommandBufferFnNvDeviceDiagnosticCheckpoints>,
    pub nv_device_generated_commands: Option<CommandBufferFnNvDeviceGeneratedCommands>,
    pub nv_device_generated_commands_compute:
        Option<CommandBufferFnNvDeviceGeneratedCommandsCompute>,
    pub nv_fragment_shading_rate_enums: Option<CommandBufferFnNvFragmentShadingRateEnums>,
    pub nv_memory_decompression: Option<CommandBufferFnNvMemoryDecompression>,
    pub nv_mesh_shader: Option<CommandBufferFnNvMeshShader>,
    pub nv_optical_flow: Option<CommandBufferFnNvOpticalFlow>,
    pub nv_partitioned_acceleration_structure:
        Option<CommandBufferFnNvPartitionedAccelerationStructure>,
    pub nv_ray_tracing: Option<CommandBufferFnNvRayTracing>,
    pub nv_scissor_exclusive: Option<CommandBufferFnNvScissorExclusive>,
    pub nv_shading_rate_image: Option<CommandBufferFnNvShadingRateImage>,
    pub nvx_binary_import: Option<CommandBufferFnNvxBinaryImport>,
    pub qcom_tile_memory_heap: Option<CommandBufferFnQcomTileMemoryHeap>,
    pub qcom_tile_shading: Option<CommandBufferFnQcomTileShading>,
}

impl CommandBufferFn {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(
        mut loader: F,
        api_version: u32,
        extensions: &[*const i8],
    ) -> Self {
        let mut out = Self {
            v1_0: CommandBufferFnv1_0::load(&mut loader),
            v1_1: if api_version >= API_VERSION_1_1 {
                CommandBufferFnv1_1::load(&mut loader)
            } else {
                CommandBufferFnv1_1::default()
            },
            v1_2: if api_version >= API_VERSION_1_2 {
                CommandBufferFnv1_2::load(&mut loader)
            } else {
                CommandBufferFnv1_2::default()
            },
            v1_3: if api_version >= API_VERSION_1_3 {
                CommandBufferFnv1_3::load(&mut loader)
            } else {
                CommandBufferFnv1_3::default()
            },
            v1_4: if api_version >= API_VERSION_1_4 {
                CommandBufferFnv1_4::load(&mut loader)
            } else {
                CommandBufferFnv1_4::default()
            },
            amd_buffer_marker: None,
            amdx_shader_enqueue: None,
            arm_data_graph: None,
            arm_tensors: None,
            ext_attachment_feedback_loop_dynamic_state: None,
            ext_color_write_enable: None,
            ext_conditional_rendering: None,
            ext_custom_resolve: None,
            ext_debug_marker: None,
            ext_debug_utils: None,
            ext_depth_bias_control: None,
            ext_depth_clamp_control: None,
            ext_descriptor_buffer: None,
            ext_descriptor_heap: None,
            ext_device_generated_commands: None,
            ext_discard_rectangles: None,
            ext_memory_decompression: None,
            ext_mesh_shader: None,
            ext_multi_draw: None,
            ext_opacity_micromap: None,
            ext_sample_locations: None,
            ext_shader_object: None,
            ext_transform_feedback: None,
            huawei_cluster_culling_shader: None,
            huawei_invocation_mask: None,
            huawei_subpass_shading: None,
            intel_performance_query: None,
            khr_acceleration_structure: None,
            khr_copy_memory_indirect: None,
            khr_fragment_shading_rate: None,
            khr_maintenance10: None,
            khr_maintenance6: None,
            khr_object_refresh: None,
            khr_ray_tracing_maintenance1: None,
            khr_ray_tracing_pipeline: None,
            khr_video_decode_queue: None,
            khr_video_encode_queue: None,
            khr_video_queue: None,
            nv_clip_space_w_scaling: None,
            nv_cluster_acceleration_structure: None,
            nv_compute_occupancy_priority: None,
            nv_cooperative_vector: None,
            nv_copy_memory_indirect: None,
            nv_cuda_kernel_launch: None,
            nv_device_diagnostic_checkpoints: None,
            nv_device_generated_commands: None,
            nv_device_generated_commands_compute: None,
            nv_fragment_shading_rate_enums: None,
            nv_memory_decompression: None,
            nv_mesh_shader: None,
            nv_optical_flow: None,
            nv_partitioned_acceleration_structure: None,
            nv_ray_tracing: None,
            nv_scissor_exclusive: None,
            nv_shading_rate_image: None,
            nvx_binary_import: None,
            qcom_tile_memory_heap: None,
            qcom_tile_shading: None,
        };
        for &ext in extensions {
            let ext = unsafe { CStr::from_ptr(ext).to_bytes() };
            match ext {
                b"VK_AMD_buffer_marker" => {
                    out.amd_buffer_marker = Some(CommandBufferFnAmdBufferMarker::load(&mut loader))
                }
                b"VK_AMDX_shader_enqueue" => {
                    out.amdx_shader_enqueue =
                        Some(CommandBufferFnAmdxShaderEnqueue::load(&mut loader))
                }
                b"VK_ARM_data_graph" => {
                    out.arm_data_graph = Some(CommandBufferFnArmDataGraph::load(&mut loader))
                }
                b"VK_ARM_tensors" => {
                    out.arm_tensors = Some(CommandBufferFnArmTensors::load(&mut loader))
                }
                b"VK_EXT_attachment_feedback_loop_dynamic_state" => {
                    out.ext_attachment_feedback_loop_dynamic_state = Some(
                        CommandBufferFnExtAttachmentFeedbackLoopDynamicState::load(&mut loader),
                    )
                }
                b"VK_EXT_color_write_enable" => {
                    out.ext_color_write_enable =
                        Some(CommandBufferFnExtColorWriteEnable::load(&mut loader))
                }
                b"VK_EXT_conditional_rendering" => {
                    out.ext_conditional_rendering =
                        Some(CommandBufferFnExtConditionalRendering::load(&mut loader))
                }
                b"VK_EXT_custom_resolve" => {
                    out.ext_custom_resolve =
                        Some(CommandBufferFnExtCustomResolve::load(&mut loader))
                }
                b"VK_EXT_debug_marker" => {
                    out.ext_debug_marker = Some(CommandBufferFnExtDebugMarker::load(&mut loader))
                }
                b"VK_EXT_debug_utils" => {
                    out.ext_debug_utils = Some(CommandBufferFnExtDebugUtils::load(&mut loader))
                }
                b"VK_EXT_depth_bias_control" => {
                    out.ext_depth_bias_control =
                        Some(CommandBufferFnExtDepthBiasControl::load(&mut loader))
                }
                b"VK_EXT_depth_clamp_control" => {
                    out.ext_depth_clamp_control =
                        Some(CommandBufferFnExtDepthClampControl::load(&mut loader))
                }
                b"VK_EXT_descriptor_buffer" => {
                    out.ext_descriptor_buffer =
                        Some(CommandBufferFnExtDescriptorBuffer::load(&mut loader))
                }
                b"VK_EXT_descriptor_heap" => {
                    out.ext_descriptor_heap =
                        Some(CommandBufferFnExtDescriptorHeap::load(&mut loader))
                }
                b"VK_EXT_device_generated_commands" => {
                    out.ext_device_generated_commands =
                        Some(CommandBufferFnExtDeviceGeneratedCommands::load(&mut loader))
                }
                b"VK_EXT_discard_rectangles" => {
                    out.ext_discard_rectangles =
                        Some(CommandBufferFnExtDiscardRectangles::load(&mut loader))
                }
                b"VK_EXT_memory_decompression" => {
                    out.ext_memory_decompression =
                        Some(CommandBufferFnExtMemoryDecompression::load(&mut loader))
                }
                b"VK_EXT_mesh_shader" => {
                    out.ext_mesh_shader = Some(CommandBufferFnExtMeshShader::load(&mut loader))
                }
                b"VK_EXT_multi_draw" => {
                    out.ext_multi_draw = Some(CommandBufferFnExtMultiDraw::load(&mut loader))
                }
                b"VK_EXT_opacity_micromap" => {
                    out.ext_opacity_micromap =
                        Some(CommandBufferFnExtOpacityMicromap::load(&mut loader))
                }
                b"VK_EXT_sample_locations" => {
                    out.ext_sample_locations =
                        Some(CommandBufferFnExtSampleLocations::load(&mut loader))
                }
                b"VK_EXT_shader_object" => {
                    out.ext_shader_object = Some(CommandBufferFnExtShaderObject::load(&mut loader))
                }
                b"VK_EXT_transform_feedback" => {
                    out.ext_transform_feedback =
                        Some(CommandBufferFnExtTransformFeedback::load(&mut loader))
                }
                b"VK_HUAWEI_cluster_culling_shader" => {
                    out.huawei_cluster_culling_shader =
                        Some(CommandBufferFnHuaweiClusterCullingShader::load(&mut loader))
                }
                b"VK_HUAWEI_invocation_mask" => {
                    out.huawei_invocation_mask =
                        Some(CommandBufferFnHuaweiInvocationMask::load(&mut loader))
                }
                b"VK_HUAWEI_subpass_shading" => {
                    out.huawei_subpass_shading =
                        Some(CommandBufferFnHuaweiSubpassShading::load(&mut loader))
                }
                b"VK_INTEL_performance_query" => {
                    out.intel_performance_query =
                        Some(CommandBufferFnIntelPerformanceQuery::load(&mut loader))
                }
                b"VK_KHR_acceleration_structure" => {
                    out.khr_acceleration_structure =
                        Some(CommandBufferFnKhrAccelerationStructure::load(&mut loader))
                }
                b"VK_KHR_copy_memory_indirect" => {
                    out.khr_copy_memory_indirect =
                        Some(CommandBufferFnKhrCopyMemoryIndirect::load(&mut loader))
                }
                b"VK_KHR_fragment_shading_rate" => {
                    out.khr_fragment_shading_rate =
                        Some(CommandBufferFnKhrFragmentShadingRate::load(&mut loader))
                }
                b"VK_KHR_maintenance10" => {
                    out.khr_maintenance10 = Some(CommandBufferFnKhrMaintenance10::load(&mut loader))
                }
                b"VK_KHR_maintenance6" => {
                    out.khr_maintenance6 = Some(CommandBufferFnKhrMaintenance6::load(&mut loader))
                }
                b"VK_KHR_object_refresh" => {
                    out.khr_object_refresh =
                        Some(CommandBufferFnKhrObjectRefresh::load(&mut loader))
                }
                b"VK_KHR_ray_tracing_maintenance1" => {
                    out.khr_ray_tracing_maintenance1 =
                        Some(CommandBufferFnKhrRayTracingMaintenance1::load(&mut loader))
                }
                b"VK_KHR_ray_tracing_pipeline" => {
                    out.khr_ray_tracing_pipeline =
                        Some(CommandBufferFnKhrRayTracingPipeline::load(&mut loader))
                }
                b"VK_KHR_video_decode_queue" => {
                    out.khr_video_decode_queue =
                        Some(CommandBufferFnKhrVideoDecodeQueue::load(&mut loader))
                }
                b"VK_KHR_video_encode_queue" => {
                    out.khr_video_encode_queue =
                        Some(CommandBufferFnKhrVideoEncodeQueue::load(&mut loader))
                }
                b"VK_KHR_video_queue" => {
                    out.khr_video_queue = Some(CommandBufferFnKhrVideoQueue::load(&mut loader))
                }
                b"VK_NV_clip_space_w_scaling" => {
                    out.nv_clip_space_w_scaling =
                        Some(CommandBufferFnNvClipSpaceWScaling::load(&mut loader))
                }
                b"VK_NV_cluster_acceleration_structure" => {
                    out.nv_cluster_acceleration_structure = Some(
                        CommandBufferFnNvClusterAccelerationStructure::load(&mut loader),
                    )
                }
                b"VK_NV_compute_occupancy_priority" => {
                    out.nv_compute_occupancy_priority =
                        Some(CommandBufferFnNvComputeOccupancyPriority::load(&mut loader))
                }
                b"VK_NV_cooperative_vector" => {
                    out.nv_cooperative_vector =
                        Some(CommandBufferFnNvCooperativeVector::load(&mut loader))
                }
                b"VK_NV_copy_memory_indirect" => {
                    out.nv_copy_memory_indirect =
                        Some(CommandBufferFnNvCopyMemoryIndirect::load(&mut loader))
                }
                b"VK_NV_cuda_kernel_launch" => {
                    out.nv_cuda_kernel_launch =
                        Some(CommandBufferFnNvCudaKernelLaunch::load(&mut loader))
                }
                b"VK_NV_device_diagnostic_checkpoints" => {
                    out.nv_device_diagnostic_checkpoints = Some(
                        CommandBufferFnNvDeviceDiagnosticCheckpoints::load(&mut loader),
                    )
                }
                b"VK_NV_device_generated_commands" => {
                    out.nv_device_generated_commands =
                        Some(CommandBufferFnNvDeviceGeneratedCommands::load(&mut loader))
                }
                b"VK_NV_device_generated_commands_compute" => {
                    out.nv_device_generated_commands_compute = Some(
                        CommandBufferFnNvDeviceGeneratedCommandsCompute::load(&mut loader),
                    )
                }
                b"VK_NV_fragment_shading_rate_enums" => {
                    out.nv_fragment_shading_rate_enums =
                        Some(CommandBufferFnNvFragmentShadingRateEnums::load(&mut loader))
                }
                b"VK_NV_memory_decompression" => {
                    out.nv_memory_decompression =
                        Some(CommandBufferFnNvMemoryDecompression::load(&mut loader))
                }
                b"VK_NV_mesh_shader" => {
                    out.nv_mesh_shader = Some(CommandBufferFnNvMeshShader::load(&mut loader))
                }
                b"VK_NV_optical_flow" => {
                    out.nv_optical_flow = Some(CommandBufferFnNvOpticalFlow::load(&mut loader))
                }
                b"VK_NV_partitioned_acceleration_structure" => {
                    out.nv_partitioned_acceleration_structure = Some(
                        CommandBufferFnNvPartitionedAccelerationStructure::load(&mut loader),
                    )
                }
                b"VK_NV_ray_tracing" => {
                    out.nv_ray_tracing = Some(CommandBufferFnNvRayTracing::load(&mut loader))
                }
                b"VK_NV_scissor_exclusive" => {
                    out.nv_scissor_exclusive =
                        Some(CommandBufferFnNvScissorExclusive::load(&mut loader))
                }
                b"VK_NV_shading_rate_image" => {
                    out.nv_shading_rate_image =
                        Some(CommandBufferFnNvShadingRateImage::load(&mut loader))
                }
                b"VK_NVX_binary_import" => {
                    out.nvx_binary_import = Some(CommandBufferFnNvxBinaryImport::load(&mut loader))
                }
                b"VK_QCOM_tile_memory_heap" => {
                    out.qcom_tile_memory_heap =
                        Some(CommandBufferFnQcomTileMemoryHeap::load(&mut loader))
                }
                b"VK_QCOM_tile_shading" => {
                    out.qcom_tile_shading = Some(CommandBufferFnQcomTileShading::load(&mut loader))
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
    pub set_device_mask_khr: Option<vkCmdSetDeviceMaskKHR>,
    pub dispatch_base: Option<vkCmdDispatchBase>,
    pub dispatch_base_khr: Option<vkCmdDispatchBaseKHR>,
}

impl CommandBufferFnv1_1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_device_mask: to_option(loader(c"vkCmdSetDeviceMask")),
            set_device_mask_khr: to_option(loader(c"vkCmdSetDeviceMaskKHR")),
            dispatch_base: to_option(loader(c"vkCmdDispatchBase")),
            dispatch_base_khr: to_option(loader(c"vkCmdDispatchBaseKHR")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_2 {
    pub begin_render_pass2: Option<vkCmdBeginRenderPass2>,
    pub begin_render_pass2_khr: Option<vkCmdBeginRenderPass2KHR>,
    pub next_subpass2: Option<vkCmdNextSubpass2>,
    pub next_subpass2_khr: Option<vkCmdNextSubpass2KHR>,
    pub end_render_pass2: Option<vkCmdEndRenderPass2>,
    pub end_render_pass2_khr: Option<vkCmdEndRenderPass2KHR>,
    pub draw_indirect_count: Option<vkCmdDrawIndirectCount>,
    pub draw_indirect_count_khr: Option<vkCmdDrawIndirectCountKHR>,
    pub draw_indirect_count_amd: Option<vkCmdDrawIndirectCountAMD>,
    pub draw_indexed_indirect_count: Option<vkCmdDrawIndexedIndirectCount>,
    pub draw_indexed_indirect_count_khr: Option<vkCmdDrawIndexedIndirectCountKHR>,
    pub draw_indexed_indirect_count_amd: Option<vkCmdDrawIndexedIndirectCountAMD>,
}

impl CommandBufferFnv1_2 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_render_pass2: to_option(loader(c"vkCmdBeginRenderPass2")),
            begin_render_pass2_khr: to_option(loader(c"vkCmdBeginRenderPass2KHR")),
            next_subpass2: to_option(loader(c"vkCmdNextSubpass2")),
            next_subpass2_khr: to_option(loader(c"vkCmdNextSubpass2KHR")),
            end_render_pass2: to_option(loader(c"vkCmdEndRenderPass2")),
            end_render_pass2_khr: to_option(loader(c"vkCmdEndRenderPass2KHR")),
            draw_indirect_count: to_option(loader(c"vkCmdDrawIndirectCount")),
            draw_indirect_count_khr: to_option(loader(c"vkCmdDrawIndirectCountKHR")),
            draw_indirect_count_amd: to_option(loader(c"vkCmdDrawIndirectCountAMD")),
            draw_indexed_indirect_count: to_option(loader(c"vkCmdDrawIndexedIndirectCount")),
            draw_indexed_indirect_count_khr: to_option(loader(c"vkCmdDrawIndexedIndirectCountKHR")),
            draw_indexed_indirect_count_amd: to_option(loader(c"vkCmdDrawIndexedIndirectCountAMD")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_3 {
    pub set_cull_mode: Option<vkCmdSetCullMode>,
    pub set_cull_mode_ext: Option<vkCmdSetCullModeEXT>,
    pub set_front_face: Option<vkCmdSetFrontFace>,
    pub set_front_face_ext: Option<vkCmdSetFrontFaceEXT>,
    pub set_primitive_topology: Option<vkCmdSetPrimitiveTopology>,
    pub set_primitive_topology_ext: Option<vkCmdSetPrimitiveTopologyEXT>,
    pub set_viewport_with_count: Option<vkCmdSetViewportWithCount>,
    pub set_viewport_with_count_ext: Option<vkCmdSetViewportWithCountEXT>,
    pub set_scissor_with_count: Option<vkCmdSetScissorWithCount>,
    pub set_scissor_with_count_ext: Option<vkCmdSetScissorWithCountEXT>,
    pub bind_vertex_buffers2: Option<vkCmdBindVertexBuffers2>,
    pub bind_vertex_buffers2_ext: Option<vkCmdBindVertexBuffers2EXT>,
    pub set_depth_test_enable: Option<vkCmdSetDepthTestEnable>,
    pub set_depth_test_enable_ext: Option<vkCmdSetDepthTestEnableEXT>,
    pub set_depth_write_enable: Option<vkCmdSetDepthWriteEnable>,
    pub set_depth_write_enable_ext: Option<vkCmdSetDepthWriteEnableEXT>,
    pub set_depth_compare_op: Option<vkCmdSetDepthCompareOp>,
    pub set_depth_compare_op_ext: Option<vkCmdSetDepthCompareOpEXT>,
    pub set_depth_bounds_test_enable: Option<vkCmdSetDepthBoundsTestEnable>,
    pub set_depth_bounds_test_enable_ext: Option<vkCmdSetDepthBoundsTestEnableEXT>,
    pub set_stencil_test_enable: Option<vkCmdSetStencilTestEnable>,
    pub set_stencil_test_enable_ext: Option<vkCmdSetStencilTestEnableEXT>,
    pub set_stencil_op: Option<vkCmdSetStencilOp>,
    pub set_stencil_op_ext: Option<vkCmdSetStencilOpEXT>,
    pub set_rasterizer_discard_enable: Option<vkCmdSetRasterizerDiscardEnable>,
    pub set_rasterizer_discard_enable_ext: Option<vkCmdSetRasterizerDiscardEnableEXT>,
    pub set_depth_bias_enable: Option<vkCmdSetDepthBiasEnable>,
    pub set_depth_bias_enable_ext: Option<vkCmdSetDepthBiasEnableEXT>,
    pub set_primitive_restart_enable: Option<vkCmdSetPrimitiveRestartEnable>,
    pub set_primitive_restart_enable_ext: Option<vkCmdSetPrimitiveRestartEnableEXT>,
    pub copy_buffer2: Option<vkCmdCopyBuffer2>,
    pub copy_buffer2_khr: Option<vkCmdCopyBuffer2KHR>,
    pub copy_image2: Option<vkCmdCopyImage2>,
    pub copy_image2_khr: Option<vkCmdCopyImage2KHR>,
    pub blit_image2: Option<vkCmdBlitImage2>,
    pub blit_image2_khr: Option<vkCmdBlitImage2KHR>,
    pub copy_buffer_to_image2: Option<vkCmdCopyBufferToImage2>,
    pub copy_buffer_to_image2_khr: Option<vkCmdCopyBufferToImage2KHR>,
    pub copy_image_to_buffer2: Option<vkCmdCopyImageToBuffer2>,
    pub copy_image_to_buffer2_khr: Option<vkCmdCopyImageToBuffer2KHR>,
    pub resolve_image2: Option<vkCmdResolveImage2>,
    pub resolve_image2_khr: Option<vkCmdResolveImage2KHR>,
    pub set_event2: Option<vkCmdSetEvent2>,
    pub set_event2_khr: Option<vkCmdSetEvent2KHR>,
    pub reset_event2: Option<vkCmdResetEvent2>,
    pub reset_event2_khr: Option<vkCmdResetEvent2KHR>,
    pub wait_events2: Option<vkCmdWaitEvents2>,
    pub wait_events2_khr: Option<vkCmdWaitEvents2KHR>,
    pub pipeline_barrier2: Option<vkCmdPipelineBarrier2>,
    pub pipeline_barrier2_khr: Option<vkCmdPipelineBarrier2KHR>,
    pub write_timestamp2: Option<vkCmdWriteTimestamp2>,
    pub write_timestamp2_khr: Option<vkCmdWriteTimestamp2KHR>,
    pub begin_rendering: Option<vkCmdBeginRendering>,
    pub begin_rendering_khr: Option<vkCmdBeginRenderingKHR>,
    pub end_rendering: Option<vkCmdEndRendering>,
    pub end_rendering_khr: Option<vkCmdEndRenderingKHR>,
}

impl CommandBufferFnv1_3 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_cull_mode: to_option(loader(c"vkCmdSetCullMode")),
            set_cull_mode_ext: to_option(loader(c"vkCmdSetCullModeEXT")),
            set_front_face: to_option(loader(c"vkCmdSetFrontFace")),
            set_front_face_ext: to_option(loader(c"vkCmdSetFrontFaceEXT")),
            set_primitive_topology: to_option(loader(c"vkCmdSetPrimitiveTopology")),
            set_primitive_topology_ext: to_option(loader(c"vkCmdSetPrimitiveTopologyEXT")),
            set_viewport_with_count: to_option(loader(c"vkCmdSetViewportWithCount")),
            set_viewport_with_count_ext: to_option(loader(c"vkCmdSetViewportWithCountEXT")),
            set_scissor_with_count: to_option(loader(c"vkCmdSetScissorWithCount")),
            set_scissor_with_count_ext: to_option(loader(c"vkCmdSetScissorWithCountEXT")),
            bind_vertex_buffers2: to_option(loader(c"vkCmdBindVertexBuffers2")),
            bind_vertex_buffers2_ext: to_option(loader(c"vkCmdBindVertexBuffers2EXT")),
            set_depth_test_enable: to_option(loader(c"vkCmdSetDepthTestEnable")),
            set_depth_test_enable_ext: to_option(loader(c"vkCmdSetDepthTestEnableEXT")),
            set_depth_write_enable: to_option(loader(c"vkCmdSetDepthWriteEnable")),
            set_depth_write_enable_ext: to_option(loader(c"vkCmdSetDepthWriteEnableEXT")),
            set_depth_compare_op: to_option(loader(c"vkCmdSetDepthCompareOp")),
            set_depth_compare_op_ext: to_option(loader(c"vkCmdSetDepthCompareOpEXT")),
            set_depth_bounds_test_enable: to_option(loader(c"vkCmdSetDepthBoundsTestEnable")),
            set_depth_bounds_test_enable_ext: to_option(loader(
                c"vkCmdSetDepthBoundsTestEnableEXT",
            )),
            set_stencil_test_enable: to_option(loader(c"vkCmdSetStencilTestEnable")),
            set_stencil_test_enable_ext: to_option(loader(c"vkCmdSetStencilTestEnableEXT")),
            set_stencil_op: to_option(loader(c"vkCmdSetStencilOp")),
            set_stencil_op_ext: to_option(loader(c"vkCmdSetStencilOpEXT")),
            set_rasterizer_discard_enable: to_option(loader(c"vkCmdSetRasterizerDiscardEnable")),
            set_rasterizer_discard_enable_ext: to_option(loader(
                c"vkCmdSetRasterizerDiscardEnableEXT",
            )),
            set_depth_bias_enable: to_option(loader(c"vkCmdSetDepthBiasEnable")),
            set_depth_bias_enable_ext: to_option(loader(c"vkCmdSetDepthBiasEnableEXT")),
            set_primitive_restart_enable: to_option(loader(c"vkCmdSetPrimitiveRestartEnable")),
            set_primitive_restart_enable_ext: to_option(loader(
                c"vkCmdSetPrimitiveRestartEnableEXT",
            )),
            copy_buffer2: to_option(loader(c"vkCmdCopyBuffer2")),
            copy_buffer2_khr: to_option(loader(c"vkCmdCopyBuffer2KHR")),
            copy_image2: to_option(loader(c"vkCmdCopyImage2")),
            copy_image2_khr: to_option(loader(c"vkCmdCopyImage2KHR")),
            blit_image2: to_option(loader(c"vkCmdBlitImage2")),
            blit_image2_khr: to_option(loader(c"vkCmdBlitImage2KHR")),
            copy_buffer_to_image2: to_option(loader(c"vkCmdCopyBufferToImage2")),
            copy_buffer_to_image2_khr: to_option(loader(c"vkCmdCopyBufferToImage2KHR")),
            copy_image_to_buffer2: to_option(loader(c"vkCmdCopyImageToBuffer2")),
            copy_image_to_buffer2_khr: to_option(loader(c"vkCmdCopyImageToBuffer2KHR")),
            resolve_image2: to_option(loader(c"vkCmdResolveImage2")),
            resolve_image2_khr: to_option(loader(c"vkCmdResolveImage2KHR")),
            set_event2: to_option(loader(c"vkCmdSetEvent2")),
            set_event2_khr: to_option(loader(c"vkCmdSetEvent2KHR")),
            reset_event2: to_option(loader(c"vkCmdResetEvent2")),
            reset_event2_khr: to_option(loader(c"vkCmdResetEvent2KHR")),
            wait_events2: to_option(loader(c"vkCmdWaitEvents2")),
            wait_events2_khr: to_option(loader(c"vkCmdWaitEvents2KHR")),
            pipeline_barrier2: to_option(loader(c"vkCmdPipelineBarrier2")),
            pipeline_barrier2_khr: to_option(loader(c"vkCmdPipelineBarrier2KHR")),
            write_timestamp2: to_option(loader(c"vkCmdWriteTimestamp2")),
            write_timestamp2_khr: to_option(loader(c"vkCmdWriteTimestamp2KHR")),
            begin_rendering: to_option(loader(c"vkCmdBeginRendering")),
            begin_rendering_khr: to_option(loader(c"vkCmdBeginRenderingKHR")),
            end_rendering: to_option(loader(c"vkCmdEndRendering")),
            end_rendering_khr: to_option(loader(c"vkCmdEndRenderingKHR")),
        }
    }
}

#[derive(Clone, Default)]
pub struct CommandBufferFnv1_4 {
    pub push_descriptor_set: Option<vkCmdPushDescriptorSet>,
    pub push_descriptor_set_khr: Option<vkCmdPushDescriptorSetKHR>,
    pub push_descriptor_set_with_template: Option<vkCmdPushDescriptorSetWithTemplate>,
    pub push_descriptor_set_with_template_khr: Option<vkCmdPushDescriptorSetWithTemplateKHR>,
    pub set_line_stipple: Option<vkCmdSetLineStipple>,
    pub set_line_stipple_khr: Option<vkCmdSetLineStippleKHR>,
    pub set_line_stipple_ext: Option<vkCmdSetLineStippleEXT>,
    pub bind_index_buffer2: Option<vkCmdBindIndexBuffer2>,
    pub bind_index_buffer2_khr: Option<vkCmdBindIndexBuffer2KHR>,
    pub bind_descriptor_sets2: Option<vkCmdBindDescriptorSets2>,
    pub bind_descriptor_sets2_khr: Option<vkCmdBindDescriptorSets2KHR>,
    pub push_constants2: Option<vkCmdPushConstants2>,
    pub push_constants2_khr: Option<vkCmdPushConstants2KHR>,
    pub push_descriptor_set2: Option<vkCmdPushDescriptorSet2>,
    pub push_descriptor_set2_khr: Option<vkCmdPushDescriptorSet2KHR>,
    pub push_descriptor_set_with_template2: Option<vkCmdPushDescriptorSetWithTemplate2>,
    pub push_descriptor_set_with_template2_khr: Option<vkCmdPushDescriptorSetWithTemplate2KHR>,
    pub set_rendering_attachment_locations: Option<vkCmdSetRenderingAttachmentLocations>,
    pub set_rendering_attachment_locations_khr: Option<vkCmdSetRenderingAttachmentLocationsKHR>,
    pub set_rendering_input_attachment_indices: Option<vkCmdSetRenderingInputAttachmentIndices>,
    pub set_rendering_input_attachment_indices_khr:
        Option<vkCmdSetRenderingInputAttachmentIndicesKHR>,
}

impl CommandBufferFnv1_4 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            push_descriptor_set: to_option(loader(c"vkCmdPushDescriptorSet")),
            push_descriptor_set_khr: to_option(loader(c"vkCmdPushDescriptorSetKHR")),
            push_descriptor_set_with_template: to_option(loader(
                c"vkCmdPushDescriptorSetWithTemplate",
            )),
            push_descriptor_set_with_template_khr: to_option(loader(
                c"vkCmdPushDescriptorSetWithTemplateKHR",
            )),
            set_line_stipple: to_option(loader(c"vkCmdSetLineStipple")),
            set_line_stipple_khr: to_option(loader(c"vkCmdSetLineStippleKHR")),
            set_line_stipple_ext: to_option(loader(c"vkCmdSetLineStippleEXT")),
            bind_index_buffer2: to_option(loader(c"vkCmdBindIndexBuffer2")),
            bind_index_buffer2_khr: to_option(loader(c"vkCmdBindIndexBuffer2KHR")),
            bind_descriptor_sets2: to_option(loader(c"vkCmdBindDescriptorSets2")),
            bind_descriptor_sets2_khr: to_option(loader(c"vkCmdBindDescriptorSets2KHR")),
            push_constants2: to_option(loader(c"vkCmdPushConstants2")),
            push_constants2_khr: to_option(loader(c"vkCmdPushConstants2KHR")),
            push_descriptor_set2: to_option(loader(c"vkCmdPushDescriptorSet2")),
            push_descriptor_set2_khr: to_option(loader(c"vkCmdPushDescriptorSet2KHR")),
            push_descriptor_set_with_template2: to_option(loader(
                c"vkCmdPushDescriptorSetWithTemplate2",
            )),
            push_descriptor_set_with_template2_khr: to_option(loader(
                c"vkCmdPushDescriptorSetWithTemplate2KHR",
            )),
            set_rendering_attachment_locations: to_option(loader(
                c"vkCmdSetRenderingAttachmentLocations",
            )),
            set_rendering_attachment_locations_khr: to_option(loader(
                c"vkCmdSetRenderingAttachmentLocationsKHR",
            )),
            set_rendering_input_attachment_indices: to_option(loader(
                c"vkCmdSetRenderingInputAttachmentIndices",
            )),
            set_rendering_input_attachment_indices_khr: to_option(loader(
                c"vkCmdSetRenderingInputAttachmentIndicesKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnAmdBufferMarker {
    pub write_buffer_marker_amd: vkCmdWriteBufferMarkerAMD,
    pub write_buffer_marker2_amd: vkCmdWriteBufferMarker2AMD,
}

impl CommandBufferFnAmdBufferMarker {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            write_buffer_marker_amd: to_panic(loader(c"vkCmdWriteBufferMarkerAMD")),
            write_buffer_marker2_amd: to_panic(loader(c"vkCmdWriteBufferMarker2AMD")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnAmdxShaderEnqueue {
    pub initialize_graph_scratch_memory_amdx: vkCmdInitializeGraphScratchMemoryAMDX,
    pub dispatch_graph_amdx: vkCmdDispatchGraphAMDX,
    pub dispatch_graph_indirect_amdx: vkCmdDispatchGraphIndirectAMDX,
    pub dispatch_graph_indirect_count_amdx: vkCmdDispatchGraphIndirectCountAMDX,
}

impl CommandBufferFnAmdxShaderEnqueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            initialize_graph_scratch_memory_amdx: to_panic(loader(
                c"vkCmdInitializeGraphScratchMemoryAMDX",
            )),
            dispatch_graph_amdx: to_panic(loader(c"vkCmdDispatchGraphAMDX")),
            dispatch_graph_indirect_amdx: to_panic(loader(c"vkCmdDispatchGraphIndirectAMDX")),
            dispatch_graph_indirect_count_amdx: to_panic(loader(
                c"vkCmdDispatchGraphIndirectCountAMDX",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnArmDataGraph {
    pub dispatch_data_graph_arm: vkCmdDispatchDataGraphARM,
}

impl CommandBufferFnArmDataGraph {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            dispatch_data_graph_arm: to_panic(loader(c"vkCmdDispatchDataGraphARM")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnArmTensors {
    pub copy_tensor_arm: vkCmdCopyTensorARM,
}

impl CommandBufferFnArmTensors {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            copy_tensor_arm: to_panic(loader(c"vkCmdCopyTensorARM")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtAttachmentFeedbackLoopDynamicState {
    pub set_attachment_feedback_loop_enable_ext: vkCmdSetAttachmentFeedbackLoopEnableEXT,
}

impl CommandBufferFnExtAttachmentFeedbackLoopDynamicState {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_attachment_feedback_loop_enable_ext: to_panic(loader(
                c"vkCmdSetAttachmentFeedbackLoopEnableEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtColorWriteEnable {
    pub set_color_write_enable_ext: vkCmdSetColorWriteEnableEXT,
}

impl CommandBufferFnExtColorWriteEnable {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_color_write_enable_ext: to_panic(loader(c"vkCmdSetColorWriteEnableEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtConditionalRendering {
    pub begin_conditional_rendering_ext: vkCmdBeginConditionalRenderingEXT,
    pub end_conditional_rendering_ext: vkCmdEndConditionalRenderingEXT,
}

impl CommandBufferFnExtConditionalRendering {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_conditional_rendering_ext: to_panic(loader(c"vkCmdBeginConditionalRenderingEXT")),
            end_conditional_rendering_ext: to_panic(loader(c"vkCmdEndConditionalRenderingEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtCustomResolve {
    pub begin_custom_resolve_ext: vkCmdBeginCustomResolveEXT,
}

impl CommandBufferFnExtCustomResolve {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_custom_resolve_ext: to_panic(loader(c"vkCmdBeginCustomResolveEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDebugMarker {
    pub debug_marker_begin_ext: vkCmdDebugMarkerBeginEXT,
    pub debug_marker_end_ext: vkCmdDebugMarkerEndEXT,
    pub debug_marker_insert_ext: vkCmdDebugMarkerInsertEXT,
}

impl CommandBufferFnExtDebugMarker {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            debug_marker_begin_ext: to_panic(loader(c"vkCmdDebugMarkerBeginEXT")),
            debug_marker_end_ext: to_panic(loader(c"vkCmdDebugMarkerEndEXT")),
            debug_marker_insert_ext: to_panic(loader(c"vkCmdDebugMarkerInsertEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDebugUtils {
    pub begin_debug_utils_label_ext: vkCmdBeginDebugUtilsLabelEXT,
    pub end_debug_utils_label_ext: vkCmdEndDebugUtilsLabelEXT,
    pub insert_debug_utils_label_ext: vkCmdInsertDebugUtilsLabelEXT,
}

impl CommandBufferFnExtDebugUtils {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_debug_utils_label_ext: to_panic(loader(c"vkCmdBeginDebugUtilsLabelEXT")),
            end_debug_utils_label_ext: to_panic(loader(c"vkCmdEndDebugUtilsLabelEXT")),
            insert_debug_utils_label_ext: to_panic(loader(c"vkCmdInsertDebugUtilsLabelEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDepthBiasControl {
    pub set_depth_bias2_ext: vkCmdSetDepthBias2EXT,
}

impl CommandBufferFnExtDepthBiasControl {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_depth_bias2_ext: to_panic(loader(c"vkCmdSetDepthBias2EXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDepthClampControl {
    pub set_depth_clamp_range_ext: vkCmdSetDepthClampRangeEXT,
}

impl CommandBufferFnExtDepthClampControl {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_depth_clamp_range_ext: to_panic(loader(c"vkCmdSetDepthClampRangeEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDescriptorBuffer {
    pub bind_descriptor_buffers_ext: vkCmdBindDescriptorBuffersEXT,
    pub set_descriptor_buffer_offsets_ext: vkCmdSetDescriptorBufferOffsetsEXT,
    pub bind_descriptor_buffer_embedded_samplers_ext: vkCmdBindDescriptorBufferEmbeddedSamplersEXT,
}

impl CommandBufferFnExtDescriptorBuffer {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            bind_descriptor_buffers_ext: to_panic(loader(c"vkCmdBindDescriptorBuffersEXT")),
            set_descriptor_buffer_offsets_ext: to_panic(loader(
                c"vkCmdSetDescriptorBufferOffsetsEXT",
            )),
            bind_descriptor_buffer_embedded_samplers_ext: to_panic(loader(
                c"vkCmdBindDescriptorBufferEmbeddedSamplersEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDescriptorHeap {
    pub bind_sampler_heap_ext: vkCmdBindSamplerHeapEXT,
    pub bind_resource_heap_ext: vkCmdBindResourceHeapEXT,
    pub push_data_ext: vkCmdPushDataEXT,
}

impl CommandBufferFnExtDescriptorHeap {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            bind_sampler_heap_ext: to_panic(loader(c"vkCmdBindSamplerHeapEXT")),
            bind_resource_heap_ext: to_panic(loader(c"vkCmdBindResourceHeapEXT")),
            push_data_ext: to_panic(loader(c"vkCmdPushDataEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDeviceGeneratedCommands {
    pub execute_generated_commands_ext: vkCmdExecuteGeneratedCommandsEXT,
    pub preprocess_generated_commands_ext: vkCmdPreprocessGeneratedCommandsEXT,
}

impl CommandBufferFnExtDeviceGeneratedCommands {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            execute_generated_commands_ext: to_panic(loader(c"vkCmdExecuteGeneratedCommandsEXT")),
            preprocess_generated_commands_ext: to_panic(loader(
                c"vkCmdPreprocessGeneratedCommandsEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtDiscardRectangles {
    pub set_discard_rectangle_ext: vkCmdSetDiscardRectangleEXT,
    pub set_discard_rectangle_enable_ext: vkCmdSetDiscardRectangleEnableEXT,
    pub set_discard_rectangle_mode_ext: vkCmdSetDiscardRectangleModeEXT,
}

impl CommandBufferFnExtDiscardRectangles {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_discard_rectangle_ext: to_panic(loader(c"vkCmdSetDiscardRectangleEXT")),
            set_discard_rectangle_enable_ext: to_panic(loader(
                c"vkCmdSetDiscardRectangleEnableEXT",
            )),
            set_discard_rectangle_mode_ext: to_panic(loader(c"vkCmdSetDiscardRectangleModeEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtMemoryDecompression {
    pub decompress_memory_ext: vkCmdDecompressMemoryEXT,
    pub decompress_memory_indirect_count_ext: vkCmdDecompressMemoryIndirectCountEXT,
}

impl CommandBufferFnExtMemoryDecompression {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            decompress_memory_ext: to_panic(loader(c"vkCmdDecompressMemoryEXT")),
            decompress_memory_indirect_count_ext: to_panic(loader(
                c"vkCmdDecompressMemoryIndirectCountEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtMeshShader {
    pub draw_mesh_tasks_ext: vkCmdDrawMeshTasksEXT,
    pub draw_mesh_tasks_indirect_ext: vkCmdDrawMeshTasksIndirectEXT,
    pub draw_mesh_tasks_indirect_count_ext: vkCmdDrawMeshTasksIndirectCountEXT,
}

impl CommandBufferFnExtMeshShader {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            draw_mesh_tasks_ext: to_panic(loader(c"vkCmdDrawMeshTasksEXT")),
            draw_mesh_tasks_indirect_ext: to_panic(loader(c"vkCmdDrawMeshTasksIndirectEXT")),
            draw_mesh_tasks_indirect_count_ext: to_panic(loader(
                c"vkCmdDrawMeshTasksIndirectCountEXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtMultiDraw {
    pub draw_multi_ext: vkCmdDrawMultiEXT,
    pub draw_multi_indexed_ext: vkCmdDrawMultiIndexedEXT,
}

impl CommandBufferFnExtMultiDraw {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            draw_multi_ext: to_panic(loader(c"vkCmdDrawMultiEXT")),
            draw_multi_indexed_ext: to_panic(loader(c"vkCmdDrawMultiIndexedEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtOpacityMicromap {
    pub build_micromaps_ext: vkCmdBuildMicromapsEXT,
    pub copy_micromap_ext: vkCmdCopyMicromapEXT,
    pub copy_micromap_to_memory_ext: vkCmdCopyMicromapToMemoryEXT,
    pub copy_memory_to_micromap_ext: vkCmdCopyMemoryToMicromapEXT,
    pub write_micromaps_properties_ext: vkCmdWriteMicromapsPropertiesEXT,
}

impl CommandBufferFnExtOpacityMicromap {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            build_micromaps_ext: to_panic(loader(c"vkCmdBuildMicromapsEXT")),
            copy_micromap_ext: to_panic(loader(c"vkCmdCopyMicromapEXT")),
            copy_micromap_to_memory_ext: to_panic(loader(c"vkCmdCopyMicromapToMemoryEXT")),
            copy_memory_to_micromap_ext: to_panic(loader(c"vkCmdCopyMemoryToMicromapEXT")),
            write_micromaps_properties_ext: to_panic(loader(c"vkCmdWriteMicromapsPropertiesEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtSampleLocations {
    pub set_sample_locations_ext: vkCmdSetSampleLocationsEXT,
}

impl CommandBufferFnExtSampleLocations {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_sample_locations_ext: to_panic(loader(c"vkCmdSetSampleLocationsEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtShaderObject {
    pub set_patch_control_points_ext: vkCmdSetPatchControlPointsEXT,
    pub set_logic_op_ext: vkCmdSetLogicOpEXT,
    pub set_tessellation_domain_origin_ext: vkCmdSetTessellationDomainOriginEXT,
    pub set_depth_clamp_enable_ext: vkCmdSetDepthClampEnableEXT,
    pub set_polygon_mode_ext: vkCmdSetPolygonModeEXT,
    pub set_rasterization_samples_ext: vkCmdSetRasterizationSamplesEXT,
    pub set_sample_mask_ext: vkCmdSetSampleMaskEXT,
    pub set_alpha_to_coverage_enable_ext: vkCmdSetAlphaToCoverageEnableEXT,
    pub set_alpha_to_one_enable_ext: vkCmdSetAlphaToOneEnableEXT,
    pub set_logic_op_enable_ext: vkCmdSetLogicOpEnableEXT,
    pub set_color_blend_enable_ext: vkCmdSetColorBlendEnableEXT,
    pub set_color_blend_equation_ext: vkCmdSetColorBlendEquationEXT,
    pub set_color_write_mask_ext: vkCmdSetColorWriteMaskEXT,
    pub set_rasterization_stream_ext: vkCmdSetRasterizationStreamEXT,
    pub set_conservative_rasterization_mode_ext: vkCmdSetConservativeRasterizationModeEXT,
    pub set_extra_primitive_overestimation_size_ext: vkCmdSetExtraPrimitiveOverestimationSizeEXT,
    pub set_depth_clip_enable_ext: vkCmdSetDepthClipEnableEXT,
    pub set_sample_locations_enable_ext: vkCmdSetSampleLocationsEnableEXT,
    pub set_color_blend_advanced_ext: vkCmdSetColorBlendAdvancedEXT,
    pub set_provoking_vertex_mode_ext: vkCmdSetProvokingVertexModeEXT,
    pub set_line_rasterization_mode_ext: vkCmdSetLineRasterizationModeEXT,
    pub set_line_stipple_enable_ext: vkCmdSetLineStippleEnableEXT,
    pub set_depth_clip_negative_one_to_one_ext: vkCmdSetDepthClipNegativeOneToOneEXT,
    pub set_viewport_w_scaling_enable_nv: vkCmdSetViewportWScalingEnableNV,
    pub set_viewport_swizzle_nv: vkCmdSetViewportSwizzleNV,
    pub set_coverage_to_color_enable_nv: vkCmdSetCoverageToColorEnableNV,
    pub set_coverage_to_color_location_nv: vkCmdSetCoverageToColorLocationNV,
    pub set_coverage_modulation_mode_nv: vkCmdSetCoverageModulationModeNV,
    pub set_coverage_modulation_table_enable_nv: vkCmdSetCoverageModulationTableEnableNV,
    pub set_coverage_modulation_table_nv: vkCmdSetCoverageModulationTableNV,
    pub set_shading_rate_image_enable_nv: vkCmdSetShadingRateImageEnableNV,
    pub set_coverage_reduction_mode_nv: vkCmdSetCoverageReductionModeNV,
    pub set_representative_fragment_test_enable_nv: vkCmdSetRepresentativeFragmentTestEnableNV,
    pub set_vertex_input_ext: vkCmdSetVertexInputEXT,
    pub bind_shaders_ext: vkCmdBindShadersEXT,
}

impl CommandBufferFnExtShaderObject {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_patch_control_points_ext: to_panic(loader(c"vkCmdSetPatchControlPointsEXT")),
            set_logic_op_ext: to_panic(loader(c"vkCmdSetLogicOpEXT")),
            set_tessellation_domain_origin_ext: to_panic(loader(
                c"vkCmdSetTessellationDomainOriginEXT",
            )),
            set_depth_clamp_enable_ext: to_panic(loader(c"vkCmdSetDepthClampEnableEXT")),
            set_polygon_mode_ext: to_panic(loader(c"vkCmdSetPolygonModeEXT")),
            set_rasterization_samples_ext: to_panic(loader(c"vkCmdSetRasterizationSamplesEXT")),
            set_sample_mask_ext: to_panic(loader(c"vkCmdSetSampleMaskEXT")),
            set_alpha_to_coverage_enable_ext: to_panic(loader(c"vkCmdSetAlphaToCoverageEnableEXT")),
            set_alpha_to_one_enable_ext: to_panic(loader(c"vkCmdSetAlphaToOneEnableEXT")),
            set_logic_op_enable_ext: to_panic(loader(c"vkCmdSetLogicOpEnableEXT")),
            set_color_blend_enable_ext: to_panic(loader(c"vkCmdSetColorBlendEnableEXT")),
            set_color_blend_equation_ext: to_panic(loader(c"vkCmdSetColorBlendEquationEXT")),
            set_color_write_mask_ext: to_panic(loader(c"vkCmdSetColorWriteMaskEXT")),
            set_rasterization_stream_ext: to_panic(loader(c"vkCmdSetRasterizationStreamEXT")),
            set_conservative_rasterization_mode_ext: to_panic(loader(
                c"vkCmdSetConservativeRasterizationModeEXT",
            )),
            set_extra_primitive_overestimation_size_ext: to_panic(loader(
                c"vkCmdSetExtraPrimitiveOverestimationSizeEXT",
            )),
            set_depth_clip_enable_ext: to_panic(loader(c"vkCmdSetDepthClipEnableEXT")),
            set_sample_locations_enable_ext: to_panic(loader(c"vkCmdSetSampleLocationsEnableEXT")),
            set_color_blend_advanced_ext: to_panic(loader(c"vkCmdSetColorBlendAdvancedEXT")),
            set_provoking_vertex_mode_ext: to_panic(loader(c"vkCmdSetProvokingVertexModeEXT")),
            set_line_rasterization_mode_ext: to_panic(loader(c"vkCmdSetLineRasterizationModeEXT")),
            set_line_stipple_enable_ext: to_panic(loader(c"vkCmdSetLineStippleEnableEXT")),
            set_depth_clip_negative_one_to_one_ext: to_panic(loader(
                c"vkCmdSetDepthClipNegativeOneToOneEXT",
            )),
            set_viewport_w_scaling_enable_nv: to_panic(loader(c"vkCmdSetViewportWScalingEnableNV")),
            set_viewport_swizzle_nv: to_panic(loader(c"vkCmdSetViewportSwizzleNV")),
            set_coverage_to_color_enable_nv: to_panic(loader(c"vkCmdSetCoverageToColorEnableNV")),
            set_coverage_to_color_location_nv: to_panic(loader(
                c"vkCmdSetCoverageToColorLocationNV",
            )),
            set_coverage_modulation_mode_nv: to_panic(loader(c"vkCmdSetCoverageModulationModeNV")),
            set_coverage_modulation_table_enable_nv: to_panic(loader(
                c"vkCmdSetCoverageModulationTableEnableNV",
            )),
            set_coverage_modulation_table_nv: to_panic(loader(
                c"vkCmdSetCoverageModulationTableNV",
            )),
            set_shading_rate_image_enable_nv: to_panic(loader(c"vkCmdSetShadingRateImageEnableNV")),
            set_coverage_reduction_mode_nv: to_panic(loader(c"vkCmdSetCoverageReductionModeNV")),
            set_representative_fragment_test_enable_nv: to_panic(loader(
                c"vkCmdSetRepresentativeFragmentTestEnableNV",
            )),
            set_vertex_input_ext: to_panic(loader(c"vkCmdSetVertexInputEXT")),
            bind_shaders_ext: to_panic(loader(c"vkCmdBindShadersEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnExtTransformFeedback {
    pub bind_transform_feedback_buffers_ext: vkCmdBindTransformFeedbackBuffersEXT,
    pub begin_transform_feedback_ext: vkCmdBeginTransformFeedbackEXT,
    pub end_transform_feedback_ext: vkCmdEndTransformFeedbackEXT,
    pub begin_query_indexed_ext: vkCmdBeginQueryIndexedEXT,
    pub end_query_indexed_ext: vkCmdEndQueryIndexedEXT,
    pub draw_indirect_byte_count_ext: vkCmdDrawIndirectByteCountEXT,
}

impl CommandBufferFnExtTransformFeedback {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            bind_transform_feedback_buffers_ext: to_panic(loader(
                c"vkCmdBindTransformFeedbackBuffersEXT",
            )),
            begin_transform_feedback_ext: to_panic(loader(c"vkCmdBeginTransformFeedbackEXT")),
            end_transform_feedback_ext: to_panic(loader(c"vkCmdEndTransformFeedbackEXT")),
            begin_query_indexed_ext: to_panic(loader(c"vkCmdBeginQueryIndexedEXT")),
            end_query_indexed_ext: to_panic(loader(c"vkCmdEndQueryIndexedEXT")),
            draw_indirect_byte_count_ext: to_panic(loader(c"vkCmdDrawIndirectByteCountEXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnHuaweiClusterCullingShader {
    pub draw_cluster_huawei: vkCmdDrawClusterHUAWEI,
    pub draw_cluster_indirect_huawei: vkCmdDrawClusterIndirectHUAWEI,
}

impl CommandBufferFnHuaweiClusterCullingShader {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            draw_cluster_huawei: to_panic(loader(c"vkCmdDrawClusterHUAWEI")),
            draw_cluster_indirect_huawei: to_panic(loader(c"vkCmdDrawClusterIndirectHUAWEI")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnHuaweiInvocationMask {
    pub bind_invocation_mask_huawei: vkCmdBindInvocationMaskHUAWEI,
}

impl CommandBufferFnHuaweiInvocationMask {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            bind_invocation_mask_huawei: to_panic(loader(c"vkCmdBindInvocationMaskHUAWEI")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnHuaweiSubpassShading {
    pub subpass_shading_huawei: vkCmdSubpassShadingHUAWEI,
}

impl CommandBufferFnHuaweiSubpassShading {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            subpass_shading_huawei: to_panic(loader(c"vkCmdSubpassShadingHUAWEI")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnIntelPerformanceQuery {
    pub set_performance_marker_intel: vkCmdSetPerformanceMarkerINTEL,
    pub set_performance_stream_marker_intel: vkCmdSetPerformanceStreamMarkerINTEL,
    pub set_performance_override_intel: vkCmdSetPerformanceOverrideINTEL,
}

impl CommandBufferFnIntelPerformanceQuery {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_performance_marker_intel: to_panic(loader(c"vkCmdSetPerformanceMarkerINTEL")),
            set_performance_stream_marker_intel: to_panic(loader(
                c"vkCmdSetPerformanceStreamMarkerINTEL",
            )),
            set_performance_override_intel: to_panic(loader(c"vkCmdSetPerformanceOverrideINTEL")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrAccelerationStructure {
    pub copy_acceleration_structure_khr: vkCmdCopyAccelerationStructureKHR,
    pub copy_acceleration_structure_to_memory_khr: vkCmdCopyAccelerationStructureToMemoryKHR,
    pub copy_memory_to_acceleration_structure_khr: vkCmdCopyMemoryToAccelerationStructureKHR,
    pub write_acceleration_structures_properties_khr: vkCmdWriteAccelerationStructuresPropertiesKHR,
    pub build_acceleration_structures_khr: vkCmdBuildAccelerationStructuresKHR,
    pub build_acceleration_structures_indirect_khr: vkCmdBuildAccelerationStructuresIndirectKHR,
}

impl CommandBufferFnKhrAccelerationStructure {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            copy_acceleration_structure_khr: to_panic(loader(c"vkCmdCopyAccelerationStructureKHR")),
            copy_acceleration_structure_to_memory_khr: to_panic(loader(
                c"vkCmdCopyAccelerationStructureToMemoryKHR",
            )),
            copy_memory_to_acceleration_structure_khr: to_panic(loader(
                c"vkCmdCopyMemoryToAccelerationStructureKHR",
            )),
            write_acceleration_structures_properties_khr: to_panic(loader(
                c"vkCmdWriteAccelerationStructuresPropertiesKHR",
            )),
            build_acceleration_structures_khr: to_panic(loader(
                c"vkCmdBuildAccelerationStructuresKHR",
            )),
            build_acceleration_structures_indirect_khr: to_panic(loader(
                c"vkCmdBuildAccelerationStructuresIndirectKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrCopyMemoryIndirect {
    pub copy_memory_indirect_khr: vkCmdCopyMemoryIndirectKHR,
    pub copy_memory_to_image_indirect_khr: vkCmdCopyMemoryToImageIndirectKHR,
}

impl CommandBufferFnKhrCopyMemoryIndirect {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            copy_memory_indirect_khr: to_panic(loader(c"vkCmdCopyMemoryIndirectKHR")),
            copy_memory_to_image_indirect_khr: to_panic(loader(
                c"vkCmdCopyMemoryToImageIndirectKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrFragmentShadingRate {
    pub set_fragment_shading_rate_khr: vkCmdSetFragmentShadingRateKHR,
}

impl CommandBufferFnKhrFragmentShadingRate {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_fragment_shading_rate_khr: to_panic(loader(c"vkCmdSetFragmentShadingRateKHR")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrMaintenance10 {
    pub end_rendering2_khr: vkCmdEndRendering2KHR,
    pub end_rendering2_ext: vkCmdEndRendering2EXT,
}

impl CommandBufferFnKhrMaintenance10 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            end_rendering2_khr: to_panic(loader(c"vkCmdEndRendering2KHR")),
            end_rendering2_ext: to_panic(loader(c"vkCmdEndRendering2EXT")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrMaintenance6 {
    pub set_descriptor_buffer_offsets2_ext: vkCmdSetDescriptorBufferOffsets2EXT,
    pub bind_descriptor_buffer_embedded_samplers2_ext:
        vkCmdBindDescriptorBufferEmbeddedSamplers2EXT,
}

impl CommandBufferFnKhrMaintenance6 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_descriptor_buffer_offsets2_ext: to_panic(loader(
                c"vkCmdSetDescriptorBufferOffsets2EXT",
            )),
            bind_descriptor_buffer_embedded_samplers2_ext: to_panic(loader(
                c"vkCmdBindDescriptorBufferEmbeddedSamplers2EXT",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrObjectRefresh {
    pub refresh_objects_khr: vkCmdRefreshObjectsKHR,
}

impl CommandBufferFnKhrObjectRefresh {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            refresh_objects_khr: to_panic(loader(c"vkCmdRefreshObjectsKHR")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrRayTracingMaintenance1 {
    pub trace_rays_indirect2_khr: vkCmdTraceRaysIndirect2KHR,
}

impl CommandBufferFnKhrRayTracingMaintenance1 {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            trace_rays_indirect2_khr: to_panic(loader(c"vkCmdTraceRaysIndirect2KHR")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrRayTracingPipeline {
    pub trace_rays_khr: vkCmdTraceRaysKHR,
    pub trace_rays_indirect_khr: vkCmdTraceRaysIndirectKHR,
    pub set_ray_tracing_pipeline_stack_size_khr: vkCmdSetRayTracingPipelineStackSizeKHR,
}

impl CommandBufferFnKhrRayTracingPipeline {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            trace_rays_khr: to_panic(loader(c"vkCmdTraceRaysKHR")),
            trace_rays_indirect_khr: to_panic(loader(c"vkCmdTraceRaysIndirectKHR")),
            set_ray_tracing_pipeline_stack_size_khr: to_panic(loader(
                c"vkCmdSetRayTracingPipelineStackSizeKHR",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrVideoDecodeQueue {
    pub decode_video_khr: vkCmdDecodeVideoKHR,
}

impl CommandBufferFnKhrVideoDecodeQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            decode_video_khr: to_panic(loader(c"vkCmdDecodeVideoKHR")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrVideoEncodeQueue {
    pub encode_video_khr: vkCmdEncodeVideoKHR,
}

impl CommandBufferFnKhrVideoEncodeQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            encode_video_khr: to_panic(loader(c"vkCmdEncodeVideoKHR")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnKhrVideoQueue {
    pub begin_video_coding_khr: vkCmdBeginVideoCodingKHR,
    pub control_video_coding_khr: vkCmdControlVideoCodingKHR,
    pub end_video_coding_khr: vkCmdEndVideoCodingKHR,
}

impl CommandBufferFnKhrVideoQueue {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            begin_video_coding_khr: to_panic(loader(c"vkCmdBeginVideoCodingKHR")),
            control_video_coding_khr: to_panic(loader(c"vkCmdControlVideoCodingKHR")),
            end_video_coding_khr: to_panic(loader(c"vkCmdEndVideoCodingKHR")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvClipSpaceWScaling {
    pub set_viewport_w_scaling_nv: vkCmdSetViewportWScalingNV,
}

impl CommandBufferFnNvClipSpaceWScaling {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_viewport_w_scaling_nv: to_panic(loader(c"vkCmdSetViewportWScalingNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvClusterAccelerationStructure {
    pub build_cluster_acceleration_structure_indirect_nv:
        vkCmdBuildClusterAccelerationStructureIndirectNV,
}

impl CommandBufferFnNvClusterAccelerationStructure {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            build_cluster_acceleration_structure_indirect_nv: to_panic(loader(
                c"vkCmdBuildClusterAccelerationStructureIndirectNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvComputeOccupancyPriority {
    pub set_compute_occupancy_priority_nv: vkCmdSetComputeOccupancyPriorityNV,
}

impl CommandBufferFnNvComputeOccupancyPriority {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_compute_occupancy_priority_nv: to_panic(loader(
                c"vkCmdSetComputeOccupancyPriorityNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvCooperativeVector {
    pub convert_cooperative_vector_matrix_nv: vkCmdConvertCooperativeVectorMatrixNV,
}

impl CommandBufferFnNvCooperativeVector {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            convert_cooperative_vector_matrix_nv: to_panic(loader(
                c"vkCmdConvertCooperativeVectorMatrixNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvCopyMemoryIndirect {
    pub copy_memory_indirect_nv: vkCmdCopyMemoryIndirectNV,
    pub copy_memory_to_image_indirect_nv: vkCmdCopyMemoryToImageIndirectNV,
}

impl CommandBufferFnNvCopyMemoryIndirect {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            copy_memory_indirect_nv: to_panic(loader(c"vkCmdCopyMemoryIndirectNV")),
            copy_memory_to_image_indirect_nv: to_panic(loader(c"vkCmdCopyMemoryToImageIndirectNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvCudaKernelLaunch {
    pub cuda_launch_kernel_nv: vkCmdCudaLaunchKernelNV,
}

impl CommandBufferFnNvCudaKernelLaunch {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            cuda_launch_kernel_nv: to_panic(loader(c"vkCmdCudaLaunchKernelNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvDeviceDiagnosticCheckpoints {
    pub set_checkpoint_nv: vkCmdSetCheckpointNV,
}

impl CommandBufferFnNvDeviceDiagnosticCheckpoints {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_checkpoint_nv: to_panic(loader(c"vkCmdSetCheckpointNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvDeviceGeneratedCommands {
    pub execute_generated_commands_nv: vkCmdExecuteGeneratedCommandsNV,
    pub preprocess_generated_commands_nv: vkCmdPreprocessGeneratedCommandsNV,
    pub bind_pipeline_shader_group_nv: vkCmdBindPipelineShaderGroupNV,
}

impl CommandBufferFnNvDeviceGeneratedCommands {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            execute_generated_commands_nv: to_panic(loader(c"vkCmdExecuteGeneratedCommandsNV")),
            preprocess_generated_commands_nv: to_panic(loader(
                c"vkCmdPreprocessGeneratedCommandsNV",
            )),
            bind_pipeline_shader_group_nv: to_panic(loader(c"vkCmdBindPipelineShaderGroupNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvDeviceGeneratedCommandsCompute {
    pub update_pipeline_indirect_buffer_nv: vkCmdUpdatePipelineIndirectBufferNV,
}

impl CommandBufferFnNvDeviceGeneratedCommandsCompute {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            update_pipeline_indirect_buffer_nv: to_panic(loader(
                c"vkCmdUpdatePipelineIndirectBufferNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvFragmentShadingRateEnums {
    pub set_fragment_shading_rate_enum_nv: vkCmdSetFragmentShadingRateEnumNV,
}

impl CommandBufferFnNvFragmentShadingRateEnums {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_fragment_shading_rate_enum_nv: to_panic(loader(
                c"vkCmdSetFragmentShadingRateEnumNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvMemoryDecompression {
    pub decompress_memory_nv: vkCmdDecompressMemoryNV,
    pub decompress_memory_indirect_count_nv: vkCmdDecompressMemoryIndirectCountNV,
}

impl CommandBufferFnNvMemoryDecompression {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            decompress_memory_nv: to_panic(loader(c"vkCmdDecompressMemoryNV")),
            decompress_memory_indirect_count_nv: to_panic(loader(
                c"vkCmdDecompressMemoryIndirectCountNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvMeshShader {
    pub draw_mesh_tasks_nv: vkCmdDrawMeshTasksNV,
    pub draw_mesh_tasks_indirect_nv: vkCmdDrawMeshTasksIndirectNV,
    pub draw_mesh_tasks_indirect_count_nv: vkCmdDrawMeshTasksIndirectCountNV,
}

impl CommandBufferFnNvMeshShader {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            draw_mesh_tasks_nv: to_panic(loader(c"vkCmdDrawMeshTasksNV")),
            draw_mesh_tasks_indirect_nv: to_panic(loader(c"vkCmdDrawMeshTasksIndirectNV")),
            draw_mesh_tasks_indirect_count_nv: to_panic(loader(
                c"vkCmdDrawMeshTasksIndirectCountNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvOpticalFlow {
    pub optical_flow_execute_nv: vkCmdOpticalFlowExecuteNV,
}

impl CommandBufferFnNvOpticalFlow {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            optical_flow_execute_nv: to_panic(loader(c"vkCmdOpticalFlowExecuteNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvPartitionedAccelerationStructure {
    pub build_partitioned_acceleration_structures_nv: vkCmdBuildPartitionedAccelerationStructuresNV,
}

impl CommandBufferFnNvPartitionedAccelerationStructure {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            build_partitioned_acceleration_structures_nv: to_panic(loader(
                c"vkCmdBuildPartitionedAccelerationStructuresNV",
            )),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvRayTracing {
    pub copy_acceleration_structure_nv: vkCmdCopyAccelerationStructureNV,
    pub write_acceleration_structures_properties_nv: vkCmdWriteAccelerationStructuresPropertiesNV,
    pub build_acceleration_structure_nv: vkCmdBuildAccelerationStructureNV,
    pub trace_rays_nv: vkCmdTraceRaysNV,
}

impl CommandBufferFnNvRayTracing {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            copy_acceleration_structure_nv: to_panic(loader(c"vkCmdCopyAccelerationStructureNV")),
            write_acceleration_structures_properties_nv: to_panic(loader(
                c"vkCmdWriteAccelerationStructuresPropertiesNV",
            )),
            build_acceleration_structure_nv: to_panic(loader(c"vkCmdBuildAccelerationStructureNV")),
            trace_rays_nv: to_panic(loader(c"vkCmdTraceRaysNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvScissorExclusive {
    pub set_exclusive_scissor_nv: vkCmdSetExclusiveScissorNV,
    pub set_exclusive_scissor_enable_nv: vkCmdSetExclusiveScissorEnableNV,
}

impl CommandBufferFnNvScissorExclusive {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            set_exclusive_scissor_nv: to_panic(loader(c"vkCmdSetExclusiveScissorNV")),
            set_exclusive_scissor_enable_nv: to_panic(loader(c"vkCmdSetExclusiveScissorEnableNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvShadingRateImage {
    pub bind_shading_rate_image_nv: vkCmdBindShadingRateImageNV,
    pub set_viewport_shading_rate_palette_nv: vkCmdSetViewportShadingRatePaletteNV,
    pub set_coarse_sample_order_nv: vkCmdSetCoarseSampleOrderNV,
}

impl CommandBufferFnNvShadingRateImage {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            bind_shading_rate_image_nv: to_panic(loader(c"vkCmdBindShadingRateImageNV")),
            set_viewport_shading_rate_palette_nv: to_panic(loader(
                c"vkCmdSetViewportShadingRatePaletteNV",
            )),
            set_coarse_sample_order_nv: to_panic(loader(c"vkCmdSetCoarseSampleOrderNV")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnNvxBinaryImport {
    pub cu_launch_kernel_nvx: vkCmdCuLaunchKernelNVX,
}

impl CommandBufferFnNvxBinaryImport {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            cu_launch_kernel_nvx: to_panic(loader(c"vkCmdCuLaunchKernelNVX")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnQcomTileMemoryHeap {
    pub bind_tile_memory_qcom: vkCmdBindTileMemoryQCOM,
}

impl CommandBufferFnQcomTileMemoryHeap {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            bind_tile_memory_qcom: to_panic(loader(c"vkCmdBindTileMemoryQCOM")),
        }
    }
}

#[derive(Clone)]
pub struct CommandBufferFnQcomTileShading {
    pub dispatch_tile_qcom: vkCmdDispatchTileQCOM,
    pub begin_per_tile_execution_qcom: vkCmdBeginPerTileExecutionQCOM,
    pub end_per_tile_execution_qcom: vkCmdEndPerTileExecutionQCOM,
}

impl CommandBufferFnQcomTileShading {
    pub fn load<F: FnMut(&CStr) -> *const c_void>(mut loader: F) -> Self {
        Self {
            dispatch_tile_qcom: to_panic(loader(c"vkCmdDispatchTileQCOM")),
            begin_per_tile_execution_qcom: to_panic(loader(c"vkCmdBeginPerTileExecutionQCOM")),
            end_per_tile_execution_qcom: to_panic(loader(c"vkCmdEndPerTileExecutionQCOM")),
        }
    }
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
        extensions: &[*const i8],
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
        extensions: &[*const i8],
    ) -> Self {
        debug_assert!(api_version >= API_VERSION_1_0);
        Self {
            device: DeviceFn::load(&mut loader, api_version, extensions),
            queue: QueueFn::load(&mut loader, api_version, extensions),
            command_buffer: CommandBufferFn::load(&mut loader, api_version, extensions),
        }
    }
}
