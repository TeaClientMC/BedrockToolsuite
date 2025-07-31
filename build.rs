use std::env;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap_or_default();
    let target_vendor = env::var("CARGO_CFG_TARGET_VENDOR").unwrap_or_default();
    let target_family = env::var("CARGO_CFG_TARGET_FAMILY").unwrap_or_default();

    if target_os == "ios" || target_os == "android" {
        println!(
            "cargo:warning=Building for {} is not fully supported. See issues #1 and #2.",
            target_os
        );
    }

    if target_vendor == "nintendo" {
        println!(
            "cargo:warning=Nintendo console targets are not supported due to NDA requirements."
        );
    }

    if target_family == "unix" && (target_os == "macos" || target_os == "linux") {
        println!(
            "cargo:warning=Bedrock features not available for {}.",
            target_os
        );
    }

    // Add platform-specific build logic here
    match target_os.as_str() {
        "windows" => {
            // Windows-specific build steps
            println!("cargo:rustc-link-lib=bedrock");
        }
        _ => {
            // Fallback for unsupported platforms
            println!("cargo:warning=Limited functionality on this platform");
        }
    }
}

