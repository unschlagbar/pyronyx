use crate::khr::surface;
use crate::vk::{self, Error, Instance};
use core::ffi::c_char;
use raw_window_handle::{RawDisplayHandle, RawWindowHandle};

#[cfg(target_os = "windows")]
use crate::khr::win32_surface;

#[cfg(target_os = "linux")]
use crate::khr::{
    xcb_surface::{self, XcbSurfaceInstance},
    xlib_surface::{self, XlibSurfaceInstance},
};

#[cfg(target_os = "linux")]
use crate::khr::wayland_surface;
#[cfg(target_os = "linux")]
use crate::khr::wayland_surface::WaylandSurfaceInstance;

#[cfg(target_os = "android")]
use crate::khr::android_surface;

#[cfg(any(target_os = "ios", target_os = "macos"))]
use crate::ext::metal_surface;

#[cfg(target_os = "macos")]
use raw_window_metal::{Layer, appkit};

#[cfg(target_os = "ios")]
use raw_window_metal::{Layer, uikit};

#[cfg(target_env = "ohos")]
use crate::ohos::surface::SurfaceInstance;

/// Create a surface from a raw display and window handle.
///
/// `instance` must have created with platform specific surface extensions enabled, acquired
/// through [`enumerate_required_extensions()`].
///
/// See the [`Entry::create_instance()`] documentation for more destruction ordering rules on
/// [`Instance`].
///
/// The window represented by `window_handle` must be associated with the display connection
/// in `display_handle`.
///
/// `window_handle` and `display_handle` must be associated with a valid window and display
/// connection, which must not be destroyed for the lifetime of the returned [`vk::SurfaceKHR`].
pub fn create_surface(
    instance: &Instance,
    display_handle: RawDisplayHandle,
    window_handle: RawWindowHandle,
) -> Result<vk::SurfaceKHR, Error> {
    match (display_handle, window_handle) {
        #[cfg(target_os = "windows")]
        (RawDisplayHandle::Windows(_), RawWindowHandle::Win32(window)) => {
            let create_info = vk::Win32SurfaceCreateInfoKHR {
                hwnd: window.hwnd.get(),
                hinstance: window
                    .hinstance
                    .ok_or(Error::ErrorInitializationFailed)?
                    .get(),
                ..Default::default()
            };
            instance.create_win32_surface(&create_info, None)
        }

        #[cfg(target_os = "linux")]
        (RawDisplayHandle::Wayland(display), RawWindowHandle::Wayland(window)) => {
            let create_info = vk::WaylandSurfaceCreateInfoKHR {
                display: display.display.as_ptr(),
                surface: window.surface.as_ptr(),
                ..Default::default()
            };
            instance.create_wayland_surface(&create_info, None)
        }

        #[cfg(target_os = "linux")]
        (RawDisplayHandle::Xlib(display), RawWindowHandle::Xlib(window)) => {
            let create_info = vk::XlibSurfaceCreateInfoKHR {
                dpy: display.display.ok_or(Error::InitializationFailed)?.as_ptr(),
                window: window.window,
                ..Default::default()
            };

            instance.create_xlib_surface(&create_info, None)
        }

        #[cfg(target_os = "linux")]
        (RawDisplayHandle::Xcb(display), RawWindowHandle::Xcb(window)) => {
            let create_info = vk::XcbSurfaceCreateInfoKHR {
                connection: display
                    .connection
                    .ok_or(Error::InitializationFailed)?
                    .as_ptr(),
                window: window.window.get(),
                ..Default::default()
            };
            instance.create_xcb_surface(&create_info, None)
        }

        #[cfg(target_os = "android")]
        (RawDisplayHandle::Android(_), RawWindowHandle::AndroidNdk(window)) => {
            let create_info = vk::AndroidSurfaceCreateInfoKHR {
                window: window.a_native_window.as_ptr(),
                ..Default::default()
            };
            instance.create_android_surface(&create_info, None)
        }

        #[cfg(target_os = "macos")]
        (RawDisplayHandle::AppKit(_), RawWindowHandle::AppKit(window)) => {
            let layer = match appkit::metal_layer_from_handle(window) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
            };

            let create_info = vk::MetalSurfaceCreateInfoEXT::default().layer(&*layer);
            instance.create_metal_surface(&create_info, None)
        }

        #[cfg(target_os = "ios")]
        (RawDisplayHandle::UiKit(_), RawWindowHandle::UiKit(window)) => {
            let layer = match uikit::metal_layer_from_handle(window) {
                Layer::Existing(layer) | Layer::Allocated(layer) => layer.cast(),
            };

            let create_info = vk::MetalSurfaceCreateInfoEXT::default().layer(&*layer);
            instance.create_metal_surface(&create_info, None)
        }

        #[cfg(target_env = "ohos")]
        (RawDisplayHandle::Ohos(_), RawWindowHandle::OhosNdk(window)) => {
            let create_info = vk::SurfaceCreateInfoOHOS {
                window: window.native_window.as_ptr().cast(),
                ..Default::default()
            };
            instance.create_surface(&create_info, None)
        }

        _ => Err(Error::ExtensionNotPresent),
    }
}

/// Query the required instance extensions for creating a surface from a raw display handle.
///
/// This [`RawDisplayHandle`] can typically be acquired from a window, but is usually also
/// accessible earlier through an "event loop" concept to allow querying required instance
/// extensions and creation of a compatible Vulkan instance prior to creating a window.
///
/// The returned extensions will include all extension dependencies.
pub fn get_required_extensions(
    display_handle: RawDisplayHandle,
) -> Result<[*const c_char; 2], Error> {
    let extensions = match display_handle {
        #[cfg(target_os = "windows")]
        RawDisplayHandle::Windows(_) => [surface::NAME.as_ptr(), win32_surface::NAME.as_ptr()],

        #[cfg(target_os = "linux")]
        RawDisplayHandle::Wayland(_) => [surface::NAME.as_ptr(), wayland_surface::NAME.as_ptr()],

        #[cfg(target_os = "linux")]
        RawDisplayHandle::Xlib(_) => [surface::NAME.as_ptr(), xlib_surface::NAME.as_ptr()],

        #[cfg(target_os = "linux")]
        RawDisplayHandle::Xcb(_) => [surface::NAME.as_ptr(), xcb_surface::NAME.as_ptr()],

        #[cfg(target_os = "android")]
        RawDisplayHandle::Android(_) => [surface::NAME.as_ptr(), android_surface::NAME.as_ptr()],

        #[cfg(any(target_os = "ios", target_os = "macos"))]
        RawDisplayHandle::AppKit(_) | RawDisplayHandle::UiKit(_) => {
            [surface::NAME.as_ptr(), metal_surface::NAME.as_ptr()]
        }

        #[cfg(target_env = "ohos")]
        RawDisplayHandle::Ohos(_) => [surface::NAME.as_ptr(), crate::ohos::surface::NAME.as_ptr()],

        _ => return Err(Error::ExtensionNotPresent),
    };

    Ok(extensions)
}
