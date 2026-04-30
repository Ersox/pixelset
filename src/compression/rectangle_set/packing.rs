use rustc_hash::{FxHashMap, FxHashSet};
use crate::{Pixel, PixelSet};

/// Build a map of y-coordinates to sorted x-coordinates for efficient rectangle expansion.
pub(super) fn build_row_index(pixel_set: &PixelSet) -> FxHashMap<u16, Vec<u16>> {
    let mut rows: FxHashMap<u16, Vec<u16>> = FxHashMap::default();
    for pixel in pixel_set.iter() {
        rows.entry(pixel.y).or_default().push(pixel.x);
    }
    for xs in rows.values_mut() {
        xs.sort_unstable();
    }
    rows
}

/// Expand a rectangle horizontally from `start_x` in row `start_y`.
/// Returns the width of contiguous pixels in the row starting at `start_x`.
pub(super) fn expand_horizontally(start_x: u16, start_y: u16, rows: &FxHashMap<u16, Vec<u16>>) -> u16 {
    let xs = &rows[&start_y];
    let pos = xs.partition_point(|&x| x < start_x);

    let mut width = 0u16;
    for &x in &xs[pos..] {
        if x == start_x + width {
            width += 1;
        } else {
            break;
        }
    }
    width
}

/// Expand a rectangle vertically from `start_y` with a fixed width.
/// Checks that subsequent rows have contiguous pixels in the range [start_x, start_x + width).
pub(super) fn expand_vertically(
    start_x: u16,
    start_y: u16,
    max_width: u16,
    rows: &FxHashMap<u16, Vec<u16>>,
) -> u16 {
    let mut height = 1u16;
    loop {
        let next_y = match start_y.checked_add(height) {
            Some(y) => y,
            None => break,
        };
        let Some(xs) = rows.get(&next_y) else { break };

        let pos = xs.partition_point(|&x| x < start_x);
        let run_ok = xs[pos..]
            .iter()
            .take(max_width as usize)
            .enumerate()
            .all(|(i, &x)| x == start_x + i as u16);

        if !run_ok || xs[pos..].len() < max_width as usize {
            break;
        }

        height += 1;
    }
    height
}

/// Mark all pixels in a rectangle as covered to avoid reprocessing them.
pub(super) fn mark_covered_pixels(
    covered: &mut FxHashSet<Pixel>,
    start_x: u16,
    start_y: u16,
    width: u16,
    height: u16,
) {
    for dy in 0..height {
        for dx in 0..width {
            covered.insert(Pixel::new(start_x + dx, start_y + dy));
        }
    }
}
