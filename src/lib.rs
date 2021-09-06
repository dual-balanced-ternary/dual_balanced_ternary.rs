pub mod complex;
pub mod digit;
pub mod primes;

pub use digit::DualBalancedTernaryDigit;
pub use primes::{parse_ternary, ternary, DualBalancedTernary, DIV_PRECISION};

use digit::DualBalancedTernaryDigit::*;

fn create_dual_balanced_ternary(x: f64) -> DualBalancedTernary {
  let mut result = DualBalancedTernary {
    integral: vec![],
    fractional: vec![],
  };

  let negative_value = x < 0.0;

  let mut integral_part = x.floor() as i64;
  let mut fractional_part = x - x.floor();
  if negative_value {
    integral_part = 0 - integral_part;
  }

  let mut idx = 0;

  while integral_part > 0 {
    let left = integral_part % 3;
    if left == 0 {
      ()
    } else if left == 1 {
      result = result.add_at(idx, Dbt3);
    } else if left == 2 {
      result = result.add_at(idx + 1, Dbt3);
      result = result.add_at(idx, Dbt7);
    } else {
      unreachable!(format!("unexpected reminder: {} from {}", left, x))
    }
    integral_part = (integral_part - left) / 3;
    idx = idx + 1;
  }

  if negative_value {
    for (idx, item) in result.integral.to_owned().iter().enumerate() {
      result.integral[idx] = item.negate();
    }
  }

  let mut f_idx = -1;
  let mut precision = DIV_PRECISION; // TODO
  while fractional_part > 0.0 && precision > 0 {
    fractional_part = fractional_part * 3.0;
    let left = fractional_part.floor();
    if left == 0.0 {
      ()
    } else if left == 1.0 {
      result = result.add_at(f_idx, Dbt3);
    } else if left == 2.0 {
      result = result.add_at(f_idx + 1, Dbt3);
      result = result.add_at(f_idx, Dbt7);
    } else {
      unreachable!(format!(
        "unexpected carry: {} from {}",
        left, fractional_part
      ));
    }
    fractional_part = fractional_part - left;
    f_idx = f_idx - 1;
    precision = precision - 1;
  }
  result
}

pub fn create_dual_balanced_ternary_from_pair(x: f64, y: f64) -> DualBalancedTernary {
  let a = create_dual_balanced_ternary(x);
  let mut b = create_dual_balanced_ternary(y);
  for (idx, item) in b.integral.to_owned().iter().enumerate() {
    b.integral[idx] = item.flip_xy();
  }
  for (idx, item) in b.fractional.to_owned().iter().enumerate() {
    b.fractional[idx] = item.flip_xy();
  }
  return a + b;
}
