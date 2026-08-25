use std::collections::HashSet;
use std::convert::TryFrom;

use dual_balanced_ternary::{
  DualBalancedTernary, DualBalancedTernaryDigit, DualBalancedTernaryDigit::*, complex::ComplexXy, ternary, try_ternary,
};

const DIGITS: [DualBalancedTernaryDigit; 9] = [Dbt1, Dbt2, Dbt3, Dbt4, Dbt5, Dbt6, Dbt7, Dbt8, Dbt9];

fn xy(value: &DualBalancedTernary) -> (i64, i64) {
  let value = ComplexXy::from(value);
  (value.x.round() as i64, value.y.round() as i64)
}

#[test]
fn digit_coordinates_round_trip() {
  for digit in DIGITS {
    let (x, y) = digit.coordinates();
    assert_eq!(DualBalancedTernaryDigit::try_from((i64::from(x), i64::from(y))), Ok(digit));
    assert_eq!(ComplexXy::from(digit), ComplexXy::new(f64::from(x), f64::from(y)));
  }

  assert!(DualBalancedTernaryDigit::try_from((2, 0)).is_err());
}

#[test]
fn digit_tables_match_gaussian_arithmetic() {
  for a in DIGITS {
    let (ax, ay) = a.coordinates();
    for b in DIGITS {
      let (bx, by) = b.coordinates();

      let (add_carry, add_unit) = a + b;
      let (cx, cy) = add_carry.coordinates();
      let (ux, uy) = add_unit.coordinates();
      assert_eq!((3 * cx + ux, 3 * cy + uy), (ax + bx, ay + by), "{a} + {b}");

      // The project's forward axis is the real axis: (x, y) maps to y + x*i.
      let expected_product = (ay * bx + ax * by, ay * by - ax * bx);
      let (mul_carry, mul_unit) = a * b;
      let (cx, cy) = mul_carry.coordinates();
      let (ux, uy) = mul_unit.coordinates();
      assert_eq!((3 * cx + ux, 3 * cy + uy), expected_product, "{a} * {b}");
    }
  }
}

#[test]
fn one_digit_residues_form_f9() {
  // Modulo 3, the carry vanishes. There are no zero divisors among the eight
  // non-zero residues, and 1+i (digit 8) generates their multiplicative group.
  for a in DIGITS.into_iter().filter(|digit| *digit != Dbt5) {
    for b in DIGITS.into_iter().filter(|digit| *digit != Dbt5) {
      let (_, unit) = a * b;
      assert_ne!(unit, Dbt5, "non-zero residues {a} and {b} multiplied to zero");
    }
  }

  let mut seen = HashSet::new();
  let mut power = Dbt1;
  for _ in 0..8 {
    seen.insert(power);
    power = (power * Dbt8).1;
  }
  assert_eq!(seen.len(), 8);
  assert_eq!(power, Dbt1);
}

#[test]
fn integer_grid_round_trips_and_obeys_ring_laws() {
  for x in -20..=20 {
    for y in -20..=20 {
      let value = DualBalancedTernary::try_new(x as f64, y as f64).unwrap();
      assert_eq!(xy(&value), (x, y));

      let conjugate = value.conjugate();
      assert_eq!(xy(&conjugate), (-x, y));
      assert_eq!(xy(&value.norm()), (0, x * x + y * y));
    }
  }

  for ax in -4..=4 {
    for ay in -4..=4 {
      for bx in -4..=4 {
        for by in -4..=4 {
          let a = DualBalancedTernary::new(ax as f64, ay as f64);
          let b = DualBalancedTernary::new(bx as f64, by as f64);
          assert_eq!(xy(&(a.clone() + b.clone())), (ax + bx, ay + by));
          assert_eq!(xy(&(a * b)), (ay * bx + ax * by, ay * by - ax * bx));
        }
      }
    }
  }
}

#[test]
fn parsing_and_binary_input_reject_malformed_data() {
  for invalid in ["", "1", "x1", "&1.2.3", "&0", "&🙂"] {
    assert!(try_ternary(invalid).is_err(), "accepted invalid literal {invalid:?}");
  }

  assert!(DualBalancedTernary::try_from(f64::NAN).is_err());
  assert!(DualBalancedTernary::try_from(f64::INFINITY).is_err());
  assert!(DualBalancedTernary::try_new(0.0, f64::NEG_INFINITY).is_err());

  assert!(DualBalancedTernary::try_from([].as_slice()).is_err());
  assert!(DualBalancedTernary::try_from([3, 0x15].as_slice()).is_err());
  assert!(DualBalancedTernary::try_from([1, 0xa5].as_slice()).is_err());
}

#[test]
fn binary_format_round_trips_many_shapes() {
  for literal in ["&5", "&1", "&.1", "&12.34", "&123456789.987654321"] {
    let original = ternary(literal);
    let encoded = Vec::<u8>::try_from(original.clone()).unwrap();
    assert_eq!(DualBalancedTernary::try_from(encoded.as_slice()).unwrap(), original);
  }
}

#[test]
fn checked_division_reports_zero() {
  assert_eq!(ternary("&11").checked_div(&ternary("&19")).unwrap(), ternary("&19"));
  assert!(ternary("&1").checked_div(&ternary("&5")).is_err());
}
