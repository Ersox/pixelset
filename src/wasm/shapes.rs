use wasm_bindgen::prelude::*;
use crate::shapes::{Ellipse, EllipseOutline, Rectangle, RectangleOutline, Shape};
use super::pixel_set::WasmPixelSet;

/// An axis-aligned filled rectangle.
/// `(x, y)` is the top-left corner; `width` and `height` are in pixels.
#[wasm_bindgen]
pub struct WasmRectangle { inner: Rectangle }

#[wasm_bindgen]
impl WasmRectangle {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> WasmRectangle {
        WasmRectangle { inner: Rectangle { x, y, width, height } }
    }

    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.set() }
    }
}

/// The border of a rectangle with a configurable stroke width (inset).
/// `(x, y)` is the top-left corner of the outer edge.
#[wasm_bindgen]
pub struct WasmRectangleOutline { inner: RectangleOutline }

#[wasm_bindgen]
impl WasmRectangleOutline {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16, stroke: u16) -> WasmRectangleOutline {
        WasmRectangleOutline { inner: RectangleOutline { x, y, width, height, stroke } }
    }

    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.set() }
    }
}

/// A filled ellipse. `(x, y)` is the top-left corner of the bounding box;
/// `width` and `height` define its extents.
#[wasm_bindgen]
pub struct WasmEllipse { inner: Ellipse }

#[wasm_bindgen]
impl WasmEllipse {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16) -> WasmEllipse {
        WasmEllipse { inner: Ellipse { x, y, width, height } }
    }

    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.set() }
    }
}

/// The border of an ellipse with a configurable stroke width (inset).
/// `(x, y)` is the top-left corner of the bounding box.
#[wasm_bindgen]
pub struct WasmEllipseOutline { inner: EllipseOutline }

#[wasm_bindgen]
impl WasmEllipseOutline {
    #[wasm_bindgen(constructor)]
    pub fn new(x: u16, y: u16, width: u16, height: u16, stroke: u16) -> WasmEllipseOutline {
        WasmEllipseOutline { inner: EllipseOutline { x, y, width, height, stroke } }
    }

    pub fn to_pixel_set(&self) -> WasmPixelSet {
        WasmPixelSet { inner: self.inner.set() }
    }
}
