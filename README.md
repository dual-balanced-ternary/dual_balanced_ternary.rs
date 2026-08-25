# Dual Balanced Ternary Arithmetic

[![crate](https://img.shields.io/crates/v/dual_balanced_ternary)](https://crates.io/crates/dual_balanced_ternary)
[![docs](https://docs.rs/dual_balanced_ternary/badge.svg)](https://docs.rs/dual_balanced_ternary/)

Dual balanced ternary (DBT) stores two balanced ternary coordinates in one
base-3 digit. The nine digits form a centered grid:

```text
6 1 8
7 5 3
2 9 4
```

Digit `5` is zero and digit `1` is the multiplicative identity. A coordinate
`(x, y)` represents the Gaussian number `y + x·i`, so digit `3` acts like `i`.

## Usage

```rust
use dual_balanced_ternary::{try_ternary, DualBalancedTernary};

let a = try_ternary("&19")?;
let b = DualBalancedTernary::try_new(1.0, 1.0)?;
assert_eq!((a.clone() * a).to_string(), "&11");
assert_eq!(b.to_string(), "&8");
# Ok::<(), String>(())
```

Use `ternary("&…")` for trusted literals, `try_ternary`/`FromStr` for input,
and `checked_div` when the divisor might be zero.

## Binary format

The stable legacy format is:

```text
[integral digit count] + [packed integral nibbles] + [packed fractional nibbles]
```

Two digits occupy one byte; an unused nibble is padded with digit `5`. It is
simple and backwards compatible, though only 81 of the 256 byte values encode
digit pairs.

## Mathematical notes

The representation is simultaneously a Gaussian-integer radix system, a set
of representatives for the finite field `F9`, a centered 3×3 spatial hierarchy,
and a route into 3-adic arithmetic. See [MATHEMATICS.md](MATHEMATICS.md) for the
derivations, limits, and experiments worth pursuing.

## Development

```bash
cargo fmt -- --check
cargo clippy --all-targets -- -D warnings
cargo test --all-targets
cargo doc --no-deps
```

Migrated from the original [Nim implementation](https://github.com/dual-balanced-ternary/dual-balanced-ternary.nim).

## License

MIT
