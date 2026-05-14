use crate::{Pixel, PixelSet};
use crate::set::Run;

/// Shared pixel-expansion logic over any run iterator.
/// Private — consumers see only `Iter` and `IntoIter`.
struct RunIter<I> {
    runs: I,
    current: Option<(Run, u16)>,
}

impl<I: Iterator<Item = Run>> Iterator for RunIter<I> {
    type Item = Pixel;

    fn next(&mut self) -> Option<Pixel> {
        loop {
            if let Some((run, x)) = &mut self.current
                && *x <= run.x_end()
            {
                let pixel = Pixel::new(*x, run.y);
                *x += 1;
                return Some(pixel);
            }
            self.current = self.runs.next().map(|r| (r, r.x_start));
            self.current?;
        }
    }
}

/// Borrowing iterator over pixels in a `PixelSet`, lazily expanding runs.
pub struct Iter<'a>(RunIter<std::iter::Copied<std::slice::Iter<'a, Run>>>);

/// Consuming iterator over pixels in a `PixelSet`, lazily expanding runs.
pub struct IntoIter(RunIter<std::vec::IntoIter<Run>>);

impl Iterator for Iter<'_> {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> { self.0.next() }
}

impl Iterator for IntoIter {
    type Item = Pixel;
    fn next(&mut self) -> Option<Pixel> { self.0.next() }
}

impl<'a> IntoIterator for &'a PixelSet {
    type Item = Pixel;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Iter<'a> {
        Iter(RunIter { runs: self.runs.iter().copied(), current: None })
    }
}

impl IntoIterator for PixelSet {
    type Item = Pixel;
    type IntoIter = IntoIter;

    fn into_iter(self) -> IntoIter {
        IntoIter(RunIter { runs: self.runs.into_iter(), current: None })
    }
}

impl PixelSet {
    /// Returns an iterator over the pixels in this set.
    pub fn iter(&self) -> Iter<'_> {
        Iter(RunIter { runs: self.runs.iter().copied(), current: None })
    }
}
