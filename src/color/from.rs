use image::Rgba;
use crate::Color;

impl From<Color> for Rgba<u8> {
    fn from(value: Color) -> Self {
        Rgba([value.r(), value.g(), value.b(), value.a()])
    }
}

impl From<Color> for [ u8; 4 ] {
    fn from(value: Color) -> Self {
        [value.r(), value.g(), value.b(), value.a()]
    }
}

impl From<[ u8; 4 ]> for Color {
    fn from(value: [ u8; 4 ]) -> Self {
        Color::new(value[0], value[1], value[2], value[3])
    }
}

impl From<Rgba<u8>> for Color {
    fn from(value: Rgba<u8>) -> Self {
        Color::new(value[0], value[1], value[2], value[3])
    }
}