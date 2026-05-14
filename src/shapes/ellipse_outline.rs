use crate::{Pixel, shapes::Shape};

/// Represents an ellipse border with adjustable stroke width.
///
/// An ellipse outline is the border of an ellipse defined by its axis-aligned bounding box
/// and stroke thickness. The border is computed by testing membership in both the outer and
/// inner ellipses using the standard ellipse equation.
///
/// ## Stroke Behavior
///
/// The `stroke` parameter defines the thickness of the border, measured inward from the
/// outer ellipse edge. A pixel is included if it's:
/// - Within the outer ellipse bounds, AND
/// - Not within the inner ellipse (which is shrunk by `stroke` on all sides)
///
/// ## Edge Cases
///
/// - If `stroke * 2 >= width` or `stroke * 2 >= height`, the inner ellipse vanishes
///   and the entire outer ellipse is included (solid ellipse)
/// - A stroke of 0 produces an empty outline
/// - Like [`Ellipse`], precision follows the same ellipse equation with pixel centers at `coord + 0.5`
///
/// [`Ellipse`]: crate::shapes::Ellipse
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct EllipseOutline {
    /// Top-left x of the bounding box
    pub x: u16,
    /// Top-left y of the bounding box
    pub y: u16,
    /// Height of the bounding box
    pub height: u16,
    /// Width of the bounding box
    pub width: u16,
    /// Stroke thickness in pixels
    pub stroke: u16,
}

impl EllipseOutline {
    fn inside_ellipse(x: u16, y: u16, cx: f64, cy: f64, rx: f64, ry: f64) -> bool {
        if rx <= 0.0 || ry <= 0.0 {
            return false;
        }
        let dx = x as f64 + 0.5 - cx;
        let dy = y as f64 + 0.5 - cy;
        (dx * dx) / (rx * rx) + (dy * dy) / (ry * ry) <= 1.0
    }
}

impl Shape for EllipseOutline {
    fn has(&self, pixel: Pixel) -> bool {
        let Pixel { x, y } = pixel;

        if x < self.x || x >= self.x + self.width || y < self.y || y >= self.y + self.height {
            return false;
        }

        let cx = self.x as f64 + self.width as f64 / 2.0;
        let cy = self.y as f64 + self.height as f64 / 2.0;
        let rx = self.width as f64 / 2.0;
        let ry = self.height as f64 / 2.0;

        if !Self::inside_ellipse(x, y, cx, cy, rx, ry) {
            return false;
        }

        let inner_w = self.width.saturating_sub(self.stroke * 2);
        let inner_h = self.height.saturating_sub(self.stroke * 2);

        if inner_w == 0 || inner_h == 0 {
            return true;
        }

        !Self::inside_ellipse(x, y, cx, cy, inner_w as f64 / 2.0, inner_h as f64 / 2.0)
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
