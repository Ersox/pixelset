use std::io::Result as IoResult;
use crate::PixelSet;
use crate::compression::CompressedPixelSet;

impl PixelSet {
    /// Compress this PixelSet into a CompressedPixelSet.
    pub fn compress(&self) -> IoResult<CompressedPixelSet> {
        let bytes = crate::compression::compress_to_bytes(self)?;
        Ok(CompressedPixelSet::new(bytes))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::Pixel;

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
}
