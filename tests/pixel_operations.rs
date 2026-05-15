use pixelset::{Pixel, PixelSet};

#[test]
fn test_add_to_empty() {
    let mut set = PixelSet::empty();
    set.add(Pixel::new(5, 5));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 1);
    assert!(set.has(Pixel::new(5, 5)));
}

#[test]
fn test_add_extends_run() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0)]);
    set.add(Pixel::new(2, 0));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 3, "Adding adjacent pixel should extend existing run");
}

#[test]
fn test_add_creates_new_run() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0)]);
    set.add(Pixel::new(5, 0));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 2, "Should have two separate pixels");
}

#[test]
fn test_add_merges_runs() {
    let mut set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0),
        Pixel::new(3, 0), Pixel::new(4, 0),
    ]);
    assert_eq!(set.len(), 4);

    set.add(Pixel::new(2, 0));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 5, "Adding gap-filler should merge runs");
}

#[test]
fn test_add_duplicate() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0)]);
    set.add(Pixel::new(0, 0));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 2, "Duplicate should be ignored");
}

// Regression tests for a bug where adding a pixel already covered by a run whose
// x_start < pixel.x was not caught by the early-return check, causing an overlapping
// run to be inserted and breaking the non-overlapping RLE invariant.
#[test]
fn test_add_duplicate_interior_of_run() {
    // Run covers x=3..=7 at y=5. Adding x=5 (interior, not x_start) should be a no-op.
    let mut set = PixelSet::new(vec![
        Pixel::new(3, 5), Pixel::new(4, 5), Pixel::new(5, 5),
        Pixel::new(6, 5), Pixel::new(7, 5),
    ]);
    assert_eq!(set.len(), 5);

    set.add(Pixel::new(5, 5));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 5, "Adding pixel interior to a run should be a no-op");
}

#[test]
fn test_add_duplicate_end_of_run() {
    // Run covers x=3..=7 at y=5. Adding x=7 (x_end, not x_start) should be a no-op.
    let mut set = PixelSet::new(vec![
        Pixel::new(3, 5), Pixel::new(4, 5), Pixel::new(5, 5),
        Pixel::new(6, 5), Pixel::new(7, 5),
    ]);
    assert_eq!(set.len(), 5);

    set.add(Pixel::new(7, 5));

    set.validate_invariants().expect("ADD result has invalid invariants");
    assert_eq!(set.len(), 5, "Adding pixel at end of a run should be a no-op");
}

#[test]
fn test_new_deduplicates_exact_duplicates() {
    let set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(0, 0), Pixel::new(1, 0),
    ]);

    set.validate_invariants().expect("new() with duplicates has invalid invariants");
    assert_eq!(set.len(), 2, "Duplicate pixels should be collapsed");
    assert!(set.has(Pixel::new(0, 0)));
    assert!(set.has(Pixel::new(1, 0)));
}

#[test]
fn test_new_deduplicates_all_same_pixel() {
    let set = PixelSet::new(vec![
        Pixel::new(3, 7), Pixel::new(3, 7), Pixel::new(3, 7),
    ]);

    set.validate_invariants().expect("new() all-duplicate has invalid invariants");
    assert_eq!(set.len(), 1);
    assert!(set.has(Pixel::new(3, 7)));
}

#[test]
fn test_new_deduplicates_unsorted_with_duplicates() {
    // Pixels arrive unsorted and with duplicates; new() must sort then dedup.
    let set = PixelSet::new(vec![
        Pixel::new(2, 0), Pixel::new(0, 0), Pixel::new(1, 0),
        Pixel::new(0, 0), Pixel::new(2, 0),
    ]);

    set.validate_invariants().expect("new() unsorted-with-duplicates has invalid invariants");
    assert_eq!(set.len(), 3);
    assert!(set.has(Pixel::new(0, 0)));
    assert!(set.has(Pixel::new(1, 0)));
    assert!(set.has(Pixel::new(2, 0)));
}

#[test]
fn test_new_deduplicates_preserves_run_merging() {
    // Duplicates shouldn't create phantom gaps that break run merging.
    // x=0,1,2 are all present (with duplicates); the result must be one run of length 3.
    let set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(0, 0),
        Pixel::new(2, 0), Pixel::new(1, 0),
    ]);

    set.validate_invariants().expect("new() duplicate-with-run-merge has invalid invariants");
    assert_eq!(set.len(), 3, "Three distinct pixels should form one run of length 3");
}

#[test]
fn test_discard_from_middle() {
    let mut set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
    ]);
    set.discard(Pixel::new(1, 0));

    set.validate_invariants().expect("DISCARD result has invalid invariants");
    assert_eq!(set.len(), 3);
    assert!(!set.has(Pixel::new(1, 0)));
    assert!(set.has(Pixel::new(0, 0)));
    assert!(set.has(Pixel::new(2, 0)));
    assert!(set.has(Pixel::new(3, 0)));
}

#[test]
fn test_discard_from_start() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0)]);
    set.discard(Pixel::new(0, 0));

    set.validate_invariants().expect("DISCARD result has invalid invariants");
    assert_eq!(set.len(), 2);
    assert!(!set.has(Pixel::new(0, 0)));
}

#[test]
fn test_discard_from_end() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0)]);
    set.discard(Pixel::new(2, 0));

    set.validate_invariants().expect("DISCARD result has invalid invariants");
    assert_eq!(set.len(), 2);
    assert!(!set.has(Pixel::new(2, 0)));
}

#[test]
fn test_discard_only_pixel() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0)]);
    set.discard(Pixel::new(0, 0));

    set.validate_invariants().expect("DISCARD result has invalid invariants");
    assert_eq!(set.len(), 0);
    assert!(set.is_empty());
}

#[test]
fn test_discard_nonexistent() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0)]);
    set.discard(Pixel::new(5, 0));

    set.validate_invariants().expect("DISCARD result has invalid invariants");
    assert_eq!(set.len(), 2, "Discarding nonexistent pixel should not change set");
}
