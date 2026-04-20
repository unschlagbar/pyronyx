use std::env;
use std::path::PathBuf;
use std::process::Command;

// This is still very wip, please make a issue on github if something doesn't work!

fn main() {
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();
    let target_pointer_width = env::var("CARGO_CFG_TARGET_POINTER_WIDTH").unwrap_or_default();

    if target_os == "android" {
        handle_android_linking();
    } else {
        handle_desktop_linking(&target_family, &target_pointer_width);
    }
}

// ── Desktop / Linux / macOS / Windows ────────────────────────────────────────

fn handle_desktop_linking(target_family: &str, target_pointer_width: &str) {
    // ── 1. Explicit LunarG Vulkan SDK (highest priority) ──────────────────
    if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
        let suffix = match (target_family, target_pointer_width) {
            ("windows", "32") => "Lib32",
            ("windows", "64") => "Lib",
            _ => "lib",
        };

        let search_path = format!("{vulkan_sdk}/{suffix}");
        println!("cargo:rustc-link-search={search_path}");

        let lib = if target_family == "windows" {
            "vulkan-1"
        } else {
            "vulkan"
        };
        println!("cargo:rustc-link-lib={lib}");
        return;
    }

    // Only attempt system-level fallbacks on non-Windows targets.
    // On Windows the LunarG SDK is practically mandatory; we warn accordingly.
    if target_family == "windows" {
        println!(
            "cargo::warning=VULKAN_SDK is not set. \
             Vulkan linking will fail on Windows. \
             Please install the LunarG Vulkan SDK from https://vulkan.lunarg.com/sdk/home"
        );
        return;
    }

    // ── 2. pkg-config (works after `apt install libvulkan-dev`) ───────────
    if try_pkgconfig_vulkan() {
        return;
    }

    // ── 3. Manual search of well-known system library paths ───────────────
    if try_system_vulkan_paths() {
        return;
    }

    // ── Nothing worked – emit a helpful, actionable error ─────────────────
    emit_install_hint();
}

/// Tries to resolve Vulkan through `pkg-config`.
/// Returns `true` if linking was configured successfully.
fn try_pkgconfig_vulkan() -> bool {
    // Query library search paths from pkg-config
    let libs_output = Command::new("pkg-config")
        .args(["--libs-only-L", "vulkan"])
        .output();

    let link_output = Command::new("pkg-config")
        .args(["--libs-only-l", "vulkan"])
        .output();

    match (libs_output, link_output) {
        (Ok(search), Ok(link)) if search.status.success() && link.status.success() => {
            // -L/some/path  →  emit rustc-link-search for each path
            let paths = String::from_utf8_lossy(&search.stdout);
            for token in paths.split_whitespace() {
                if let Some(path) = token.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={path}");
                }
            }

            // -lvulkan  →  emit rustc-link-lib for each lib
            let libs = String::from_utf8_lossy(&link.stdout);
            for token in libs.split_whitespace() {
                if let Some(lib) = token.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib={lib}");
                }
            }
            true
        }
        _ => false,
    }
}

/// Resolves the exact directory where `apt install libvulkan-dev` places the library.
/// Debian/Ubuntu/Raspberry Pi OS always use the multiarch path derived from the target triple.
/// Returns `true` if the library was found and linking was configured.
fn try_system_vulkan_paths() -> bool {
    // Map Cargo target arch → Debian multiarch tuple.
    // These are the install paths written by dpkg – no scanning needed.
    let multiarch_dir = match env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_default()
        .as_str()
    {
        "aarch64" => "/usr/lib/aarch64-linux-gnu", // Raspberry Pi OS 64-bit, Ubuntu arm64
        "arm" => "/usr/lib/arm-linux-gnueabihf",   // Raspberry Pi OS 32-bit
        "x86_64" => "/usr/lib/x86_64-linux-gnu",   // Debian/Ubuntu amd64
        "x86" => "/usr/lib/i386-linux-gnu",        // Debian/Ubuntu i386
        "riscv64" => "/usr/lib/riscv64-linux-gnu",
        // Non-multiarch distros (Arch, Alpine, Fedora) put libs here
        _ => "/usr/lib",
    };

    // libvulkan-dev installs libvulkan.so as a symlink to libvulkan.so.1
    let lib = PathBuf::from(multiarch_dir).join("libvulkan.so");
    if lib.exists() {
        println!("cargo:rustc-link-search=native={multiarch_dir}");
        println!("cargo:rustc-link-lib=vulkan");
        return true;
    }

    false
}

/// Emits a warning with OS-specific install instructions when no Vulkan
/// library could be found through any of the supported mechanisms.
fn emit_install_hint() {
    // Detect the host OS at compile time so we can give the right command.
    let hint = if cfg!(target_os = "linux") {
        // Try to identify the distro family for a more precise suggestion.
        let is_debian = PathBuf::from("/etc/debian_version").exists()
            || PathBuf::from("/etc/raspbian_version").exists();
        let is_arch = PathBuf::from("/etc/arch-release").exists();
        let is_fedora = PathBuf::from("/etc/fedora-release").exists()
            || PathBuf::from("/etc/redhat-release").exists();

        if is_debian {
            "Run: sudo apt update && sudo apt install libvulkan-dev vulkan-tools"
        } else if is_arch {
            "Run: sudo pacman -S vulkan-icd-loader vulkan-headers"
        } else if is_fedora {
            "Run: sudo dnf install vulkan-loader-devel vulkan-tools"
        } else {
            "Install the Vulkan loader development package for your distribution \
             (e.g. libvulkan-dev on Debian/Ubuntu, vulkan-icd-loader on Arch, \
             vulkan-loader-devel on Fedora)"
        }
    } else if cfg!(target_os = "macos") {
        "Install the LunarG Vulkan SDK from https://vulkan.lunarg.com/sdk/home \
         or use MoltenVK via Homebrew: brew install molten-vk"
    } else {
        "Install the LunarG Vulkan SDK from https://vulkan.lunarg.com/sdk/home"
    };

    println!(
        "cargo::warning=No Vulkan library found. Linking will fail.\n\
         Tried: VULKAN_SDK env var, pkg-config, and common system library paths.\n\
         {hint}"
    );
}

// ── Android ──────────────────────────────────────────────────────────────────

fn handle_android_linking() {
    let ndk_root = match option_env!("ANDROID_NDK_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var("ANDROID_NDK_HOME").map(PathBuf::from).ok())
    {
        Some(path) if path.is_dir() => path,
        _ => {
            println!(
                "cargo::warning=ANDROID_NDK_HOME is not set. \
                 Vulkan linking for Android will fail. \
                 Please install the Android NDK from https://developer.android.com/ndk/downloads"
            );
            return;
        }
    };

    // Find host toolchain directory (linux-x86_64, darwin-x86_64, windows-x86_64, …)
    let host = if cfg!(target_os = "windows") {
        "windows-x86_64"
    } else if cfg!(target_os = "macos") {
        "darwin-x86_64"
    } else {
        "linux-x86_64"
    };

    let prebuilt = ndk_root.join("toolchains/llvm/prebuilt").join(host);
    if !prebuilt.is_dir() {
        println!(
            "cargo::warning=Could not find NDK prebuilt toolchain at {:?}",
            prebuilt
        );
        return;
    }

    // Find the highest API level directory (e.g. 35 > 34 > 33 …)
    let sysroot_lib = prebuilt.join("sysroot/usr/lib");
    if !sysroot_lib.is_dir() {
        println!("cargo::warning=NDK sysroot not found");
        return;
    }

    let latest_api = std::fs::read_dir(&sysroot_lib)
        .into_iter()
        .flatten()
        .filter_map(|entry| {
            let path = entry.unwrap().path();
            if path.is_dir() {
                path.file_name()?.to_str()?.parse::<u32>().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap_or(34); // fallback

    // Target architecture directory (aarch64-linux-android, x86_64-linux-android, …)
    let target_arch = match env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_default()
        .as_str()
    {
        "aarch64" => "aarch64-linux-android",
        "arm" | "armeabi" => "arm-linux-androideabi",
        "x86_64" => "x86_64-linux-android",
        "x86" => "i686-linux-android",
        arch => {
            println!("cargo::warning=Unknown Android architecture: {}", arch);
            return;
        }
    };

    let lib_path = sysroot_lib.join(target_arch).join(latest_api.to_string());

    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=vulkan"); // libvulkan.so from NDK
}
