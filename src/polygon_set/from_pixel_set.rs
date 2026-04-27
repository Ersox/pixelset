use std::collections::{HashSet, VecDeque};
use crate::Pixel;
use super::polygon::Polygon;

/// Converts a PixelSet into connected components and extracts their boundaries.
pub(crate) fn from_pixel_set(set: &crate::PixelSet) -> Vec<Polygon> {
    if set.is_empty() {
        return vec![];
    }

    // Build O(1) lookup using pixel keys
    let lookup: HashSet<u32> = set.iter().map(|p| p.key()).collect();

    // Find connected components via BFS (4-connected)
    let components = connected_components(set, &lookup);

    // Extract boundary pixels from each component
    components
        .into_iter()
        .map(|component| {
            let boundary = extract_boundary(&component, &lookup);
            Polygon::new(boundary)
        })
        .collect()
}

/// Finds all 4-connected components in the pixel set.
/// Returns a list of components, each as an (unsorted) Vec<Pixel>.
fn connected_components(
    set: &crate::PixelSet,
    lookup: &HashSet<u32>,
) -> Vec<Vec<Pixel>> {
    let mut visited: HashSet<u32> = HashSet::new();
    let mut components = Vec::new();

    for &pixel in set.iter() {
        let key = pixel.key();
        if visited.contains(&key) {
            continue;
        }

        // Start a new component
        let component = bfs_component(pixel, set, lookup, &mut visited);
        components.push(component);
    }

    components
}

/// BFS to find all pixels in a 4-connected component starting from `start`.
fn bfs_component(
    start: Pixel,
    set: &crate::PixelSet,
    lookup: &HashSet<u32>,
    visited: &mut HashSet<u32>,
) -> Vec<Pixel> {
    let mut component = Vec::new();
    let mut queue = VecDeque::new();

    queue.push_back(start);
    visited.insert(start.key());

    while let Some(current) = queue.pop_front() {
        component.push(current);

        // Check 4-connected neighbors (N, S, E, W)
        let neighbors = [
            (current.x, current.y.saturating_sub(1)), // North
            (current.x, current.y.saturating_add(1)), // South
            (current.x.saturating_add(1), current.y), // East
            (current.x.saturating_sub(1), current.y), // West
        ];

        for (x, y) in neighbors {
            let neighbor = Pixel::new(x, y);
            let key = neighbor.key();

            if !visited.contains(&key) && lookup.contains(&key) {
                visited.insert(key);
                queue.push_back(neighbor);
            }
        }
    }

    component
}

/// Extracts boundary pixels from a connected component.
/// Boundary pixels are those that have at least one 4-neighbor outside the component.
fn extract_boundary(component: &[Pixel], _lookup: &HashSet<u32>) -> Vec<Pixel> {
    let component_set: HashSet<u32> = component.iter().map(|p| p.key()).collect();

    let mut boundary = Vec::new();

    for &pixel in component {
        // Check if this pixel has any 4-neighbor outside the component.
        // Use i32 arithmetic to properly detect boundaries at edges (x=0, y=0).
        let is_boundary = {
            let x = pixel.x as i32;
            let y = pixel.y as i32;
            let neighbors = [
                (x - 1, y),     // West
                (x + 1, y),     // East
                (x, y - 1),     // North
                (x, y + 1),     // South
            ];

            neighbors.iter().any(|&(nx, ny)| {
                // Neighbor is outside if it's out of the valid u16 range or not in component
                if nx < 0 || ny < 0 || nx > 65535 || ny > 65535 {
                    true  // Out of bounds = outside component
                } else {
                    let neighbor = Pixel::new(nx as u16, ny as u16);
                    !component_set.contains(&neighbor.key())
                }
            })
        };

        if is_boundary {
            boundary.push(pixel);
        }
    }

    boundary
}
