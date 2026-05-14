use crate::{Pixel, shapes::Shape};

/// Represents a filled ellipse within a bounding box.
///
/// An ellipse is defined by its axis-aligned bounding box with top-left corner `(x, y)`
/// and dimensions `(width, height)`. The ellipse is computed using the standard ellipse
/// equation, testing each pixel's distance from the center.
///
/// ## Precision
///
/// Pixels are included in the ellipse if their center (at `pixel_coord + 0.5`) satisfies
/// the ellipse equation `(dx² / rx²) + (dy² / ry²) ≤ 1.0`, where `(rx, ry)` are the
/// semi-axes of the ellipse.
///
/// ## Edge Cases
///
/// - Zero width or height results in an empty ellipse
/// - The actual pixel count may differ from the mathematical area due to discrete sampling
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
        let rx = self.width as f64 / 2.0;
        let ry = self.height as f64 / 2.0;

        if rx == 0.0 || ry == 0.0 {
            return false;
        }

        let cx = self.x as f64 + rx;
        let cy = self.y as f64 + ry;

        let dx = pixel.x as f64 + 0.5 - cx;
        let dy = pixel.y as f64 + 0.5 - cy;

        pixel.x >= self.x
            && pixel.x < self.x + self.width
            && pixel.y >= self.y
            && pixel.y < self.y + self.height
            && (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) <= 1.0
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
}
