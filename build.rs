fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut cfg = cmake::Config::new("c-blosc");

    for option in &[
        "BUILD_SHARED",
        "BUILD_TESTS",
        "BUILD_FUZZERS",
        "BUILD_BENCHMARKS",
        "DEACTIVATE_ZSTD",
    ] {
        cfg.define(option, "OFF");
    }

    if !cfg!(target_feature = "sse2") {
        cfg.define("DEACTIVATE_SSE2", "OFF");
    }
    if !cfg!(target_feature = "avx") {
        cfg.define("DEACTIVATE_AVX", "OFF");
    }

    let dst = cfg.build();
    println!("cargo:root={}", dst.display());
    let incdir = format!("{}/include", dst.display());
    println!("cargo:include={}", incdir);
    let linklib = if cfg!(target_env = "msvc") {
        "libblosc"
    } else {
        "blosc"
    };
    println!("cargo:library={}", linklib);

    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-lib=static={}", linklib);
}
