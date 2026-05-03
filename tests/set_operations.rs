use pixelset::{Pixel, PixelSet};

#[test]
fn test_and_with_multiple_runs_per_row() {
    let pixels_a = vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ];
    let pixels_b = vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(6, 0), Pixel::new(7, 0), Pixel::new(8, 0),
    ];

    let set_a = PixelSet::new(pixels_a);
    let set_b = PixelSet::new(pixels_b);

    let result = set_a.and(&set_b);
    result.validate_invariants().expect("AND result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 4);
    assert!(result_pixels.contains(&Pixel::new(1, 0)));
    assert!(result_pixels.contains(&Pixel::new(2, 0)));
    assert!(result_pixels.contains(&Pixel::new(6, 0)));
    assert!(result_pixels.contains(&Pixel::new(7, 0)));
}

#[test]
fn test_or_with_gaps() {
    let pixels_a = vec![
        Pixel::new(0, 0), Pixel::new(1, 0),
        Pixel::new(5, 0), Pixel::new(6, 0),
    ];
    let pixels_b = vec![
        Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(7, 0), Pixel::new(8, 0),
    ];

    let set_a = PixelSet::new(pixels_a);
    let set_b = PixelSet::new(pixels_b);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("OR result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 8);
}

#[test]
fn test_or_disjoint_runs_same_row() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("OR result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 6, "Disjoint gaps should NOT be filled");
    assert!(!result_pixels.contains(&Pixel::new(3, 0)));
    assert!(!result_pixels.contains(&Pixel::new(4, 0)));
}

#[test]
fn test_or_adjacent_runs_same_row() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(3, 0), Pixel::new(4, 0), Pixel::new(5, 0),
    ]);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("OR result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 6, "Adjacent runs should merge");
}

#[test]
fn test_or_overlapping_runs_same_row() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(2, 0), Pixel::new(3, 0), Pixel::new(4, 0), Pixel::new(5, 0),
    ]);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("OR result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 6, "Overlapping runs should merge");
}

#[test]
fn test_or_multiple_disjoint_runs_multiple_rows() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
        Pixel::new(0, 1), Pixel::new(1, 1), Pixel::new(2, 1),
        Pixel::new(5, 1), Pixel::new(6, 1), Pixel::new(7, 1),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0), Pixel::new(4, 0),
        Pixel::new(5, 0), Pixel::new(6, 0),
    ]);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("OR result has invalid invariants");

    assert_eq!(result.len(), 8 + 6, "Row 0 merges to 0-7, Row 1 stays 0-2,5-7");
}

#[test]
fn test_xor_disjoint_sets() {
    let set_a = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0)]);
    let set_b = PixelSet::new(vec![Pixel::new(5, 0), Pixel::new(6, 0)]);

    let result = set_a.xor(&set_b);
    result.validate_invariants().expect("XOR result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 4);
}

#[test]
fn test_xor_overlapping_sets() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
    ]);

    let result = set_a.xor(&set_b);
    result.validate_invariants().expect("XOR result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 2);
    assert!(result_pixels.contains(&Pixel::new(0, 0)));
    assert!(result_pixels.contains(&Pixel::new(3, 0)));
}

#[test]
fn test_difference_disjoint_sets() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(5, 0), Pixel::new(6, 0),
    ]);

    let result = set_a.difference(&set_b);
    result.validate_invariants().expect("DIFFERENCE result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 3);
}

#[test]
fn test_difference_overlapping_sets() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(2, 0), Pixel::new(3, 0), Pixel::new(4, 0),
    ]);

    let result = set_a.difference(&set_b);
    result.validate_invariants().expect("DIFFERENCE result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 2);
    assert!(result_pixels.contains(&Pixel::new(0, 0)));
    assert!(result_pixels.contains(&Pixel::new(1, 0)));
}

#[test]
fn test_difference_partial_coverage() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(4, 0), Pixel::new(5, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(2, 0), Pixel::new(3, 0),
    ]);

    let result = set_a.difference(&set_b);
    result.validate_invariants().expect("DIFFERENCE result has invalid invariants");

    let result_pixels: Vec<_> = result.iter().collect();
    assert_eq!(result_pixels.len(), 4);
    assert!(result_pixels.contains(&Pixel::new(0, 0)));
    assert!(result_pixels.contains(&Pixel::new(1, 0)));
    assert!(result_pixels.contains(&Pixel::new(4, 0)));
    assert!(result_pixels.contains(&Pixel::new(5, 0)));
}

#[test]
fn test_is_subset_true() {
    let set_a = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
    ]);

    assert!(set_a.is_subset(&set_b));
}

#[test]
fn test_is_subset_false() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(5, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);

    assert!(!set_a.is_subset(&set_b));
}

#[test]
fn test_intersects_true() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(2, 0), Pixel::new(3, 0),
    ]);

    assert!(set_a.intersects(&set_b));
}

#[test]
fn test_intersects_false() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(5, 0), Pixel::new(6, 0),
    ]);

    assert!(!set_a.intersects(&set_b));
}

#[test]
fn test_empty_or_empty() {
    let set_a = PixelSet::empty();
    let set_b = PixelSet::empty();

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("Empty|Empty has invalid invariants");
    assert!(result.is_empty());
}

#[test]
fn test_empty_and_nonempty() {
    let set_a = PixelSet::empty();
    let set_b = PixelSet::new(vec![Pixel::new(0, 0)]);

    let result = set_a.and(&set_b);
    result.validate_invariants().expect("Empty&NonEmpty has invalid invariants");
    assert!(result.is_empty());
}

#[test]
fn test_empty_or_nonempty() {
    let set_a = PixelSet::empty();
    let set_b = PixelSet::new(vec![Pixel::new(0, 0)]);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("Empty|NonEmpty has invalid invariants");
    assert_eq!(result.len(), 1);
}

#[test]
fn test_all_operations_maintain_invariants() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(6, 0), Pixel::new(7, 0), Pixel::new(8, 0),
    ]);

    set_a.and(&set_b).validate_invariants().expect("AND should maintain invariants");
    set_a.or(&set_b).validate_invariants().expect("OR should maintain invariants");
    set_a.xor(&set_b).validate_invariants().expect("XOR should maintain invariants");
    set_a.difference(&set_b).validate_invariants().expect("DIFFERENCE should maintain invariants");
}
