# ternary-core

**The foundation. Shared traits, Z₃ arithmetic, grids, and graphs for the entire ternary fleet.**

Every building needs a foundation. `ternary-core` is the bedrock of the SuperInstance ternary fleet — the shared abstractions that every other ternary crate builds on. Instead of every crate reimplementing Z₃ arithmetic, grid operations, and graph algorithms, they import from here.

This crate defines the algebraic primitives (`tadd`, `tsub`, `tmul`, `tneg`, `tinv`), the core types (`TernaryGrid`, `TernaryGraph`), and the universal traits (`TernaryValue`, `TernaryDynamics`, `TernaryMeasure`) that make the fleet a coherent system rather than 240 independent repos.

## What's Inside

### Z₃ Arithmetic
- **`tadd(a, b)`** — addition modulo 3: 1+1 = -1, -1+-1 = 1
- **`tsub(a, b)`** — subtraction modulo 3
- **`tmul(a, b)`** — multiplication modulo 3: -1×-1 = 1, 1×-1 = -1
- **`tneg(a)`** — negation: 1↔-1, 0↔0
- **`tinv(a)`** — multiplicative inverse: 1⁻¹=1, -1⁻¹=-1, 0 has no inverse
- **`tdist(a, b)`** — modular distance on Z₃ circle
- **`tdot(a, b)`** — inner product of ternary vectors mod 3

### Types
- **`TernaryGrid`** — 2D grid with get/set, histogram, map, zip_with, Laplacian, neighbor counting, BFS-ready
- **`TernaryGraph`** — weighted graph with ternary edges, BFS, connectivity, components
- **`TernaryValue`** — trait for types that map to {-1, 0, 1}

### Traits
- **`TernaryDynamics`** — for systems that evolve in discrete steps
- **`TernaryMeasure`** — for anything that produces a ternary summary

## Quick Example

```rust
use ternary_core::*;

// Z₃ arithmetic
assert_eq!(tadd(1, 1), -1);   // 1+1 = 2 ≡ -1 mod 3
assert_eq!(tmul(-1, -1), 1);  // -1 × -1 = 1
assert_eq!(tinv(0), None);     // zero has no inverse

// Grid operations
let mut grid = TernaryGrid::new(5, 5);
grid.set(2, 2, 1);
let (neg, zero, pos) = grid.histogram();

// Graph operations
let mut graph = TernaryGraph::new(4);
graph.add_undirected(0, 1, 1);
graph.add_undirected(1, 2, 1);
assert!(graph.is_connected());
```

## The Deeper Truth

**This crate exists because 240 crates duplicating Z₃ arithmetic is violence.** Every ternary crate needs the same operations: add mod 3, multiply mod 3, clamp to {-1, 0, 1}, compute distances on the Z₃ circle. Without a shared core, each crate implements these slightly differently — one uses `% 3`, another uses `rem_euclid`, a third uses match statements. Bugs multiply. Interfaces diverge.

The deeper insight: Z₃ is a *field*. It has additive inverses (for all elements) and multiplicative inverses (for nonzero elements). This means every linear algebra operation — matrix multiply, determinant, eigenvalue — works over Z₃ exactly as it does over the reals, but with simpler arithmetic. The ternary core makes this algebraic structure explicit and shared.

`TernaryGrid` is the workhorse: a flat `Vec<i8>` with 2D indexing, neighbor counting (Moore and von Neumann neighborhoods), Laplacian computation, histogram, and functional combinators (map, zip_with). Any crate that operates on a 2D ternary field — which is most of them — can use `TernaryGrid` instead of reimplementing the same boilerplate.

`TernaryGraph` similarly provides the basic graph operations that underlie at least 30 crates in the fleet: adjacency, BFS, connectivity, connected components, edge weights. The ternary twist: edges can be *negative* (adversarial), and positive-edge-only connectivity is the default.

**The architectural bet:** by making `ternary-core` the single source of truth for Z₃ arithmetic and common data structures, every crate in the fleet becomes:
1. **Smaller** — no duplicated boilerplate
2. **Consistent** — same arithmetic everywhere
3. **Composable** — grids from one crate feed directly into another
4. **Testable** — core operations tested once, trusted everywhere

## See Also

This crate is imported by (or should be) every ternary crate. Key dependents:

- **ternary-grid crates** — life, fire, sandpile, ising, morphogenesis, etc.
- **ternary-graph crates** — network, mesh, pagerank, resilience, etc.
- **ternary-algebra** — ring, matrix, tensor, codes
- **ternary-cipher** — cryptography uses Z₃ arithmetic extensively
- **ternary-counterpoint** — music intervals are Z₃ arithmetic
- **ternary-crystal** — crystallographic symmetry is Z₃ group theory

## Install

```bash
cargo add ternary-core
```

## License

MIT
