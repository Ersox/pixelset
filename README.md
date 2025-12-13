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

Internally, `PixelSet` is a `Vec<Pixel>`, where a pixel is a 4-byte structure with `x` and `y` coordinates.

```rs
pub struct Pixel {
    pub y: u16,
    pub x: u16,
}
```

To ensure its performance guarantees, `PixelSet` ensures that the list of provided pixels are sorted in row-major order `(y, x)`. 

`PixelSet::new` runs in `O(n)` time, using radix sort and then deduplicating pixels after. If you're sure the initial set of pixels is sorted already, you can use `PixelSet::new_unchecked` to skip that step.

### List Operations

Membership checks (`PixelSet::has`) can be guaranteed in `O(log n)` time, using binary search.

Adding or removing individual pixels from the set will take `O(n)`, since they have to be sorted into the existing structure. Instead, to add or remove multiple pixels, combine them into a set (with `PixelSet::new`), and then use a union or difference.

`PixelSet::filter` (and similar utilities) allows for filtering a set without needing an intermediate iterator, instead providing another `PixelSet`.

### Set Operations

`PixelSet` provides efficient `O(n)` implementations for the following set operations:
- Union (`PixelSet::extend`), which provides all elements in either set.
- Intersection (`PixelSet::and`), which provides all elements in both sets.
- Difference (`PixelSet::without`), which provides all elements from one set, except those of the other.

These can be implemented quickly through linear algorithms that take advantage of sorting guarantees.

### Efficient Caching

For saving and loading complex sets with continuous areas, `PixelCache` can be used, which compresses the set into a list of continuous rectangles that can be easily loaded again. Saving into a cache may be expensive in performance, though.