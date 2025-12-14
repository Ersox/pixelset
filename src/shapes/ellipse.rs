use crate::{Pixel, PixelSet, shapes::Shape};

/// Represents a filled ellipse inside a bounding box.
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Ellipse {
    /// Top-left x of the bounding box
    pub x: u16,
    /// Top-left y of the bounding box
    pub y: u16,
    /// Height of the bounding box
    pub height: u16,
    /// Width of the bounding box
    pub width: u16,
}

impl Shape for Ellipse {
    fn has(&self, pixel: Pixel) -> bool {
        let Pixel { x, y } = pixel;

        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width;
        let y1 = self.y + self.height;

        if x < x0 || x >= x1 || y < y0 || y >= y1 {
            return false;
        }

        // Center of the ellipse
        let cx = x0 as f64 + self.width as f64 / 2.0;
        let cy = y0 as f64 + self.height as f64 / 2.0;

        let rx = self.width as f64 / 2.0;
        let ry = self.height as f64 / 2.0;

        if rx == 0.0 || ry == 0.0 {
            return false;
        }

        let dx = x as f64 + 0.5 - cx;
        let dy = y as f64 + 0.5 - cy;

        (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) <= 1.0
    }

    fn iter_pixels(&self) -> impl Iterator<Item = Pixel> {
        let x0 = self.x;
        let y0 = self.y;
        let x1 = self.x + self.width;
        let y1 = self.y + self.height;

        (y0..y1).flat_map(move |y| {
            (x0..x1).filter_map(move |x| {
                let p = Pixel::new(x, y);
                self.has(p).then_some(p)
            })
        })
    }

    fn set(&self) -> PixelSet {
        let pixels: Vec<_> = self.iter_pixels().collect();
        PixelSet::new_unchecked(pixels)
    }

    fn len(&self) -> usize {
        // Cheap upper bound; exact count requires scanning anyway
        (self.width as usize * self.height as usize)
    }
}
