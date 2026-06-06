# Architecture — ternary-core-tmp

> *Internal design, data flow, and extension points.*

## Overview

This crate implements ternary {-1, 0, +1} logic for the `core-tmp` domain.
It is one of ~160 ternary crates in the SuperInstance fleet, all sharing Z₃ arithmetic
from [ternary-core](https://github.com/SuperInstance/ternary-core).

The ternary principle: **0 is not nothing** — it is the "neutral" or "abstain" state,
distinct from both positive and negative. This three-state encoding is more expressive
than binary for systems that need to represent an off-ramp or undecided state.

## Source Structure

1 Rust source file(s) in `src/`:

## Core Types

- **`TernaryGrid`** — primary data structure
- **`TernaryGraph`** — primary data structure

## Traits

- **`TernaryValue`** — behavior contract
- **`TernaryDynamics`** — behavior contract
- **`TernaryMeasure`** — behavior contract

## Key Functions

- `tadd()`
- `tsub()`
- `tmul()`
- `tneg()`
- `tinv()`
- `tclamp()`
- `tdist()`
- `tdot()`

## Data Flow

```
Input → ternary_core_tmp::transform → Ternary {-1,0,+1} → Output
```

## Design Principles

1. **Zero-dependency where possible** — keep the trust chain minimal
2. **Ternary by default** — all operations expose or consume {-1, 0, +1}
3. **No hidden state** — pure functions over explicit parameters
4. **Fail closed** — errors return safe defaults (typically 0/neutral)

## Ternary Mapping

| Value | Meaning |
|-------|---------|
| +1 | Positive / true |
| 0  | Neutral / zero |
| -1 | Negative / false |

## Cross-Repo References

- [ternary-core](https://github.com/SuperInstance/ternary-core) — shared traits
- [ternary-types](https://github.com/SuperInstance/ternary-types) — type-level encodings
