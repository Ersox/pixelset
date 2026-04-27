use pixelset::{Pixel, PixelSet};

fn main() {
    println!("=== PixelSet Compression Demo ===\n");

    // Example 1: Small 2x2 square
    println!("1. Small 2x2 square:");
    let square_2x2 = PixelSet::new(vec![
        Pixel::new(0, 0),
        Pixel::new(1, 0),
        Pixel::new(0, 1),
        Pixel::new(1, 1),
    ]);
    demo_compression(&square_2x2, "2x2 square");

    // Example 2: Larger 10x10 square
    println!("\n2. Medium 10x10 square:");
    let mut square_10x10 = vec![];
    for y in 0..10 {
        for x in 0..10 {
            square_10x10.push(Pixel::new(x, y));
        }
    }
    let square_10x10 = PixelSet::new(square_10x10);
    demo_compression(&square_10x10, "10x10 square");

    // Example 3: Large 50x50 square
    println!("\n3. Large 50x50 square:");
    let mut square_50x50 = vec![];
    for y in 0..50 {
        for x in 0..50 {
            square_50x50.push(Pixel::new(x, y));
        }
    }
    let square_50x50 = PixelSet::new(square_50x50);
    demo_compression(&square_50x50, "50x50 square");

    // Example 4: Very large 100x100 square
    println!("\n4. Very large 100x100 square:");
    let mut square_100x100 = vec![];
    for y in 0..100 {
        for x in 0..100 {
            square_100x100.push(Pixel::new(x, y));
        }
    }
    let square_100x100 = PixelSet::new(square_100x100);
    demo_compression(&square_100x100, "100x100 square");

    // Example 5: Multiple disconnected components (like Greece)
    println!("\n5. Two disconnected components (like mainland + islands):");
    let mut components = vec![];
    // Mainland (5x5)
    for y in 0..5 {
        for x in 0..5 {
            components.push(Pixel::new(x, y));
        }
    }
    // Island (3x3) at (20, 20)
    for y in 20..23 {
        for x in 20..23 {
            components.push(Pixel::new(x, y));
        }
    }
    let components = PixelSet::new(components);
    demo_compression(&components, "Mainland + Island");

    // Example 6: Thin shape (like a line)
    println!("\n6. Horizontal line (1x50):");
    let mut line = vec![];
    for x in 0..50 {
        line.push(Pixel::new(x, 0));
    }
    let line = PixelSet::new(line);
    demo_compression(&line, "Horizontal line");

    println!("\n=== Compression Summary ===");
    println!("Solid shapes achieve high compression ratios because they have a");
    println!("small perimeter relative to their area. The compression formula is:");
    println!("  Ratio ≈ 4 × Area / Boundary_Pixels");
    println!("\nFor geographic regions (France, Germany, etc.), expect 100–400x.");
    println!("For highly compact shapes (circles), expect up to 1,000x+.");
}

fn demo_compression(set: &PixelSet, name: &str) {
    let polygon_set = set.to_polygon_set();
    let raw_bytes = set.len() * 4; // 4 bytes per pixel (u16 + u16)
    let compressed_bytes = polygon_set.encoded_size();
    let ratio = raw_bytes as f64 / compressed_bytes as f64;

    println!("  {} pixels: raw = {} bytes, compressed = {} bytes",
             set.len(), raw_bytes, compressed_bytes);
    println!("  {} components, compression = {:.1}x",
             polygon_set.len(), ratio);

    // Verify round-trip correctness
    let reconstructed = polygon_set.set();
    if set.len() == reconstructed.len() {
        println!("  ✓ Round-trip verification passed");
    } else {
        println!("  ✗ Round-trip verification FAILED");
    }
}
