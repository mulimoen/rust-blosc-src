use blosc_src::*;

fn compressors_available() -> Vec<&'static [u8]> {
    let mut compressors: Vec<&[u8]> = vec![];

    compressors.push(BLOSC_BLOSCLZ_COMPNAME);
    #[cfg(feature = "zstd")]
    compressors.push(BLOSC_ZSTD_COMPNAME);
    #[cfg(feature = "zlib")]
    compressors.push(BLOSC_ZLIB_COMPNAME);
    #[cfg(feature = "snappy")]
    compressors.push(BLOSC_SNAPPY_COMPNAME);

    compressors
}

#[test]
fn roundtrip() {
    unsafe {
        let text =
            "I am here writing some very cool and novel words which I will compress and decompress";

        let bytes = text.as_bytes();

        for compressor in compressors_available() {
            let mut compressed = vec![0; bytes.len() * 2];

            let stat = blosc_compress_ctx(
                9,
                BLOSC_NOSHUFFLE as _,
                std::mem::size_of::<u8>(),
                bytes.len(),
                bytes.as_ptr().cast(),
                compressed.as_mut_ptr().cast(),
                compressed.len(),
                compressor.as_ptr().cast(),
                0,
                1,
            );
            assert!(stat > 0);

            let mut outtext = vec![0_u8; bytes.len()];
            let stat = blosc_decompress_ctx(
                compressed.as_ptr().cast(),
                outtext.as_mut_ptr().cast(),
                outtext.len(),
                1,
            );
            assert!(stat > 0);

            assert_eq!(text, std::str::from_utf8(&outtext).unwrap());
        }
    }
}

#[test]
fn floats_roundtrip() {
    // generate numerical data
    let src: Vec<f32> = (0..10000)
        .map(|num| ((num * 8923) % 100) as f32 / 2f32) // multiply by big prime number
        .collect();

    for compressor in compressors_available() {
        // compress
        let dest: Vec<u8> = {
            let typesize = std::mem::size_of::<f32>();
            let src_size = src.len() * typesize;
            let dest_size = src_size + BLOSC_MAX_OVERHEAD as usize;
            let mut dest = vec![0; dest_size];

            let rsize = unsafe {
                blosc_compress_ctx(
                    9i32,
                    BLOSC_BITSHUFFLE as i32,
                    typesize,
                    src_size,
                    src.as_ptr().cast(),
                    dest.as_mut_ptr().cast(),
                    dest_size,
                    compressor.as_ptr().cast(),
                    0,
                    1,
                )
            };

            assert!(rsize > 0);
            dest.drain(rsize as usize..);
            dest
        };

        // make sure it actually compresses
        assert!(src.len() * std::mem::size_of::<f32>() > dest.len());

        // decompress
        let result = {
            let mut nbytes: usize = 0;
            let mut _cbytes: usize = 0;
            let mut _blocksize: usize = 0;
            unsafe {
                blosc_cbuffer_sizes(
                    dest.as_ptr().cast(),
                    &mut nbytes,
                    &mut _cbytes,
                    &mut _blocksize,
                )
            };
            assert!(nbytes != 0);
            let dest_size = nbytes / std::mem::size_of::<f32>();
            let mut result = vec![0f32; dest_size];
            let error = unsafe {
                blosc_decompress_ctx(dest.as_ptr().cast(), result.as_mut_ptr().cast(), nbytes, 1)
            };
            assert!(error >= 1);
            result
        };

        // check if the values in both arrays are equal
        assert_eq!(src, result);
    }
}
