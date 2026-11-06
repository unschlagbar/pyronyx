//! Runtime loading of the Vulkan loader, behind the `loaded` feature.
//!
//! Without it, `vkGetInstanceProcAddr` is linked, so a machine without a
//! Vulkan loader can't even start the program. With it, nothing is linked:
//! the program calls [`load`] (or [`load_from`]) first and handles the error.

use std::ffi::{OsStr, c_char};
use std::sync::OnceLock;

use crate::vk;

type GetInstanceProcAddr =
    unsafe extern "system" fn(vk::vkInstance, *const c_char) -> vk::PFN_vkVoidFunction;

/// The opened library, kept open for the rest of the program, and its entry
/// point.
static ENTRY: OnceLock<(libloading::Library, GetInstanceProcAddr)> = OnceLock::new();

#[cfg(windows)]
const LIBRARY_NAMES: &[&str] = &["vulkan-1.dll"];
#[cfg(target_os = "android")]
const LIBRARY_NAMES: &[&str] = &["libvulkan.so"];
#[cfg(target_vendor = "apple")]
const LIBRARY_NAMES: &[&str] = &["libvulkan.1.dylib", "libvulkan.dylib", "libMoltenVK.dylib"];
#[cfg(all(unix, not(target_os = "android"), not(target_vendor = "apple")))]
const LIBRARY_NAMES: &[&str] = &["libvulkan.so.1", "libvulkan.so"];

/// Opening the Vulkan loader, or finding its entry point, failed.
#[derive(Debug)]
pub struct LoadError(libloading::Error);

impl std::fmt::Display for LoadError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "failed to load the Vulkan loader: {}", self.0)
    }
}

impl std::error::Error for LoadError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.0)
    }
}

/// Loads the platform's Vulkan loader by its usual names. Loading again once
/// it succeeded does nothing.
pub fn load() -> Result<(), LoadError> {
    let mut last = None;
    for name in LIBRARY_NAMES {
        // SAFETY: the Vulkan loader's initialisers are safe to run.
        match unsafe { load_from(name) } {
            Ok(()) => return Ok(()),
            Err(error) => last = Some(error),
        }
    }
    Err(last.expect("every platform names at least one library"))
}

/// Loads the Vulkan loader at `path`. Loading again once it succeeded does
/// nothing.
///
/// # Safety
///
/// `path` must be a Vulkan loader: opening a library runs its initialisers,
/// and its `vkGetInstanceProcAddr` is trusted to have Vulkan's signature.
pub unsafe fn load_from(path: impl AsRef<OsStr>) -> Result<(), LoadError> {
    if ENTRY.get().is_some() {
        return Ok(());
    }
    // SAFETY: the caller guarantees `path` is a Vulkan loader.
    let library = unsafe { libloading::Library::new(path.as_ref()) }.map_err(LoadError)?;
    // SAFETY: Vulkan defines `vkGetInstanceProcAddr` with this signature.
    let entry = *unsafe { library.get::<GetInstanceProcAddr>(b"vkGetInstanceProcAddr\0") }
        .map_err(LoadError)?;
    let _ = ENTRY.set((library, entry));
    Ok(())
}

/// <https://docs.vulkan.org/refpages/latest/refpages/source/vkGetInstanceProcAddr.html>,
/// named as the linked one is so the rest of the crate calls either.
///
/// # Safety
///
/// The arguments must meet Vulkan's valid usage for the command.
///
/// # Panics
///
/// When [`load`] or [`load_from`] hasn't succeeded yet.
#[allow(non_snake_case)]
pub unsafe fn vkGetInstanceProcAddr(
    instance: vk::vkInstance,
    name: *const c_char,
) -> vk::PFN_vkVoidFunction {
    let (_, entry) = ENTRY
        .get()
        .expect("Vulkan loader not loaded: call `pyronyx::load()` or `pyronyx::load_from()` and check that it returned Ok before making any Vulkan call");
    // SAFETY: forwarded unchanged to the loader's own entry point.
    unsafe { entry(instance, name) }
}
