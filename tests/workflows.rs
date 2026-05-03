use pixelset::{Pixel, PixelSet, Color};
use image::{DynamicImage, ImageBuffer, Rgba, GenericImageView};

#[test]
fn test_chained_select_or_with_multiple_colors() {
    let mut img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(10, 10);

    let color_a = Rgba([255, 0, 0, 255]);      // Red
    let color_b = Rgba([0, 255, 0, 255]);      // Green
    let color_c = Rgba([0, 0, 255, 255]);      // Blue

    // Red square: (0-2, 0-2)
    for y in 0..3 {
        for x in 0..3 {
            img_buf.put_pixel(x, y, color_a);
        }
    }

    // Green square: (5-7, 0-2)
    for y in 0..3 {
        for x in 5..8 {
            img_buf.put_pixel(x, y, color_b);
        }
    }

    // Blue square: (0-2, 5-7)
    for y in 5..8 {
        for x in 0..3 {
            img_buf.put_pixel(x, y, color_c);
        }
    }

    let image = DynamicImage::ImageRgba8(img_buf);
    let full_set = PixelSet::from_image(&image);
    full_set.validate_invariants().expect("Initial full set has invalid invariants");

    let red = full_set.select(&image, Color::from(color_a));
    red.validate_invariants().expect("Red selection has invalid invariants");
    assert_eq!(red.len(), 9);

    let green = full_set.select(&image, Color::from(color_b));
    green.validate_invariants().expect("Green selection has invalid invariants");
    assert_eq!(green.len(), 9);

    let blue = full_set.select(&image, Color::from(color_c));
    blue.validate_invariants().expect("Blue selection has invalid invariants");
    assert_eq!(blue.len(), 9);

    let red_or_green = red.or(&green);
    red_or_green.validate_invariants().expect("red|green has invalid invariants");
    assert_eq!(red_or_green.len(), 18, "Two disjoint 3x3 regions");

    let all_three = red_or_green.or(&blue);
    all_three.validate_invariants().expect("(red|green)|blue has invalid invariants");
    assert_eq!(all_three.len(), 27, "Three disjoint 3x3 regions");

    // Verify all pixels in result are correctly colored
    for pixel in all_three.iter() {
        let color = image.get_pixel(pixel.x as u32, pixel.y as u32);
        let is_red = color == color_a;
        let is_green = color == color_b;
        let is_blue = color == color_c;
        assert!(
            is_red || is_green || is_blue,
            "Pixel {:?} in result has unexpected color {:?}",
            pixel,
            color
        );
    }
}

#[test]
fn test_chained_operations_complex() {
    // Simulate: (A & B) | (C - D)
    let a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
    ]);
    let b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0), Pixel::new(4, 0),
    ]);
    let c = PixelSet::new(vec![
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);
    let d = PixelSet::new(vec![
        Pixel::new(6, 0),
    ]);

    let and_result = a.and(&b);
    and_result.validate_invariants().expect("(A & B) has invalid invariants");

    let diff_result = c.difference(&d);
    diff_result.validate_invariants().expect("(C - D) has invalid invariants");

    let final_result = and_result.or(&diff_result);
    final_result.validate_invariants().expect("(A&B)|(C-D) has invalid invariants");

    assert_eq!(final_result.len(), 5);
}

#[test]
fn test_repeated_unions_no_corruption() {
    // Simulate repeated union operations like in mapgen workflows
    let mut result = PixelSet::empty();

    for i in 0..5 {
        let set = PixelSet::new(vec![
            Pixel::new(i * 10, 0), Pixel::new(i * 10 + 1, 0), Pixel::new(i * 10 + 2, 0),
        ]);
        result = result.or(&set);
        result.validate_invariants().expect(&format!("Iteration {} has invalid invariants", i));
    }

    assert_eq!(result.len(), 15, "Should have 5 * 3 pixels");
}

#[test]
fn test_select_with_gaps() {
    let mut img_buf: ImageBuffer<Rgba<u8>, Vec<u8>> = ImageBuffer::new(20, 10);
    let red = Rgba([255, 0, 0, 255]);

    // Create red pixels at x=[0-2] and x=[10-12] to test gap handling
    for y in 0..10 {
        for x in 0..3 {
            img_buf.put_pixel(x, y, red);
        }
        for x in 10..13 {
            img_buf.put_pixel(x, y, red);
        }
    }

    let image = DynamicImage::ImageRgba8(img_buf);
    let full_set = PixelSet::from_image(&image);
    let red_set = full_set.select(&image, Color::from(red));

    red_set.validate_invariants().expect("Selection with gaps has invalid invariants");
    assert_eq!(red_set.len(), 60, "Two 3x10 regions");

    // Verify the gap exists (no pixels at x=5)
    for y in 0..10 {
        assert!(!red_set.has(Pixel::new(5, y)), "Gap should not be filled");
    }
}
