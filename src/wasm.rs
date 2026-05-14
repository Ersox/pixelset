#![cfg(feature = "wasm")]

use wasm_bindgen::prelude::*;
use crate::{PixelSet, Pixel, Color, shapes::*};

/// WASM-compatible Color wrapper
#[wasm_bindgen]
pub struct WasmColor {
    inner: Color,
}

#[wasm_bindgen]
impl WasmColor {
    /// Create a new color from RGBA components (0-255)
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> WasmColor {
        WasmColor {
            inner: Color::new(r, g, b, a),
        }
    }

    /// Parse a hex color string (e.g., "#FF0000" or "FF0000")
    #[wasm_bindgen]
    pub fn from_hex(hex: &str) -> Result<WasmColor, String> {
        Color::hex(hex)
            .map(|inner| WasmColor { inner })
            .map_err(|e| e.to_string())
    }

    /// Create white color
    #[wasm_bindgen]
    pub fn white() -> WasmColor {
        WasmColor { inner: crate::color::WHITE }
    }

    /// Create black color
    #[wasm_bindgen]
    pub fn black() -> WasmColor {
        WasmColor { inner: crate::color::BLACK }
    }

    /// Get the grayscale representation
    #[wasm_bindgen]
    pub fn grayscale(&self) -> WasmColor {
        WasmColor { inner: self.inner.grayscale() }
    }

    /// Get red component
    #[wasm_bindgen(getter)]
    pub fn r(&self) -> u8 {
        self.inner.r()
    }

    /// Get green component
    #[wasm_bindgen(getter)]
    pub fn g(&self) -> u8 {
        self.inner.g()
    }

    /// Get blue component
    #[wasm_bindgen(getter)]
    pub fn b(&self) -> u8 {
        self.inner.b()
    }

    /// Get alpha component
    #[wasm_bindgen(getter)]
    pub fn a(&self) -> u8 {
        self.inner.a()
    }
}

impl AsRef<Color> for WasmColor {
    fn as_ref(&self) -> &Color {
        &self.inner
    }
}

/// WASM-compatible Pixel wrapper
#[wasm_bindgen]
pub struct WasmPixel {
    inner: Pixel,
}

#[wasm_bindgen]
impl WasmPixel {
    /// Create a new pixel at the given coordinates
    #[wasm_bindgen(constructor)]
    pub fn new(y: u16, x: u16) -> WasmPixel {
        WasmPixel {
            inner: Pixel { y, x },
        }
    }

    /// Get the Y coordinate
    #[wasm_bindgen(getter)]
    pub fn y(&self) -> u16 {
        self.inner.y
    }

    /// Get the X coordinate
    #[wasm_bindgen(getter)]
    pub fn x(&self) -> u16 {
        self.inner.x
    }
}

/// WASM-compatible PixelSet wrapper
#[wasm_bindgen]
pub struct WasmPixelSet {
    inner: PixelSet,
}

#[wasm_bindgen]
impl WasmPixelSet {
    /// Create an empty PixelSet
    #[wasm_bindgen(constructor)]
    pub fn new() -> WasmPixelSet {
        WasmPixelSet {
            inner: PixelSet::new(vec![]),
        }
    }

    /// Create a PixelSet from pixel coordinates (y, x) pairs
    #[wasm_bindgen]
    pub fn from_coordinates(coords: Vec<u16>) -> Result<WasmPixelSet, String> {
        if coords.len() % 2 != 0 {
            return Err("Coordinate array must have even length (pairs of y, x)".to_string());
        }

        let pixels: Vec<Pixel> = coords
            .chunks(2)
            .map(|chunk| Pixel { y: chunk[0], x: chunk[1] })
            .collect();

        Ok(WasmPixelSet {
            inner: PixelSet::new(pixels),
        })
    }

    /// Check if a pixel is in the set
    #[wasm_bindgen]
    pub fn has(&self, y: u16, x: u16) -> bool {
        self.inner.has(Pixel { y, x })
    }

    /// Get the number of pixels in the set
    #[wasm_bindgen]
    pub fn len(&self) -> u32 {
        self.inner.len() as u32
    }

    /// Check if the set is empty
    #[wasm_bindgen]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Add a pixel to the set
    #[wasm_bindgen]
    pub fn add(&mut self, y: u16, x: u16) {
        self.inner.add(Pixel { y, x });
    }

    /// Remove a pixel from the set
    #[wasm_bindgen]
    pub fn discard(&mut self, y: u16, x: u16) {
        self.inner.discard(Pixel { y, x });
    }

    /// Get the intersection of two sets
    #[wasm_bindgen]
    pub fn and(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.and(&other.inner),
        }
    }

    /// Get the union of two sets
    #[wasm_bindgen]
    pub fn or(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.or(&other.inner),
        }
    }

    /// Get the symmetric difference of two sets
    #[wasm_bindgen]
    pub fn xor(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.xor(&other.inner),
        }
    }

    /// Get pixels in this set but not in another
    #[wasm_bindgen]
    pub fn difference(&self, other: &WasmPixelSet) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.difference(&other.inner),
        }
    }

    /// Check if this set is a subset of another
    #[wasm_bindgen]
    pub fn is_subset(&self, other: &WasmPixelSet) -> bool {
        self.inner.is_subset(&other.inner)
    }

    /// Check if this set intersects with another
    #[wasm_bindgen]
    pub fn intersects(&self, other: &WasmPixelSet) -> bool {
        self.inner.intersects(&other.inner)
    }

    /// Get the bounding box of the set as [min_y, min_x, max_y, max_x]
    #[wasm_bindgen]
    pub fn bounds(&self) -> Box<[u16]> {
        if let Some((min_y, min_x, max_y, max_x)) = self.inner.bounds() {
            vec![min_y, min_x, max_y, max_x].into_boxed_slice()
        } else {
            vec![].into_boxed_slice()
        }
    }

    /// Get all pixels as a flat array of coordinates [y1, x1, y2, x2, ...]
    #[wasm_bindgen]
    pub fn to_coordinates(&self) -> Vec<u16> {
        self.inner
            .iter()
            .flat_map(|p| vec![p.y, p.x])
            .collect()
    }

    /// Get the number of pixels (alias for len)
    #[wasm_bindgen]
    pub fn pixel_count(&self) -> u32 {
        self.len()
    }
}

/// WASM-compatible Rectangle shape
#[wasm_bindgen]
pub struct WasmRectangle {
    inner: Rectangle,
}

#[wasm_bindgen]
impl WasmRectangle {
    /// Create a filled rectangle
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> WasmRectangle {
        WasmRectangle {
            inner: Rectangle { x, y, width, height },
        }
    }

    /// Convert to a PixelSet
    #[wasm_bindgen]
    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.set(),
        }
    }
}

/// WASM-compatible Ellipse shape
#[wasm_bindgen]
pub struct WasmEllipse {
    inner: Ellipse,
}

#[wasm_bindgen]
impl WasmEllipse {
    /// Create a filled ellipse
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> WasmEllipse {
        WasmEllipse {
            inner: Ellipse { x, y, width, height },
        }
    }

    /// Convert to a PixelSet
    #[wasm_bindgen]
    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.set(),
        }
    }
}

/// WASM-compatible Rectangle outline shape
#[wasm_bindgen]
pub struct WasmRectangleOutline {
    inner: RectangleOutline,
}

#[wasm_bindgen]
impl WasmRectangleOutline {
    /// Create a rectangle outline with adjustable stroke width
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16, stroke: u16) -> WasmRectangleOutline {
        WasmRectangleOutline {
            inner: RectangleOutline { x, y, width, height, stroke },
        }
    }

    /// Convert to a PixelSet
    #[wasm_bindgen]
    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.set(),
        }
    }
}

/// WASM-compatible Ellipse outline shape
#[wasm_bindgen]
pub struct WasmEllipseOutline {
    inner: EllipseOutline,
}

#[wasm_bindgen]
impl WasmEllipseOutline {
    /// Create an ellipse outline with adjustable stroke width
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16, stroke: u16) -> WasmEllipseOutline {
        WasmEllipseOutline {
            inner: EllipseOutline { x, y, width, height, stroke },
        }
    }

    /// Convert to a PixelSet
    #[wasm_bindgen]
    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet {
            inner: self.inner.set(),
        }
    }
}
