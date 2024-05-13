//! Custom build script for sugar backend library.

use std::{fs, env, path::{Path, PathBuf}};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
}

fn android() {
    println!("cargo:rustc-link-search=native=src/static");
    println!("cargo:rustc-link-lib=c++_shared");
    println!("cargo:rustc-link-lib=usb-1.0");
    println!("cargo:rustc-link-lib=hidapi-libusb");

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        
        // Copy libc++_shared.so
        let libcxx_path = sysroot_libs_path.join("libc++_shared.so");
        fs::copy(
            &libcxx_path,
            Path::new(&output_path)
                .join(&env::var("CARGO_NDK_ANDROID_TARGET").unwrap())
                .join("libc++_shared.so"),
        ).unwrap();

        // Copy libusb-1.0.so
        let libusb_path = Path::new("src/static/libusb-1.0.so");
        fs::copy(
            &libusb_path,
            Path::new(&output_path)
                .join(&env::var("CARGO_NDK_ANDROID_TARGET").unwrap())
                .join("libusb1.0.so"),
        ).unwrap();


        // Copy libhidapi-libusb.so
        let libusb_path = Path::new("src/static/libhidapi-libusb.so");
        fs::copy(
            &libusb_path,
            Path::new(&output_path)
                .join(&env::var("CARGO_NDK_ANDROID_TARGET").unwrap())
                .join("libhidapi.so"),
        ).unwrap();
    }
}

