use base64::{engine::general_purpose::STANDARD, Engine};
use wasm_bindgen::prelude::*;
use crate::compression::CompressedPixelSet;
use super::pixel_set::WasmPixelSet;

/// A zstd-compressed `PixelSet`. Useful for storing or transferring pixel sets
/// without decompressing until needed.
///
/// Mirrors the serde representation of both `PixelSet` and `CompressedPixelSet`:
/// - Human-readable (JSON): base64-encoded compressed bytes — use `from_base64` / `to_base64`
/// - Binary (bincode etc.): raw compressed bytes — use `from_bytes` / `bytes`
#[wasm_bindgen]
pub struct WasmCompressedPixelSet {
    pub(super) inner: CompressedPixelSet,
}

#[wasm_bindgen]
impl WasmCompressedPixelSet {
    /// Wrap raw compressed bytes. Matches the binary serde format (e.g. bincode).
    /// The bytes must have been produced by `WasmPixelSet.compress()` or equivalent.
    pub fn from_bytes(bytes: &[u8]) -> WasmCompressedPixelSet {
        WasmCompressedPixelSet {
            inner: CompressedPixelSet::new(bytes.to_vec()),
        }
    }

    /// Deserialize from a base64 string. Matches the human-readable serde format
    /// (e.g. serde_json) for both `PixelSet` and `CompressedPixelSet`.
    pub fn from_base64(encoded: &str) -> Result<WasmCompressedPixelSet, String> {
        let bytes = STANDARD.decode(encoded).map_err(|e| e.to_string())?;
        Ok(WasmCompressedPixelSet {
            inner: CompressedPixelSet::new(bytes),
        })
    }

    /// Decompress into a `PixelSet`. Returns an error if the compressed data is malformed.
    pub fn decompress(&self) -> Result<WasmPixelSet, String> {
        self.inner
            .decompress()
            .map(|inner| WasmPixelSet { inner })
            .map_err(|e| e.to_string())
    }

    /// Serialize to a base64 string. Matches the human-readable serde format
    /// (e.g. serde_json) for both `PixelSet` and `CompressedPixelSet`.
    pub fn to_base64(&self) -> String {
        STANDARD.encode(self.inner.bytes())
    }

    /// Return the raw compressed bytes. Matches the binary serde format (e.g. bincode).
    pub fn bytes(&self) -> Vec<u8> {
        self.inner.bytes().to_vec()
    }

    /// Size of the compressed data in bytes.
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }
}
