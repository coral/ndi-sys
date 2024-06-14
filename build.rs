// build.rs

#[allow(unused_imports)]
use std::io::ErrorKind;
#[allow(unused_imports)]
use std::path::{Path, PathBuf};
use std::{env, fs};

#[allow(dead_code)]
fn find_sdk() -> Option<String> {
    // Follow the 'recommended' sdk path
    if let Ok(path) = env::var("NDI_SDK_DIR") {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    #[cfg(target_os = "macos")]
    {
        // Try the standard SDK install location on Mac OS
        let std_location = Path::new("/Library/NDI SDK for macOS 6/");
        if std_location.exists() {
            return std_location.to_str().map(|s| s.to_string());
        }
    }

    #[cfg(target_os = "windows")]
    {
        // Try the standard SDK install location on Windows
        let std_location = Path::new("C:\\NDI\\NDI 6 SDK");
        if std_location.exists() {
            return std_location.to_str().map(|s| s.to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Try the standard SDK install location on Linux
        let std_location = Path::new("/usr/local/NDI SDK for Linux 6/");
        if std_location.exists() {
            return std_location.to_str().map(|s| s.to_string());
        }
    }

    None
}

fn main() {
    let os = env::var("CARGO_CFG_TARGET_OS").expect("Could not get OS");

    #[cfg(feature = "bindings")]
    {
        // Generate fresh bindings
        let sdk = find_sdk().expect(
            "Could not find the SDK to generate bindings. Please set
            the NDI_SDK_DIR env variable to the root of the SDK install.",
        );
        let sdk = Path::new(&sdk);

        let include_file = match os.as_str() {
            "windows" => "include/Processing.NDI.Lib.h",
            "linux" => "include/Processing.NDI.Lib.h",
            "macos" => "include/Processing.NDI.Lib.h",
            _ => panic!("Unsupported OS for NDI"),
        };

        let bindings = bindgen::Builder::default()
            .header(sdk.join(include_file).to_str().unwrap())
            //.allowlist_function("NDIlib_v5_load")
            //.allowlist_type(".*")
            //.allowlist_var(".*")
            .clang_arg("-fdeclspec")
            .generate()
            .expect("Unable to generate bindings");

        let mut out_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
        out_path.push("src/sdk.rs");
        bindings.write_to_file(&out_path).expect(&format!(
            "Couldn't write bindings to: {}",
            out_path.display()
        ));
    }

    match os.as_str() {
        "windows" => setup_win(),
        "linux" => setup_linux(),
        "macos" => setup_macos(),
        _ => panic!("Unsupported OS for NDI"),
    };
}

fn setup_win() {
    let source_dir = find_library();

    // Copy the .dll/.lib files to the deps folder, to make it build
    if let Some(path) = source_dir {
        let source_path = Path::new(&path);
        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");
        fs::copy(
            source_path.join("..\\..\\NDI 6 SDK\\Lib\\x64\\Processing.NDI.Lib.x64.lib"),
            dest_path.join("Processing.NDI.Lib.x64.lib"),
        )
        .expect("copy Processing.NDI.Lib.x64.lib");
        fs::copy(
            source_path.join("Processing.NDI.Lib.x64.dll"),
            dest_path.join("Processing.NDI.Lib.x64.dll"),
        )
        .expect("copy Processing.NDI.Lib.x64.dll");
    }

    if cfg!(not(feature = "dynamic-link")) {
        // Static link against it
        println!("cargo:rustc-link-lib=Processing.NDI.Lib.x64");
    }
}

fn setup_linux() {
    let source_dir = find_library();

    // Copy the .so files to the deps folder, to make it build
    if let Some(path) = source_dir {
        let source_path = Path::new(&path);
        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");
        fs::copy(
            source_path.join("libndi.so.6"),
            dest_path.join("libndi.so.6"),
        )
        .expect("copy libndi.so.6");

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let sl_res =
                symlink(Path::new("libndi.so.6"), dest_path.join("libndi.so"));
            if let Err(e) = sl_res {
                if e.kind() != ErrorKind::AlreadyExists {
                    panic!("Unknown error: {}", e);
                }
            }
        }
    }

    if cfg!(not(feature = "dynamic-link")) {
        // Static link against it
        println!("cargo:rustc-link-lib=ndi");
    }
}

fn setup_macos() {
    let source_dir = find_library();
    // Copy the .dylib files to the deps folder, to make it build
    if let Some(path) = source_dir {
        let source_path = Path::new(&path);
        let dest_path = Path::new(&env::var("OUT_DIR").unwrap()).join("../../../deps");
        fs::copy(
            source_path.join("libndi.dylib"),
            dest_path.join("libndi.dylib"),
        )
        .expect("copy libndi.dylib");

        #[cfg(unix)]
        {
            use std::os::unix::fs::symlink;
            let sl_res =
                symlink(Path::new("libndi.dylib"), dest_path.join("libndi.dylib"));
            if let Err(e) = sl_res {
                if e.kind() != ErrorKind::AlreadyExists {
                    panic!("Unknown error: {}", e);
                }
            }
        }
    }

    if cfg!(not(feature = "dynamic-link")) {
        // Static link against it
        println!("cargo:rustc-link-lib=ndi");
    }
}

fn find_library() -> Option<String> {
    // Follow the 'recommended' install path
    if let Ok(path) = env::var("NDI_RUNTIME_DIR_V6") {
        if Path::new(&path).exists() {
            return Some(path);
        }
    }

    // Try the local lib folder
    let dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let path = Path::new(&dir).join("lib");
    if path.exists() {
        return path.to_str().map(|s| s.to_string());
    }

    #[cfg(target_os = "macos")]
    {
        // Try the standard SDK install location on Mac OS
        let std_location = Path::new("/Library/NDI SDK for macOS 6/lib/macOS/");
        if std_location.exists() {
            return std_location.to_str().map(|s| s.to_string());
        }
    }

    #[cfg(target_os = "linux")]
    {
        // Try the standard SDK install location on Linux
        let std_location = Path::new("/usr/local/NDI SDK for Linux 6/lib/");
        if std_location.exists() {
            return std_location.to_str().map(|s| s.to_string());
        }
    }

    None
}
