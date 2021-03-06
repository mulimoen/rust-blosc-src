/* automatically generated by rust-bindgen 0.55.1 */

pub const BLOSC_VERSION_MAJOR: u32 = 1;
pub const BLOSC_VERSION_MINOR: u32 = 20;
pub const BLOSC_VERSION_RELEASE: u32 = 0;
pub const BLOSC_VERSION_STRING: &'static [u8; 7usize] = b"1.20.0\0";
pub const BLOSC_VERSION_REVISION: &'static [u8; 6usize] = b"$Rev$\0";
pub const BLOSC_VERSION_DATE: &'static [u8; 22usize] = b"$Date:: 2020-07-25 #$\0";
pub const BLOSCLZ_VERSION_STRING: &'static [u8; 6usize] = b"2.3.0\0";
pub const BLOSC_VERSION_FORMAT: u32 = 2;
pub const BLOSC_MIN_HEADER_LENGTH: u32 = 16;
pub const BLOSC_MAX_OVERHEAD: u32 = 16;
pub const BLOSC_MAX_TYPESIZE: u32 = 255;
pub const BLOSC_MAX_THREADS: u32 = 256;
pub const BLOSC_NOSHUFFLE: u32 = 0;
pub const BLOSC_SHUFFLE: u32 = 1;
pub const BLOSC_BITSHUFFLE: u32 = 2;
pub const BLOSC_DOSHUFFLE: u32 = 1;
pub const BLOSC_MEMCPYED: u32 = 2;
pub const BLOSC_DOBITSHUFFLE: u32 = 4;
pub const BLOSC_BLOSCLZ: u32 = 0;
pub const BLOSC_LZ4: u32 = 1;
pub const BLOSC_LZ4HC: u32 = 2;
pub const BLOSC_SNAPPY: u32 = 3;
pub const BLOSC_ZLIB: u32 = 4;
pub const BLOSC_ZSTD: u32 = 5;
pub const BLOSC_BLOSCLZ_COMPNAME: &'static [u8; 8usize] = b"blosclz\0";
pub const BLOSC_LZ4_COMPNAME: &'static [u8; 4usize] = b"lz4\0";
pub const BLOSC_LZ4HC_COMPNAME: &'static [u8; 6usize] = b"lz4hc\0";
pub const BLOSC_SNAPPY_COMPNAME: &'static [u8; 7usize] = b"snappy\0";
pub const BLOSC_ZLIB_COMPNAME: &'static [u8; 5usize] = b"zlib\0";
pub const BLOSC_ZSTD_COMPNAME: &'static [u8; 5usize] = b"zstd\0";
pub const BLOSC_BLOSCLZ_LIB: u32 = 0;
pub const BLOSC_LZ4_LIB: u32 = 1;
pub const BLOSC_SNAPPY_LIB: u32 = 2;
pub const BLOSC_ZLIB_LIB: u32 = 3;
pub const BLOSC_ZSTD_LIB: u32 = 4;
pub const BLOSC_BLOSCLZ_LIBNAME: &'static [u8; 8usize] = b"BloscLZ\0";
pub const BLOSC_LZ4_LIBNAME: &'static [u8; 4usize] = b"LZ4\0";
pub const BLOSC_SNAPPY_LIBNAME: &'static [u8; 7usize] = b"Snappy\0";
pub const BLOSC_ZLIB_LIBNAME: &'static [u8; 5usize] = b"Zlib\0";
pub const BLOSC_ZSTD_LIBNAME: &'static [u8; 5usize] = b"Zstd\0";
pub const BLOSC_BLOSCLZ_FORMAT: u32 = 0;
pub const BLOSC_LZ4_FORMAT: u32 = 1;
pub const BLOSC_LZ4HC_FORMAT: u32 = 1;
pub const BLOSC_SNAPPY_FORMAT: u32 = 2;
pub const BLOSC_ZLIB_FORMAT: u32 = 3;
pub const BLOSC_ZSTD_FORMAT: u32 = 4;
pub const BLOSC_BLOSCLZ_VERSION_FORMAT: u32 = 1;
pub const BLOSC_LZ4_VERSION_FORMAT: u32 = 1;
pub const BLOSC_LZ4HC_VERSION_FORMAT: u32 = 1;
pub const BLOSC_SNAPPY_VERSION_FORMAT: u32 = 1;
pub const BLOSC_ZLIB_VERSION_FORMAT: u32 = 1;
pub const BLOSC_ZSTD_VERSION_FORMAT: u32 = 1;
pub const BLOSC_ALWAYS_SPLIT: u32 = 1;
pub const BLOSC_NEVER_SPLIT: u32 = 2;
pub const BLOSC_AUTO_SPLIT: u32 = 3;
pub const BLOSC_FORWARD_COMPAT_SPLIT: u32 = 4;
extern "C" {
    #[doc = "Initialize the Blosc library environment."]
    #[doc = ""]
    #[doc = "You must call this previous to any other Blosc call, unless you want"]
    #[doc = "Blosc to be used simultaneously in a multi-threaded environment, in"]
    #[doc = "which case you should *exclusively* use the"]
    #[doc = "blosc_compress_ctx()/blosc_decompress_ctx() pair (see below)."]
    pub fn blosc_init();
}
extern "C" {
    #[doc = "Destroy the Blosc library environment."]
    #[doc = ""]
    #[doc = "You must call this after to you are done with all the Blosc calls,"]
    #[doc = "unless you have not used blosc_init() before (see blosc_init()"]
    #[doc = "above)."]
    pub fn blosc_destroy();
}
extern "C" {
    #[doc = "Compress a block of data in the `src` buffer and returns the size of"]
    #[doc = "the compressed block.  The size of `src` buffer is specified by"]
    #[doc = "`nbytes`.  There is not a minimum for `src` buffer size (`nbytes`)."]
    #[doc = ""]
    #[doc = "`clevel` is the desired compression level and must be a number"]
    #[doc = "between 0 (no compression) and 9 (maximum compression)."]
    #[doc = ""]
    #[doc = "`doshuffle` specifies whether the shuffle compression filters"]
    #[doc = "should be applied or not.  BLOSC_NOSHUFFLE means not applying it,"]
    #[doc = "BLOSC_SHUFFLE means applying it at a byte level and BLOSC_BITSHUFFLE"]
    #[doc = "at a bit level (slower but may achieve better entropy alignment)."]
    #[doc = ""]
    #[doc = "`typesize` is the number of bytes for the atomic type in binary"]
    #[doc = "`src` buffer.  This is mainly useful for the shuffle filters."]
    #[doc = "For implementation reasons, only a 1 < `typesize` < 256 will allow the"]
    #[doc = "shuffle filter to work.  When `typesize` is not in this range, shuffle"]
    #[doc = "will be silently disabled."]
    #[doc = ""]
    #[doc = "The `dest` buffer must have at least the size of `destsize`.  Blosc"]
    #[doc = "guarantees that if you set `destsize` to, at least,"]
    #[doc = "(`nbytes` + BLOSC_MAX_OVERHEAD), the compression will always succeed."]
    #[doc = "The `src` buffer and the `dest` buffer can not overlap."]
    #[doc = ""]
    #[doc = "Compression is memory safe and guaranteed not to write the `dest`"]
    #[doc = "buffer beyond what is specified in `destsize`."]
    #[doc = ""]
    #[doc = "If `src` buffer cannot be compressed into `destsize`, the return"]
    #[doc = "value is zero and you should discard the contents of the `dest`"]
    #[doc = "buffer."]
    #[doc = ""]
    #[doc = "A negative return value means that an internal error happened.  This"]
    #[doc = "should never happen.  If you see this, please report it back"]
    #[doc = "together with the buffer data causing this and compression settings."]
    #[doc = ""]
    #[doc = "Environment variables"]
    #[doc = "---------------------"]
    #[doc = ""]
    #[doc = "blosc_compress() honors different environment variables to control"]
    #[doc = "internal parameters without the need of doing that programatically."]
    #[doc = "Here are the ones supported:"]
    #[doc = ""]
    #[doc = "BLOSC_CLEVEL=(INTEGER): This will overwrite the `clevel` parameter"]
    #[doc = "before the compression process starts."]
    #[doc = ""]
    #[doc = "BLOSC_SHUFFLE=[NOSHUFFLE | SHUFFLE | BITSHUFFLE]: This will"]
    #[doc = "overwrite the `doshuffle` parameter before the compression process"]
    #[doc = "starts."]
    #[doc = ""]
    #[doc = "BLOSC_TYPESIZE=(INTEGER): This will overwrite the `typesize`"]
    #[doc = "parameter before the compression process starts."]
    #[doc = ""]
    #[doc = "BLOSC_COMPRESSOR=[BLOSCLZ | LZ4 | LZ4HC | SNAPPY | ZLIB]: This will"]
    #[doc = "call blosc_set_compressor(BLOSC_COMPRESSOR) before the compression"]
    #[doc = "process starts."]
    #[doc = ""]
    #[doc = "BLOSC_NTHREADS=(INTEGER): This will call"]
    #[doc = "blosc_set_nthreads(BLOSC_NTHREADS) before the compression process"]
    #[doc = "starts."]
    #[doc = ""]
    #[doc = "BLOSC_BLOCKSIZE=(INTEGER): This will call"]
    #[doc = "blosc_set_blocksize(BLOSC_BLOCKSIZE) before the compression process"]
    #[doc = "starts.  *NOTE:* The blocksize is a critical parameter with"]
    #[doc = "important restrictions in the allowed values, so use this with care."]
    #[doc = ""]
    #[doc = "BLOSC_NOLOCK=(ANY VALUE): This will call blosc_compress_ctx() under"]
    #[doc = "the hood, with the `compressor`, `blocksize` and"]
    #[doc = "`numinternalthreads` parameters set to the same as the last calls to"]
    #[doc = "blosc_set_compressor(), blosc_set_blocksize() and"]
    #[doc = "blosc_set_nthreads().  BLOSC_CLEVEL, BLOSC_SHUFFLE, BLOSC_TYPESIZE"]
    #[doc = "environment vars will also be honored."]
    #[doc = ""]
    #[doc = "BLOSC_SPLITMODE=[ FORWARD_COMPAT | AUTO | ALWAYS | NEVER ]:"]
    #[doc = "This will call blosc_set_splitmode() with the different supported values."]
    #[doc = "See blosc_set_splitmode() docstrings for more info on each mode."]
    #[doc = ""]
    #[doc = "BLOSC_WARN=(INTEGER): This will print some warning message on stderr"]
    #[doc = "showing more info in situations where data inputs cannot be compressed."]
    #[doc = "The values can range from 1 (less verbose) to 10 (full verbose).  0 is"]
    #[doc = "the same as if the BLOSC_WARN envvar was not defined."]
    pub fn blosc_compress(
        clevel: ::std::os::raw::c_int,
        doshuffle: ::std::os::raw::c_int,
        typesize: usize,
        nbytes: usize,
        src: *const ::std::os::raw::c_void,
        dest: *mut ::std::os::raw::c_void,
        destsize: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Context interface to blosc compression. This does not require a call"]
    #[doc = "to blosc_init() and can be called from multithreaded applications"]
    #[doc = "without the global lock being used, so allowing Blosc be executed"]
    #[doc = "simultaneously in those scenarios."]
    #[doc = ""]
    #[doc = "It uses the same parameters than the blosc_compress() function plus:"]
    #[doc = ""]
    #[doc = "`compressor`: the string representing the type of compressor to use."]
    #[doc = ""]
    #[doc = "`blocksize`: the requested size of the compressed blocks.  If 0, an"]
    #[doc = "automatic blocksize will be used."]
    #[doc = ""]
    #[doc = "`numinternalthreads`: the number of threads to use internally."]
    #[doc = ""]
    #[doc = "A negative return value means that an internal error happened.  This"]
    #[doc = "should never happen.  If you see this, please report it back"]
    #[doc = "together with the buffer data causing this and compression settings."]
    pub fn blosc_compress_ctx(
        clevel: ::std::os::raw::c_int,
        doshuffle: ::std::os::raw::c_int,
        typesize: usize,
        nbytes: usize,
        src: *const ::std::os::raw::c_void,
        dest: *mut ::std::os::raw::c_void,
        destsize: usize,
        compressor: *const ::std::os::raw::c_char,
        blocksize: usize,
        numinternalthreads: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Decompress a block of compressed data in `src`, put the result in"]
    #[doc = "`dest` and returns the size of the decompressed block."]
    #[doc = ""]
    #[doc = "Call `blosc_cbuffer_validate` to determine the size of the destination buffer."]
    #[doc = ""]
    #[doc = "The `src` buffer and the `dest` buffer can not overlap."]
    #[doc = ""]
    #[doc = "Decompression is memory safe and guaranteed not to write the `dest`"]
    #[doc = "buffer beyond what is specified in `destsize`."]
    #[doc = ""]
    #[doc = "If an error occurs, e.g. the compressed data is corrupted or the"]
    #[doc = "output buffer is not large enough, then 0 (zero) or a negative value"]
    #[doc = "will be returned instead."]
    #[doc = ""]
    #[doc = "Environment variables"]
    #[doc = "---------------------"]
    #[doc = ""]
    #[doc = "blosc_decompress() honors different environment variables to control"]
    #[doc = "internal parameters without the need of doing that programatically."]
    #[doc = "Here are the ones supported:"]
    #[doc = ""]
    #[doc = "BLOSC_NTHREADS=(INTEGER): This will call"]
    #[doc = "blosc_set_nthreads(BLOSC_NTHREADS) before the proper decompression"]
    #[doc = "process starts."]
    #[doc = ""]
    #[doc = "BLOSC_NOLOCK=(ANY VALUE): This will call blosc_decompress_ctx()"]
    #[doc = "under the hood, with the `numinternalthreads` parameter set to the"]
    #[doc = "same value as the last call to blosc_set_nthreads()."]
    pub fn blosc_decompress(
        src: *const ::std::os::raw::c_void,
        dest: *mut ::std::os::raw::c_void,
        destsize: usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Context interface to blosc decompression. This does not require a"]
    #[doc = "call to blosc_init() and can be called from multithreaded"]
    #[doc = "applications without the global lock being used, so allowing Blosc"]
    #[doc = "be executed simultaneously in those scenarios."]
    #[doc = ""]
    #[doc = "Call `blosc_cbuffer_validate` to determine the size of the destination buffer."]
    #[doc = ""]
    #[doc = "It uses the same parameters than the blosc_decompress() function plus:"]
    #[doc = ""]
    #[doc = "`numinternalthreads`: number of threads to use internally."]
    #[doc = ""]
    #[doc = "Decompression is memory safe and guaranteed not to write the `dest`"]
    #[doc = "buffer more than what is specified in `destsize`."]
    #[doc = ""]
    #[doc = "If an error occurs, e.g. the compressed data is corrupted or the"]
    #[doc = "output buffer is not large enough, then 0 (zero) or a negative value"]
    #[doc = "will be returned instead."]
    pub fn blosc_decompress_ctx(
        src: *const ::std::os::raw::c_void,
        dest: *mut ::std::os::raw::c_void,
        destsize: usize,
        numinternalthreads: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Get `nitems` (of typesize size) in `src` buffer starting in `start`."]
    #[doc = "The items are returned in `dest` buffer, which has to have enough"]
    #[doc = "space for storing all items."]
    #[doc = ""]
    #[doc = "Returns the number of bytes copied to `dest` or a negative value if"]
    #[doc = "some error happens."]
    pub fn blosc_getitem(
        src: *const ::std::os::raw::c_void,
        start: ::std::os::raw::c_int,
        nitems: ::std::os::raw::c_int,
        dest: *mut ::std::os::raw::c_void,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Returns the current number of threads that are used for"]
    #[doc = "compression/decompression."]
    pub fn blosc_get_nthreads() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Initialize a pool of threads for compression/decompression.  If"]
    #[doc = "`nthreads` is 1, then the serial version is chosen and a possible"]
    #[doc = "previous existing pool is ended.  If this is not called, `nthreads`"]
    #[doc = "is set to 1 internally."]
    #[doc = ""]
    #[doc = "Returns the previous number of threads."]
    pub fn blosc_set_nthreads(nthreads: ::std::os::raw::c_int) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Returns the current compressor that is being used for compression."]
    pub fn blosc_get_compressor() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "Select the compressor to be used.  The supported ones are \"blosclz\","]
    #[doc = "\"lz4\", \"lz4hc\", \"snappy\", \"zlib\" and \"zstd\".  If this function is not"]
    #[doc = "called, then \"blosclz\" will be used by default."]
    #[doc = ""]
    #[doc = "In case the compressor is not recognized, or there is not support"]
    #[doc = "for it in this build, it returns a -1.  Else it returns the code for"]
    #[doc = "the compressor (>=0)."]
    pub fn blosc_set_compressor(compname: *const ::std::os::raw::c_char) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Get the `compname` associated with the `compcode`."]
    #[doc = ""]
    #[doc = "If the compressor code is not recognized, or there is not support"]
    #[doc = "for it in this build, -1 is returned.  Else, the compressor code is"]
    #[doc = "returned."]
    pub fn blosc_compcode_to_compname(
        compcode: ::std::os::raw::c_int,
        compname: *mut *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Return the compressor code associated with the compressor name."]
    #[doc = ""]
    #[doc = "If the compressor name is not recognized, or there is not support"]
    #[doc = "for it in this build, -1 is returned instead."]
    pub fn blosc_compname_to_compcode(
        compname: *const ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Get a list of compressors supported in the current build.  The"]
    #[doc = "returned value is a string with a concatenation of \"blosclz\", \"lz4\","]
    #[doc = "\"lz4hc\", \"snappy\", \"zlib\" or \"zstd \"separated by commas, depending"]
    #[doc = "on which ones are present in the build."]
    #[doc = ""]
    #[doc = "This function does not leak, so you should not free() the returned"]
    #[doc = "list."]
    #[doc = ""]
    #[doc = "This function should always succeed."]
    pub fn blosc_list_compressors() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "Return the version of the C-Blosc library in string format."]
    #[doc = ""]
    #[doc = "Useful for dynamic libraries."]
    pub fn blosc_get_version_string() -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "Get info from compression libraries included in the current build."]
    #[doc = "In `compname` you pass the compressor name that you want info from."]
    #[doc = ""]
    #[doc = "In `complib` and `version` you get a pointer to the compressor"]
    #[doc = "library name and the version in string format respectively.  After"]
    #[doc = "using the name and version, you should free() them so as to avoid"]
    #[doc = "leaks.  If any of `complib` and `version` are NULL, they will not be"]
    #[doc = "assigned to anything, and the user should not need to free them."]
    #[doc = ""]
    #[doc = "If the compressor is supported, it returns the code for the library"]
    #[doc = "(>=0).  If it is not supported, this function returns -1."]
    pub fn blosc_get_complib_info(
        compname: *const ::std::os::raw::c_char,
        complib: *mut *mut ::std::os::raw::c_char,
        version: *mut *mut ::std::os::raw::c_char,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Free possible memory temporaries and thread resources.  Use this"]
    #[doc = "when you are not going to use Blosc for a long while.  In case of"]
    #[doc = "problems releasing the resources, it returns a negative number, else"]
    #[doc = "it returns 0."]
    pub fn blosc_free_resources() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Return information about a compressed buffer, namely the number of"]
    #[doc = "uncompressed bytes (`nbytes`) and compressed (`cbytes`).  It also"]
    #[doc = "returns the `blocksize` (which is used internally for doing the"]
    #[doc = "compression by blocks)."]
    #[doc = ""]
    #[doc = "You only need to pass the first BLOSC_MIN_HEADER_LENGTH bytes of a"]
    #[doc = "compressed buffer for this call to work."]
    #[doc = ""]
    #[doc = "If the format is not supported by the library, all output arguments will be"]
    #[doc = "filled with zeros."]
    pub fn blosc_cbuffer_sizes(
        cbuffer: *const ::std::os::raw::c_void,
        nbytes: *mut usize,
        cbytes: *mut usize,
        blocksize: *mut usize,
    );
}
extern "C" {
    #[doc = "Checks that the compressed buffer starting at `cbuffer` of length `cbytes` may"]
    #[doc = "contain valid blosc compressed data, and that it is safe to call"]
    #[doc = "blosc_decompress/blosc_decompress_ctx/blosc_getitem."]
    #[doc = ""]
    #[doc = "On success, returns 0 and sets *nbytes to the size of the uncompressed data."]
    #[doc = "This does not guarantee that the decompression function won't return an error,"]
    #[doc = "but does guarantee that it is safe to attempt decompression."]
    #[doc = ""]
    #[doc = "On failure, returns -1."]
    pub fn blosc_cbuffer_validate(
        cbuffer: *const ::std::os::raw::c_void,
        cbytes: usize,
        nbytes: *mut usize,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Return meta-information about a compressed buffer, namely the type size"]
    #[doc = "(`typesize`), as well as some internal `flags`."]
    #[doc = ""]
    #[doc = "The `flags` is a set of bits, where the used ones are:"]
    #[doc = " bit 0: whether the shuffle filter has been applied or not"]
    #[doc = " bit 1: whether the internal buffer is a pure memcpy or not"]
    #[doc = " bit 2: whether the bit shuffle filter has been applied or not"]
    #[doc = ""]
    #[doc = "You can use the `BLOSC_DOSHUFFLE`, `BLOSC_DOBITSHUFFLE` and"]
    #[doc = "`BLOSC_MEMCPYED` symbols for extracting the interesting bits"]
    #[doc = "(e.g. ``flags & BLOSC_DOSHUFFLE`` says whether the buffer is"]
    #[doc = "byte-shuffled or not)."]
    #[doc = ""]
    #[doc = "You only need to pass the first BLOSC_MIN_HEADER_LENGTH bytes of a"]
    #[doc = "compressed buffer for this call to work."]
    #[doc = ""]
    #[doc = "If the format is not supported by the library, all output arguments will be"]
    #[doc = "filled with zeros."]
    pub fn blosc_cbuffer_metainfo(
        cbuffer: *const ::std::os::raw::c_void,
        typesize: *mut usize,
        flags: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = "Return information about a compressed buffer, namely the internal"]
    #[doc = "Blosc format version (`version`) and the format for the internal"]
    #[doc = "compressor used (`compversion`)."]
    #[doc = ""]
    #[doc = "This function should always succeed."]
    pub fn blosc_cbuffer_versions(
        cbuffer: *const ::std::os::raw::c_void,
        version: *mut ::std::os::raw::c_int,
        compversion: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    #[doc = "Return the compressor library/format used in a compressed buffer."]
    #[doc = ""]
    #[doc = "This function should always succeed."]
    pub fn blosc_cbuffer_complib(
        cbuffer: *const ::std::os::raw::c_void,
    ) -> *const ::std::os::raw::c_char;
}
extern "C" {
    #[doc = "Get the internal blocksize to be used during compression.  0 means"]
    #[doc = "that an automatic blocksize is computed internally (the default)."]
    pub fn blosc_get_blocksize() -> ::std::os::raw::c_int;
}
extern "C" {
    #[doc = "Force the use of a specific blocksize.  If 0, an automatic"]
    #[doc = "blocksize will be used (the default)."]
    #[doc = ""]
    #[doc = "The blocksize is a critical parameter with important restrictions in"]
    #[doc = "the allowed values, so use this with care."]
    pub fn blosc_set_blocksize(blocksize: usize);
}
extern "C" {
    #[doc = "Set the split mode."]
    #[doc = ""]
    #[doc = "This function can take the next values:"]
    #[doc = "  BLOSC_FORWARD_COMPAT_SPLIT"]
    #[doc = "  BLOSC_AUTO_SPLIT"]
    #[doc = "  BLOSC_NEVER_SPLIT"]
    #[doc = "  BLOSC_ALWAYS_SPLIT"]
    #[doc = ""]
    #[doc = "BLOSC_FORWARD_COMPAT offers reasonably forward compatibility,"]
    #[doc = "BLOSC_AUTO_SPLIT is for nearly optimal results (based on heuristics),"]
    #[doc = "BLOSC_NEVER_SPLIT and BLOSC_ALWAYS_SPLIT are for the user experimenting"]
    #[doc = "when trying to get best compression ratios and/or speed."]
    #[doc = ""]
    #[doc = "If not called, the default mode is BLOSC_FORWARD_COMPAT_SPLIT."]
    #[doc = ""]
    #[doc = "This function should always succeed."]
    pub fn blosc_set_splitmode(splitmode: ::std::os::raw::c_int);
}
