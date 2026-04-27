use crate::Pixel;
use super::polygon::Polygon;

/// Converts a PolygonSet to a PixelSet.
/// Each polygon's boundary is used to reconstruct the full set of pixels.
pub(crate) fn to_pixel_set(polygons: &[Polygon]) -> crate::PixelSet {
    let mut all_pixels = Vec::new();

    for polygon in polygons {
        all_pixels.extend(polygon.set().iter().copied());
    }

    crate::PixelSet::new(all_pixels)
}
