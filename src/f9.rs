//! Arithmetic in the nine-element field represented by one DBT digit.

use std::{
  fmt,
  ops::{Add, Mul, Neg, Sub},
};

use crate::DualBalancedTernaryDigit;

/// An element of `F9 = Z[i] / 3Z[i]`.
///
/// Unlike ordinary digit arithmetic, field arithmetic discards the carry and
/// keeps the residue modulo 3. The wrapper makes that distinction explicit in
/// the type system.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)]
pub struct F9(DualBalancedTernaryDigit);

impl F9 {
  pub const ZERO: Self = Self(DualBalancedTernaryDigit::Dbt5);
  pub const ONE: Self = Self(DualBalancedTernaryDigit::Dbt1);
  /// `1 + i`, a generator of the eight non-zero elements.
  pub const GENERATOR: Self = Self(DualBalancedTernaryDigit::Dbt8);

  pub const fn new(digit: DualBalancedTernaryDigit) -> Self {
    Self(digit)
  }

  pub const fn digit(self) -> DualBalancedTernaryDigit {
    self.0
  }

  /// Exponentiation by squaring in `F9`.
  pub fn pow(mut self, mut exponent: u64) -> Self {
    let mut result = Self::ONE;
    while exponent > 0 {
      if exponent & 1 == 1 {
        result = result * self;
      }
      exponent >>= 1;
      if exponent > 0 {
        self = self * self;
      }
    }
    result
  }

  /// Multiplicative inverse, or `None` for zero.
  pub fn inverse(self) -> Option<Self> {
    (self != Self::ZERO).then(|| self.pow(7))
  }

  /// Division in `F9`, or `None` when `divisor` is zero.
  pub fn checked_div(self, divisor: Self) -> Option<Self> {
    divisor.inverse().map(|inverse| self * inverse)
  }

  /// The Frobenius automorphism `z -> z^3`; here it is complex conjugation.
  pub fn frobenius(self) -> Self {
    self.pow(3)
  }

  /// Field trace from `F9` to `F3`, embedded on DBT's real axis.
  pub fn trace(self) -> Self {
    self + self.frobenius()
  }

  /// Field norm from `F9` to `F3`, embedded on DBT's real axis.
  pub fn norm(self) -> Self {
    self * self.frobenius()
  }
}

impl From<DualBalancedTernaryDigit> for F9 {
  fn from(value: DualBalancedTernaryDigit) -> Self {
    Self(value)
  }
}

impl From<F9> for DualBalancedTernaryDigit {
  fn from(value: F9) -> Self {
    value.0
  }
}

impl fmt::Display for F9 {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    self.0.fmt(f)
  }
}

impl Add for F9 {
  type Output = Self;

  fn add(self, rhs: Self) -> Self::Output {
    Self((self.0 + rhs.0).1)
  }
}

impl Sub for F9 {
  type Output = Self;

  fn sub(self, rhs: Self) -> Self::Output {
    self + -rhs
  }
}

impl Mul for F9 {
  type Output = Self;

  fn mul(self, rhs: Self) -> Self::Output {
    Self((self.0 * rhs.0).1)
  }
}

impl Neg for F9 {
  type Output = Self;

  fn neg(self) -> Self::Output {
    Self(-self.0)
  }
}
