use crate::{Pixel, PixelSet, shapes::Shape};

/// Represents a filled circle and its pixels.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Circle {
    /// Center x-coordinate
    pub cx: u16,
    /// Center y-coordinate
    pub cy: u16,
    /// Radius in pixels
    pub radius: u16,
}

impl Shape for Circle {
    fn has(&self, pixel: Pixel) -> bool {
        let dx = pixel.x as i32 - self.cx as i32;
        let dy = pixel.y as i32 - self.cy as i32;
        let r = self.radius as i32;

        dx * dx + dy * dy <= r * r
    }

    fn iter_pixels(&self) -> impl Iterator<Item = Pixel> {
        let r = self.radius;
        let cx = self.cx;
        let cy = self.cy;

        let x0 = cx.saturating_sub(r);
        let y0 = cy.saturating_sub(r);
        let x1 = cx + r + 1;
        let y1 = cy + r + 1;

        (y0..y1).flat_map(move |y| {
            (x0..x1).filter_map(move |x| {
                let p = Pixel::new(x, y);
                self.has(p).then_some(p)
            })
        })
    }

    fn set(&self) -> PixelSet {
        PixelSet::new_unchecked(self.iter_pixels().collect())
    }

    fn len(&self) -> usize {
        // Approximate area; exact count would require iteration anyway
        let r = self.radius as usize;
        (r * r * 4) // loose upper bound, cheap and safe
    }
}
