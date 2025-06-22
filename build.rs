fn main() {
    #[cfg(target_os = "windows")]
    {
        if let Ok(env_path) = std::env::var("PATH") {
            let env_paths: Vec<&str> = env_path
                .split(";")
                .collect();
            if let Some(lib) = env_paths
                .iter()
                .find(|env_path| env_path.ends_with("libvips"))
            {
                println!(
                    "cargo:rustc-link-search=native={}",
                    lib
                );
            }
        }
        println!("cargo:rustc-link-lib=dylib=libvips");
        println!("cargo:rustc-link-lib=dylib=vipscache");
        println!("cargo:rustc-link-lib=dylib=gobject-2.0");
        println!("cargo:rustc-link-lib=dylib=glib-2.0");
        println!("cargo:rustc-link-lib=dylib=voption");
    }
    #[cfg(not(target_os = "windows"))]
    {
        println!("cargo:rustc-link-lib=vips");
        println!("cargo:rustc-link-lib=glib-2.0");
        println!("cargo:rustc-link-lib=gobject-2.0");
    }
}
