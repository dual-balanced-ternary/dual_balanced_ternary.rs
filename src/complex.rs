use crate::digit::{DualBalancedTernaryDigit, DualBalancedTernaryDigit::*};

#[derive(PartialEq, Debug, Clone)]
pub struct ComplextXy {
  pub x: f64,
  pub y: f64,
}

// pub const fractional_base: i64 = 1 / 3;

pub fn to_digit(x: i64, y: i64) -> DualBalancedTernaryDigit {
  match x {
    -1 => match y {
      -1 => Dbt2,
      0 => Dbt7,
      1 => Dbt6,
      _ => unreachable!(format!("unexpected y: {}", y)),
    },
    0 => match y {
      -1 => Dbt9,
      0 => Dbt5,
      1 => Dbt1,
      _ => unreachable!(format!("unexpected y: {}", y)),
    },
    1 => match y {
      -1 => Dbt8,
      0 => Dbt3,
      1 => Dbt4,
      _ => unreachable!(format!("unexpected y: {}", y)),
    },
    _ => unreachable!(format!("unexpected x: {}", x)),
  }
}

impl ComplextXy {
  pub fn flip_xy(&self) -> ComplextXy {
    ComplextXy {
      x: self.y,
      y: self.x,
    }
  }
}
