//!  - Dual Balanced Ternary Arithmetic
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
//! The math is roughly equal to Complex numbers, expect for that its identity value is `1` pointing at at front.

pub mod complex;
pub mod digit;
pub mod primes;

pub use digit::DualBalancedTernaryDigit;
pub use primes::{DualBalancedTernary, DIV_PRECISION};

use std::str::FromStr;

/// an alias for quick creating a DualBalancedTernary, might fail
pub fn ternary(s: &str) -> DualBalancedTernary {
  DualBalancedTernary::from_str(s).unwrap()
}

/// expose internal digits for inspecting
pub fn dbt_digits(x: DualBalancedTernary) -> Vec<(i64, DualBalancedTernaryDigit)> {
  let mut ys: Vec<(i64, DualBalancedTernaryDigit)> = vec![];
  for idx in 0..x.integral.len() {
    let i = x.integral.len() - idx - 1;
    ys.push((i as i64, x.integral[i].to_owned()));
  }

  for (idx, n) in x.fractional.iter().enumerate() {
    let i = -1 - idx as i64;
    ys.push((i, n.to_owned()))
  }

  ys
}
