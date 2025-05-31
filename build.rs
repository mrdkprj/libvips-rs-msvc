// use std::env;
// use std::fs;
use std::path::PathBuf;

// (c) Copyright 2019-2024 OLX
fn main() {
    // println!("cargo:rustc-link-lib=vips");
    // println!("cargo:rustc-link-lib=glib-2.0");
    // println!("cargo:rustc-link-lib=gobject-2.0");

    let libvips_root = PathBuf::from(r#"C:\DevProjects\libvips-rust-bindings\libvips\out"#); // .lib location

    // let lib_dir = libvips_root.join("lib");
    // let bin_dir = libvips_root.join("bin");

    // let include_dir = libvips_root.join("include");
    println!(
        "cargo:rustc-link-search=native={}",
        libvips_root.display()
    );
    println!("cargo:rustc-link-lib=dylib=vips"); // for vips_bridge.dll
    println!("cargo:rustc-link-lib=dylib=libvips-42"); // for libvips-42.lib

    // If needed, link to system libraries
    println!("cargo:rustc-link-lib=dylib=gobject-2.0");
    println!("cargo:rustc-link-lib=dylib=glib-2.0");
    // vcpkg::find_package("glib").unwrap();
    // vcpkg::find_package("gobject").unwrap();
    // vcpkg::Config::new()
    //     .find_package("glib")
    //     .unwrap();
    // Tell cargo to rerun the build script if these files change
    println!("cargo:rerun-if-changed=build.rs");

    // // Optional: Set include dir as env var for bindgen or later use
    // println!(
    //     "cargo:include={}",
    //     include_dir.display()
    // );

    // // Optional: If your tests or examples need the DLL path at runtime
    // // Copy the DLLs into target directory (not needed if user sets PATH)
    // let out_dir = env::var("OUT_DIR").unwrap();
    // let bin_dir = libvips_root.join("bin");

    // for entry in fs::read_dir(bin_dir).unwrap() {
    //     let entry = entry.unwrap();
    //     let path = entry.path();
    //     if path
    //         .extension()
    //         .map_or(
    //             false,
    //             |e| e == "dll",
    //         )
    //     {
    //         let filename = path
    //             .file_name()
    //             .unwrap();
    //         let target = PathBuf::from(&out_dir).join(filename);
    //         fs::copy(
    //             &path,
    //             &target,
    //         )
    //         .expect("Failed to copy DLL");
    //     }
    // }
}
