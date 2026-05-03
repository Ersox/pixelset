use crate::{Pixel, PixelSet};
use crate::set::Run;

/// Iterator over pixels in a PixelSet, lazily expanding runs.
pub struct Iter<'a> {
    runs: std::slice::Iter<'a, Run>,
    current: Option<(Run, u16)>,
}

impl<'a> Iterator for Iter<'a> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Pixel> {
        loop {
            if let Some((run, x)) = &mut self.current {
                if *x <= run.x_end() {
                    let pixel = Pixel::new(*x, run.y);
                    *x += 1;
                    return Some(pixel);
                }
            }

            self.current = self.runs.next().map(|&r| (r, r.x_start));
            if self.current.is_none() {
                return None;
            }
        }
    }
}

/// Consuming iterator over pixels in a PixelSet, lazily expanding runs.
pub struct IntoIter {
    runs: std::vec::IntoIter<Run>,
    current: Option<(Run, u16)>,
}

impl Iterator for IntoIter {
    type Item = Pixel;

    fn next(&mut self) -> Option<Pixel> {
        loop {
            if let Some((run, x)) = &mut self.current {
                if *x <= run.x_end() {
                    let pixel = Pixel::new(*x, run.y);
                    *x += 1;
                    return Some(pixel);
                }
            }

            self.current = self.runs.next().map(|r| (r, r.x_start));
            if self.current.is_none() {
                return None;
            }
        }
    }
}

impl<'a> IntoIterator for &'a PixelSet {
    type Item = Pixel;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            runs: self.runs.iter(),
            current: None,
        }
    }
}

impl IntoIterator for PixelSet {
    type Item = Pixel;
    type IntoIter = IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            runs: self.runs.into_iter(),
            current: None,
        }
    }
}

impl PixelSet {
    /// Returns an iterator over the pixels in this set.
    pub fn iter(&self) -> Iter<'_> {
        Iter {
            runs: self.runs.iter(),
            current: None,
        }
    }
}
