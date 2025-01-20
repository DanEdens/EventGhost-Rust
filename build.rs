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
} 