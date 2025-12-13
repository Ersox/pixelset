#[cfg(feature = "rand")]
use rand::seq::IteratorRandom;
#[cfg(feature = "rand")]
use crate::cache::grow::grow_pixel_into_box;

use crate::{PixelSet, shapes::{Rectangle, Shape}};

pub mod grow;

/// A high-performance spatial cache for `PixelSet` data, organized as a
/// collection of `PixelBox` containers.
///
/// ## Overview
///
/// `PixelCache` is designed to store pixels in **spatially contiguous boxes**
/// (`PixelBox`), making it easily to compactly store continuous sets of
/// pixels.
/// 
/// A `PixelCache` can be very quickly iterated over, and converted to a
/// `PixelSet`, but cannot be directly operated on, and generating one
/// may take longer.
#[derive(Clone)]
pub struct PixelCache {
    pub boxes: Vec<Rectangle>
}

impl PixelCache {
    /// Creates a new empty `PixelCache`.
    pub fn new() -> Self {
        Self { boxes: vec![] }
    }

    /// Combines all cached `PixelBox` containers into a single, sorted `PixelSet`.
    pub fn group(&self) -> PixelSet {
        let mut pixels = Vec::with_capacity(self.len());
        for rectangle in &self.boxes {
            pixels.extend(rectangle.iter_pixels());
        }

        PixelSet::new(pixels)
    }

    /// Builds a `PixelCache` by repeatedly selecting **random pixels** from a
    /// given `PixelSet` and expanding each into a `Rectangle`.
    /// 
    /// Creates a far more efficient and compact set than the original
    /// `PixelSet`.
    #[cfg(feature = "rand")]
    pub fn generate_from_set(set: &PixelSet) -> Self {
        let mut set = set.clone();
        let mut rng = rand::rng();

        let mut rectangles = vec![];
        while set.len() > 0 {
            let &pixel = set.iter().choose(&mut rng).unwrap();
            let rectangle = grow_pixel_into_box(pixel, &set);
            set = set.without(&rectangle.set());

            rectangles.push(rectangle);
        }

        Self { boxes: rectangles }
    }

    /// Returns the total number of pixels contained across all cached boxes.
    pub fn len(&self) -> usize {
        self.boxes.iter()
            .map(|rectangle| rectangle.len())
            .sum()
    }

    /// Returns `true` if the cache contains no pixels.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
