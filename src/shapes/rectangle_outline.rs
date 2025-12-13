use crate::{Pixel, PixelSet, shapes::Shape};

/// Represents a rectangular outline and its pixels.
pub struct RectangleOutline {
    /// The x-coordinate of the top-left corner of the box.
    pub x: u16,
    /// The y-coordinate of the top-left corner of the box.
    pub y: u16,
    /// The height of the box in pixels.
    pub height: u16,
    /// The width of the box in pixels.
    pub width: u16,
    /// Thickness of the outline in pixels
    pub stroke: u16,
}

impl Shape for RectangleOutline {
    fn has(&self, pixel: Pixel) -> bool {
        let Pixel { x, y } = pixel;

        let outer =
            x >= self.x &&
            x < self.x + self.width &&
            y >= self.y &&
            y < self.y + self.height;

        if !outer {
            return false;
        }

        let inner_x = self.x + self.stroke;
        let inner_y = self.y + self.stroke;
        let inner_w = self.width.saturating_sub(self.stroke * 2);
        let inner_h = self.height.saturating_sub(self.stroke * 2);

        let inner =
            inner_w > 0 &&
            inner_h > 0 &&
            x >= inner_x &&
            x < inner_x + inner_w &&
            y >= inner_y &&
            y < inner_y + inner_h;

        !inner
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
        let outer = self.width as usize * self.height as usize;

        let inner_w = self.width.saturating_sub(self.stroke * 2) as usize;
        let inner_h = self.height.saturating_sub(self.stroke * 2) as usize;

        let inner = inner_w * inner_h;

        outer - inner
    }
}