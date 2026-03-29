#![allow(non_camel_case_types)]
use core::ffi::*;
pub type RROutput = c_ulong;
pub type VisualID = c_uint;
pub type Display = c_void;
pub type Window = c_ulong;
pub type xcb_connection_t = c_void;
pub type xcb_window_t = u32;
pub type xcb_visualid_t = u32;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HANDLE.html>
pub type HANDLE = isize;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HINSTANCE.html>
pub type HINSTANCE = HANDLE;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Foundation/struct.HWND.html>
pub type HWND = HANDLE;
/// <https://microsoft.github.io/windows-docs-rs/doc/windows/Win32/Graphics/Gdi/struct.HMONITOR.html>
pub type HMONITOR = HANDLE;
pub type wl_display = c_void;
pub type wl_surface = c_void;
pub type DWORD = c_ulong;
pub type LPCWSTR = *const u16;
pub type zx_handle_t = u32;
pub type _screen_buffer = c_void;
pub type _screen_context = c_void;
pub type _screen_window = c_void;
pub type SECURITY_ATTRIBUTES = c_void;
// Opaque types
// This definition is behind an NDA with a best effort guess from
// <https://github.com/google/gapid/commit/22aafebec4638c6aaa77667096bca30f6e842d95#diff-ab3ab4a7d89b4fc8a344ff4e9332865f268ea1669ee379c1b516a954ecc2e7a6R20-R21>
pub type GgpStreamDescriptor = u32;
pub type GgpFrameToken = u64;
pub type IDirectFB = c_void;
pub type IDirectFBSurface = c_void;
pub type __IOSurface = c_void;

// NvSciSync — nvidia-specific sync primitives (nvscisync.h)
// Opaque handles; only available when linking against the NvSci SDK.
/// <https://docs.nvidia.com/drive/drive_os_5.1.6.1L/nvvib_docs/index.html#page/DRIVE_OS_Linux_SDK_Development_Guide/Graphics/graphics_nvscisync.html>
pub type NvSciSyncAttrList = *mut c_void;
/// <https://docs.nvidia.com/drive/drive_os_5.1.6.1L/nvvib_docs/index.html#page/DRIVE_OS_Linux_SDK_Development_Guide/Graphics/graphics_nvscisync.html>
pub type NvSciSyncObj = *mut c_void;

// NvSciSyncFence is a value type (not a pointer), so we use a fixed-size
// opaque byte array matching the ABI size defined in nvscisync.h.
// The actual definition is: typedef struct { uint64_t payload[6]; } NvSciSyncFence;
#[repr(C)]
#[derive(Copy, Clone)]
pub struct NvSciSyncFence {
    pub payload: [u64; 6],
}

// NvSciBuf — nvidia-specific buffer primitives (nvscibuf.h)
/// <https://docs.nvidia.com/drive/drive_os_5.1.6.1L/nvvib_docs/index.html#page/DRIVE_OS_Linux_SDK_Development_Guide/Graphics/graphics_nvscibuf.html>
pub type NvSciBufAttrList = *mut c_void;
/// <https://docs.nvidia.com/drive/drive_os_5.1.6.1L/nvvib_docs/index.html#page/DRIVE_OS_Linux_SDK_Development_Guide/Graphics/graphics_nvscibuf.html>
pub type NvSciBufObj = *mut c_void;
