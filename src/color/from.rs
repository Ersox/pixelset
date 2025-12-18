use image::Rgba;
use crate::Color;

impl From<Color> for Rgba<u8> {
    fn from(value: Color) -> Self {
        Rgba(value.0)
    }
}

impl From<Color> for [ u8; 4 ] {
    fn from(value: Color) -> Self {
        value.0
    }
}

impl From<[ u8; 4 ]> for Color {
    fn from(value: [ u8; 4 ]) -> Self {
        Color(value)
    }
}

impl From<Rgba<u8>> for Color {
    fn from(value: Rgba<u8>) -> Self {
        Color(value.0)
    }
}