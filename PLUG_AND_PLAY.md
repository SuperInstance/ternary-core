# PLUG_AND_PLAY — Core

> Core traits, arithmetic, and shared types for the ternary fleet

## 🚀 Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
ternary-core = { git = "https://github.com/SuperInstance/ternary-core" }
```

Use in your code:

```rust
use ternary_core::{tadd, tsub, tmul, TernaryValue};

let a: i8 = 1;
let b: i8 = -1;
let sum = tadd(a, b); // 0
assert!(a.is_positive());
```

## 📚 Available Documentation

| Document | Description |
|----------|-------------|
| `docs/FROM_BINARY.md` | Understanding ternary concepts as a binary programmer |
| `docs/MIGRATION.md` | Version migration guide |
| `docs/FUTURE-INTEGRATION.md` | Planned features and roadmap |

## 🔗 Integration

This crate is part of the [SuperInstance ternary fleet](https://github.com/SuperInstance). It uses the canonical `Ternary` type from `ternary-types` for cross-crate compatibility.

## 📄 License

MIT
