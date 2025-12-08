use image::{DynamicImage, GenericImageView};
use radsort::sort_by_key;

use crate::{Pixel, PixelSet};

impl PixelSet {
    /// Creates a new `PixelSet` from an **unsorted** list of pixels.
    ///
    /// The pixels are sorted into `(y, x)` order, removing all duplicates,
    /// operating in `O(n)` time with the usage of radix sort.
    pub fn new(mut pixels: Vec<Pixel>) -> Self {
        sort_by_key(&mut pixels, |pixel| pixel.key());
        pixels.dedup();
        Self::new_unchecked(pixels)
    }

    /// Constructs a `PixelSet` from a vector that is already known to be
    /// sorted in strictly ascending `(y, x)` order and contains no duplicates.
    ///
    /// This constructor performs **no validation**. Callers must ensure the
    /// input satisfies sorting order and lacks duplicates.
    pub fn new_unchecked(pixels: Vec<Pixel>) -> Self {
        Self { pixels }
    }

    /// Returns an empty `PixelSet`.
    pub fn empty() -> Self {
        Self::new_unchecked(vec![])
    }

    /// Creates a `PixelSet` containing a pixel for every coordinate in the
    /// given image.
    pub fn from_image(image: &DynamicImage) -> Self {
        let (width, height) = image.dimensions();

        let capacity = (width as usize) * (height as usize);
        let mut pixels = Vec::with_capacity(capacity);

        for y in 0..height {
            for x in 0..width {
                pixels.push(Pixel::new(x as u16, y as u16));
            }
        }

        Self::new_unchecked(pixels)
    }
}