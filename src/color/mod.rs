
#[cfg(feature = "rand")]
use rand::{Rng, seq::IteratorRandom};

use crate::color::error::ColorParseError;

mod error;
mod from;

/// Represents a color with RGBA components.
#[derive(PartialEq, Eq, Hash, Copy, Clone, Debug)]
pub struct Color(pub [ u8; 4 ]);

impl Color {
    /// Creates a `Color` from a hexadecimal color code.
    ///
    /// Supports `#RRGGBB` and `#RRGGBBAA` formats. If alpha is omitted,
    /// it defaults to 255 (fully opaque).
    pub fn hex(hex_code: &str) -> Result<Self, ColorParseError> {
        let hex = hex_code.trim_start_matches('#');

        if hex.len() != 6 && hex.len() != 8 {
            return Err(ColorParseError::InvalidLength(hex.len()));
        }

        let r = u8::from_str_radix(&hex[0..2], 16)
            .map_err(|e| ColorParseError::InvalidHex("R", e))?;
        let g = u8::from_str_radix(&hex[2..4], 16)
            .map_err(|e| ColorParseError::InvalidHex("G", e))?;
        let b = u8::from_str_radix(&hex[4..6], 16)
            .map_err(|e| ColorParseError::InvalidHex("B", e))?;

        let a = if hex.len() == 8 {
            u8::from_str_radix(&hex[6..8], 16)
                .map_err(|e| ColorParseError::InvalidHex("A", e))?
        } else {
            255
        };

        Ok(Self([r, g, b, a]))
    }

    /// Generates a random fully opaque color.
    ///
    /// Each RGB channel is randomly selected from 0-255, and the alpha channel
    /// is always set to 255 (fully opaque). Requires the `rand` feature.
    #[cfg(feature = "rand")]
    pub fn random(rng: &mut impl Rng) -> Self {
        Self([
            (0..=255).choose(rng).unwrap(),
            (0..=255).choose(rng).unwrap(),
            (0..=255).choose(rng).unwrap(),
            255
        ])
    }

    /// Linearly interpolates between `self` and another color based on opacity.
    ///
    /// The `opacity` parameter controls the blend ratio: `0` returns `self` unchanged,
    /// `255` returns `other`, and values in between produce intermediate colors.
    /// All four RGBA channels are blended independently.
    pub fn blend(&self, color: Color, opacity: u8) -> Self {
        let a = opacity as u16; // to avoid overflow
        let inv_a = 255 - a;

        let blended = [
            ((self.0[0] as u16 * inv_a + color.0[0] as u16 * a) / 255) as u8,
            ((self.0[1] as u16 * inv_a + color.0[1] as u16 * a) / 255) as u8,
            ((self.0[2] as u16 * inv_a + color.0[2] as u16 * a) / 255) as u8,
            ((self.0[3] as u16 * inv_a + color.0[3] as u16 * a) / 255) as u8,
        ];

        Self(blended)
    }

    /// Converts the color to perceptual grayscale using standard luminance weights.
    ///
    /// Applies the standard ITU-R BT.601 luma coefficients: `Y = 0.299*R + 0.587*G + 0.114*B`.
    /// The resulting luminance value is used for all three RGB channels, preserving the
    /// original alpha channel unchanged.
    pub fn grayscale(self) -> Self {
        let [ r, g, b, a ] = self.0.map(|el| el as u32);

        let luma = ((299 * r) + (587 * g) + (114 * b)) / 1_000;
        let luma = luma as u8;

        Self([ luma, luma, luma, a as u8 ])
    }
}

/// Pure black with full opacity (RGBA: 0, 0, 0, 255).
pub const BLACK: Color = Color([ 0, 0, 0, 255 ]);

/// Pure white with full opacity (RGBA: 255, 255, 255, 255).
pub const WHITE: Color = Color([ 255, 255, 255, 255 ]);