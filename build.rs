// build.rs

fn main() {
    // === Part 1: Build odgi and its dependencies from source ===
    let dst = cmake::Config::new("vendor/odgi")
        .define("BUILD_TESTS", "OFF")
        .define("ODGI_BUILD_DOCS", "OFF")
        .build();

    // === Part 2: Tell Cargo where to find the compiled libraries ===
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-search=native={}/build/handlegraph-prefix/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/build/sdsl-lite-prefix/src/sdsl-lite-build/lib", dst.display());

    // === Part 3: Tell Cargo which libraries to link ===
    println!("cargo:rustc-link-lib=static=odgi");
    println!("cargo:rustc-link-lib=static=handlegraph");
    println!("cargo:rustc-link-lib=static=sdsl");
    println!("cargo:rustc-link-lib=dylib=jemalloc");
    println!("cargo:rustc-link-lib=dylib=gomp");
    println!("cargo:rustc-link-lib=dylib=atomic");

    // === Part 4: Build our C++ FFI wrapper code ===
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
        .compile("odgi-ffi-cc");
}