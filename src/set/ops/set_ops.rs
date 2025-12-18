use std::ops::{BitAnd, BitOr, Sub};

use crate::{Pixel, PixelSet};

impl PixelSet {
    /// Returns `true` if every pixel in this set is also present in `other`.
    ///
    /// Complexity: `O(n log m)`.
    pub fn is_subset(&self, other: &PixelSet) -> bool {
        self.into_iter().all(|pixel| other.has(*pixel))
    }

    /// Returns `true` if this set shares any pixel with another set.
    ///
    /// Complexity: `O(n log m)`.
    pub fn intersects(&self, other: &Self) -> bool {
        self.into_iter().any(|&pixel| other.has(pixel))
    }

    /// Inserts a single new pixel into the set while maintaining sorted
    /// order and uniqueness.
    /// 
    /// Uses binary search to find the insertion point.
    /// Worst-case complexity: `O(n)` due to element shifting.
    pub fn add(&mut self, pixel: Pixel) {
        match self.pixels.binary_search(&pixel) {
            Ok(_) => {}
            Err(idx) => {
                self.pixels.insert(idx, pixel);
            }
        }
    }

    /// Removes a pixel from the set, maintaining sort order.
    /// 
    /// Uses binary search to locate the pixel.
    /// Worst-case complexity: `O(n)` due to element shifting.
    pub fn discard(&mut self, pixel: Pixel) {
        if let Ok(idx) = self.pixels.binary_search(&pixel) {
            self.pixels.remove(idx);
        }
    }

    /// Returns a new `PixelSet` containing only the pixels that appear in
    /// both sets, performing a set intersection.
    /// 
    /// Complexity: `O(n + m)`
    pub fn and(&self, other: &Self) -> Self {
        let mut pixels = Vec::with_capacity(
            self.pixels.len().min(other.pixels.len())
        );

        let mut self_ind = 0;
        let mut other_ind = 0;

        while self_ind < self.len() && other_ind < other.len() {
            let self_pixel = self.pixels[self_ind];
            let other_pixel = other.pixels[other_ind];

            if self_pixel < other_pixel {
                self_ind += 1;
            } else if self_pixel > other_pixel {
                other_ind += 1;
            } else {
                // Pixels match → intersection
                pixels.push(self_pixel);
                self_ind += 1;
                other_ind += 1;
            }
        }

        PixelSet::new_unchecked(pixels)
    }

    /// Returns a new `PixelSet` representing the union of this set and another, with all 
    /// pixels from both sets are included.
    /// 
    /// Complexity: `O(n + m)`.
    pub fn or(&self, other: &Self) -> Self {
        if self.is_empty() {
            return other.clone();
        }

        let mut pixels = Vec::with_capacity(self.pixels.len() + other.pixels.len());

        let mut self_ind = 0;
        let mut other_ind = 0;

        while self_ind < self.pixels.len() && other_ind < other.pixels.len() {
            if self.pixels[self_ind] < other.pixels[other_ind] {
                pixels.push(self.pixels[self_ind]);
                self_ind += 1;
            } else if self.pixels[self_ind] > other.pixels[other_ind] {
                pixels.push(other.pixels[other_ind]);
                other_ind += 1;
            } else {
                pixels.push(self.pixels[self_ind]);
                self_ind += 1;
                other_ind += 1;
            }
        }

        pixels.extend_from_slice(&self.pixels[self_ind..]);
        pixels.extend_from_slice(&other.pixels[other_ind..]);

        Self { pixels }
    }

    /// Returns the symmetric difference of two sets:
    /// pixels that appear in exactly one of the sets.
    ///
    /// Complexity: `O(n + m)`.
    pub fn xor(&self, other: &Self) -> Self {
        let mut pixels = Vec::with_capacity(
            self.pixels.len() + other.pixels.len()
        );

        let mut self_ind = 0;
        let mut other_ind = 0;

        while self_ind < self.pixels.len() && other_ind < other.pixels.len() {
            let a = self.pixels[self_ind];
            let b = other.pixels[other_ind];

            if a < b {
                pixels.push(a);
                self_ind += 1;
            } else if a > b {
                pixels.push(b);
                other_ind += 1;
            } else {
                // Present in both → exclude
                self_ind += 1;
                other_ind += 1;
            }
        }

        // Remaining elements are all unique
        pixels.extend_from_slice(&self.pixels[self_ind..]);
        pixels.extend_from_slice(&other.pixels[other_ind..]);

        PixelSet::new_unchecked(pixels)
    }

    /// Returns a new `PixelSet` with pixels in this set that are not in `other`,
    /// performing a set difference.
    ///
    /// Complexity: `O(n + m)`.
    pub fn difference(&self, other: &Self) -> Self {
        let mut result = Vec::with_capacity(self.pixels.len());

        let mut self_index = 0;
        let mut other_index = 0;

        while self_index < self.pixels.len() && other_index < other.pixels.len() {
            let self_pixel = self.pixels[self_index];
            let other_pixel = other.pixels[other_index];

            if self_pixel < other_pixel {
                result.push(self_pixel);
                self_index += 1;
            } else if self_pixel > other_pixel {
                other_index += 1;
            } else {
                self_index += 1;
                other_index += 1;
            }
        }

        result.extend_from_slice(&self.pixels[self_index..]);

        PixelSet::new_unchecked(result)
    }
}