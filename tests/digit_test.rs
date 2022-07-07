extern crate dual_balanced_ternary;

use dual_balanced_ternary::{dbt_digits, ternary, DualBalancedTernaryDigit::*};

#[test]
fn equality() {
  assert_eq!(
    dbt_digits(ternary("&23.456")),
    vec![(1, Dbt2), (0, Dbt3), (-1, Dbt4), (-2, Dbt5), (-3, Dbt6)],
  )
}
