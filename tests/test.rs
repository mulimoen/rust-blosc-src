use blosc_src::*;

#[test]
fn roundtrip() {
    unsafe {
        blosc_init();

        let text =
            "I am here writing some very cool and novel words which I will compress and decompress";

        let bytes = text.as_bytes();

        let mut compressed = vec![0; bytes.len() * 2];

        let stat = blosc_compress(
            9,
            BLOSC_NOSHUFFLE as _,
            std::mem::size_of::<u8>(),
            bytes.len(),
            bytes.as_ptr().cast(),
            compressed.as_mut_ptr().cast(),
            compressed.len(),
        );
        assert!(stat > 0);

        let mut outtext = vec![0_u8; bytes.len()];
        let stat = blosc_decompress(
            compressed.as_ptr().cast(),
            outtext.as_mut_ptr().cast(),
            outtext.len(),
        );
        assert!(stat > 0);

        assert_eq!(text, std::str::from_utf8(&outtext).unwrap());

        blosc_destroy();
    }
}
