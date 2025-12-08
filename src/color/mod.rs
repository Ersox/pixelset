
use image::Rgba;
#[cfg(feature = "rand")]
use rand::{Rng, seq::IteratorRandom};

/// Represents a color with RGBA components.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Color(
    /// The underlying `Rgba<u8>` value.
    pub Rgba<u8>
);

impl From<Rgba<u8>> for Color {
    fn from(value: Rgba<u8>) -> Self {
        Color(value)
    }
}

impl Color {
    /// Creates a `Color` from a hexadecimal color code.
    ///
    /// Supports `#RRGGBB` and `#RRGGBBAA` formats. If alpha is omitted,
    /// it defaults to 255 (fully opaque).
    pub fn hex(hex_code: &str) -> Self {
        let hex = hex_code.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            panic!("Invalid hex code length");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).expect("Invalid R value");
        let g = u8::from_str_radix(&hex[2..4], 16).expect("Invalid G value");
        let b = u8::from_str_radix(&hex[4..6], 16).expect("Invalid B value");

        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16).expect("Invalid A value")
        } else {
            255
        };

        Self(Rgba([ r, g, b, a ]))
    }

    /// Generates a random opaque color.
    #[cfg(feature = "rand")]
    pub fn random(rng: &mut impl Rng) -> Self {
        Self(Rgba([
            (0..=255).choose(rng).unwrap(),
            (0..=255).choose(rng).unwrap(),
            (0..=255).choose(rng).unwrap(),
            255
        ]))
    }

    /// Blends `self` with another `color` using an `opacity` from 0-255.
    /// `opacity = 0` → full `self`, `opacity = 255` → full `color`.
    pub fn blend(&self, color: Color, opacity: u8) -> Self {
        let a = opacity as u16; // to avoid overflow
        let inv_a = 255 - a;

        let blended = [
            ((self.0[0] as u16 * inv_a + color.0[0] as u16 * a) / 255) as u8,
            ((self.0[1] as u16 * inv_a + color.0[1] as u16 * a) / 255) as u8,
            ((self.0[2] as u16 * inv_a + color.0[2] as u16 * a) / 255) as u8,
            ((self.0[3] as u16 * inv_a + color.0[3] as u16 * a) / 255) as u8,
        ];

        Self(Rgba(blended))
    }
}

pub const BLACK: Color = Color(Rgba([ 0, 0, 0, 255 ]));
pub const WHITE: Color = Color(Rgba([ 255, 255, 255, 255 ]));