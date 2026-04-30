mod codec;
mod serialization;
mod rectangle_set;

use std::io::Result as IoResult;
use crate::PixelSet;

use serde::{Serialize, Deserialize};

/// A losslessly compressed PixelSet using rectangle packing and zstd.
///
/// This type wraps compressed bytes that can be decompressed back to a PixelSet.
/// Achieves 75-150x compression on regular geographic data.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CompressedPixelSet {
    bytes: Vec<u8>,
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
