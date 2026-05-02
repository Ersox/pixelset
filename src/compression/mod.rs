mod codec;
mod serialization;
mod rectangle_set;
pub(crate) mod serde;

use std::io::Result as IoResult;
use crate::PixelSet;

use ::serde::{Serialize, Deserialize, Serializer, Deserializer};
use ::serde::de;
use base64::Engine;

/// A losslessly compressed PixelSet using rectangle packing and zstd.
///
/// This type wraps compressed bytes that can be decompressed back to a PixelSet.
/// Achieves 75-150x compression on regular geographic data.
#[derive(Clone, Debug)]
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

/// Compress a PixelSet to compressed bytes.
pub(crate) fn compress_to_bytes(pixel_set: &PixelSet) -> IoResult<Vec<u8>> {
    rectangle_set::compress_to_bytes(pixel_set)
}

/// Decompress bytes back to a PixelSet.
pub(crate) fn decompress_from_bytes(compressed: &[u8]) -> IoResult<PixelSet> {
    rectangle_set::decompress_from_bytes(compressed)
}
