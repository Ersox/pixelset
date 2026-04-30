use std::io::Result as IoResult;
use rustc_hash::FxHashSet;
use crate::{Pixel, PixelSet};
use super::codec;
use super::serialization::{self, Rectangle};

mod packing;

#[derive(Clone, Debug)]
struct RectangleSet {
    rectangles: Vec<Rectangle>,
}

impl RectangleSet {
    fn new(rectangles: Vec<Rectangle>) -> Self {
        Self { rectangles }
    }

    /// Convert a PixelSet to a RectangleSet using greedy rectangle packing.
    /// Iterates through pixels in scanline order, greedily expanding rectangles
    /// horizontally and vertically as much as possible.
    #[inline(never)]
    pub fn from_pixel_set(pixel_set: &PixelSet) -> Self {
        if pixel_set.is_empty() {
            return Self::new(vec![]);
        }

        let rows = packing::build_row_index(pixel_set);
        let pixels: Vec<Pixel> = {
            let mut p: Vec<Pixel> = pixel_set.iter().copied().collect();
            p.sort_unstable_by_key(|p| (p.y, p.x));
            p
        };

        let mut covered: FxHashSet<Pixel> = FxHashSet::default();
        let mut rectangles = vec![];

        for &pixel in &pixels {
            if covered.contains(&pixel) {
                continue;
            }

            let start_x = pixel.x;
            let start_y = pixel.y;

            let max_width = packing::expand_horizontally(start_x, start_y, &rows);
            let height = packing::expand_vertically(start_x, start_y, max_width, &rows);

            packing::mark_covered_pixels(&mut covered, start_x, start_y, max_width, height);
            rectangles.push(Rectangle::new(start_x, start_y, max_width, height));
        }

        Self::new(rectangles)
    }

    /// Convert a RectangleSet back to a PixelSet.
    pub fn to_pixel_set(&self) -> PixelSet {
        let mut pixels = vec![];

        for rect in &self.rectangles {
            for y in 0..rect.height {
                for x in 0..rect.width {
                    pixels.push(Pixel::new(
                        rect.x.saturating_add(x),
                        rect.y.saturating_add(y),
                    ));
                }
            }
        }

        PixelSet::new(pixels)
    }

    /// Serialize this RectangleSet to raw binary format using dimension lookup tables.
    pub fn to_bytes(&self) -> Vec<u8> {
        serialization::to_bytes(&self.rectangles)
    }

    /// Compress this RectangleSet using zstd compression on the serialized bytes.
    pub fn to_compressed_bytes(&self) -> IoResult<Vec<u8>> {
        let uncompressed = self.to_bytes();
        codec::compress_bytes(&uncompressed)
    }

    /// Decompress a RectangleSet from zstd-compressed bytes.
    pub fn from_compressed_bytes(compressed: &[u8]) -> IoResult<Self> {
        let uncompressed = codec::decompress_bytes(compressed)?;
        Self::from_bytes(&uncompressed)
    }

    /// Deserialize a RectangleSet from raw binary format with dimension lookup tables.
    pub fn from_bytes(bytes: &[u8]) -> IoResult<Self> {
        let rectangles = serialization::from_bytes(bytes)?;
        Ok(Self::new(rectangles))
    }
}

/// Compress a PixelSet to compressed bytes using rectangle packing and zstd.
pub fn compress_to_bytes(pixel_set: &PixelSet) -> IoResult<Vec<u8>> {
    let rect_set = RectangleSet::from_pixel_set(pixel_set);
    rect_set.to_compressed_bytes()
}

/// Decompress bytes back to a PixelSet.
pub fn decompress_from_bytes(compressed: &[u8]) -> IoResult<PixelSet> {
    let rect_set = RectangleSet::from_compressed_bytes(compressed)?;
    Ok(rect_set.to_pixel_set())
}

#[cfg(test)]
#[path = "tests.rs"]
mod tests;
