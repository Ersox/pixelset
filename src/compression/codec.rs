use std::io::{Error, ErrorKind, Result as IoResult};
use zstd::{encode_all, decode_all};

/// Compress bytes using zstd compression.
pub fn compress_bytes(uncompressed: &[u8]) -> IoResult<Vec<u8>> {
    encode_all(uncompressed, 18)
        .map_err(Error::other)
}

/// Decompress bytes that were compressed with zstd.
pub fn decompress_bytes(compressed: &[u8]) -> IoResult<Vec<u8>> {
    decode_all(compressed)
        .map_err(|e| Error::new(ErrorKind::InvalidData, e))
}
