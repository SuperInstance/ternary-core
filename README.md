# Ternary Core — Foundational Types and Traits for Ternary Logic

**Ternary Core** defines the fundamental type system for balanced-ternary computation: the `Trit` enum (False/Unknown/True), the `Tryte` word (6 trits), and core traits like `TernaryEval`. It implements Kleene three-valued logic with AND, OR, NOT, and consensus operators, providing the algebraic foundation upon which the entire SuperInstance ternary ecosystem is built.

## Why It Matters

Every ternary crate in the ecosystem depends on these type definitions. Getting the foundation right — the representation of unknown values, the propagation rules for uncertainty, the consensus operator for agreement — determines the correctness of everything built on top. The choice of a 3-valued logic over classical boolean enables graceful handling of uncertainty: where binary systems must commit to True or False, ternary systems can represent "insufficient information" explicitly. This is not academic — it directly affects how ternary neural networks handle sparse activation, how distributed agents handle missing data, and how consensus protocols handle abstentions.

## How It Works

### Trit Representation

A `Trit` is represented as an enum with three variants: `False(0)`, `Unknown(1)`, `True(2)`. This encoding (rather than -1/0/1) uses non-negative discriminants for safe `as u8` conversion. The `is_known()`, `is_true()`, `is_false()` predicates provide readable queries.

### Kleene Three-Valued Logic

The AND, OR, and NOT operators implement **Kleene K₃ logic**:

- **AND**: if either operand is False → False; if either is Unknown → Unknown; only True AND True → True
- **OR**: if either operand is True → True; if either is Unknown → Unknown; only False OR False → False  
- **NOT**: True ↔ False, Unknown stays Unknown

These operations are **monotonic** — changing an Unknown to either True or False can never change a result from True to False or vice versa. This monotonicity is critical for sound fixpoint computations in constraint propagation.

### Consensus Operator

The `consensus()` function returns the shared value if both trits agree, otherwise Unknown. This is the **meet** operation in the ternary lattice and is essential for distributed agreement: two agents observing different evidence can compute consensus to find common ground.

### Tryte (6-Trit Word)

A `Tryte` packs 6 trits into a fixed-size array. Using balanced ternary positional notation, it represents integers in the range [-364, +364]:

```
value = Σᵢ (2·tᵢ - 1) · 3ⁱ   for i = 0..5
```

The `to_balance()` method performs this conversion in O(6) = O(1).

## Quick Start

```rust
use ternary_core::{Trit, Tryte, TernaryEval};

// Trit operations
let a = Trit::True;
let b = Trit::Unknown;
assert_eq!(a.and(b), Trit::Unknown);
assert_eq!(a.or(b), Trit::True);
assert_eq!(Trit::consensus(Trit::True, Trit::True), Trit::True);
assert_eq!(Trit::consensus(Trit::True, Trit::False), Trit::Unknown);

// Convert from bool
assert_eq!(bool::true_value().eval(), Trit::True);

// Tryte arithmetic
let zero = Tryte::zero();
assert_eq!(zero.to_balance(), 0);
```

```bash
cargo add ternary-core
```

## API

| Type / Function | Description |
|---|---|
| `Trit` | Enum: `False`, `Unknown`, `True` |
| `Trit::and()`, `or()`, `not()` | Kleene K₃ logic operators |
| `Trit::consensus()` | Lattice meet: agree or Unknown |
| `Tryte` | 6-trit word (range -364 to +364) |
| `TernaryEval` | Trait: `eval() → Trit` for bool, Trit, custom types |

## Architecture Notes

This is the foundational crate of **SuperInstance** — every other ternary crate imports it. The `Trit::Unknown` state is the algebraic representation of the η (entropy) term in γ + η = C: it captures the uncertainty that balances growth. The consensus operator is the algebraic basis for fleet-wide agreement. See [Architecture](https://github.com/SuperInstance/SuperInstance/blob/main/ARCHITECTURE.md).

## References

- Kleene, Stephen C. *Introduction to Metamathematics*, North-Holland, 1952 — K₃ three-valued logic.
- Knuth, Donald E. *The Art of Computer Programming, Vol. 2*, §4.1 — balanced ternary arithmetic.
- Belnap, Nuel D. "A Useful Four-Valued Logic," *Modern Uses of Multiple-Valued Logic*, 1977 — multi-valued logic foundations.

## License

MIT
