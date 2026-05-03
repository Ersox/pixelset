use pixelset::{Pixel, PixelSet};

#[test]
fn test_validate_invariants_empty() {
    let set = PixelSet::empty();
    set.validate_invariants().expect("Empty set should be valid");
}

#[test]
fn test_validate_invariants_single() {
    let set = PixelSet::new(vec![Pixel::new(0, 0)]);
    set.validate_invariants().expect("Single pixel set should be valid");
}

#[test]
fn test_validate_invariants_multiple() {
    let set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(0, 1), Pixel::new(1, 1),
    ]);
    set.validate_invariants().expect("Normal set should be valid");
}

#[test]
fn test_invariants_after_and() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(6, 0), Pixel::new(7, 0), Pixel::new(8, 0),
    ]);

    let result = set_a.and(&set_b);
    result.validate_invariants().expect("AND should maintain invariants");
}

#[test]
fn test_invariants_after_or() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(6, 0), Pixel::new(7, 0), Pixel::new(8, 0),
    ]);

    let result = set_a.or(&set_b);
    result.validate_invariants().expect("OR should maintain invariants");
}

#[test]
fn test_invariants_after_xor() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(6, 0), Pixel::new(7, 0), Pixel::new(8, 0),
    ]);

    let result = set_a.xor(&set_b);
    result.validate_invariants().expect("XOR should maintain invariants");
}

#[test]
fn test_invariants_after_difference() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(1, 0), Pixel::new(2, 0), Pixel::new(3, 0),
        Pixel::new(6, 0), Pixel::new(7, 0), Pixel::new(8, 0),
    ]);

    let result = set_a.difference(&set_b);
    result.validate_invariants().expect("DIFFERENCE should maintain invariants");
}

#[test]
fn test_invariants_after_add() {
    let mut set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);

    set.add(Pixel::new(10, 0));
    set.validate_invariants().expect("ADD should maintain invariants");
}

#[test]
fn test_invariants_after_discard() {
    let mut set = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);

    set.discard(Pixel::new(0, 0));
    set.validate_invariants().expect("DISCARD should maintain invariants");
}

#[test]
fn test_invariants_after_chained_mutations() {
    let mut set = PixelSet::new(vec![Pixel::new(0, 0), Pixel::new(1, 0)]);

    set.add(Pixel::new(2, 0));
    set.validate_invariants().expect("After first add");

    set.add(Pixel::new(5, 0));
    set.validate_invariants().expect("After second add");

    set.add(Pixel::new(3, 0));
    set.validate_invariants().expect("After merge add");

    set.discard(Pixel::new(1, 0));
    set.validate_invariants().expect("After discard");

    set.add(Pixel::new(1, 0));
    set.validate_invariants().expect("After re-add");
}

#[test]
fn test_invariants_through_complex_workflow() {
    let set_a = PixelSet::new(vec![
        Pixel::new(0, 0), Pixel::new(1, 0), Pixel::new(2, 0),
    ]);
    let set_b = PixelSet::new(vec![
        Pixel::new(5, 0), Pixel::new(6, 0), Pixel::new(7, 0),
    ]);

    // Build (A | B) & (B | A)
    let union_ab = set_a.or(&set_b);
    union_ab.validate_invariants().expect("A|B has invalid invariants");

    let union_ba = set_b.or(&set_a);
    union_ba.validate_invariants().expect("B|A has invalid invariants");

    let final_result = union_ab.and(&union_ba);
    final_result.validate_invariants().expect("(A|B)&(B|A) has invalid invariants");

    // Result should equal A | B
    assert_eq!(final_result.len(), union_ab.len());
}
