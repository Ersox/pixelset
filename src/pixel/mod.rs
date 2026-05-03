use std::hash::{Hash, Hasher};
use image::{DynamicImage, GenericImage, GenericImageView};
use serde::{Deserialize, Serialize};
use crate::{PixelSet, Color};

const OFFSETS: [(i32, i32); 8] = [
    (-1, -1), 
    (0, -1), 
    (1, -1),
    (-1, 0),           
    ( 1, 0),
    (-1, 1), 
    (0, 1), 
    (1, 1),
];

/// Represents a single 2D pixel coordinate within an image.
///
/// ## Overview
/// 
/// A `Pixel` stores its position using unsigned `x` and `y` coordinates,
/// implementing ordering (`Ord`, `PartialOrd`) such that pixels sort in
/// row-major `(y, x)` order.
#[derive(Clone, Copy, Debug, Eq, PartialEq, PartialOrd, Ord, Serialize, Deserialize)]
pub struct Pixel {
    /// The vertical coordinate of the pixel.
    pub y: u16,
    /// The horizontal coordinate of the pixel.
    pub x: u16,
}

impl Pixel {
    /// Produces a compact numeric key uniquely representing this pixel,
    /// ordered lexicographically as `(y, x)`.
    ///
    /// This key is used for fast sorting, comparison, and hashing,
    /// preserving sort order.
    pub fn key(&self) -> u32 {
        ((self.y as u32) << 16) | (self.x as u32)
    }
}

impl Hash for Pixel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let key = self.key();
        state.write_u32(key);
    }
}

impl Pixel {
    /// Creates a new `Pixel` from `(x, y)` coordinates.
    ///
    /// The coordinates are stored as `u16`, allowing pixel positions up to 65535 in each dimension.
    pub fn new(x: u16, y: u16) -> Self {
        Self { x, y }
    }

    /// Retrieves the color of this pixel from the provided `DynamicImage`.
    ///
    /// This reads a single pixel's color value from the image at this pixel's coordinates.
    /// The image must be large enough to contain this pixel (i.e., `x < width` and `y < height`).
    pub fn color(self, image: &DynamicImage) -> Color {
        image.get_pixel(self.x as u32, self.y as u32).into()
    }

    /// Sets the color of this pixel in the given image.
    ///
    /// This writes a new color value to the image at this pixel's coordinates.
    /// The image must be large enough to contain this pixel (i.e., `x < width` and `y < height`).
    pub fn set(self, image: &mut DynamicImage, color: Color) {
        image.put_pixel(self.x as u32, self.y as u32, color.into());
    }

    /// Returns a `PixelSet` containing all valid 8-connected neighboring pixels around
    /// this pixel.
    ///
    /// The 8-connected neighborhood includes all pixels that share an edge or corner with
    /// this pixel: the positions directly above, below, left, right, and all four diagonals.
    /// Neighbors outside the image bounds are automatically excluded.
    ///
    /// This is commonly used for flood-fill algorithms, edge detection, and spatial queries.
    pub fn neighbors(self, image: &DynamicImage) -> PixelSet {
        let x = self.x as i32;
        let y = self.y as i32;

        let (width, height) = image.dimensions();
        let width = width as i32;
        let height = height as i32;

        let mut pixels = Vec::with_capacity(8);
        for (dx, dy) in OFFSETS {
            let new_x = x + dx;
            let new_y = y + dy;

            if new_x < 0 || new_y < 0 || new_x >= width || new_y >= height {
                continue;
            }

            pixels.push(Pixel::new(new_x as u16, new_y as u16));
        }

        PixelSet::new_unchecked(pixels)
    }
}