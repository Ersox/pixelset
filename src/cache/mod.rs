#[cfg(feature = "rand")]
use rand::seq::IteratorRandom;
#[cfg(feature = "rand")]
use crate::cache::grow::grow_pixel_into_box;

use crate::{PixelSet, cache::pixel_box::PixelBox};

pub mod pixel_box;
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
    pub boxes: Vec<PixelBox>
}

impl PixelCache {
    /// Creates a new empty `PixelCache`.
    pub fn new() -> Self {
        Self { boxes: vec![] }
    }

    /// Combines all cached `PixelBox` containers into a single, sorted `PixelSet`.
    pub fn group(&self) -> PixelSet {
        let mut pixels = Vec::with_capacity(self.len());
        for pixel_box in &self.boxes {
            pixels.extend(pixel_box.iter_pixels());
        }

        PixelSet::new(pixels)
    }

    /// Builds a `PixelCache` by repeatedly selecting **random pixels** from a
    /// given `PixelSet` and expanding each into a `PixelBox`.
    /// 
    /// Creates a far more efficient and compact set than the original
    /// `PixelSet`.
    #[cfg(feature = "rand")]
    pub fn generate_from_set(set: &PixelSet) -> Self {
        let mut set = set.clone();
        let mut rng = rand::rng();

        let mut boxes = vec![];
        while set.len() > 0 {
            let &pixel = set.iter().choose(&mut rng).unwrap();
            let pixel_box = grow_pixel_into_box(pixel, &set);
            set = set.without(&pixel_box.group());

            boxes.push(pixel_box);
        }

        Self { boxes }
    }

    /// Returns the total number of pixels contained across all cached boxes.
    pub fn len(&self) -> usize {
        self.boxes.iter()
            .map(|pixel_box| pixel_box.len())
            .sum()
    }

    /// Returns `true` if the cache contains no pixels.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}
