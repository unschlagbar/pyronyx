use std::env;
use std::path::PathBuf;

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

    println!(
        "cargo::info=Android Vulkan linked against NDK API level {} at {:?}",
        latest_api, lib_path
    );
}

fn handle_desktop_linking(target_family: &str, target_pointer_width: &str) {
    if let Ok(vulkan_sdk) = env::var("VULKAN_SDK") {
        let suffix = match (target_family, target_pointer_width) {
            ("windows", "32") => "Lib32",
            ("windows", "64") => "Lib",
            _ => "lib",
        };

        let search_path = format!("{vulkan_sdk}/{suffix}");
        println!("cargo:rustc-link-search={search_path}");

        let lib = match target_family {
            "windows" => "vulkan-1",
            _ => "vulkan",
        };
        println!("cargo:rustc-link-lib={lib}");
    } else {
        println!(
            "cargo::warning=VULKAN_SDK environment variable is not set. \
             Vulkan linking will fail. \
             Please install the Vulkan SDK from https://vulkan.lunarg.com/sdk/home"
        );
    }
}
