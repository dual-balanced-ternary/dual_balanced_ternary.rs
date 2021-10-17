use dual_balanced_ternary::complex::ComplextXy;
use dual_balanced_ternary::{
  create_dual_balanced_ternary_from_pair, dbt_digits, ternary, DualBalancedTernaryDigit::*,
};

#[test]
fn equality() {
  assert_eq!(
    dbt_digits(ternary("&23.456")),
    vec![(1, Dbt2), (0, Dbt3), (-1, Dbt4), (-2, Dbt5), (-3, Dbt6)],
  )
}
