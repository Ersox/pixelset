//! # pixelset — High-performance pixel set operations
//!
//! A Rust library for efficient 2D pixel manipulation and geometric operations on pixel collections.
//! Optimized for image processing tasks that involve selecting, transforming, and analyzing pixels
//! based on spatial relationships and color properties.
//!
//! ## Core Types
//!
//! - **[`PixelSet`]**: A compact, sorted collection of 2D pixel coordinates optimized for fast
//!   set operations, spatial queries, and iteration. Pixels are always stored in row-major `(y, x)`
//!   order for cache efficiency and quick binary-search lookups.
//!
//! - **[`Pixel`]**: Represents a single 2D coordinate with `x` and `y` fields. Implements custom
//!   ordering and hashing that preserves the `(y, x)` sort order used throughout the library.
//!
//! - **[`Color`]**: A simple RGBA color representation with methods for parsing hex codes,
//!   blending, grayscale conversion, and optional random color generation (with `rand` feature).
//!
//! - **[`PixelCache`]**: A spatial cache that groups pixels into [`Rectangle`] containers,
//!   allowing compact storage and fast conversion to `PixelSet`.
//!
//! ## Working with Shapes
//!
//! The [`shapes`] module provides geometric primitives that can be converted to pixel sets:
//!
//! - **[`Rectangle`]**: Axis-aligned filled rectangles
//! - **[`RectangleOutline`]**: Rectangle borders with adjustable stroke width
//! - **[`Ellipse`]**: Filled ellipses using the standard ellipse equation
//! - **[`EllipseOutline`]**: Ellipse borders with adjustable stroke width
//!
//! All shapes implement the [`Shape`] trait, allowing generic code that works with any shape.
//!
//! ## Common Operations
//!
//! **Set Operations** (boolean operations on pixel collections):
//! ```ignore
//! let intersection = set_a.and(&set_b);     // pixels in both
//! let union = set_a.or(&set_b);             // pixels in either
//! let difference = set_a.difference(&set_b);// pixels in a but not b
//! let xor = set_a.xor(&set_b);              // pixels in exactly one
//! ```
//!
//! **Spatial Queries**:
//! ```ignore
//! let outline = pixels.outline(&image);           // boundary pixels
//! let neighbors = pixels.neighbors(&image);       // adjacent pixels
//! let touching = pixels.touching(&other, &image); // pixels adjacent to another set
//! ```
//!
//! **Color Operations**:
//! ```ignore
//! pixels.fill(&mut image, Color::WHITE);           // solid color
//! pixels.transform(&mut image, |c| c.grayscale()); // apply transformation
//! let avg = pixels.mean_color(&image);             // average color
//! ```
//!
//! ## Design Philosophy
//!
//! **Performance-First**: Pixels are stored in sorted order, enabling:
//! - `O(log n)` membership checks via binary search
//! - `O(n)` set operations with linear scans
//! - Cache-friendly iteration in scanline order
//!
//! **Trade-offs**: While set operations are fast, adding/removing individual pixels
//! is `O(n)` due to vector shifting. Use [`PixelCache`] or batch operations for
//! bulk modifications.
//!
//! ## Example
//!
//! ```ignore
//! use image::DynamicImage;
//! use pixelset::{Pixel, PixelSet, Color, shapes::Circle};
//!
//! let mut image = DynamicImage::new_rgb8(100, 100);
//!
//! // Create a circular region
//! let circle = Circle { x: 50, y: 50, radius: 25 };
//! let circle_pixels = circle.set();
//!
//! // Fill with color
//! circle_pixels.fill(&mut image, Color::WHITE);
//!
//! // Get the outline
//! let outline = circle_pixels.outline(&image);
//! outline.fill(&mut image, Color::BLACK);
//! ```

mod pixel;
mod set;
pub mod shapes;
pub mod color;
pub mod direction;
pub mod compression;

pub use pixel::Pixel;
pub use color::Color;
pub use set::PixelSet;
pub use shapes::Shape;
pub use direction::Direction;
pub use compression::CompressedPixelSet;