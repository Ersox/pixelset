use image::DynamicImage;

use crate::{Color, Pixel, PixelSet};

impl PixelSet {
    /// Applies a color-producing function to each pixel and writes the resulting
    /// color into the image.
    ///
    /// The closure may return `None` to indicate that a pixel's color should
    /// remain unchanged.  
    pub fn recolor<T : Into<Color>>(
        &self, 
        image: &mut DynamicImage,
        applier: impl Fn(Pixel) -> Option<T>
    ) {
        for &pixel in self {
            let Some(color) = applier(pixel) else { continue; };
            pixel.set(image, color.into());
        }
    }
    /// Fills all pixels in the set with a single uniform color.
    pub fn fill(
        &self,
        image: &mut DynamicImage,
        color: Color
    ) {
        self.recolor(image, |_| Some(color));
    }

    /// Reads the color of each pixel from the image, applies a transformation
    /// closure, and writes back a new color if one is produced.
    pub fn transform<T : Into<Color>>(
        &self, 
        image: &mut DynamicImage,
        applier: impl Fn(Color) -> Option<T>
    ) {
        for &pixel in self {
            let found_color = pixel.color(&image);
            let Some(color) = applier(found_color) else { continue; };
            pixel.set(image, color.into());
        }
    }

    /// Returns a `PixelSet` representing all 8-connected neighbors of all
    /// pixels in this set, constrained to the image bounds.
    pub fn neighbors(&self, image: &DynamicImage) -> Self {
        let pixels: Vec<_> = self.pixels.iter()
            .flat_map(|pixel| pixel.neighbors(image))
            .collect();

        Self::new(pixels)
    }

    /// Returns the subset of pixels in this set that are adjacent to another set.
    ///
    /// A pixel is included in the result if included in `self`, and adjacent to
    /// a pixel in `other`.
    pub fn touching(&self, other: &Self, image: &DynamicImage) -> Self {
        self.and(&other.neighbors(image))
    }

    /// Returns an iterator over the colors of all pixels in this set when
    /// viewed in the provided image.
    pub fn as_colors(&self, image: &DynamicImage) -> impl Iterator<Item = Color> {
        self.into_iter()
            .map(|pixel| pixel.color(image))
    }

    /// Computes the average color of all pixels in the set when sampled from
    /// the given image.
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
            let [r, g, b, a] = color.0.0;
            r_sum += r as u64;
            g_sum += g as u64;
            b_sum += b as u64;
            a_sum += a as u64;
        }

        let len = self.len() as u64;

        let avg = image::Rgba([
            (r_sum / len) as u8,
            (g_sum / len) as u8,
            (b_sum / len) as u8,
            (a_sum / len) as u8,
        ]);

        Some(Color(avg))
    }
}