use dual_balanced_ternary::{DualBalancedTernary, DualBalancedTernaryDigit, F9, ternary};

use DualBalancedTernaryDigit::*;

const ELEMENTS: [F9; 9] = [
  F9::new(Dbt1),
  F9::new(Dbt2),
  F9::new(Dbt3),
  F9::new(Dbt4),
  F9::new(Dbt5),
  F9::new(Dbt6),
  F9::new(Dbt7),
  F9::new(Dbt8),
  F9::new(Dbt9),
];

fn exact_coordinates(value: &DualBalancedTernary) -> (i128, i128) {
  assert!(value.fractional.is_empty());
  let mut x = 0_i128;
  let mut y = 0_i128;
  let mut place = 1_i128;
  for digit in &value.integral {
    let (digit_x, digit_y) = digit.coordinates();
    x += i128::from(digit_x) * place;
    y += i128::from(digit_y) * place;
    place *= 3;
  }
  (x, y)
}

#[test]
fn f9_obeys_field_laws_exhaustively() {
  for a in ELEMENTS {
    assert_eq!(a + F9::ZERO, a);
    assert_eq!(a * F9::ONE, a);
    assert_eq!(a + -a, F9::ZERO);

    for b in ELEMENTS {
      assert_eq!(a + b, b + a);
      assert_eq!(a * b, b * a);
      for c in ELEMENTS {
        assert_eq!((a + b) + c, a + (b + c));
        assert_eq!((a * b) * c, a * (b * c));
        assert_eq!(a * (b + c), a * b + a * c);
      }
    }
  }
}

#[test]
fn f9_inverse_generator_and_frobenius_are_correct() {
  let mut power = F9::ONE;
  for _ in 0..8 {
    assert_ne!(power, F9::ZERO);
    power = power * F9::GENERATOR;
  }
  assert_eq!(power, F9::ONE);

  for value in ELEMENTS {
    assert_eq!(value.pow(0), F9::ONE);
    assert_eq!(value.frobenius().frobenius(), value);
    assert_eq!(value.frobenius(), F9::new(value.digit().flip_left_right()));
    assert_eq!(value.trace().digit().coordinates().0, 0);
    assert_eq!(value.norm().digit().coordinates().0, 0);

    if value == F9::ZERO {
      assert_eq!(value.inverse(), None);
      assert_eq!(F9::ONE.checked_div(value), None);
    } else {
      let inverse = value.inverse().unwrap();
      assert_eq!(value * inverse, F9::ONE);
      assert_eq!(value.pow(8), F9::ONE);
      assert_eq!(value.checked_div(value), Some(F9::ONE));
    }
  }
}

#[test]
fn exact_integer_coordinates_cover_the_full_i64_range() {
  for (x, y) in [(0, 0), (1, -1), (-20, 37), (i64::MAX, i64::MIN), (i64::MIN, i64::MAX)] {
    let value = DualBalancedTernary::from_i64_coordinates(x, y);
    assert_eq!(exact_coordinates(&value), (i128::from(x), i128::from(y)));
  }
}

#[test]
fn powers_and_radix_shifts_preserve_exact_arithmetic() {
  let integer = DualBalancedTernary::from_i64_coordinates(4, 6);
  assert_eq!(integer.to_string(), "&143");
  assert_eq!(integer.pow(2).to_string(), "&36289");

  let value = ternary("&18.37");
  assert_eq!(value.pow(0), ternary("&1"));
  assert_eq!(value.pow(1), value);
  assert_eq!(value.pow(5), value.pow(2) * value.pow(3));

  for shift in -12..=12 {
    assert_eq!(value.move_by(shift).move_by(-shift), value);
  }
}
