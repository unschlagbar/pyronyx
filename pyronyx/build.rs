use std::env;
use std::path::PathBuf;
use std::process::Command;

// This is still higly unstable und not finished
// Feel free to contribute if you want to implement or fix it for you platform

fn main() {
    println!("cargo:rerun-if-env-changed=VULKAN_SDK");
    println!("cargo:rerun-if-env-changed=ANDROID_NDK_HOME");

    // `pyronyx::load` opens the loader at runtime instead.
    if env::var_os("CARGO_FEATURE_LOADED").is_some() {
        return;
    }

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
        println!("cargo:rustc-link-search={vulkan_sdk}/{suffix}");
        let lib = if target_family == "windows" {
            "vulkan-1"
        } else {
            "vulkan"
        };
        println!("cargo:rustc-link-lib={lib}");
        return;
    }

    if target_family == "windows" {
        println!(
            "cargo::warning=VULKAN_SDK is not set. \
             Vulkan linking will fail on Windows. \
             Please install the LunarG Vulkan SDK: https://vulkan.lunarg.com/sdk/home"
        );
        return;
    }

    // ── 2. pkg-config ─────────────────────────────────────────────────────
    if try_pkgconfig_vulkan() {
        return;
    }

    // ── 3. Well-known system paths (tries all distros, first hit wins) ────
    if try_system_vulkan_paths() {
        return;
    }

    emit_install_hint();
}

fn try_pkgconfig_vulkan() -> bool {
    let search = Command::new("pkg-config")
        .args(["--libs-only-L", "vulkan"])
        .output();
    let link = Command::new("pkg-config")
        .args(["--libs-only-l", "vulkan"])
        .output();

    match (search, link) {
        (Ok(s), Ok(l)) if s.status.success() && l.status.success() => {
            for token in String::from_utf8_lossy(&s.stdout).split_whitespace() {
                if let Some(path) = token.strip_prefix("-L") {
                    println!("cargo:rustc-link-search=native={path}");
                }
            }
            for token in String::from_utf8_lossy(&l.stdout).split_whitespace() {
                if let Some(lib) = token.strip_prefix("-l") {
                    println!("cargo:rustc-link-lib={lib}");
                }
            }
            true
        }
        _ => false,
    }
}

/// Probes all known distro-specific paths for libvulkan.so – no distro
/// detection needed, just try them all and use the first hit.
///
/// Layout by distro family:
///   Debian/Ubuntu  →  /usr/lib/<multiarch-triple>/
///   Fedora/RHEL    →  /usr/lib64/
///   Arch/Alpine/…  →  /usr/lib/
fn try_system_vulkan_paths() -> bool {
    let arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap_or_default();

    let candidates: &[&str] = match arch.as_str() {
        "x86_64" => &[
            "/usr/lib/x86_64-linux-gnu", // Debian/Ubuntu
            "/usr/lib64",                // Fedora/RHEL/openSUSE
            "/usr/lib",                  // Arch, Alpine, CachyOS, …
        ],
        "aarch64" => &["/usr/lib/aarch64-linux-gnu", "/usr/lib64", "/usr/lib"],
        "arm" => &["/usr/lib/arm-linux-gnueabihf", "/usr/lib"],
        "x86" => &["/usr/lib/i386-linux-gnu", "/usr/lib"],
        "riscv64" => &["/usr/lib/riscv64-linux-gnu", "/usr/lib"],
        _ => &["/usr/lib"],
    };

    for &dir in candidates {
        if PathBuf::from(dir).join("libvulkan.so").exists() {
            println!("cargo:rustc-link-search=native={dir}");
            println!("cargo:rustc-link-lib=vulkan");
            return true;
        }
    }

    false
}

fn emit_install_hint() {
    let hint = if cfg!(target_os = "linux") {
        "Install the Vulkan dev package for your distro:\n\
         \n\
         Debian / Ubuntu / Mint:   sudo apt install libvulkan-dev vulkan-tools\n\
         Arch / Manjaro / CachyOS: sudo pacman -S vulkan-icd-loader vulkan-headers\n\
         Fedora / RHEL / Alma:     sudo dnf install vulkan-loader-devel vulkan-tools\n\
         openSUSE:                 sudo zypper install vulkan-devel\n\
         Alpine:                   sudo apk add vulkan-loader-dev"
    } else if cfg!(target_os = "macos") {
        "Install the LunarG Vulkan SDK: https://vulkan.lunarg.com/sdk/home\n\
         or via Homebrew: brew install molten-vk"
    } else {
        "Install the LunarG Vulkan SDK: https://vulkan.lunarg.com/sdk/home"
    };

    println!(
        "cargo::warning=No Vulkan library found. Linking will fail.\n\
         Tried: VULKAN_SDK env var, pkg-config, and common system paths.\n\
         \n\
         {hint}"
    );
}

// ── Android ──────────────────────────────────────────────────────────────────

fn handle_android_linking() {
    let ndk_root = match option_env!("ANDROID_NDK_HOME")
        .map(PathBuf::from)
        .or_else(|| env::var("ANDROID_NDK_HOME").map(PathBuf::from).ok())
    {
        Some(p) if p.is_dir() => p,
        _ => {
            println!(
                "cargo::warning=ANDROID_NDK_HOME is not set. \
                 Vulkan linking for Android will fail. \
                 Install the Android NDK: https://developer.android.com/ndk/downloads"
            );
            return;
        }
    };

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

    let target_arch = match env::var("CARGO_CFG_TARGET_ARCH")
        .unwrap_or_default()
        .as_str()
    {
        "aarch64" => "aarch64-linux-android",
        "arm" | "armeabi" => "arm-linux-androideabi",
        "x86_64" => "x86_64-linux-android",
        "x86" => "i686-linux-android",
        arch => {
            println!("cargo::warning=Unknown Android architecture: {arch}");
            return;
        }
    };

    // NDK layout: sysroot/usr/lib/<abi>/<api-level>/libvulkan.so
    // The original code scanned the wrong directory (sysroot/usr/lib instead
    // of sysroot/usr/lib/<abi>), so latest_api always fell back to 34.
    let abi_dir = prebuilt.join("sysroot/usr/lib").join(target_arch);
    if !abi_dir.is_dir() {
        println!(
            "cargo::warning=NDK sysroot ABI directory not found: {:?}",
            abi_dir
        );
        return;
    }

    let latest_api = std::fs::read_dir(&abi_dir)
        .into_iter()
        .flatten()
        .filter_map(|e| {
            let path = e.ok()?.path();
            if path.is_dir() {
                path.file_name()?.to_str()?.parse::<u32>().ok()
            } else {
                None
            }
        })
        .max()
        .unwrap_or(34);

    let lib_path = abi_dir.join(latest_api.to_string());
    println!("cargo:rustc-link-search=native={}", lib_path.display());
    println!("cargo:rustc-link-lib=vulkan");
}
