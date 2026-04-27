use std::collections::HashSet;
use crate::{Pixel, PixelSet};

#[derive(Clone, Debug)]
pub struct Rectangle {
    pub x: u16,
    pub y: u16,
    pub width: u16,
    pub height: u16,
}

/// A compressed representation of a PixelSet using axis-aligned rectangles.
/// Each rectangle is 8 bytes, enabling significant compression for regular shapes.
#[derive(Clone, Debug)]
pub struct RectangleSet {
    rectangles: Vec<Rectangle>,
}

impl Rectangle {
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> Self {
        Self { x, y, width, height }
    }
}

impl RectangleSet {
    pub fn new(rectangles: Vec<Rectangle>) -> Self {
        Self { rectangles }
    }

    pub fn len(&self) -> usize {
        self.rectangles.len()
    }

    pub fn is_empty(&self) -> bool {
        self.rectangles.is_empty()
    }

    pub fn rectangles(&self) -> &[Rectangle] {
        &self.rectangles
    }

    pub fn into_rectangles(self) -> Vec<Rectangle> {
        self.rectangles
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_pixel() {
        let pixels = vec![Pixel::new(5, 5)];
        let pixel_set = PixelSet::new(pixels);
        let rect_set = RectangleSet::from_pixel_set(&pixel_set);

        assert_eq!(rect_set.len(), 1);
        assert_eq!(rect_set.rectangles()[0].x, 5);
        assert_eq!(rect_set.rectangles()[0].y, 5);
        assert_eq!(rect_set.rectangles()[0].width, 1);
        assert_eq!(rect_set.rectangles()[0].height, 1);

        let recovered = rect_set.to_pixel_set();
        assert_eq!(recovered.iter().count(), 1);
    }

    #[test]
    fn test_horizontal_line() {
        let pixels = vec![
            Pixel::new(0, 0),
            Pixel::new(1, 0),
            Pixel::new(2, 0),
            Pixel::new(3, 0),
        ];
        let pixel_set = PixelSet::new(pixels);
        let rect_set = RectangleSet::from_pixel_set(&pixel_set);

        assert_eq!(rect_set.len(), 1);
        assert_eq!(rect_set.rectangles()[0].width, 4);
        assert_eq!(rect_set.rectangles()[0].height, 1);

        let recovered = rect_set.to_pixel_set();
        assert_eq!(recovered.iter().count(), 4);
    }

    #[test]
    fn test_rectangle() {
        let mut pixels = vec![];
        for y in 0..3 {
            for x in 0..5 {
                pixels.push(Pixel::new(x, y));
            }
        }
        let pixel_set = PixelSet::new(pixels);
        let rect_set = RectangleSet::from_pixel_set(&pixel_set);

        assert_eq!(rect_set.len(), 1);
        assert_eq!(rect_set.rectangles()[0].width, 5);
        assert_eq!(rect_set.rectangles()[0].height, 3);

        let recovered = rect_set.to_pixel_set();
        assert_eq!(recovered.iter().count(), 15);
    }

    #[test]
    fn test_lossless_roundtrip() {
        let mut pixels = vec![];
        // Create an L-shaped region
        for y in 0..4 {
            for x in 0..3 {
                pixels.push(Pixel::new(x, y));
            }
        }
        for y in 4..6 {
            pixels.push(Pixel::new(0, y));
        }

        let original = PixelSet::new(pixels);
        let rect_set = RectangleSet::from_pixel_set(&original);
        let recovered = rect_set.to_pixel_set();

        let original_count = original.iter().count();
        let recovered_count = recovered.iter().count();
        assert_eq!(original_count, recovered_count, "Pixel count mismatch");

        for pixel in original.iter() {
            assert!(
                recovered.iter().any(|p| p == pixel),
                "Pixel {:?} missing in recovered set",
                pixel
            );
        }
    }

    #[test]
    fn test_empty_set() {
        let pixel_set = PixelSet::empty();
        let rect_set = RectangleSet::from_pixel_set(&pixel_set);

        assert_eq!(rect_set.len(), 0);
        assert!(rect_set.is_empty());

        let recovered = rect_set.to_pixel_set();
        assert_eq!(recovered.iter().count(), 0);
    }

    #[test]
    fn test_compression_ratio() {
        let mut pixels = vec![];
        // Create a 100x100 square
        for y in 0..100 {
            for x in 0..100 {
                pixels.push(Pixel::new(x, y));
            }
        }

        let pixel_set = PixelSet::new(pixels);
        let rect_set = RectangleSet::from_pixel_set(&pixel_set);

        // 10,000 pixels should compress to 1 rectangle
        assert_eq!(rect_set.len(), 1);
        let bytes_per_pixel = 4; // u16 x, u16 y
        let bytes_per_rectangle = 8; // 4 u16 fields
        let original_bytes = 10_000 * bytes_per_pixel;
        let compressed_bytes = 1 * bytes_per_rectangle;
        let ratio = original_bytes as f64 / compressed_bytes as f64;
        assert!(ratio >= 5000.0, "Expected compression ratio >= 5000, got {}", ratio);
    }

    #[test]
    fn test_scattered_rectangles() {
        let mut pixels = vec![];
        // Create three separate 2x2 rectangles
        for y in 0..2 {
            for x in 0..2 {
                pixels.push(Pixel::new(x, y));
            }
        }
        for y in 0..2 {
            for x in 5..7 {
                pixels.push(Pixel::new(x, y));
            }
        }
        for y in 5..7 {
            for x in 0..2 {
                pixels.push(Pixel::new(x, y));
            }
        }

        let pixel_set = PixelSet::new(pixels);
        let rect_set = RectangleSet::from_pixel_set(&pixel_set);

        assert_eq!(rect_set.len(), 3);
        let recovered = rect_set.to_pixel_set();
        assert_eq!(recovered.iter().count(), 12);
    }
}

