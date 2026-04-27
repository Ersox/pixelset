#[cfg(test)]
mod tests {
    use crate::{Pixel, PixelSet, PolygonSet};

    fn round_trip(pixels: Vec<Pixel>) {
        let original = PixelSet::new(pixels);
        let polygon_set = PolygonSet::from_pixel_set(&original);
        let reconstructed = polygon_set.set();

        assert_eq!(
            original.len(),
            reconstructed.len(),
            "round-trip failed: original has {} pixels, reconstructed has {}",
            original.len(),
            reconstructed.len()
        );

        for (a, b) in original.iter().zip(reconstructed.iter()) {
            assert_eq!(
                a, b,
                "pixel mismatch: original had {:?}, reconstructed had {:?}",
                a, b
            );
        }
    }

    #[test]
    fn test_empty() {
        round_trip(vec![]);
    }

    #[test]
    fn test_single_pixel() {
        round_trip(vec![Pixel::new(5, 5)]);
    }

    #[test]
    fn test_horizontal_line() {
        let pixels: Vec<Pixel> = (0..10).map(|x| Pixel::new(x, 5)).collect();
        round_trip(pixels);
    }

    #[test]
    fn test_vertical_line() {
        let pixels: Vec<Pixel> = (0..10).map(|y| Pixel::new(5, y)).collect();
        round_trip(pixels);
    }

    #[test]
    fn test_2x2_square() {
        round_trip(vec![
            Pixel::new(0, 0),
            Pixel::new(1, 0),
            Pixel::new(0, 1),
            Pixel::new(1, 1),
        ]);
    }

    #[test]
    fn test_3x3_square() {
        let mut pixels = Vec::new();
        for y in 0..3 {
            for x in 0..3 {
                pixels.push(Pixel::new(x, y));
            }
        }
        round_trip(pixels);
    }

    #[test]
    fn test_10x10_rectangle() {
        let mut pixels = Vec::new();
        for y in 0..10 {
            for x in 0..10 {
                pixels.push(Pixel::new(x, y));
            }
        }
        round_trip(pixels);
    }

    #[test]
    fn test_two_separate_single_pixels() {
        round_trip(vec![Pixel::new(0, 0), Pixel::new(10, 10)]);
    }

    #[test]
    fn test_two_separate_rectangles() {
        let mut pixels = Vec::new();

        // First rectangle (5x5 at (0, 0))
        for y in 0..5 {
            for x in 0..5 {
                pixels.push(Pixel::new(x, y));
            }
        }

        // Second rectangle (5x5 at (20, 20))
        for y in 20..25 {
            for x in 20..25 {
                pixels.push(Pixel::new(x, y));
            }
        }

        round_trip(pixels);
    }

    #[test]
    fn test_diagonally_adjacent_pixels_are_separate_components() {
        // These two pixels only touch diagonally, so they should be 2 components
        let polygon_set = PolygonSet::from_pixel_set(&PixelSet::new(vec![
            Pixel::new(0, 0),
            Pixel::new(1, 1),
        ]));
        assert_eq!(polygon_set.len(), 2, "diagonally adjacent pixels should be 2 components");
    }

    #[test]
    fn test_horizontally_adjacent_pixels_are_same_component() {
        let polygon_set = PolygonSet::from_pixel_set(&PixelSet::new(vec![
            Pixel::new(0, 0),
            Pixel::new(1, 0),
        ]));
        assert_eq!(
            polygon_set.len(),
            1,
            "horizontally adjacent pixels should be 1 component"
        );
    }

    #[test]
    fn test_compression_ratio_on_large_rectangle() {
        let mut pixels = Vec::new();
        for y in 0..100 {
            for x in 0..100 {
                pixels.push(Pixel::new(x, y));
            }
        }

        let original = PixelSet::new(pixels);
        let polygon_set = PolygonSet::from_pixel_set(&original);

        let raw_bytes = original.len() * 4;
        let compressed_bytes = polygon_set.encoded_size();
        let compression_ratio = raw_bytes as f64 / compressed_bytes as f64;

        eprintln!(
            "100x100 rectangle: raw {} bytes, compressed {} bytes, ratio {:.1}x",
            raw_bytes, compressed_bytes, compression_ratio
        );

        // Expect at least 50x compression for a 100x100 solid rectangle
        assert!(compression_ratio > 50.0);
    }

    #[test]
    fn test_roundtrip_from_polygon_conversion() {
        // Test the From trait
        let pixels = vec![
            Pixel::new(0, 0),
            Pixel::new(1, 0),
            Pixel::new(0, 1),
            Pixel::new(1, 1),
        ];
        let original = PixelSet::new(pixels);

        // Use From trait
        let polygon_set: PolygonSet = From::from(&original);
        let reconstructed: PixelSet = From::from(polygon_set);

        assert_eq!(original.len(), reconstructed.len());
        for (a, b) in original.iter().zip(reconstructed.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_to_polygon_set_method() {
        let pixels = vec![
            Pixel::new(0, 0),
            Pixel::new(1, 0),
            Pixel::new(0, 1),
            Pixel::new(1, 1),
        ];
        let original = PixelSet::new(pixels);

        let polygon_set = original.to_polygon_set();
        let reconstructed = polygon_set.set();

        assert_eq!(original.len(), reconstructed.len());
        for (a, b) in original.iter().zip(reconstructed.iter()) {
            assert_eq!(a, b);
        }
    }

    #[test]
    fn test_concave_u_shape() {
        // A U-shaped region to test concavity handling
        let mut pixels = Vec::new();

        // Left vertical bar
        for y in 0..10 {
            pixels.push(Pixel::new(0, y));
        }

        // Bottom horizontal bar
        for x in 0..5 {
            pixels.push(Pixel::new(x, 9));
        }

        // Right vertical bar
        for y in 0..10 {
            pixels.push(Pixel::new(4, y));
        }

        round_trip(pixels);
    }

    #[test]
    fn test_single_pixel_per_row() {
        // A vertical zig-zag: pixels at alternating columns
        let pixels = vec![
            Pixel::new(0, 0),
            Pixel::new(1, 1),
            Pixel::new(0, 2),
            Pixel::new(1, 3),
        ];
        round_trip(pixels);
    }
}
