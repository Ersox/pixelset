# PixelSet

A high-performance, **sorted collection of pixels** for Rust, optimized for fast set-like operations and spatial queries.

---

## Overview

`PixelSet` stores `Pixel` values in **strict `(y, x)` order**, providing:

- **Efficient construction** with radix sort (`O(n)`.)
- **Fast membership checks** with binary search (`O(log n)`.)
- **Efficient set operations** like union, intersection, and difference (`O(n)` linear scans.)
- **Minimal memory overhead** compared to hash-based or tree-based structures.