# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

```sh
# Build
cargo build

# Run all tests
cargo test

# Run a single test by name
cargo test <test_name>

# Run tests in a specific file
cargo test --test <file_name>   # e.g. cargo test --test set_operations

# Check without building
cargo check

# Lint
cargo clippy
```

## Architecture

`pixelset` is a Rust library providing a high-performance, sorted 2D pixel set optimized for set operations, designed to integrate with the `image` crate.

### Core Data Representation

The fundamental encoding is **run-length encoding (RLE)**. Rather than storing individual pixels, the library stores horizontal runs:

```rust
struct Run { y: u16, x_start: u16, length: u16 }
```

Runs are always kept **sorted by `(y, x_start)`**, non-overlapping, and non-adjacent on the same row. All public and internal methods must preserve this invariant. The `from_runs_unchecked` constructor skips validation — callers are responsible.

### Module Layout

- `src/set/` — `PixelSet` struct and all its methods, split across:
  - `mod.rs` — struct definition, `Serialize`/`Deserialize` impls (delegate to compression serde)
  - `new.rs` — constructors (`new`, `new_unchecked`, `from_image`, `from_runs_unchecked`, `encode_runs`)
  - `iter.rs` — pixel iteration (expands runs back to individual pixels)
  - `compress.rs` — `compress()` method bridging to the compression module
  - `ops/set_ops.rs` — `and`, `or`, `xor`, `difference`, `add`, `discard`, `is_subset`, `intersects`
  - `ops/list_ops.rs` — `has`, `len`, `filter`, `filter_color`, `select`, `apply`, `bounds`, `centroid`, `closest_to`
  - `ops/image_ops.rs` — `fill`, `transform`, `mean_color`, `outline`, `neighbors`, `touching`
  - `ops/diagnostics.rs` — internal consistency checks / debug helpers

- `src/compression/` — `CompressedPixelSet` type with zstd compression over the binary RLE format; also handles `Serialize`/`Deserialize` for `PixelSet` itself (human-readable → base64, binary → raw bytes)

- `src/shapes/` — `Shape` trait + `Rectangle`, `RectangleOutline`, `Ellipse`, `EllipseOutline`; each generates a `PixelSet` directly without intermediate pixel lists where possible

- `src/pixel/` — `Pixel` struct with `(y, x)` sort key (y-major ordering used everywhere)
- `src/color/` — `Color` RGBA type with hex parsing, blending, grayscale
- `src/direction/` — `Direction` enum used by neighbor/outline operations

### Key Complexity Properties

| Operation | Complexity |
|---|---|
| `new` (unsorted) | O(n) — radix sort + dedup + RLE encode |
| `new_unchecked` | O(n) — single-pass RLE encode |
| `from_image` | O(height) — generates full-width runs directly |
| `has` | O(log k) — binary search on runs |
| `add` / `discard` | O(k) — run splitting/merging |
| `and`, `or`, `xor`, `difference` | O(k1 + k2) — merge scan |
| `len` | O(k) — sum run lengths |

Where `k` = number of runs, `n` = number of pixels.

### Integration Tests

The `tests/` directory has five focused files:
- `set_operations.rs` — `and`, `or`, `xor`, `difference`, `is_subset`, `intersects`
- `pixel_operations.rs` — `add`, `discard`, `has`, `filter`, `select`
- `workflows.rs` — end-to-end scenarios combining multiple operations
- `compression.rs` — `compress`/`decompress` round-trips and `CompressedPixelSet` serde
- `invariants.rs` — internal RLE invariant checks (sorted, non-overlapping, non-adjacent)
