//! The nine digits used by dual balanced ternary.

use crate::complex::ComplexXy;
use std::{
  convert::TryFrom,
  fmt,
  ops::{Add, Mul, Neg},
};

/// A digit in the centered 3×3 grid.
///
/// ```text
/// 6 1 8
/// 7 5 3
/// 2 9 4
/// ```
///
/// Digit `5` is zero, `1` is the multiplicative identity, and `3` is the
/// quarter-turn unit corresponding to `i` under the map `(x, y) ↦ y + x·i`.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DualBalancedTernaryDigit {
  Dbt1,
  Dbt2,
  Dbt3,
  Dbt4,
  Dbt5,
  Dbt6,
  Dbt7,
  Dbt8,
  Dbt9,
}

use DualBalancedTernaryDigit::*;

type DigitsPair = (DualBalancedTernaryDigit, DualBalancedTernaryDigit);

impl fmt::Display for DualBalancedTernaryDigit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}", u8::from(*self))
  }
}

/// Splits a component in `-2..=2` into `3 * carry + unit`.
const fn balanced_component(value: i8) -> (i8, i8) {
  match value {
    -2 => (-1, 1),
    -1 => (0, -1),
    0 => (0, 0),
    1 => (0, 1),
    2 => (1, -1),
    _ => panic!("balanced component is outside -2..=2"),
  }
}

impl Add for DualBalancedTernaryDigit {
  type Output = DigitsPair;

  fn add(self, other: Self) -> Self::Output {
    let (ax, ay) = self.coordinates();
    let (bx, by) = other.coordinates();
    let (carry_x, unit_x) = balanced_component(ax + bx);
    let (carry_y, unit_y) = balanced_component(ay + by);
    (Self::from_coordinates(carry_x, carry_y), Self::from_coordinates(unit_x, unit_y))
  }
}

impl Mul for DualBalancedTernaryDigit {
  type Output = DigitsPair;

  fn mul(self, other: Self) -> Self::Output {
    let (ax, ay) = self.coordinates();
    let (bx, by) = other.coordinates();

    // The forward y-axis is the real axis, so (x, y) represents y + x*i.
    let product_x = ay * bx + ax * by;
    let product_y = ay * by - ax * bx;
    let (carry_x, unit_x) = balanced_component(product_x);
    let (carry_y, unit_y) = balanced_component(product_y);
    (Self::from_coordinates(carry_x, carry_y), Self::from_coordinates(unit_x, unit_y))
  }
}

impl Neg for DualBalancedTernaryDigit {
  type Output = Self;

  fn neg(self) -> Self::Output {
    let (x, y) = self.coordinates();
    Self::from_coordinates(-x, -y)
  }
}

impl From<DualBalancedTernaryDigit> for u8 {
  fn from(value: DualBalancedTernaryDigit) -> Self {
    match value {
      Dbt1 => 1,
      Dbt2 => 2,
      Dbt3 => 3,
      Dbt4 => 4,
      Dbt5 => 5,
      Dbt6 => 6,
      Dbt7 => 7,
      Dbt8 => 8,
      Dbt9 => 9,
    }
  }
}

impl TryFrom<u8> for DualBalancedTernaryDigit {
  type Error = String;

  fn try_from(value: u8) -> Result<Self, Self::Error> {
    match value {
      1 => Ok(Dbt1),
      2 => Ok(Dbt2),
      3 => Ok(Dbt3),
      4 => Ok(Dbt4),
      5 => Ok(Dbt5),
      6 => Ok(Dbt6),
      7 => Ok(Dbt7),
      8 => Ok(Dbt8),
      9 => Ok(Dbt9),
      _ => Err(format!("unknown DBT digit: {value}")),
    }
  }
}

impl TryFrom<(i64, i64)> for DualBalancedTernaryDigit {
  type Error = String;

  fn try_from((x, y): (i64, i64)) -> Result<Self, Self::Error> {
    if (-1..=1).contains(&x) && (-1..=1).contains(&y) {
      Ok(Self::from_coordinates(x as i8, y as i8))
    } else {
      Err(format!("digit coordinates must be in -1..=1, got ({x}, {y})"))
    }
  }
}

impl From<DualBalancedTernaryDigit> for ComplexXy {
  fn from(value: DualBalancedTernaryDigit) -> Self {
    let (x, y) = value.coordinates();
    Self::new(f64::from(x), f64::from(y))
  }
}

impl DualBalancedTernaryDigit {
  /// Cartesian coordinates `(x, y)` of this digit in the 3×3 grid.
  pub const fn coordinates(self) -> (i8, i8) {
    match self {
      Dbt1 => (0, 1),
      Dbt2 => (-1, -1),
      Dbt3 => (1, 0),
      Dbt4 => (1, -1),
      Dbt5 => (0, 0),
      Dbt6 => (-1, 1),
      Dbt7 => (-1, 0),
      Dbt8 => (1, 1),
      Dbt9 => (0, -1),
    }
  }

  const fn from_coordinates(x: i8, y: i8) -> Self {
    match (x, y) {
      (0, 1) => Dbt1,
      (-1, -1) => Dbt2,
      (1, 0) => Dbt3,
      (1, -1) => Dbt4,
      (0, 0) => Dbt5,
      (-1, 1) => Dbt6,
      (-1, 0) => Dbt7,
      (1, 1) => Dbt8,
      (0, -1) => Dbt9,
      _ => panic!("DBT digit coordinate is outside -1..=1"),
    }
  }

  /// Reflects the digit across the horizontal axis.
  pub fn flip_front_back(&self) -> Self {
    let (x, y) = self.coordinates();
    Self::from_coordinates(x, -y)
  }

  /// Reflects the digit across the vertical axis (complex conjugation).
  pub fn flip_left_right(&self) -> Self {
    let (x, y) = self.coordinates();
    Self::from_coordinates(-x, y)
  }

  /// Rotates the digit clockwise by 90 degrees.
  pub fn rotate3(&self) -> Self {
    let (x, y) = self.coordinates();
    Self::from_coordinates(y, -x)
  }

  /// Rotates the digit counter-clockwise by 90 degrees.
  pub fn rotate7(&self) -> Self {
    let (x, y) = self.coordinates();
    Self::from_coordinates(-y, x)
  }

  /// Reflects the digit across the `x = y` diagonal.
  pub fn flip_xy(&self) -> Self {
    let (x, y) = self.coordinates();
    Self::from_coordinates(y, x)
  }

  /// Splits a digit into `(y_axis, x_axis)` linear components.
  pub fn split_yx(&self) -> DigitsPair {
    let (x, y) = self.coordinates();
    (Self::from_coordinates(0, y), Self::from_coordinates(x, 0))
  }
}
