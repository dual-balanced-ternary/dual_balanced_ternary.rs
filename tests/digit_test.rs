extern crate dual_balanced_ternary;

use dual_balanced_ternary::{DualBalancedTernaryDigit::*, dbt_digits, ternary};

#[test]
fn equality() {
  assert_eq!(
    dbt_digits(ternary("&23.456")),
    vec![(1, Dbt2), (0, Dbt3), (-1, Dbt4), (-2, Dbt5), (-3, Dbt6)],
  )
}
