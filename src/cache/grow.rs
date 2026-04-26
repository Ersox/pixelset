use crate::{Pixel, PixelSet, shapes::{Rectangle, Shape}};

struct Stretch;
impl Stretch {
    fn right(
        rectangle: &Rectangle
    ) -> Rectangle {
        Rectangle {
            x: rectangle.x + rectangle.width,
            y: rectangle.y,
            height: rectangle.height,
            width: 1
        }
    }

    fn left(rectangle: &Rectangle) -> Option<Rectangle> {
        if rectangle.x > 0 {
            Some(Rectangle {
                x: rectangle.x - 1,
                y: rectangle.y,
                height: rectangle.height,
                width: 1,
            })
        } else { None }
    }

    fn up(rectangle: &Rectangle) -> Option<Rectangle> {
        if rectangle.y > 0 {
            Some(Rectangle {
                x: rectangle.x,
                y: rectangle.y - 1,
                height: 1,
                width: rectangle.width,
            })
        } else { None }
    }

    fn down(
        rectangle: &Rectangle
    ) -> Rectangle {
        Rectangle {
            x: rectangle.x,
            y: rectangle.y + rectangle.height,
            height: 1,
            width: rectangle.width
        }
    }
}

/// Grows a single pixel into the largest possible axis-aligned rectangle.
///
/// Starting from a seed pixel, this function expands a rectangle in four directions
/// (left, right, up, down) as long as all newly added pixels belong to the provided
/// `PixelSet`. Expansion stops in a direction once a pixel is found outside the set.
///
/// This is a greedy algorithm that produces an axis-aligned rectangle; it does not
/// guarantee an optimal decomposition, but works well for compact rectangular regions.
///
/// ## Complexity
///
/// The expansion continues until all directions are blocked, which can be `O(n)`
/// in the worst case where `n` is the size of the PixelSet.
///
/// ## Use Case
///
/// This function is primarily used by [`PixelCache::generate_from_set`] to decompose
/// a PixelSet into rectangular regions for more compact storage.
///
/// [`PixelCache::generate_from_set`]: crate::PixelCache::generate_from_set
pub fn grow_pixel_into_box(
    pixel: Pixel,
    set: &PixelSet
) -> Rectangle {
    let mut rectangle = Rectangle::at_pixel(pixel);

    let mut can_go_right = true;
    let mut can_go_left = true;
    let mut can_go_down = true;
    let mut can_go_up = true;

    while can_go_right || can_go_left || can_go_down || can_go_up {
        if can_go_right {
            if Stretch::right(&rectangle).set().is_subset(&set) {
                rectangle.width += 1;
            } else {
                can_go_right = false;
            }
        }

        if can_go_left {
            if Stretch::left(&rectangle).is_some_and(|s| s.set().is_subset(&set)) {
                rectangle.width += 1;
                rectangle.x -= 1;
            } else {
                can_go_left = false;
            }
        }

        if can_go_down {
            if Stretch::down(&rectangle).set().is_subset(&set) {
                rectangle.height += 1;
            } else {
                can_go_down = false;
            }
        }

        if can_go_up {
            if Stretch::up(&rectangle).is_some_and(|s| s.set().is_subset(&set)) {
                rectangle.y -= 1;
                rectangle.height += 1;
            } else {
                can_go_up = false;
            }
        }
    }

    rectangle
}