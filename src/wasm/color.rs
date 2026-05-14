use wasm_bindgen::prelude::*;
use crate::Color;

/// An RGBA color with components in the range 0–255.
#[wasm_bindgen]
pub struct WasmColor {
    pub(super) inner: Color,
}

#[wasm_bindgen]
impl WasmColor {
    /// Create a color from RGBA components (0–255 each).
    #[wasm_bindgen(constructor)]
    pub fn new(r: u8, g: u8, b: u8, a: u8) -> WasmColor {
        WasmColor { inner: Color::new(r, g, b, a) }
    }

    /// Parse a hex color string. Accepts `#RRGGBB` and `#RRGGBBAA`; the `#` is optional.
    /// Alpha defaults to 255 (fully opaque) when omitted.
    pub fn from_hex(hex: &str) -> Result<WasmColor, String> {
        Color::hex(hex)
            .map(|inner| WasmColor { inner })
            .map_err(|e| e.to_string())
    }

    /// Opaque white (`#FFFFFF`).
    pub fn white() -> WasmColor {
        WasmColor { inner: crate::color::WHITE }
    }

    /// Opaque black (`#000000`).
    pub fn black() -> WasmColor {
        WasmColor { inner: crate::color::BLACK }
    }

    /// Convert to perceptual grayscale using ITU-R BT.601 luma coefficients.
    /// Alpha is preserved.
    pub fn grayscale(&self) -> WasmColor {
        WasmColor { inner: self.inner.grayscale() }
    }

    #[wasm_bindgen(getter)]
    pub fn r(&self) -> u8 { self.inner.r() }

    #[wasm_bindgen(getter)]
    pub fn g(&self) -> u8 { self.inner.g() }

    #[wasm_bindgen(getter)]
    pub fn b(&self) -> u8 { self.inner.b() }

    #[wasm_bindgen(getter)]
    pub fn a(&self) -> u8 { self.inner.a() }
}
