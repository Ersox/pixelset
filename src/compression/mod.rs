mod codec;
pub(crate) mod serde;

use std::io::{Error, ErrorKind, Result as IoResult};
use crate::{PixelSet};
use crate::set::Run;

use ::serde::{Serialize, Deserialize, Serializer, Deserializer};
use ::serde::de;
use base64::Engine;

/// A losslessly compressed PixelSet using run-length encoding and zstd.
///
/// This type wraps compressed bytes that can be decompressed back to a PixelSet.
/// Achieves 75-150x compression on regular geographic data.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompressedPixelSet {
    bytes: Vec<u8>,
}

impl Serialize for CompressedPixelSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        if serializer.is_human_readable() {
            let encoded = base64::engine::general_purpose::STANDARD.encode(&self.bytes);
            serializer.serialize_str(&encoded)
        } else {
            serializer.serialize_bytes(&self.bytes)
        }
    }
}

impl<'de> Deserialize<'de> for CompressedPixelSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            let encoded: String = Deserialize::deserialize(deserializer)?;
            let bytes = base64::engine::general_purpose::STANDARD.decode(encoded)
                .map_err(de::Error::custom)?;
            Ok(CompressedPixelSet { bytes })
        } else {
            let bytes: Vec<u8> = Deserialize::deserialize(deserializer)?;
            Ok(CompressedPixelSet { bytes })
        }
    }
}

impl CompressedPixelSet {
    /// Create a CompressedPixelSet from raw compressed bytes.
    pub fn new(bytes: Vec<u8>) -> Self {
        Self { bytes }
    }

    /// Get the compressed bytes.
    pub fn bytes(&self) -> &[u8] {
        &self.bytes
    }

    /// Get the compressed size in bytes.
    pub fn len(&self) -> usize {
        self.bytes.len()
    }

    /// Check if the compressed data is empty.
    pub fn is_empty(&self) -> bool {
        self.bytes.is_empty()
    }

    /// Consume self and return the compressed bytes.
    pub fn into_bytes(self) -> Vec<u8> {
        self.bytes
    }

    /// Decompress this CompressedPixelSet back into a PixelSet.
    pub fn decompress(&self) -> IoResult<PixelSet> {
        decompress_from_bytes(&self.bytes)
    }
}

const HEADER_BYTES: usize = 4;
const RUN_BYTES: usize = 6; // y (2) + x_start (2) + length (2)

/// Compress a PixelSet to compressed bytes using RLE binary format and zstd.
pub(crate) fn compress_to_bytes(pixel_set: &PixelSet) -> IoResult<Vec<u8>> {
    let runs = pixel_set.runs();

    let mut buf = Vec::with_capacity(HEADER_BYTES + runs.len() * RUN_BYTES);
    buf.extend_from_slice(&(runs.len() as u32).to_le_bytes());
    for run in runs {
        buf.extend_from_slice(&run.to_bytes());
    }

    codec::compress_bytes(&buf)
}

/// Decompress bytes back to a PixelSet using RLE binary format and zstd.
pub(crate) fn decompress_from_bytes(compressed: &[u8]) -> IoResult<PixelSet> {
    let buf = codec::decompress_bytes(compressed)?;

    if buf.len() < HEADER_BYTES {
        return Err(Error::new(ErrorKind::InvalidData, "insufficient data for run count"));
    }

    let n_runs = u32::from_le_bytes([buf[0], buf[1], buf[2], buf[3]]) as usize;
    let expected_len = HEADER_BYTES + n_runs * RUN_BYTES;

    if buf.len() != expected_len {
        return Err(Error::new(
            ErrorKind::InvalidData,
            format!("malformed run data: expected {} bytes, got {}", expected_len, buf.len()),
        ));
    }

    let runs = (0..n_runs)
        .map(|i| {
            let off = HEADER_BYTES + i * RUN_BYTES;
            Run::from_bytes(buf[off..off + RUN_BYTES].try_into().unwrap())
        })
        .collect();

    Ok(PixelSet::from_runs_unchecked(runs))
}
