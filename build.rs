use std::fs;

use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cc::Build::new();

    let target_mscv = cfg!(target_env = "msvc");
    let add_file = |builder: &mut Build, folder: &str| {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "c" || extension == "cpp" || (!target_mscv && extension == "S") {
                    builder.file(path);
                }
            }
        }
    };

    add_file(&mut build, "c-blosc/blosc");

    if cfg!(target_feature = "sse2") {
        build.define("SHUFFLE_SSE2_ENABLED", "1");
        if cfg!(target_env = "msvc") {
            if cfg!(target_pointer_width = "32") {
                build.flag("/arch:SSE2");
            }
        } else {
            build.flag("-msse2");
        }
    }
    if cfg!(target_feature = "avx2") {
        build.define("SHUFFLE_AVX2_ENABLED", "1");
        if cfg!(target_env = "msvc") {
            build.flag("/arch:AVX2");
        } else {
            build.flag("-mavx2");
        }
    }

    if cfg!(feature = "lz4") {
        let lz4_include_dir = std::env::var_os("DEP_LZ4_INCLUDE").unwrap();
        build.include(&lz4_include_dir);
        build.define("HAVE_LZ4", None);
    }
    if cfg!(feature = "zlib") {
        let zlib_include_dir = std::env::var_os("DEP_Z_INCLUDE").unwrap();
        build.include(&zlib_include_dir);
        build.define("HAVE_ZLIB", None);
    }

    if cfg!(feature = "zstd") {
        let zstd_include_dir = std::env::var_os("DEP_ZSTD_INCLUDE").unwrap();
        build.include(&zstd_include_dir);
        build.define("HAVE_ZSTD", None);
    }

    let linklib = if cfg!(target_env = "msvc") {
        "libblosc"
    } else {
        "blosc"
    };
    build.compile(linklib);
}
