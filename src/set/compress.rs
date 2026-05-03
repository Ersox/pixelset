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
