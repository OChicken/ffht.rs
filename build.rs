// Build script for FFHT Rust wrapper
// Compiles the C library with appropriate architecture-specific optimizations

use std::env;

fn main() {
    let mut build = cc::Build::new();

    // Common settings
    // Compile fht.c which includes fast_copy.c (single translation unit to avoid linking issues)
    build
        .file("fht.c")
        .include(".")         // Include current directory FIRST
        .include("FFHT")      // Include FFHT headers for fht_sse.c, fht_avx.c, etc.
        .opt_level(3)
        .flag("-std=c99")
        .flag("-Wall")
        .flag("-Wextra");

    // Detect target architecture and add appropriate SIMD implementation
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_arch.as_str() {
        "x86_64" => {
            // Use native architecture optimizations
            // fht.c will automatically include the right SIMD version
            if target_os != "windows" {
                build.flag("-march=native");
            }

            println!("cargo:rustc-cfg=has_simd");
            println!("cargo:rustc-cfg=has_avx");
        }
        "aarch64" => {
            // ARM64 with NEON
            // fht.c will automatically include fht_neon.c
            if target_os == "linux" || target_os == "android" {
                build.flag("-march=armv8-a+simd");
            } else if target_os == "macos" {
                // Apple Silicon
                build.flag("-mcpu=apple-m1");
            }

            println!("cargo:rustc-cfg=has_simd");
            println!("cargo:rustc-cfg=has_neon");
        }
        _ => {
            // Fallback: portable C implementation
            // fht.c will include fht_sse.c
            println!("cargo:warning=Building FFHT without SIMD optimizations for {}", target_arch);
        }
    }

    build.compile("ffht");

    // Tell cargo to rerun if any of the C files change
    println!("cargo:rerun-if-changed=FFHT/fht.c");
    println!("cargo:rerun-if-changed=FFHT/fht.h");
    // SIMD implementations (from FFHT submodule and our additions)
    println!("cargo:rerun-if-changed=FFHT/fht_sse.c");
    println!("cargo:rerun-if-changed=FFHT/fht_avx.c");
    println!("cargo:rerun-if-changed=fht_neon.c");  // Our new ARM NEON implementation
}
