# ternary-core

*`#![no_std]` core traits and types for the entire ternary fleet. Z₃ arithmetic, ternary grids, ternary graphs — the foundation everything else builds on.*

## Why This Exists

Every ternary crate needs the same arithmetic operations, the same data structures, the same clamping logic. Rather than duplicating (and diverging) across 300+ crates, this crate provides the shared foundation. It's `#![no_std]` because ternary logic should run everywhere — from an x86 server to an ESP32 with 279 bytes of ternary lookup table.

## Architecture

### Z₃ Arithmetic

The integers modulo 3 form a finite field. The three elements {-1, 0, +1} with these operations form the mathematical foundation for everything ternary:

```
Addition (Z₃):        Multiplication (Z₃):
+ | -1  0  1          × | -1  0  1
--|----------          --|----------
-1|  1 -1  0          -1|  1  0 -1
 0| -1  0  1           0|  0  0  0
 1|  0  1 -1           1| -1  0  1
```

Key functions:
- **`tadd(a, b)`** — Z₃ addition via modular arithmetic
- **`tsub(a, b)`** — Z₃ subtraction
- **`tmul(a, b)`** — Z₃ multiplication
- **`tneg(a)`** — Additive inverse (flip sign)
- **`tinv(a)`** — Multiplicative inverse (only defined for ±1)
- **`tclamp(v)`** — Clamp any i8 to {-1, 0, +1}
- **`tdist(a, b)`** — Ternary distance (0 if equal, 1 if adjacent, 2 if opposite)
- **`tdot(a, b)`** — Ternary dot product using Z₃ multiplication

### Data Structures

- **`TernaryGrid`** — 2D grid of trits (rows × cols). Supports get/set, row/column access, neighbor queries (4-connectivity), and mapping functions.
- **`TernaryGraph`** — Adjacency-list graph with ternary edge weights. Supports BFS, DFS, shortest paths (ternary-weighted), and connected components.

### Traits

- **`TernaryValue`** — Trait for types that can be converted to/from trits. Implement this for your own types to make them work with all ternary operations.

## Usage

```rust
use ternary_core::*;

// Z₃ arithmetic
assert_eq!(tadd(1, 1), -1);    // 1 + 1 = 2 ≡ -1 (mod 3)
assert_eq!(tmul(-1, -1), 1);   // (-1) × (-1) = 1
assert_eq!(tinv(1), Some(-1)); // multiplicative inverse

// Ternary dot product
let a = &[1, -1, 0, 1];
let b = &[1, 1, -1, 1];
let d = tdot(a, b);

// Grid operations
let mut grid = TernaryGrid::new(3, 3);
grid.set(1, 1, 1);
let neighbors = grid.neighbors(1, 1); // 4-connected

// Graph operations
let mut graph = TernaryGraph::new(4);
graph.add_edge(0, 1, 1);
graph.add_edge(1, 2, -1);
let components = graph.connected_components();
```

## The Deeper Idea

`#![no_std]` isn't a gimmick — it's the architectural promise that ternary logic is universal. The same `tadd` that powers a neural network layer on an RTX 4050 also runs on an ESP32 microcontroller with 520KB of RAM. The math doesn't change. The hardware doesn't matter. Ternary is the portable abstraction.

This connects to `ternary-types` (concrete types like TritVec/TritMatrix that depend on std) and `ternary-compiler` (which emits operations defined here).

## Related Crates

- `ternary-types` — Concrete types (TritVec, TritMatrix) that depend on std
- `ternary-pack` — Packing trits into u32 for efficient storage
- `ternary-compiler` — Compiles ternary expressions using these operations
- `ternary-cortex` — Neural network layers built on this foundation
