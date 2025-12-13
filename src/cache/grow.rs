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