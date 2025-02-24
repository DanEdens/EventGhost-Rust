use std::process::Command;
use std::env;
use std::path::PathBuf;

fn main() {
    if cfg!(target_os = "windows") {
        // Tell cargo to look for GTK4 in the MSYS2 installation directory
        println!("cargo:rustc-link-search=C:/msys64/mingw64/lib");
        println!("cargo:rustc-env=PKG_CONFIG_PATH=C:/msys64/mingw64/lib/pkgconfig");
        
        // Tell pkg-config where to find .pc files
        if std::env::var("PKG_CONFIG_PATH").is_err() {
            std::env::set_var("PKG_CONFIG_PATH", "C:/msys64/mingw64/lib/pkgconfig");
        }
    }
    
    // Detect GTK4
    pkg_config::Config::new()
        .atleast_version("4.6")
        .probe("gtk4")
        .unwrap();

    // Get the manifest directory (where Cargo.toml is)
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    let mut src_path = PathBuf::from(&manifest_dir);
    src_path.push("src");

    // Change to the src directory before running glib-compile-resources
    env::set_current_dir(&src_path).expect("Failed to change to src directory");

    // Compile the GResource file
    let status = Command::new("glib-compile-resources")
        .args(&[
            "--target=resources.gresource",
            "resources.gresource.xml",
        ])
        .status()
        .expect("Failed to compile resources");

    if !status.success() {
        panic!("Failed to compile GResource file");
    }

    println!("cargo:rerun-if-changed=src/resources.gresource.xml");
    println!("cargo:rerun-if-changed=src/images/");

    // Create plugins directory in target directory
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let plugins_dir = out_dir.join("plugins");
    std::fs::create_dir_all(&plugins_dir).unwrap();

    // Tell cargo to rerun if plugin source changes
    println!("cargo:rerun-if-changed=src/plugins/logger/mod.rs");
    println!("cargo:rerun-if-changed=plugins/logger/Cargo.toml");
} 