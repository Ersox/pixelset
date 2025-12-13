use crate::{Pixel, PixelSet, shapes::Shape};

/// Represents a rectangular shape and its pixels.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Rectangle {
    /// The x-coordinate of the top-left corner of the box.
    pub x: u16,
    /// The y-coordinate of the top-left corner of the box.
    pub y: u16,
    /// The height of the box in pixels.
    pub height: u16,
    /// The width of the box in pixels.
    pub width: u16,
}

impl Rectangle {
    /// Creates a `Rectangle` that covers a single pixel.
    pub fn at_pixel(pixel: Pixel) -> Self {
        Self {
            x: pixel.x,
            y: pixel.y,
            height: 1,
            width: 1,
        }
    }
}

impl Shape for Rectangle {    
    fn set(&self) -> PixelSet {
        let mut pixels = Vec::with_capacity(self.len());

        for y in self.y..(self.y + self.height) {
            for x in self.x..(self.x + self.width) {
                pixels.push(Pixel::new(x, y));
            }
        }

        PixelSet::new_unchecked(pixels)
    }

    fn iter_pixels(&self) -> impl Iterator<Item = Pixel> {
        (self.y..self.y + self.height).flat_map(move |y| {
            (self.x..self.x + self.width).map(move |x| Pixel::new(x, y))
        })
    }

    fn len(&self) -> usize {
        (self.width * self.height) as usize
    }

    fn has(&self, pixel: Pixel) -> bool {
        pixel.x >= self.x
            && pixel.x < self.x + self.width
            && pixel.y >= self.y
            && pixel.y < self.y + self.height
    }
}