use std::fs;

use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cc::Build::new();

    let compile = |builder: &mut Build, folder: &str| {
        dbg!(folder);
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "c" || extension == "cpp" {
                    builder.file(path);
                }
            }
        }
    };

    compile(&mut build, "c-blosc/blosc");
    compile(&mut build, "c-blosc/internal-complibs/lz4-1.9.4");
    compile(&mut build, "c-blosc/internal-complibs/zlib-1.2.13");
    compile(&mut build, "c-blosc/internal-complibs/zstd-1.5.4/common");
    compile(&mut build, "c-blosc/internal-complibs/zstd-1.5.4/compress");
    compile(
        &mut build,
        "c-blosc/internal-complibs/zstd-1.5.4/decompress",
    );
    compile(
        &mut build,
        "c-blosc/internal-complibs/zstd-1.5.4/dictBuilder",
    );

    if cfg!(target_feature = "lz4") {
        build.include("c-blosc/internal-complibs/lz4-1.9.4");
        build.define("HAVE_LZ4", None);
    }
    if cfg!(target_feature = "zlib") {
        build.include("c-blosc/internal-complibs/zlib-1.2.13");
        build.define("HAVE_ZLIB", None);
    }
    if cfg!(target_feature = "zstd") {
        build.include("c-blosc/internal-complibs/zstd-1.5.4");
        build.define("HAVE_ZSTD", None);
    }

    let linklib = if cfg!(target_env = "msvc") {
        "libblosc"
    } else {
        "blosc"
    };
    build.compile(linklib);
}
