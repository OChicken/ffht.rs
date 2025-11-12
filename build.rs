// Build script for FFHT Rust wrapper
// Compiles the C library with appropriate architecture-specific optimizations

use std::env;

fn main() {
    let mut build = cc::Build::new();

    // Common settings
    build
        .file("fht.c")
        // Note: Not compiling fast_copy.c separately - it's included by fht_impl.h
        // We'll define USE_MEMCPY to use memcpy instead of SIMD for now
        .include(".")
        .opt_level(3)
        .flag("-std=c99")
        .flag("-Wall")
        .flag("-Wextra")
        .define("USE_MEMCPY_FOR_FAST_COPY", None);

    // Detect target architecture and add appropriate SIMD implementation
    let target_arch = env::var("CARGO_CFG_TARGET_ARCH").unwrap();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();

    match target_arch.as_str() {
        "x86_64" => {
            // Use native architecture optimizations
            // fht.c will automatically include the right SIMD version via fht_impl.h
            if target_os != "windows" {
                build.flag("-march=native");
            }

            println!("cargo:rustc-cfg=has_simd");
            println!("cargo:rustc-cfg=has_avx");
        }
        "aarch64" => {
            // ARM64 with NEON
            // fht.c will automatically include fht_neon.c via fht_impl.h
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
            // fht.c will include fht_sse.c via fht_impl.h
            println!("cargo:warning=Building FFHT without SIMD optimizations for {}", target_arch);
        }
    }

    build.compile("ffht");

    // Tell cargo to rerun if any of the C files change
    println!("cargo:rerun-if-changed=fht.c");
    println!("cargo:rerun-if-changed=fht.h");
    println!("cargo:rerun-if-changed=fht_impl.h");
    println!("cargo:rerun-if-changed=fast_copy.c");
    println!("cargo:rerun-if-changed=fast_copy.h");
    // SIMD implementations are included by fht_impl.h
    println!("cargo:rerun-if-changed=fht_sse.c");
    println!("cargo:rerun-if-changed=fht_avx.c");
    println!("cargo:rerun-if-changed=fht_neon.c");
}
