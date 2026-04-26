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

    /// Returns the number of pixels in this set.
    pub fn len(&self) -> usize {
        self.pixels.len()
    }

    /// Returns a new `PixelSet` containing only pixels that satisfy the given predicate.
    ///
    /// The predicate function receives each pixel and should return `true` to include it
    /// in the result, or `false` to exclude it. This is useful for filtering pixels based
    /// on spatial properties or computed conditions.
    pub fn filter(&self, predicate: impl Fn(Pixel) -> bool) -> Self {
        Self::new_unchecked(
            self.pixels.iter()
                .copied()
                .filter(|&pixel| predicate(pixel))
                .collect()
        )
    }

    /// Returns a new `PixelSet` containing only pixels whose colors satisfy the given predicate.
    ///
    /// For each pixel in the set, this method reads its color from the image and passes it
    /// to the predicate function. Pixels whose colors return `true` are included in the result.
    pub fn filter_color(
        &self,
        image: &DynamicImage,
        predicate: impl Fn(Color) -> bool
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
    pub fn select(
        &self,
        image: &DynamicImage,
        query: Color
    ) -> Self {
        self.filter_color(image, |color| color == query)
    }

    /// Returns a modified copy of this `PixelSet` after applying a transformation function.
    ///
    /// The provided function receives a mutable reference to a cloned copy of this set,
    /// allowing in-place mutations like `add()`, `discard()`, etc. The mutated copy is returned.
    /// The original set is left unchanged.
    pub fn apply(
        &self,
        applier: impl Fn(&mut PixelSet)
    ) -> Self {
        let mut set = self.clone();
        applier(&mut set);
        set
    }
}