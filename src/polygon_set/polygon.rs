use crate::Pixel;
use super::chain_code::ChainDir;

/// A single connected component boundary.
///
/// Stores all boundary pixels (pixels with at least one neighbor outside the component).
/// These are stored in sorted (y, x) order for efficient reconstruction.
#[derive(Clone, Debug)]
pub struct Polygon {
    /// All boundary pixels of this connected component, sorted in (y, x) order.
    pub(crate) boundary: Vec<Pixel>,
}

impl Polygon {
    /// Creates a new polygon from boundary pixels.
    pub(crate) fn new(boundary: Vec<Pixel>) -> Self {
        Self { boundary }
    }

    /// Returns the topmost-leftmost boundary pixel of this component.
    pub fn start(&self) -> Pixel {
        self.boundary.iter().copied().min().unwrap_or_else(|| Pixel::new(0, 0))
    }

    /// Returns the number of boundary pixels.
    pub fn step_count(&self) -> usize {
        self.boundary.len()
    }

    /// Returns the boundary pixels.
    pub fn boundary_pixels(&self) -> Vec<Pixel> {
        self.boundary.clone()
    }

    /// Reconstructs the filled `PixelSet` for this polygon alone.
    pub fn set(&self) -> crate::PixelSet {
        let boundary = &self.boundary;

        if boundary.is_empty() {
            return crate::PixelSet::empty();
        }

        if boundary.len() == 1 {
            return crate::PixelSet::new(boundary.to_vec());
        }

        let mut pixels = boundary.to_vec();

        // For scanline fill, we iterate over unique y-values and find x-ranges
        let min_y = boundary.iter().map(|p| p.y).min().unwrap_or(0);
        let max_y = boundary.iter().map(|p| p.y).max().unwrap_or(0);

        // Build a map of y -> sorted list of unique x values on that y
        let mut y_to_xs: std::collections::BTreeMap<u16, Vec<u16>> =
            std::collections::BTreeMap::new();
        for &pixel in boundary {
            y_to_xs.entry(pixel.y).or_insert_with(Vec::new).push(pixel.x);
        }

        // Sort and deduplicate x values for each y
        for xs in y_to_xs.values_mut() {
            xs.sort_unstable();
            xs.dedup();
        }

        // For each scanline, fill interior pixels between pairs of boundary x-values
        for y in min_y..=max_y {
            if let Some(xs) = y_to_xs.get(&y) {
                if xs.len() >= 2 {
                    // Use pairs of x-values to define fill regions (even-odd rule)
                    for i in (0..xs.len()).step_by(2) {
                        if i + 1 < xs.len() {
                            let x_start = xs[i];
                            let x_end = xs[i + 1];
                            // Fill interior between the pair
                            for x in (x_start + 1)..x_end {
                                pixels.push(Pixel::new(x, y));
                            }
                        }
                    }
                }
            }
        }

        crate::PixelSet::new(pixels)
    }
}
