use image::{DynamicImage, GenericImageView};
use radsort::sort_by_key;

use crate::{Pixel, PixelSet};
use crate::set::Run;

impl PixelSet {
    /// Creates a new `PixelSet` from an **unsorted** list of pixels.
    ///
    /// The pixels are sorted into `(y, x)` order, removing all duplicates,
    /// operating in `O(n)` time with the usage of radix sort. Pixels are then
    /// run-length encoded.
    pub fn new(mut pixels: Vec<Pixel>) -> Self {
        sort_by_key(&mut pixels, |pixel| pixel.key());
        pixels.dedup();
        Self::new_unchecked(pixels)
    }

    /// Constructs a `PixelSet` from a vector that is already known to be
    /// sorted in strictly ascending `(y, x)` order and contains no duplicates.
    ///
    /// This constructor performs a single-pass run-length encoding in `O(n)` time.
    /// Callers must ensure the input satisfies sorting order and lacks duplicates.
    pub fn new_unchecked(pixels: Vec<Pixel>) -> Self {
        let runs = Self::encode_runs(pixels);
        Self::from_runs_unchecked(runs)
    }

    /// Constructs a `PixelSet` from pre-built, valid run-length encoded runs.
    ///
    /// This constructor performs **no validation**. Callers must ensure the
    /// runs are sorted by `(y, x_start)`, non-overlapping, and non-adjacent.
    pub(crate) fn from_runs_unchecked(runs: Vec<Run>) -> Self {
        Self { runs }
    }

    /// Returns an empty `PixelSet`.
    pub fn empty() -> Self {
        Self::from_runs_unchecked(vec![])
    }

    /// Creates a `PixelSet` containing a pixel for every coordinate in the
    /// given image. Optimized to O(height) by directly generating full-width runs.
    pub fn from_image(image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();
        let (width, height) = (width as u16, height as u16);

        let mut runs = Vec::with_capacity(height as usize);
        for y in 0..height {
            if width > 0 {
                runs.push(Run {
                    y,
                    x_start: 0,
                    length: width,
                });
            }
        }

        Self::from_runs_unchecked(runs)
    }

    /// Encode a sorted, deduplicated pixel list into run-length encoded runs.
    /// Assumes pixels are sorted by (y, x) and contain no duplicates.
    fn encode_runs(pixels: Vec<Pixel>) -> Vec<Run> {
        if pixels.is_empty() {
            return vec![];
        }

        let mut runs = Vec::new();
        let mut current_y = pixels[0].y;
        let mut current_x_start = pixels[0].x;
        let mut current_length = 1u16;

        for i in 1..pixels.len() {
            let pixel = pixels[i];

            if pixel.y == current_y && pixel.x == current_x_start + current_length {
                current_length += 1;
            } else {
                runs.push(Run {
                    y: current_y,
                    x_start: current_x_start,
                    length: current_length,
                });

                current_y = pixel.y;
                current_x_start = pixel.x;
                current_length = 1;
            }
        }

        runs.push(Run {
            y: current_y,
            x_start: current_x_start,
            length: current_length,
        });

        runs
    }
}