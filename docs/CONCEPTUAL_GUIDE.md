# The Symmetry Behind the Code

*A conceptual guide to understanding why the ternary-core traits are structured the way they are — and why it matters.*

---

## The Problem with Interfaces

When you write an interface (a trait in Rust), you're making a promise. "Anything that implements this trait will behave a certain way."

But most traits are just a list of function signatures. They don't encode *laws*. Two implementations of the same trait can behave completely differently — one might be correct, the other buggy, and the compiler can't tell the difference.

Traditional traits are shapes without constraints.

---

## The Core Insight

The `ternary-core` traits aren't just interfaces. They're **mathematical symmetries** expressed as code.

A symmetry is a transformation that preserves something. For example:

- **Rotating a square** preserves its shape.
- **Adding zero** preserves the sum.
- **Applying a ternary operator** preserves the Z₃ invariant.

When a type implements `TernaryValue`, it's not just saying "I have a `tadd()` method." It's saying: **"I obey the conservation law of the ternary group."**

---

## The Trait Family Tree

```
TernaryValue           — "I represent one of {-1, 0, +1}"
    │
    ├── TernaryDynamics  — "I evolve over time while preserving the invariant"
    │
    └── TernaryMeasure   — "I measure distances and similarities in ternary space"
```

### TernaryValue

The root trait. Any type that implements this can be treated as an element of Z₃.

**The law**: The identity element (0) must satisfy `tadd(x, 0) == x` for all x.

**The a-ha moment**: This isn't a convention. It's a *proof obligation*. If your implementation doesn't satisfy this, it's not a valid ternary value — it's something else.

```rust
// This compiles, but is it correct?
impl TernaryValue for MyType {
    fn tadd(&self, other: &Self) -> Self {
        // If this doesn't satisfy the group laws,
        // it'll break every system that depends on them
    }
}
```

### TernaryDynamics

A type that evolves over time, where each transition conserves the ternaty invariant.

**The law**: `measure(initial_state) == measure(evolved_state)`

**The a-ha moment**: This means you can *prove* that a dynamics implementation doesn't create or destroy information. It only transforms it.

```rust
impl TernaryDynamics for CellularAutomaton {
    fn step(&self) -> Self {
        // Each cell's state changes based on neighbors,
        // but the total sum of all states is conserved
    }
}
```

### TernaryMeasure

A type that measures similarity in ternary space.

**The law**: `tdist(a, b) == tdist(b, a)` (symmetry) and `tdist(a, a) == 0` (identity).

**The a-ha moment**: This means you can use `TernaryMeasure` as a drop-in for clustering, nearest-neighbor search, and anomaly detection — and trust the results.

---

## From Traits to Systems

When you compose these traits, something remarkable happens:

| Composition | What emerges | Example |
|-------------|--------------|---------|
| `TernaryValue + TernaryDynamics` | A system with a conservation law | A voting machine where no votes are lost |
| `TernaryValue + TernaryMeasure` | A metric space over ternary values | A routing cost function |
| `TernaryDynamics + TernaryMeasure` | A self-monitoring system | An anomaly detector that alerts when conservation breaks |

Each composition inherits the laws of its parts. You're not just adding features — you're composing mathematical guarantees.

---

## The Big Reveal

Most code is written as a sequence of instructions. It's hard to prove things about sequences.

Ternary code, built on these traits, is written as **transformations within a constrained algebraic space**. The constraints aren't limitations — they're the guarantees that make the system composable, provable, and predictable.

An interface that encodes its own mathematics isn't just a contract. It's a **symmetry**.

Once you see that, every trait becomes a question: "What does this preserve?"
