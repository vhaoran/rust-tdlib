use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=TDLIB_DIR");
    if let Ok(tdlib_dir) = env::var("TDLIB_DIR") {
        let lib_dir = PathBuf::from(tdlib_dir.as_str()).join("lib");
        if lib_dir.exists() {
            println!("cargo:rustc-link-search=native={}", lib_dir.display());
        } else {
            panic!(
                "TDLIB_DIR is set to '{}', but the 'lib' directory was not found.",
                tdlib_dir
            );
        }
    } else {
        if cfg!(target_os = "macos") {
            println!("cargo:rustc-link-search=native=/opt/homebrew/lib");
            println!("cargo:rustc-link-search=native=/usr/local/lib");
        } else if cfg!(target_os = "linux") {
            println!("cargo:rustc-link-search=native=/usr/local/lib");
            println!("cargo:rustc-link-search=native=/usr/lib");
        } else if cfg!(target_os = "windows") {
            // println!("cargo:warning=TDLIB_DIR environment variable not set. Please set it to your TDLib installation directory (e.g., C:\\path\\to\\tdlib).");
        }
    }

    println!("cargo:rustc-link-lib=tdjson");
}
