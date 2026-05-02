#[cfg(test)]
mod tests {
    use crate::{Pixel, PixelSet, Color};
    use crate::CompressedPixelSet;

    #[test]
    fn test_pixelset_json_serialization_is_base64() {
        let mut pixels = vec![];
        for y in 0..3 {
            for x in 0..3 {
                pixels.push(Pixel::new(x as u16, y as u16));
            }
        }
        let pixel_set = PixelSet::new(pixels);

        let json = serde_json::to_string(&pixel_set).unwrap();

        // Should be a string (base64)
        assert!(json.starts_with('"') && json.ends_with('"'),
                "PixelSet should serialize as base64 string in JSON");

        // Should deserialize correctly
        let deserialized: PixelSet = serde_json::from_str(&json).unwrap();
        assert_eq!(pixel_set, deserialized);
    }

    #[test]
    fn test_color_hex_serialization() {
        let color = Color::new(0xAA, 0xBB, 0xCC, 0xDD);

        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"#AABBCCDD\"", "Color should serialize as hex string");

        let deserialized: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_color_hex_without_alpha() {
        let color = Color::new(0xFF, 0x00, 0x99, 0xFF);

        let json = serde_json::to_string(&color).unwrap();
        assert_eq!(json, "\"#FF0099FF\"");

        let deserialized: Color = serde_json::from_str(&json).unwrap();
        assert_eq!(color, deserialized);
    }

    #[test]
    fn test_compressed_pixelset_json_serialization_is_base64() {
        let compressed = CompressedPixelSet::new(vec![1, 2, 3, 4, 5]);

        let json = serde_json::to_string(&compressed).unwrap();

        // Should be a string (base64)
        assert!(json.starts_with('"') && json.ends_with('"'),
                "CompressedPixelSet should serialize as base64 string in JSON");

        let deserialized: CompressedPixelSet = serde_json::from_str(&json).unwrap();
        assert_eq!(compressed.bytes(), deserialized.bytes());
    }
}
