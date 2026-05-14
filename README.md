# `pixelset`

`pixelset` is a high-performance, sorted set of pixels optimized for set operations, written in Rust. It provides very fast construction, membership checks, and set operations, while remaining space efficient. Designed to integrate with the `image` crate.

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

`PixelSet::new` runs in `O(n)` time, using radix sort to organize pixels, deduplicating them, then performing a single-pass RLE encode. Use `PixelSet::new_unchecked` to skip sorting and dedup if your input is already clean.

### Shapes

The `shapes` module provides geometric primitives that generate `PixelSet` values directly:

- `Rectangle` / `RectangleOutline` — axis-aligned rectangles, filled or stroked
- `Ellipse` / `EllipseOutline` — filled or stroked ellipses

All shapes implement the `Shape` trait with a `set()` method.

### List Operations

Membership checks (`PixelSet::has`) run in `O(log k)` using binary search on runs.

Adding or removing individual pixels is `O(k)` due to run splitting and merging. For bulk modifications, construct a new set and use set operations instead.

`PixelSet::filter` allows predicate-based filtering without an intermediate iterator.

### Set Operations

Efficient `O(k₁ + k₂)` implementations via merge algorithms over runs:

- `or` — union
- `and` — intersection
- `difference` — elements in one set but not another
- `xor` — symmetric difference

### Image Operations

`PixelSet` integrates directly with `DynamicImage`:

- `fill` — paint pixels a solid color
- `transform` — apply a color transformation per pixel
- `mean_color` — average color of covered pixels
- `outline` — boundary pixels of the set
- `neighbors` — pixels adjacent to the set
- `touching` — pixels in this set adjacent to another

### Compression

`compress()` produces a `CompressedPixelSet` backed by zstd over the binary RLE format. Typically achieves 75–150x compression on geographic or coherent image data. Supports `serde` for both human-readable (base64) and binary formats.

---

## WebAssembly

Enable the `wasm` feature to build pixelset as a WASM module via `wasm-bindgen`:

```toml
pixelset = { version = "...", features = ["wasm"] }
```

Build with `wasm-pack`:

```sh
rustup target add wasm32-unknown-unknown
cargo install wasm-pack
wasm-pack build --target web
```

The `wasm` module exposes `WasmPixelSet`, `WasmColor`, and shape types (`WasmRectangle`, `WasmEllipse`, etc.) with JavaScript-friendly APIs. Set operations, membership checks, and coordinate I/O are all available.

```js
import init, { WasmEllipse, WasmRectangle } from './pkg/pixelset.js';
await init();

const circle = new WasmEllipse(50, 50, 40, 40).to_pixel_set();
const square = new WasmRectangle(30, 30, 40, 40).to_pixel_set();
console.log(circle.and(square).len()); // intersection pixel count
```
