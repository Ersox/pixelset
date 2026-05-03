use crate::{Pixel, PixelSet, shapes::Shape};
use crate::set::Run;

/// Represents an axis-aligned filled rectangle with its pixels.
///
/// A rectangle is defined by its top-left corner `(x, y)` and dimensions `(width, height)`.
/// It includes all pixels within the bounds, from `(x, y)` to `(x + width - 1, y + height - 1)`.
///
/// ## Behavior
///
/// - A rectangle with zero width or height contains no pixels
/// - Pixel coordinates are inclusive of the starting corner and exclusive of the far edge
/// - The shape is immutable and all coordinates use unsigned 16-bit integers
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
        let mut runs = Vec::with_capacity(self.height as usize);

        for y in self.y..(self.y + self.height) {
            runs.push(Run {
                y,
                x_start: self.x,
                length: self.width,
            });
        }

        PixelSet::from_runs_unchecked(runs)
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