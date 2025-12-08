use crate::{PixelSet, Pixel, PixelBox};

struct Stretch;
impl Stretch {
    fn right(
        pixel_box: &PixelBox
    ) -> PixelBox {
        PixelBox {
            x: pixel_box.x + pixel_box.width,
            y: pixel_box.y,
            height: pixel_box.height,
            width: 1
        }
    }

    fn left(pixel_box: &PixelBox) -> Option<PixelBox> {
        if pixel_box.x > 0 {
            Some(PixelBox {
                x: pixel_box.x - 1,
                y: pixel_box.y,
                height: pixel_box.height,
                width: 1,
            })
        } else { None }
    }

    fn up(pixel_box: &PixelBox) -> Option<PixelBox> {
        if pixel_box.y > 0 {
            Some(PixelBox {
                x: pixel_box.x,
                y: pixel_box.y - 1,
                height: 1,
                width: pixel_box.width,
            })
        } else { None }
    }

    fn down(
        pixel_box: &PixelBox
    ) -> PixelBox {
        PixelBox {
            x: pixel_box.x,
            y: pixel_box.y + pixel_box.height,
            height: 1,
            width: pixel_box.width
        }
    }
}

pub fn grow_pixel_into_box(
    pixel: Pixel,
    set: &PixelSet
) -> PixelBox {
    let mut pixel_box = PixelBox::at_pixel(pixel);

    let mut can_go_right = true;
    let mut can_go_left = true;
    let mut can_go_down = true;
    let mut can_go_up = true;

    while can_go_right || can_go_left || can_go_down || can_go_up {
        if can_go_right {
            if Stretch::right(&pixel_box).group().is_subset(&set) {
                pixel_box.width += 1;
            } else {
                can_go_right = false;
            }
        }

        if can_go_left {
            if Stretch::left(&pixel_box).is_some_and(|s| s.group().is_subset(&set)) {
                pixel_box.width += 1;
                pixel_box.x -= 1;
            } else {
                can_go_left = false;
            }
        }

        if can_go_down {
            if Stretch::down(&pixel_box).group().is_subset(&set) {
                pixel_box.height += 1;
            } else {
                can_go_down = false;
            }
        }

        if can_go_up {
            if Stretch::up(&pixel_box).is_some_and(|s| s.group().is_subset(&set)) {
                pixel_box.y -= 1;
                pixel_box.height += 1;
            } else {
                can_go_up = false;
            }
        }
    }

    pixel_box
}