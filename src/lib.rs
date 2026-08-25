//! Dual Balanced Ternary Arithmetic
//!
//! Dual balanced ternary(DBT) is an extension to balanced ternary in 2D space.
//! Unit values of DBT is has a layout like a magic square, where `1` is the front direction.
//!
//! ```cirru
//!   6 1 8
//!   7 5 3
//!   2 9 4
//! ```
//!
//! At a bigger scale, the layout of the magic square repeats, which leads to `&11` and `&19` like in decimals.
//! `&` in this case is a special mark indicating it's a DBT value.
//!
//! There are some interesting features for the basic math:
//!
//! ```cirru
//! = (* &1 &1) &1
//! = (* &1 &9) &9
//! = (* &9 &9) &1
//! = (* &3 &3) &9
//! ```
//!
//! and:
//!
//! ```cirru
//! = (+ &1 &3) &8
//! = (+ &1 &1) &19
//! = (* &3 &7) &5
//! ```
//!
//! Algebraically this is a radix-3 representation of Gaussian numbers. The
//! digit `1` is the multiplicative identity and points forward; under the
//! standard complex convention, a grid coordinate `(x, y)` represents
//! `y + x·i`.

pub mod complex;
pub mod digit;
pub mod f9;
pub mod primes;

pub use digit::DualBalancedTernaryDigit;
pub use f9::F9;
pub use primes::{DIV_PRECISION, DualBalancedTernary};

use std::str::FromStr;

/// Convenience parser for literals known to be valid.
///
/// # Panics
///
/// Panics when `s` is not a valid DBT literal. Use [`try_ternary`] for
/// untrusted input.
pub fn ternary(s: &str) -> DualBalancedTernary {
  try_ternary(s).expect("invalid dual balanced ternary literal")
}

/// Parses a DBT literal such as `&18.3`.
pub fn try_ternary(s: &str) -> Result<DualBalancedTernary, String> {
  DualBalancedTernary::from_str(s)
}

/// expose internal digits for inspecting
pub fn dbt_digits(x: DualBalancedTernary) -> Vec<(i64, DualBalancedTernaryDigit)> {
  let mut ys: Vec<(i64, DualBalancedTernaryDigit)> = vec![];
  for idx in 0..x.integral.len() {
    let i = x.integral.len() - idx - 1;
    ys.push((i as i64, x.integral[i]));
  }

  for (idx, n) in x.fractional.iter().enumerate() {
    let i = -1 - idx as i64;
    ys.push((i, *n))
  }

  ys
}
