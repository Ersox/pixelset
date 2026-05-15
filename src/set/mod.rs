use serde::{Deserialize, Serialize};

mod ops;
mod new;
mod iter;
mod compress;

/// A horizontal run-length encoded pixel span.
/// Encodes all consecutive pixels at a given y-coordinate from x_start to x_start + length - 1.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub(crate) struct Run {
    pub y: u16,
    pub x_start: u16,
    pub length: u16,
}

impl Run {
    /// The last x-coordinate in this run (inclusive).
    #[inline]
    pub fn x_end(self) -> u16 {
        self.x_start + self.length - 1
    }

    /// Construct a run from an inclusive x range.
    #[inline]
    pub fn from_span(y: u16, x_start: u16, x_end: u16) -> Self {
        Self { y, x_start, length: x_end - x_start + 1 }
    }

    /// Check if this run contains the given x-coordinate.
    #[inline]
    pub fn contains_x(self, x: u16) -> bool {
        x >= self.x_start && x <= self.x_end()
    }

    /// Returns `true` if this run overlaps with `other` (both on the same row).
    #[inline]
    pub fn overlaps(self, other: Run) -> bool {
        self.x_start <= other.x_end() && self.x_end() >= other.x_start
    }

    /// Encode this run as a sortable key.
    #[inline]
    pub fn key(self) -> u32 {
        ((self.y as u32) << 16) | (self.x_start as u32)
    }

    /// Serialize to 6 bytes (little-endian y, x_start, length).
    #[inline]
    pub fn to_bytes(self) -> [u8; 6] {
        let mut b = [0u8; 6];
        b[0..2].copy_from_slice(&self.y.to_le_bytes());
        b[2..4].copy_from_slice(&self.x_start.to_le_bytes());
        b[4..6].copy_from_slice(&self.length.to_le_bytes());
        b
    }

    /// Deserialize from 6 bytes (little-endian y, x_start, length).
    #[inline]
    pub fn from_bytes(b: &[u8; 6]) -> Self {
        Self {
            y: u16::from_le_bytes([b[0], b[1]]),
            x_start: u16::from_le_bytes([b[2], b[3]]),
            length: u16::from_le_bytes([b[4], b[5]]),
        }
    }
}

/// A compact, run-length encoded collection of pixels, optimized for fast set-like
/// operations and spatial queries on coherent regions.
///
/// ## Overview
///
/// `PixelSet` stores pixels in run-length encoded form: horizontal spans of consecutive
/// pixels are represented as a single `Run` value. This ensures:
///
/// - **Memory efficiency** (O(k) storage where k = number of runs, vs O(n) for pixels)
/// - **Fast binary-search membership checks** (`O(log k)` per lookup)
/// - **Efficient merges, intersections, and differences** (`O(k)` with linear scans over runs)
/// - **Minimal memory overhead** compared to flat pixel storage
///
/// ## Usage
///
/// ```rust
/// use image::DynamicImage;
/// use pixelset::{Pixel, PixelSet, Color};
///
/// let image = DynamicImage::new_rgb8(1, 1);
/// let mut set = PixelSet::from_image(&image);
/// ```
///
/// ## Guarantees
///
/// Internally, runs are always sorted by `(y, x_start)` with no overlapping or adjacent runs
/// on the same row. Iteration yields individual pixels in sorted `(y, x)` order.
/// Methods preserve this invariant with the exception of [`new`], which accepts
/// an unsorted pixel list.
///
/// Highly optimized for set operations on coherent regions. Performance scales with the
/// number of runs (typically O(height) for filled rectangles) rather than pixel count.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PixelSet {
    /// Horizontal run-length encoded pixels, sorted by (y, x_start).
    runs: Vec<Run>,
}

impl PixelSet {
    /// Get the underlying run-length encoded runs.
    pub(crate) fn runs(&self) -> &[Run] {
        &self.runs
    }
}

impl Serialize for PixelSet {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        crate::compression::serde::serialize(self, serializer)
    }
}

impl<'de> Deserialize<'de> for PixelSet {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        crate::compression::serde::deserialize(deserializer)
    }
}
