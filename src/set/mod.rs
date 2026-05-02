use serde::{Deserialize, Serialize};

use crate::Pixel;

mod ops;
mod new;
mod iter;
mod compress;
mod serialization_tests;

/// A compact, sorted collection of pixels, optimized for fast set-like
/// operations and spatial queries.
///
/// ## Overview
///
/// `PixelSet` is a high-performance container that stores `Pixel` values in a
/// strictly sorted `(y, x)` order. This ensures:
///
/// - **Cache-friendly iteration** (pixels are stored linearly in scanline order)
/// - **Fast binary-search membership checks** (`O(log n)` per lookup)
/// - **Efficient merges, intersections, and differences** (`O(n)` with linear scans)
/// - **Minimal memory overhead** compared to hash-based or tree-based structures
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
/// Internally, pixel order is always sorted by `(y, x)`.  
/// Methods preserve this sorting parity with the exception of [`new`],
/// which expects a sanitized sorted list.
/// 
/// Highly optimized for set operations, only struggling with
/// additions or removals of individual pixels.
#[derive(Clone, Debug, PartialEq)]
pub struct PixelSet {
    /// The list of pixels in this set, sorted (y, x).
    pixels: Vec<Pixel>
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compression_via_serialization() {
        let mut pixels = vec![];
        for y in 0..50 {
            for x in 0..50 {
                pixels.push(Pixel::new(x, y));
            }
        }

        let original = PixelSet::new(pixels);
        let original_size = std::mem::size_of_val(&original.pixels[..]);

        let compressed = crate::compression::compress_to_bytes(&original)
            .expect("compression failed");
        let decompressed = crate::compression::decompress_from_bytes(&compressed)
            .expect("decompression failed");

        assert_eq!(original.len(), decompressed.len());
        assert_eq!(original, decompressed);

        let compression_ratio = compressed.len() as f64 / original_size as f64;
        assert!(compression_ratio < 1.0, "compression should reduce size");
    }
}
