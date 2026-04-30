#[cfg(test)]
mod tests {
    use crate::{Pixel, PixelSet};

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
    fn test_compression_lshape() {
        let mut pixels = vec![];
        // Create an L-shaped region
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
        // Create a 100x100 square
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
        // Create three separate 2x2 rectangles
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
}
