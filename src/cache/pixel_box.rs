use crate::{PixelSet, Pixel};

/// Represents a rectangular area of pixels in an image.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct PixelBox {
    /// The x-coordinate of the top-left corner of the box.
    pub x: u16,
    /// The y-coordinate of the top-left corner of the box.
    pub y: u16,
    /// The height of the box in pixels.
    pub height: u16,
    /// The width of the box in pixels.
    pub width: u16,
}

impl PixelBox {
    /// Creates a `PixelBox` that covers a single pixel.
    pub fn at_pixel(pixel: Pixel) -> Self {
        Self {
            x: pixel.x,
            y: pixel.y,
            height: 1,
            width: 1,
        }
    }

    /// Generates a `PixelSet` containing all pixels inside the box.
    pub fn group(&self) -> PixelSet {
        let mut pixels = Vec::with_capacity(self.len());

        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                pixels.push(Pixel::new(x, y));
            }
        }

        PixelSet::new_unchecked(pixels)
    }

    /// An iterator of pixels in this box.
    pub fn iter_pixels(&self) -> impl Iterator<Item = Pixel> + '_ {
        (self.x..self.x + self.width).flat_map(move |x| {
            (self.y..self.y + self.height).map(move |y| Pixel::new(x, y))
        })
    }

    /// Returns the total number of pixels in the box.
    pub fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    /// Checks if a given pixel is inside the box.
    pub fn has(&self, pixel: Pixel) -> bool {
        pixel.x >= self.x
            && pixel.x < self.x + self.width
            && pixel.y >= self.y
            && pixel.y < self.y + self.height
    }
}
