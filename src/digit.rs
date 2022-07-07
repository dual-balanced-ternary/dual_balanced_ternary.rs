//! Digits for DBT, with 9 values, `5` at center, `1` at front
use crate::complex::ComplexXy;

use std::{
  convert::TryFrom,
  fmt,
  hash::Hash,
  ops::{Add, Mul, Neg},
};

/// Digits
/// ```cirru
/// 6 1 8
/// 7 5 3
/// 2 9 4
/// ```
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum DualBalancedTernaryDigit {
  /// ```cirru
  /// _ 1 _
  /// _ 5 _
  /// _ _ _
  /// ```
  Dbt1,
  /// ```cirru
  /// _ _ _
  /// _ 5 _
  /// 2 _ _
  /// ```
  Dbt2,
  /// ```cirru
  /// _ _ _
  /// _ 5 3
  /// _ _ _
  /// ```
  Dbt3,
  /// ```cirru
  /// _ _ _
  /// _ 5 _
  /// _ _ 4
  /// ```
  Dbt4,
  /// ```cirru
  /// _ _ _
  /// _ 5 _
  /// _ _ _
  /// ```
  Dbt5,
  /// ```cirru
  /// 6 _ _
  /// _ 5 _
  /// _ _ _
  /// ```
  Dbt6,
  /// ```cirru
  /// _ _ _
  /// 7 5 _
  /// _ _ _
  /// ```
  Dbt7,
  /// ```cirru
  /// _ _ 8
  /// _ 5 _
  /// _ _ _
  /// ```
  Dbt8,
  /// ```cirru
  /// _ _ _
  /// _ 5 _
  /// _ 9 _
  /// ```
  Dbt9,
}

use DualBalancedTernaryDigit::*;

impl fmt::Display for DualBalancedTernaryDigit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    use DualBalancedTernaryDigit::*;
    match self {
      Dbt1 => write!(f, "1"),
      Dbt2 => write!(f, "2"),
      Dbt3 => write!(f, "3"),
      Dbt4 => write!(f, "4"),
      Dbt5 => write!(f, "5"),
      Dbt6 => write!(f, "6"),
      Dbt7 => write!(f, "7"),
      Dbt8 => write!(f, "8"),
      Dbt9 => write!(f, "9"),
    }
  }
}

impl Add for DualBalancedTernaryDigit {
  type Output = DigitsPair;

  fn add(self, b: Self) -> Self::Output {
    match self {
      Dbt1 => match b {
        Dbt1 => (Dbt1, Dbt9),
        Dbt2 => (Dbt5, Dbt7),
        Dbt3 => (Dbt5, Dbt8),
        Dbt4 => (Dbt5, Dbt3),
        Dbt5 => (Dbt5, Dbt1),
        Dbt6 => (Dbt1, Dbt2),
        Dbt7 => (Dbt5, Dbt6),
        Dbt8 => (Dbt1, Dbt4),
        Dbt9 => (Dbt5, Dbt5),
      },
      Dbt2 => match b {
        Dbt1 => (Dbt5, Dbt7),
        Dbt2 => (Dbt2, Dbt8),
        Dbt3 => (Dbt5, Dbt9),
        Dbt4 => (Dbt9, Dbt1),
        Dbt5 => (Dbt5, Dbt2),
        Dbt6 => (Dbt7, Dbt3),
        Dbt7 => (Dbt7, Dbt4),
        Dbt8 => (Dbt5, Dbt5),
        Dbt9 => (Dbt9, Dbt6),
      },
      Dbt3 => match b {
        Dbt1 => (Dbt5, Dbt8),
        Dbt2 => (Dbt5, Dbt9),
        Dbt3 => (Dbt3, Dbt7),
        Dbt4 => (Dbt3, Dbt2),
        Dbt5 => (Dbt5, Dbt3),
        Dbt6 => (Dbt5, Dbt1),
        Dbt7 => (Dbt5, Dbt5),
        Dbt8 => (Dbt3, Dbt6),
        Dbt9 => (Dbt5, Dbt4),
      },
      Dbt4 => match b {
        Dbt1 => (Dbt5, Dbt3),
        Dbt2 => (Dbt9, Dbt1),
        Dbt3 => (Dbt3, Dbt2),
        Dbt4 => (Dbt4, Dbt6),
        Dbt5 => (Dbt5, Dbt4),
        Dbt6 => (Dbt5, Dbt5),
        Dbt7 => (Dbt5, Dbt9),
        Dbt8 => (Dbt3, Dbt7),
        Dbt9 => (Dbt9, Dbt8),
      },
      Dbt5 => (Dbt5, b),
      Dbt6 => match b {
        Dbt1 => (Dbt1, Dbt2),
        Dbt2 => (Dbt7, Dbt3),
        Dbt3 => (Dbt5, Dbt1),
        Dbt4 => (Dbt5, Dbt5),
        Dbt5 => (Dbt5, Dbt6),
        Dbt6 => (Dbt6, Dbt4),
        Dbt7 => (Dbt7, Dbt8),
        Dbt8 => (Dbt1, Dbt9),
        Dbt9 => (Dbt5, Dbt7),
      },
      Dbt7 => match b {
        Dbt1 => (Dbt5, Dbt6),
        Dbt2 => (Dbt7, Dbt4),
        Dbt3 => (Dbt5, Dbt5),
        Dbt4 => (Dbt5, Dbt9),
        Dbt5 => (Dbt5, Dbt7),
        Dbt6 => (Dbt7, Dbt8),
        Dbt7 => (Dbt7, Dbt3),
        Dbt8 => (Dbt5, Dbt1),
        Dbt9 => (Dbt5, Dbt2),
      },
      Dbt8 => match b {
        Dbt1 => (Dbt1, Dbt4),
        Dbt2 => (Dbt5, Dbt5),
        Dbt3 => (Dbt3, Dbt6),
        Dbt4 => (Dbt3, Dbt7),
        Dbt5 => (Dbt5, Dbt8),
        Dbt6 => (Dbt1, Dbt9),
        Dbt7 => (Dbt5, Dbt1),
        Dbt8 => (Dbt8, Dbt2),
        Dbt9 => (Dbt5, Dbt3),
      },
      Dbt9 => match b {
        Dbt1 => (Dbt5, Dbt5),
        Dbt2 => (Dbt9, Dbt6),
        Dbt3 => (Dbt5, Dbt4),
        Dbt4 => (Dbt9, Dbt8),
        Dbt5 => (Dbt5, Dbt9),
        Dbt6 => (Dbt5, Dbt7),
        Dbt7 => (Dbt5, Dbt2),
        Dbt8 => (Dbt5, Dbt3),
        Dbt9 => (Dbt9, Dbt1),
      },
    }
  }
}

impl Mul for DualBalancedTernaryDigit {
  type Output = DigitsPair;
  fn mul(self, b: Self) -> Self::Output {
    match self {
      Dbt1 => (Dbt5, b),
      Dbt2 => match b {
        Dbt1 => (Dbt5, Dbt2),
        Dbt2 => (Dbt3, Dbt7),
        Dbt3 => (Dbt5, Dbt6),
        Dbt4 => (Dbt1, Dbt9),
        Dbt5 => (Dbt5, Dbt5),
        Dbt6 => (Dbt9, Dbt1),
        Dbt7 => (Dbt5, Dbt4),
        Dbt8 => (Dbt7, Dbt3),
        Dbt9 => (Dbt5, Dbt8),
      },
      Dbt3 => (Dbt5, b.rotate3()),
      Dbt4 => match b {
        Dbt1 => (Dbt5, Dbt4),
        Dbt2 => (Dbt1, Dbt9),
        Dbt3 => (Dbt5, Dbt2),
        Dbt4 => (Dbt7, Dbt3),
        Dbt5 => (Dbt5, Dbt5),
        Dbt6 => (Dbt3, Dbt7),
        Dbt7 => (Dbt5, Dbt8),
        Dbt8 => (Dbt9, Dbt1),
        Dbt9 => (Dbt5, Dbt6),
      },
      Dbt5 => (Dbt5, Dbt5),
      Dbt6 => match b {
        Dbt1 => (Dbt5, Dbt6),
        Dbt2 => (Dbt9, Dbt1),
        Dbt3 => (Dbt5, Dbt8),
        Dbt4 => (Dbt3, Dbt7),
        Dbt5 => (Dbt5, Dbt5),
        Dbt6 => (Dbt7, Dbt3),
        Dbt7 => (Dbt5, Dbt2),
        Dbt8 => (Dbt1, Dbt9),
        Dbt9 => (Dbt5, Dbt4),
      },
      Dbt7 => (Dbt5, b.rotate7()),
      Dbt8 => match b {
        Dbt1 => (Dbt1, Dbt8),
        Dbt2 => (Dbt7, Dbt3),
        Dbt3 => (Dbt5, Dbt4),
        Dbt4 => (Dbt9, Dbt1),
        Dbt5 => (Dbt5, Dbt5),
        Dbt6 => (Dbt1, Dbt9),
        Dbt7 => (Dbt5, Dbt6),
        Dbt8 => (Dbt3, Dbt7),
        Dbt9 => (Dbt5, Dbt2),
      },
      Dbt9 => (Dbt5, -b),
    }
  }
}

// pub const fractional_base: i64 = 1 / 3;

impl TryFrom<(i64, i64)> for DualBalancedTernaryDigit {
  type Error = String;

  fn try_from(value: (i64, i64)) -> Result<Self, Self::Error> {
    let (x, y) = value;
    match x {
      -1 => match y {
        -1 => Ok(Dbt2),
        0 => Ok(Dbt7),
        1 => Ok(Dbt6),
        _ => Err(format!("unexpected y: {}", y)),
      },
      0 => match y {
        -1 => Ok(Dbt9),
        0 => Ok(Dbt5),
        1 => Ok(Dbt1),
        _ => Err(format!("unexpected y: {}", y)),
      },
      1 => match y {
        -1 => Ok(Dbt8),
        0 => Ok(Dbt3),
        1 => Ok(Dbt4),
        _ => Err(format!("unexpected y: {}", y)),
      },
      _ => Err(format!("unexpected x: {}", x)),
    }
  }
}

// an alias
type DigitsPair = (DualBalancedTernaryDigit, DualBalancedTernaryDigit);

impl Neg for DualBalancedTernaryDigit {
  type Output = DualBalancedTernaryDigit;
  fn neg(self) -> Self::Output {
    match self {
      Dbt1 => Dbt9,
      Dbt2 => Dbt8,
      Dbt3 => Dbt7,
      Dbt4 => Dbt6,
      Dbt5 => Dbt5,
      Dbt6 => Dbt4,
      Dbt7 => Dbt3,
      Dbt8 => Dbt2,
      Dbt9 => Dbt1,
    }
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

// 1 points at y direction, 3 points at x direction
impl From<DualBalancedTernaryDigit> for ComplexXy {
  fn from(v: DualBalancedTernaryDigit) -> Self {
    use DualBalancedTernaryDigit::*;

    match v {
      Dbt1 => ComplexXy { x: 0.0, y: 1.0 },
      Dbt2 => ComplexXy { x: -1.0, y: -1.0 },
      Dbt3 => ComplexXy { x: 1.0, y: 0.0 },
      Dbt4 => ComplexXy { x: 1.0, y: -1.0 },
      Dbt5 => ComplexXy { x: 0.0, y: 0.0 },
      Dbt6 => ComplexXy { x: -1.0, y: 1.0 },
      Dbt7 => ComplexXy { x: -1.0, y: 0.0 },
      Dbt8 => ComplexXy { x: 1.0, y: 1.0 },
      Dbt9 => ComplexXy { x: 0.0, y: -1.0 },
    }
  }
}

impl TryFrom<u8> for DualBalancedTernaryDigit {
  type Error = String;
  fn try_from(x: u8) -> Result<Self, Self::Error> {
    match x {
      1 => Ok(Dbt1),
      2 => Ok(Dbt2),
      3 => Ok(Dbt3),
      4 => Ok(Dbt4),
      5 => Ok(Dbt5),
      6 => Ok(Dbt6),
      7 => Ok(Dbt7),
      8 => Ok(Dbt8),
      9 => Ok(Dbt9),
      _ => Err(format!("unknown digit for dbt: {}", x)),
    }
  }
}

impl DualBalancedTernaryDigit {
  /// ```cirru
  /// 6 1 8
  /// 7 5 3
  /// 2 9 4
  /// ```
  /// into
  /// ```cirru
  /// 2 9 4
  /// 7 5 3
  /// 6 1 8
  /// ```
  pub fn flip_front_back(&self) -> DualBalancedTernaryDigit {
    match self {
      Dbt1 => Dbt9,
      Dbt2 => Dbt6,
      Dbt3 => Dbt7,
      Dbt4 => Dbt8,
      Dbt5 => Dbt5,
      Dbt6 => Dbt2,
      Dbt7 => Dbt7,
      Dbt8 => Dbt4,
      Dbt9 => Dbt1,
    }
  }

  /// ```cirru
  /// 6 1 8
  /// 7 5 3
  /// 2 9 4
  /// ```
  /// into
  /// ```cirru
  /// 8 1 6
  /// 3 5 7
  /// 4 9 2
  /// ```
  pub fn flip_left_right(&self) -> DualBalancedTernaryDigit {
    match self {
      Dbt1 => Dbt1,
      Dbt2 => Dbt4,
      Dbt3 => Dbt7,
      Dbt4 => Dbt2,
      Dbt5 => Dbt5,
      Dbt6 => Dbt8,
      Dbt7 => Dbt3,
      Dbt8 => Dbt6,
      Dbt9 => Dbt9,
    }
  }

  /// clockwise rotation
  pub fn rotate3(&self) -> DualBalancedTernaryDigit {
    match self {
      Dbt1 => Dbt3,
      Dbt2 => Dbt6,
      Dbt3 => Dbt9,
      Dbt4 => Dbt2,
      Dbt5 => Dbt5,
      Dbt6 => Dbt8,
      Dbt7 => Dbt1,
      Dbt8 => Dbt4,
      Dbt9 => Dbt7,
    }
  }

  /// anti-clockwise rotation
  pub fn rotate7(&self) -> DualBalancedTernaryDigit {
    match self {
      Dbt1 => Dbt7,
      Dbt2 => Dbt4,
      Dbt3 => Dbt1,
      Dbt4 => Dbt8,
      Dbt5 => Dbt5,
      Dbt6 => Dbt2,
      Dbt7 => Dbt9,
      Dbt8 => Dbt6,
      Dbt9 => Dbt3,
    }
  }

  /// ```cirru
  /// 6 1 8
  /// 7 5 3
  /// 2 9 4
  /// ```
  /// into
  /// ```cirru
  /// 4 3 8
  /// 9 5 1
  /// 2 7 6
  /// ```
  pub fn flip_xy(&self) -> DualBalancedTernaryDigit {
    match self {
      Dbt1 => Dbt3,
      Dbt2 => Dbt2,
      Dbt3 => Dbt1,
      Dbt4 => Dbt6,
      Dbt5 => Dbt5,
      Dbt6 => Dbt4,
      Dbt7 => Dbt9,
      Dbt8 => Dbt8,
      Dbt9 => Dbt7,
    }
  }

  /// split a digit into 2 linear digits(consists of &1, &5, &9)
  pub fn split_yx(&self) -> DigitsPair {
    match self {
      Dbt1 => (Dbt1, Dbt5),
      Dbt2 => (Dbt9, Dbt7),
      Dbt3 => (Dbt5, Dbt3),
      Dbt4 => (Dbt9, Dbt3),
      Dbt5 => (Dbt5, Dbt5),
      Dbt6 => (Dbt1, Dbt7),
      Dbt7 => (Dbt5, Dbt7),
      Dbt8 => (Dbt1, Dbt3),
      Dbt9 => (Dbt9, Dbt5),
    }
  }
}
