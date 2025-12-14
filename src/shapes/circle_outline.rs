use crate::{Pixel, PixelSet, shapes::Shape};

/// Represents a circular outline and its pixels.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct CircleOutline {
    /// Center x-coordinate
    pub cx: u16,
    /// Center y-coordinate
    pub cy: u16,
    /// Outer radius
    pub radius: u16,
    /// Stroke thickness in pixels
    pub stroke: u16,
}

impl Shape for CircleOutline {
    fn has(&self, pixel: Pixel) -> bool {
        let dx = pixel.x as i32 - self.cx as i32;
        let dy = pixel.y as i32 - self.cy as i32;

        let dist2 = dx * dx + dy * dy;

        let r_outer = self.radius as i32;
        let r_inner = (self.radius.saturating_sub(self.stroke)) as i32;

        let outer = dist2 <= r_outer * r_outer;
        let inner = dist2 < r_inner * r_inner;

        outer && !inner
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
        let r = self.radius as usize;
        let inner = self.radius.saturating_sub(self.stroke) as usize;

        r * r * 4 - inner * inner * 4
    }
}
