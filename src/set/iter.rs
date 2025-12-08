use std::slice::{Iter, IterMut};

use crate::{Pixel, PixelSet};

impl<'a> IntoIterator for &'a PixelSet {
    type Item = &'a Pixel;
    type IntoIter = std::slice::Iter<'a, Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.iter()
    }
}

impl<'a> IntoIterator for &'a mut PixelSet {
    type Item = &'a mut Pixel;
    type IntoIter = std::slice::IterMut<'a, Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.iter_mut()
    }
}

impl IntoIterator for PixelSet {
    type Item = Pixel;
    type IntoIter = std::vec::IntoIter<Pixel>;

    fn into_iter(self) -> Self::IntoIter {
        self.pixels.into_iter()
    }
}

impl PixelSet {
    /// Returns an iterator over immutable references to the pixels in this set.
    pub fn iter(&self) -> Iter<'_, Pixel> {
        self.pixels.iter()
    }

    /// Returns an iterator over mutable references to the pixels in this set.
    pub fn iter_mut(&mut self) -> IterMut<'_, Pixel> {
        self.pixels.iter_mut()
    }
}
