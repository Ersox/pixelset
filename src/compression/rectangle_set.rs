use std::collections::HashSet;
use std::io::Result as IoResult;
use crate::{Pixel, PixelSet};
use super::codec;
use super::serialization::{self, Rectangle};

#[derive(Clone, Debug)]
struct RectangleSet {
    rectangles: Vec<Rectangle>,
}

impl RectangleSet {
    fn new(rectangles: Vec<Rectangle>) -> Self {
        Self { rectangles }
    }

    /// Convert a PixelSet to a RectangleSet using greedy rectangle packing.
    /// Iterates through pixels in scanline order, expanding rectangles horizontally
    /// and vertically as much as possible.
    pub fn from_pixel_set(pixel_set: &PixelSet) -> Self {
        if pixel_set.is_empty() {
            return Self::new(vec![]);
        }

        let pixels: Vec<Pixel> = pixel_set.iter().copied().collect();
        let pixel_set_map: HashSet<Pixel> = pixels.iter().copied().collect();
        let mut covered = vec![false; pixels.len()];
        let mut rectangles = vec![];

        for (i, &pixel) in pixels.iter().enumerate() {
            if covered[i] {
                continue;
            }

            // Start a new rectangle at this pixel
            let start_x = pixel.x;
            let start_y = pixel.y;

            // Find the maximum width by expanding right from this position on this row
            let mut max_width = 1u16;
            loop {
                let next_x = start_x.saturating_add(max_width);
                if next_x == u16::MAX || !pixel_set_map.contains(&Pixel::new(next_x, start_y)) {
                    break;
                }
                max_width += 1;
            }

            // Now try to expand downward, limited by the width we found
            let mut height = 1u16;
            'height_loop: loop {
                let next_y = start_y.saturating_add(height);
                if next_y == u16::MAX {
                    break;
                }

                // Check if all pixels in the next row exist
                for x_offset in 0..max_width {
                    let target_x = start_x.saturating_add(x_offset);
                    let target_pixel = Pixel::new(target_x, next_y);
                    if !pixel_set_map.contains(&target_pixel) {
                        break 'height_loop;
                    }
                }

                height += 1;
            }

            // Mark all pixels in this rectangle as covered
            for j in i..pixels.len() {
                let p = pixels[j];
                if p.x >= start_x
                    && p.x < start_x.saturating_add(max_width)
                    && p.y >= start_y
                    && p.y < start_y.saturating_add(height)
                {
                    covered[j] = true;
                }
            }

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
