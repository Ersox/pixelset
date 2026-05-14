use base64::{engine::general_purpose::STANDARD, Engine};
use wasm_bindgen::prelude::*;
use crate::{Pixel, PixelSet};
use crate::compression::CompressedPixelSet;
use super::compressed_pixel_set::WasmCompressedPixelSet;

/// A single 2D pixel coordinate with `y` (row) and `x` (column) fields.
/// Coordinates are zero-indexed and limited to `u16` (0–65535).
#[wasm_bindgen]
pub struct WasmPixel {
    pub(super) inner: Pixel,
}

#[wasm_bindgen]
impl WasmPixel {
    /// Create a pixel at the given row and column.
    #[wasm_bindgen(constructor)]
    pub fn new(y: u16, x: u16) -> WasmPixel {
        WasmPixel { inner: Pixel { y, x } }
    }

    /// Row index (y-axis).
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u16 { self.inner.y }

    /// Column index (x-axis).
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u16 { self.inner.x }
}

/// A run-length encoded set of 2D pixel coordinates, sorted by `(y, x)`.
///
/// Internally stores horizontal runs rather than individual pixels, giving
/// O(k) memory and set operations where k is the number of runs — typically
/// much smaller than the pixel count for coherent regions.
#[wasm_bindgen]
pub struct WasmPixelSet {
    pub(super) inner: PixelSet,
}

#[wasm_bindgen]
impl WasmPixelSet {
    /// Create an empty `PixelSet`.
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmPixelSet {
        WasmPixelSet { inner: PixelSet::new(vec![]) }
    }

    /// Create a `PixelSet` from a flat coordinate array of interleaved `[y, x]` pairs:
    /// `[y0, x0, y1, x1, ...]`. Returns an error if the array has odd length.
    /// Duplicate or unsorted pixels are handled automatically.
    pub fn from_coordinates(coords: Vec<u16>) -> Result<WasmPixelSet, String> {
        if coords.len() % 2 != 0 {
            return Err("coordinate array must have even length (y, x pairs)".to_string());
        }
        let pixels = coords
            .chunks(2)
            .map(|c| Pixel { y: c[0], x: c[1] })
            .collect();
        Ok(WasmPixelSet { inner: PixelSet::new(pixels) })
    }

    /// Decompress raw compressed bytes into a `PixelSet`. Matches the binary serde
    /// format (e.g. bincode). For the JSON / base64 format, use `from_base64`.
    pub fn from_compressed(bytes: &[u8]) -> Result<WasmPixelSet, String> {
        CompressedPixelSet::new(bytes.to_vec())
            .decompress()
            .map(|inner| WasmPixelSet { inner })
            .map_err(|e| e.to_string())
    }

    /// Deserialize from a base64 string. Convenience shorthand for
    /// `WasmCompressedPixelSet.from_base64(s).decompress()`. Matches the
    /// human-readable serde format (e.g. serde_json).
    pub fn from_base64(encoded: &str) -> Result<WasmPixelSet, String> {
        let bytes = STANDARD.decode(encoded).map_err(|e| e.to_string())?;
        CompressedPixelSet::new(bytes)
            .decompress()
            .map(|inner| WasmPixelSet { inner })
            .map_err(|e| e.to_string())
    }

    /// Compress this `PixelSet` into a `WasmCompressedPixelSet`, which can then be
    /// serialized via `to_base64()` or `bytes()`.
    pub fn compress(&self) -> Result<WasmCompressedPixelSet, String> {
        self.inner
            .compress()
            .map(|inner| WasmCompressedPixelSet { inner })
            .map_err(|e| e.to_string())
    }

    /// Serialize to a base64 string. Convenience shorthand for `compress().to_base64()`.
    /// Matches the human-readable serde format (e.g. serde_json).
    pub fn to_base64(&self) -> Result<String, String> {
        self.inner
            .compress()
            .map(|c| STANDARD.encode(c.into_bytes()))
            .map_err(|e| e.to_string())
    }

    /// Returns `true` if the pixel at `(y, x)` is in the set. O(log k).
    pub fn has(&self, y: u16, x: u16) -> bool {
        self.inner.has(Pixel { y, x })
    }

    /// Total number of pixels in the set. O(k).
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }

    pub fn is_empty(&self) -> bool {
        self.inner.len() == 0
    }

    /// Add a single pixel. O(k) due to run merging.
    /// For bulk insertions, prefer `from_coordinates` + `or`.
    pub fn add(&mut self, y: u16, x: u16) {
        self.inner.add(Pixel { y, x });
    }

    /// Remove a single pixel. O(k) due to run splitting.
    /// For bulk removals, prefer `difference`.
    pub fn discard(&mut self, y: u16, x: u16) {
        self.inner.discard(Pixel { y, x });
    }

    /// Intersection: pixels present in both sets. O(k₁ + k₂).
    pub fn and(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.and(&other.inner) }
    }

    /// Union: pixels present in either set. O(k₁ + k₂).
    pub fn or(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.or(&other.inner) }
    }

    /// Symmetric difference: pixels present in exactly one set. O(k₁ + k₂).
    pub fn xor(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.xor(&other.inner) }
    }

    /// Difference: pixels in this set but not in `other`. O(k₁ + k₂).
    pub fn difference(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.difference(&other.inner) }
    }

    /// Returns `true` if every pixel in this set is also in `other`.
    pub fn is_subset(&self, other: &WasmPixelSet) -> bool {
        self.inner.is_subset(&other.inner)
    }

    /// Returns `true` if this set shares at least one pixel with `other`.
    pub fn intersects(&self, other: &WasmPixelSet) -> bool {
        self.inner.intersects(&other.inner)
    }

    /// Bounding box as `[min_y, min_x, max_y, max_x]`, or an empty array if the set is empty.
    pub fn bounds(&self) -> Box<[u16]> {
        match self.inner.bounds() {
            Some((min_y, min_x, max_y, max_x)) => vec![min_y, min_x, max_y, max_x].into_boxed_slice(),
            None => vec![].into_boxed_slice(),
        }
    }

    /// All pixels as a flat `[y0, x0, y1, x1, ...]` array, in `(y, x)` order.
    pub fn to_coordinates(&self) -> Vec<u16> {
        self.inner.iter().flat_map(|p| [p.y, p.x]).collect()
    }
}
