use pixelset::{Pixel, PixelSet, Color, CompressedPixelSet};

#[test]
fn test_pixel_set_compress_decompress() {
    let pixels = vec![
        Pixel::new(0, 0),
        Pixel::new(1, 0),
        Pixel::new(2, 0),
        Pixel::new(0, 1),
        Pixel::new(1, 1),
        Pixel::new(2, 1),
    ];
    let original = PixelSet::new(pixels);

    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), recovered.iter().count());
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_roundtrip_single() {
    let pixels = vec![Pixel::new(5, 5)];
    let original = PixelSet::new(pixels);

    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), recovered.iter().count());
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_roundtrip_line() {
    let pixels = vec![
        Pixel::new(0, 0),
        Pixel::new(1, 0),
        Pixel::new(2, 0),
        Pixel::new(3, 0),
    ];
    let original = PixelSet::new(pixels);

    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), recovered.iter().count());
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_roundtrip_rectangle() {
    let mut pixels = vec![];
    for y in 0..3 {
        for x in 0..5 {
            pixels.push(Pixel::new(x, y));
        }
    }
    let original = PixelSet::new(pixels);

    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), recovered.iter().count());
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_l_shape() {
    let mut pixels = vec![];
    for y in 0..4 {
        for x in 0..3 {
            pixels.push(Pixel::new(x, y));
        }
    }
    for y in 4..6 {
        pixels.push(Pixel::new(0, y));
    }

    let original = PixelSet::new(pixels);
    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), recovered.iter().count());
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_empty() {
    let original = PixelSet::empty();
    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), 0);
    assert_eq!(recovered.iter().count(), 0);
}

#[test]
fn test_compression_large_rectangle() {
    let mut pixels = vec![];
    for y in 0..100 {
        for x in 0..100 {
            pixels.push(Pixel::new(x, y));
        }
    }

    let original = PixelSet::new(pixels);
    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), 10_000);
    assert_eq!(recovered.iter().count(), 10_000);
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_scattered() {
    let mut pixels = vec![];
    for y in 0..2 {
        for x in 0..2 {
            pixels.push(Pixel::new(x, y));
        }
    }
    for y in 0..2 {
        for x in 5..7 {
            pixels.push(Pixel::new(x, y));
        }
    }
    for y in 5..7 {
        for x in 0..2 {
            pixels.push(Pixel::new(x, y));
        }
    }

    let original = PixelSet::new(pixels);
    let compressed = original.compress().unwrap();
    let recovered = compressed.decompress().unwrap();

    assert_eq!(original.iter().count(), 12);
    assert_eq!(recovered.iter().count(), 12);
    for pixel in original.iter() {
        assert!(recovered.iter().any(|p| p == pixel));
    }
}

#[test]
fn test_compression_via_serialization() {
    let mut pixels = vec![];
    for y in 0..50 {
        for x in 0..50 {
            pixels.push(Pixel::new(x, y));
        }
    }

    let original = PixelSet::new(pixels);

    let compressed = original.compress().expect("compression failed");
    let decompressed = compressed.decompress().expect("decompression failed");

    assert_eq!(original.len(), decompressed.len());
    assert_eq!(original, decompressed);
}

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

    assert!(json.starts_with('"') && json.ends_with('"'),
            "PixelSet should serialize as base64 string in JSON");

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
    assert_eq!(json, "\"#FF0099\"", "Fully opaque colors omit alpha channel");

    let deserialized: Color = serde_json::from_str(&json).unwrap();
    assert_eq!(color, deserialized);
}

#[test]
fn test_compressed_pixelset_json_serialization_is_base64() {
    let compressed = CompressedPixelSet::new(vec![1, 2, 3, 4, 5]);

    let json = serde_json::to_string(&compressed).unwrap();

    assert!(json.starts_with('"') && json.ends_with('"'),
            "CompressedPixelSet should serialize as base64 string in JSON");

    let deserialized: CompressedPixelSet = serde_json::from_str(&json).unwrap();
    assert_eq!(compressed.bytes(), deserialized.bytes());
}
