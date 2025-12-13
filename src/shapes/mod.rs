use crate::{Pixel, PixelSet};

pub use crate::shapes::rectangle_outline::RectangleOutline;
pub use crate::shapes::rectangle::Rectangle;

mod rectangle;
mod rectangle_outline;

/// A geometric shape that can be represented as a set of pixels.
///
/// A `Shape` defines a region in 2D pixel space and provides multiple ways
/// to interact with it, and convert it to a `PixelSet`.
pub trait Shape {
    /// Generates a `PixelSet` containing all pixels inside the shape.
    fn set(&self) -> PixelSet;

    /// An iterator of pixels in this shape.
    fn iter_pixels(&self) -> impl Iterator<Item = Pixel>;

    /// Returns the total number of pixels in the shape.
    fn len(&self) -> usize;
    
    /// Checks if a given pixel is inside the shape.
    fn has(&self, pixel: Pixel) -> bool;
}