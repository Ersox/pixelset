use image::{DynamicImage, GenericImageView};

use crate::{Color, Pixel, PixelSet};

impl PixelSet {
    /// Returns `true` if the set contains no pixels.
    pub fn is_empty(&self) -> bool {
        self.runs.is_empty()
    }

    /// Returns `true` if the set contains the specified pixel,
    /// performing binary search on runs.
    ///
    /// Complexity: `O(log k)` where k is the number of runs.
    pub fn has(&self, pixel: Pixel) -> bool {
        let key = ((pixel.y as u32) << 16) | (pixel.x as u32);
        let idx = self.runs.partition_point(|r| r.key() <= key);

        if idx == 0 {
            return false;
        }

        let run = self.runs[idx - 1];
        run.y == pixel.y && run.contains_x(pixel.x)
    }

    /// Returns the number of pixels in this set.
    ///
    /// Complexity: `O(k)` where k is the number of runs.
    pub fn len(&self) -> usize {
        self.runs.iter().map(|r| r.length as usize).sum()
    }

    /// Returns a new `PixelSet` containing only pixels that satisfy the given predicate.
    ///
    /// The predicate function receives each pixel and should return `true` to include it
    /// in the result, or `false` to exclude it.
    pub fn filter(&self, predicate: impl Fn(Pixel) -> bool) -> Self {
        let pixels: Vec<Pixel> = self
            .iter()
            .filter(|&pixel| predicate(pixel))
            .collect();
        Self::new_unchecked(pixels)
    }

    /// Returns a new `PixelSet` containing only pixels whose colors satisfy the given predicate.
    ///
    /// For each pixel in the set, this method reads its color from the image and passes it
    /// to the predicate function. Pixels whose colors return `true` are included in the result.
    pub fn filter_color(
        &self,
        image: &DynamicImage,
        predicate: impl Fn(Color) -> bool,
    ) -> Self {
        self.filter(|pixel| {
            let color = image.get_pixel(pixel.x as u32, pixel.y as u32);
            predicate(color.into())
        })
    }

    /// Returns a new `PixelSet` containing only pixels whose color in the provided image
    /// exactly equals the query color.
    ///
    /// This performs exact RGBA matching; colors must match on all four channels.
    pub fn select(&self, image: &DynamicImage, query: Color) -> Self {
        self.filter_color(image, |color| color == query)
    }

    /// Returns a modified copy of this `PixelSet` after applying a transformation function.
    ///
    /// The provided function receives a mutable reference to a cloned copy of this set,
    /// allowing in-place mutations like `add()`, `discard()`, etc. The mutated copy is returned.
    /// The original set is left unchanged.
    pub fn apply(&self, applier: impl Fn(&mut PixelSet)) -> Self {
        let mut set = self.clone();
        applier(&mut set);
        set
    }

    /// Returns the bounding box of this set as (min_x, min_y, max_x, max_y).
    ///
    /// Returns `None` if the set is empty.
    ///
    /// Complexity: `O(k)` where k is the number of runs.
    pub fn bounds(&self) -> Option<(u16, u16, u16, u16)> {
        if self.is_empty() {
            return None;
        }

        let min_y = self.runs[0].y;
        let max_y = self.runs[self.runs.len() - 1].y;

        let min_x = self.runs.iter().map(|r| r.x_start).min().unwrap();
        let max_x = self
            .runs
            .iter()
            .map(|r| r.x_end())
            .max()
            .unwrap();

        Some((min_x, min_y, max_x, max_y))
    }

    /// Returns the centroid (average position) of all pixels in this set.
    ///
    /// Returns `None` if the set is empty.
    ///
    /// Complexity: `O(k)` where k is the number of runs.
    pub fn centroid(&self) -> Option<(f64, f64)> {
        if self.is_empty() {
            return None;
        }

        let mut x_sum = 0.0;
        let mut y_sum = 0.0;
        let mut total_pixels = 0.0;

        for run in &self.runs {
            let run_len = run.length as f64;
            let run_x_sum = run_len * (run.x_start as f64 + run.x_end() as f64) / 2.0;
            x_sum += run_x_sum;
            y_sum += run.y as f64 * run_len;
            total_pixels += run_len;
        }

        Some((x_sum / total_pixels, y_sum / total_pixels))
    }

    /// Returns the pixel in this set closest to the given coordinates.
    ///
    /// Closest is determined by Euclidean distance. Returns `None` if the set is empty.
    ///
    /// Complexity: `O(n)` where n is the total number of pixels.
    pub fn closest_to(&self, x: u16, y: u16) -> Option<Pixel> {
        let mut iter = self.iter();
        let mut closest = iter.next()?;
        let dx = closest.x as i32 - x as i32;
        let dy = closest.y as i32 - y as i32;
        let mut min_dist_sq = (dx * dx + dy * dy) as u64;

        for pixel in iter {
            let dx = pixel.x as i32 - x as i32;
            let dy = pixel.y as i32 - y as i32;
            let dist_sq = (dx * dx + dy * dy) as u64;
            if dist_sq < min_dist_sq {
                min_dist_sq = dist_sq;
                closest = pixel;
            }
        }

        Some(closest)
    }
}
