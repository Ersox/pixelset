use crate::{Pixel, PixelSet};

pub use crate::shapes::rectangle_outline::RectangleOutline;
pub use crate::shapes::rectangle::Rectangle;
pub use crate::shapes::ellipse::Ellipse;
pub use crate::shapes::ellipse_outline::EllipseOutline;

mod ellipse;
mod ellipse_outline;
mod rectangle;
mod rectangle_outline;

/// A geometric shape that can be represented as a set of pixels.
///
/// A `Shape` defines a region in 2D pixel space and provides multiple ways
/// to interact with it, and convert it to a `PixelSet`.
pub trait Shape {
    /// Checks if a given pixel is inside the shape.
    fn has(&self, pixel: Pixel) -> bool;

    /// An iterator of pixels in this shape, yielded in `(y, x)` order.
    fn iter_pixels(&self) -> impl Iterator<Item = Pixel>;

    /// Generates a `PixelSet` containing all pixels inside the shape.
    ///
    /// The default implementation collects `iter_pixels()`. Override for shapes
    /// that can build runs directly (e.g. `Rectangle`).
    fn set(&self) -> PixelSet {
        PixelSet::new_unchecked(self.iter_pixels().collect())
    }

    /// Returns the exact number of pixels in the shape.
    ///
    /// The default implementation counts `iter_pixels()`. Override when an
    /// analytical formula is available (e.g. `Rectangle`, `RectangleOutline`).
    fn len(&self) -> usize {
        self.iter_pixels().count()
    }

    /// Returns `true` if the shape contains no pixels.
    fn is_empty(&self) -> bool {
        self.iter_pixels().next().is_none()
    }
}
