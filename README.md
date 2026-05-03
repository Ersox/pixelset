# `pixelset`

`pixelset` is a high-performance, sorted set of pixels optimized for set operations, written in Rust. It provides very fast construction, membership checks and set operations, while remaining very space efficient, and designed to be used with the `image` crate.

---

## Quickstart

```rs
let mut image: DynamicImage = /* ... */;
PixelSet::from_image(&image)
    .select(&image, BLACK)
    .fill(&mut image, WHITE);
```

--- 

## Design

`PixelSet` uses run-length encoding (RLE) to represent pixels efficiently. Instead of storing individual pixels, it stores horizontal runs of consecutive pixels—spans where all pixels on the same row have consecutive x-coordinates.

```rs
pub struct Run {
    pub y: u16,
    pub x_start: u16,
    pub length: u16,
}
```

This encoding ensures excellent performance on coherent regions (common in image processing) while maintaining O(k) memory where k is the number of runs, vs O(n) for individual pixels.

`PixelSet::new` runs in `O(n)` time, using radix sort to organize pixels, deduplicating them, then performing single-pass RLE encoding. If you're sure the initial set of pixels is already sorted and deduplicated, you can use `PixelSet::new_unchecked` to skip that step.

### List Operations

Membership checks (`PixelSet::has`) run in `O(log k)` time using binary search on runs.

Adding or removing individual pixels is `O(k)` due to run splitting and merging. For bulk modifications, combine pixels into a set (with `PixelSet::new`) and use set operations instead.

`PixelSet::filter` allows filtering a set without an intermediate iterator, returning another `PixelSet` directly.

### Set Operations

`PixelSet` provides efficient `O(k)` implementations (where k is the number of runs) for:
- Union (`PixelSet::or`), which provides all elements in either set.
- Intersection (`PixelSet::and`), which provides all elements in both sets.
- Difference (`PixelSet::difference`), which provides all elements from one set, except those of the other.
- Symmetric Difference (`PixelSet::xor`), which provides all elements in either set, but not both.

These are implemented via merge algorithms over runs, scaling with coherent regions rather than pixel count.

### Compression

For serialization, use the `compress()` method to create a `CompressedPixelSet`, which applies zstd compression on top of RLE. This typically achieves 75-150x compression on geographic or coherent image data.