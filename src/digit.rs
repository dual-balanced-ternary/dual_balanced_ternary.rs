use crate::complex::ComplextXy;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
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

pub type DigitsPair = (DualBalancedTernaryDigit, DualBalancedTernaryDigit);

impl DualBalancedTernaryDigit {
  pub fn negate(&self) -> DualBalancedTernaryDigit {
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

  // x: 3->1 , y: 1->3
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

  pub fn add_digits(&self, b: DualBalancedTernaryDigit) -> DigitsPair {
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

  pub fn mutiply_digits(&self, b: DualBalancedTernaryDigit) -> DigitsPair {
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
      Dbt9 => (Dbt5, b.negate()),
    }
  }

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

  // 1 points at y direction, 3 points at x direction
  pub fn to_float(&self) -> ComplextXy {
    match self {
      Dbt1 => ComplextXy { x: 0.0, y: 1.0 },
      Dbt2 => ComplextXy { x: -1.0, y: -1.0 },
      Dbt3 => ComplextXy { x: 1.0, y: 0.0 },
      Dbt4 => ComplextXy { x: 1.0, y: -1.0 },
      Dbt5 => ComplextXy { x: 0.0, y: 0.0 },
      Dbt6 => ComplextXy { x: -1.0, y: 1.0 },
      Dbt7 => ComplextXy { x: -1.0, y: 0.0 },
      Dbt8 => ComplextXy { x: 1.0, y: 1.0 },
      Dbt9 => ComplextXy { x: 0.0, y: -1.0 },
    }
  }
}
