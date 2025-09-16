use std::env;
use std::path::PathBuf;
use fs_extra::dir::{copy, CopyOptions};

fn main() {
    // === Part 0: Copy C++ source to a temporary, writable directory ===
    // This is the definitive fix for the "dirty working directory" error during packaging.
    // We copy the vendored source to OUT_DIR, which is a scratch space for build scripts.
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let odgi_source_dir = PathBuf::from("vendor/odgi");
    let odgi_build_source_dir = out_dir.join("odgi-source-for-build");

    let mut options = CopyOptions::new();
    options.overwrite = true;
    options.content_only = true;
    copy(&odgi_source_dir, &odgi_build_source_dir, &options)
        .expect("Failed to copy odgi source to temporary build directory");


    // === Part 1: Build odgi from the COPIED source ===
    // Now, we point CMake to our temporary copy. All build artifacts will be created
    // within the OUT_DIR, leaving the original vendor/odgi directory untouched.
    let dst = cmake::Config::new(&odgi_build_source_dir)
        .define("BUILD_TESTS", "OFF")
        .define("ODGI_BUILD_DOCS", "OFF")
        .build();

    // === Part 2: Make the compiled odgi executable path available to our Rust code ===
    let odgi_exe_path = dst.join("bin").join("odgi");
    println!("cargo:rustc-env=ODGI_EXE={}", odgi_exe_path.display());


    // === Part 3: Tell Cargo where to find the compiled libraries ===
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-search=native={}/build/handlegraph-prefix/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/build/sdsl-lite-prefix/src/sdsl-lite-build/lib", dst.display());


    // === Part 4: Tell Cargo which libraries to link ===
    println!("cargo:rustc-link-lib=static=odgi");
    println!("cargo:rustc-link-lib=static=handlegraph");
    println!("cargo:rustc-link-lib=static=sdsl");
    println!("cargo:rustc-link-lib=dylib=gomp");
    println!("cargo:rustc-link-lib=dylib=atomic");


    // === Part 5: Build our C++ FFI wrapper code ===
    // These include paths still point to the original vendor directory for simplicity,
    // as they are only needed for header discovery and don't create artifacts.
    cxx_build::bridge("src/lib.rs")
        .file("src/odgi.cpp")
        .flag("-fopenmp")
        .flag_if_supported("-std=c++17")
        .include("vendor/odgi/src")
        .include("vendor/odgi/deps/libhandlegraph/src/include")
        .include("vendor/odgi/deps/DYNAMIC/include")
        .include("vendor/odgi/deps/hopscotch-map/include")
        .include("vendor/odgi/deps/sparsepp/sparsepp")
        .include("vendor/odgi/deps/flat_hash_map")
        .include("vendor/odgi/deps/atomicbitvector/include")
        .include("vendor/odgi/deps/IITree/src")
        .include("vendor/odgi/deps/BBHash")
        .include("vendor/odgi/deps/popv")
        .include("vendor/odgi/deps/nameof/include")
        .include("vendor/odgi/lib/sdsl-lite/include")
        .compile("odgi_cxx_bridge");


    // === Part 6: Tell Cargo to rerun this script if C++ sources change ===
    println!("cargo:rerun-if-changed=src/odgi.cpp");
    println!("cargo:rerun-if-changed=src/odgi_wrapper.hpp");
    println!("cargo:rerun-if-changed=vendor/odgi");
}
