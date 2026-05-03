use image::{DynamicImage, GenericImageView};
use rustc_hash::FxHashSet;
use radsort::sort_by_key;

use crate::{Color, Pixel, PixelSet};

impl PixelSet {
    /// Applies a color-producing function to each pixel and writes the result to the image.
    ///
    /// For each pixel in this set, the function is called with the pixel's coordinates.
    /// If it returns `Some(color)`, that color is written to the image at the pixel's
    /// location. If it returns `None`, the pixel's color is left unchanged.
    ///
    /// The color value is converted via `Into<Color>`, allowing flexibility in input types.
    pub fn recolor<T: Into<Color>>(
        &self,
        image: &mut DynamicImage,
        applier: impl Fn(Pixel) -> Option<T>,
    ) {
        for pixel in self {
            let Some(color) = applier(pixel) else { continue; };
            pixel.set(image, color.into());
        }
    }

    /// Fills all pixels in this set with a single uniform color.
    ///
    /// Every pixel in the set is set to the provided color, overwriting any previous color.
    pub fn fill(&self, image: &mut DynamicImage, color: Color) {
        self.recolor(image, |_| Some(color));
    }

    /// Reads each pixel's color from the image, applies a transformation function,
    /// and writes the new color back.
    ///
    /// For each pixel, this reads its current color, passes it through the transformation
    /// function, and writes the result back only if the color changed.
    pub fn transform(&self, image: &mut DynamicImage, applier: impl Fn(Color) -> Color) {
        for pixel in self {
            let found_color = pixel.color(image);
            let new_color = applier(found_color);

            if new_color == found_color {
                continue;
            }
            pixel.set(image, new_color);
        }
    }

    /// Returns all pixels on the boundary of this set (pixels with neighbors outside the set).
    ///
    /// A pixel is included in the result if it belongs to this set and has at least one
    /// 8-connected neighbor outside the set (or outside the image bounds). This is useful
    /// for edge detection, stroke rendering, or isolation of region boundaries.
    ///
    /// The result is a subset of this set (no new pixels are added).
    pub fn outline(&self, image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let (width, height) = (width as i32, height as i32);

        const OFFSETS: [(i32, i32); 8] = [
            (-1, -1), (0, -1), (1, -1), (-1, 0), (1, 0), (-1, 1), (0, 1), (1, 1),
        ];

        self.filter(|pixel| {
            let x = pixel.x as i32;
            let y = pixel.y as i32;

            OFFSETS.iter().any(|&(dx, dy)| {
                let nx = x + dx;
                let ny = y + dy;
                if nx < 0 || ny < 0 || nx >= width || ny >= height {
                    return true;
                }
                !self.has(Pixel::new(nx as u16, ny as u16))
            })
        })
    }

    /// Returns all 8-connected neighbors of all pixels in this set.
    ///
    /// For each pixel in this set, all of its valid 8-connected neighbors (within image bounds)
    /// are collected into a new set. The result may include pixels from the original set if
    /// they are neighbors of other pixels. Duplicates are automatically removed.
    ///
    /// This is useful for flood-fill algorithms, dilation operations, or spatial expansion.
    pub fn neighbors(&self, image: &DynamicImage) -> Self {
        let mut seen = FxHashSet::default();
        for pixel in self.iter() {
            for neighbor in pixel.neighbors(image) {
                seen.insert(neighbor);
            }
        }

        let mut pixels: Vec<_> = seen.into_iter().collect();
        sort_by_key(&mut pixels, |p| p.key());
        Self::new_unchecked(pixels)
    }

    /// Returns pixels in this set that are adjacent to pixels in another set.
    ///
    /// A pixel is included in the result if:
    /// - It belongs to this set (`self`), AND
    /// - It has at least one 8-connected neighbor in `other`
    ///
    /// This is useful for finding contact regions between two regions or detecting
    /// when two sets touch.
    pub fn touching(&self, other: &Self, image: &DynamicImage) -> Self {
        if other.len() < self.len() {
            other.neighbors(image).and(self)
        } else {
            self.and(&other.neighbors(image))
        }
    }

    /// Returns an iterator over the colors of all pixels in this set.
    ///
    /// For each pixel in this set (in `(y, x)` sorted order), the color is read from
    /// the provided image and yielded. This is a convenience method for operations
    /// that need to inspect or analyze the actual color data.
    pub fn as_colors<'a>(&'a self, image: &'a DynamicImage) -> impl Iterator<Item = Color> + 'a {
        self.iter().map(|pixel| pixel.color(image))
    }

    /// Computes the average RGBA color of all pixels in this set.
    ///
    /// Each of the four channels (R, G, B, A) is independently averaged. Integer division
    /// is used, potentially losing precision in the least significant bit.
    ///
    /// Returns `None` if the set is empty.
    pub fn mean_color(&self, image: &DynamicImage) -> Option<Color> {
        if self.is_empty() {
            return None;
        }

        let mut r_sum: u64 = 0;
        let mut g_sum: u64 = 0;
        let mut b_sum: u64 = 0;
        let mut a_sum: u64 = 0;

        for color in self.as_colors(image) {
            r_sum += color.r() as u64;
            g_sum += color.g() as u64;
            b_sum += color.b() as u64;
            a_sum += color.a() as u64;
        }

        let len = self.len() as u64;

        let avg = [
            (r_sum / len) as u8,
            (g_sum / len) as u8,
            (b_sum / len) as u8,
            (a_sum / len) as u8,
        ];

        Some(avg.into())
    }
}
