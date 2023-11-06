fn define_bool(b: bool) -> &'static str {
    if b {
        "ON"
    } else {
        "OFF"
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    let mut config = cmake::Config::new("c-blosc");
    config.define("DEACTIVATE_LZ4", define_bool(!cfg!(feature = "lz4")));
    config.define("DEACTIVATE_SNAPPY", define_bool(!cfg!(feature = "snappy")));
    config.define("DEACTIVATE_ZLIB", define_bool(!cfg!(feature = "zlib")));
    config.define("DEACTIVATE_ZSTD", define_bool(!cfg!(feature = "zstd")));
    let dst = config.build();
    println!("cargo:rustc-link-search=native={}", dst.display());
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    println!("cargo:rustc-link-search=native={}/lib64", dst.display());
    println!("cargo:rustc-link-lib=static=blosc");
}
