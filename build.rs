use std::fs;

use cc::Build;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let mut build = cc::Build::new();

    let compile = |builder: &mut Build, folder: &str| {
        for entry in fs::read_dir(folder).unwrap() {
            let path = entry.unwrap().path();
            if let Some(extension) = path.extension() {
                if extension == "c" || extension == "cpp" || extension == "S" {
                    builder.file(path);
                }
            }
        }
    };

    compile(&mut build, "c-blosc/blosc");

    if cfg!(feature = "lz4") {
        compile(&mut build, "c-blosc/internal-complibs/lz4-1.9.3");
        build.include("c-blosc/internal-complibs/lz4-1.9.3");
        build.define("HAVE_LZ4", None);
    }
    if cfg!(feature = "zlib") {
        compile(&mut build, "c-blosc/internal-complibs/zlib-1.2.11");
        build.include("c-blosc/internal-complibs/zlib-1.2.11");
        build.define("HAVE_ZLIB", None);
    }
    if cfg!(feature = "zstd") {
        compile(&mut build, "c-blosc/internal-complibs/zstd-1.5.2/common");
        compile(&mut build, "c-blosc/internal-complibs/zstd-1.5.2/compress");
        compile(
            &mut build,
            "c-blosc/internal-complibs/zstd-1.5.2/decompress",
        );
        compile(
            &mut build,
            "c-blosc/internal-complibs/zstd-1.5.2/dictBuilder",
        );
        build.include("c-blosc/internal-complibs/zstd-1.5.2");
        build.define("HAVE_ZSTD", None);
    }

    let linklib = if cfg!(target_env = "msvc") {
        "libblosc"
    } else {
        "blosc"
    };
    build.compile(linklib);
}
