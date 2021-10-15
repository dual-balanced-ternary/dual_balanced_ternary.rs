## Dual Balanced Ternary Arithmetic

> Migrated from https://github.com/dual-balanced-ternary/dual-balanced-ternary.nim

![crate](https://img.shields.io/crates/v/dual-balanced-ternary)

[Rust Docs](https://docs.rs/dual_balanced_ternary/).

### Usages

```rs
use dual_balanced_ternary::{ternary};

ternary("&1.1")
```

### Development

```bash
cargo test
cargo run --example buffer
```

Notice, current buffer format is not compact, but conceptually:

```text
[integral length]+[integral pairs]+[fractional pairs]
```

since a dbt digits pair takes `81` and `u8` takes `256`, `2/3` of spaces are wasted.

### License

MIT
