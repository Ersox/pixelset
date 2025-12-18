use crate::Pixel;

mod ops;
mod new;
mod iter;

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
#[derive(Clone)]
pub struct PixelSet {
    /// The list of pixels in this set, sorted (y, x).
    pixels: Vec<Pixel>
}
