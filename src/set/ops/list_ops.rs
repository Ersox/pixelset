use image::{DynamicImage, GenericImageView};

use crate::{Color, Pixel, PixelSet};

impl PixelSet {
    /// Returns `true` if the set contains no pixels.
    pub fn is_empty(&self) -> bool {
        self.pixels.is_empty()
    }

    /// Returns `true` if the set contains the specified pixel,
    /// performing binary search.
    /// 
    /// Complexity: `O(log n)`.
    pub fn has(&self, pixel: Pixel) -> bool {
        self.pixels.binary_search(&pixel).is_ok()
    }

    /// Returns `true` if every pixel in this set is also present in `other`.
    ///
    /// Complexity: `O(n log m)`.
    pub fn is_subset(&self, other: &PixelSet) -> bool {
        self.into_iter().all(|pixel| other.has(*pixel))
    }

    /// Returns the number of pixels in this set.
    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    /// Returns a new `PixelSet` containing pixels whose colors in the given image
    /// satisfy a predicate.
    pub fn filter(&self, predicate: impl Fn(Pixel) -> bool) -> Self {
        Self::new_unchecked(
            self.pixels.iter()
                .filter(|&&pixel| predicate(pixel))
                .map(|pixel| *pixel)
                .collect()
        )
    }
    
    /// Returns a new `PixelSet` containing only pixels whose color in the given image
    /// exactly matches the specified color.
    pub fn filter_color(
        &self, 
        image: &DynamicImage,
        predicate: impl Fn(Color) -> bool
    ) -> Self {
        self.filter(|pixel| {
            let color = image.get_pixel(pixel.x as u32, pixel.y as u32);
            predicate(Color(color))
        })
    }

    /// Returns a new `PixelSet` containing only those pixels whose color in
    /// the provided image exactly matches the specified color.
    pub fn select(
        &self,
        image: &DynamicImage,
        query: Color
    ) -> Self {
        self.filter_color(image, |color| color == query)
    }

    /// Returns a modified copy of the `PixelSet` after applying a
    /// user-provided transformation function.
    pub fn apply(
        &self,
        applier: impl Fn(&mut PixelSet)
    ) -> Self {
        let mut set = self.clone();
        applier(&mut set);
        set
    }
}