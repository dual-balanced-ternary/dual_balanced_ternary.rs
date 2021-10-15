use dual_balanced_ternary::complex::ComplextXy;
use dual_balanced_ternary::{
  create_dual_balanced_ternary_from_pair, ternary, DualBalancedTernary, DualBalancedTernaryDigit::*,
};

#[test]
fn to_buffer() -> Result<(), String> {
  assert_eq!(
    DualBalancedTernary::from_buffer(&ternary("&.").to_buffer()?),
    Ok(ternary("&."))
  );
  assert_eq!(
    DualBalancedTernary::from_buffer(&ternary("&1.").to_buffer()?),
    Ok(ternary("&1."))
  );
  assert_eq!(
    DualBalancedTernary::from_buffer(&ternary("&.1").to_buffer()?),
    Ok(ternary("&.1"))
  );

  assert_eq!(
    DualBalancedTernary::from_buffer(&ternary("&12.12").to_buffer()?),
    Ok(ternary("&12.12"))
  );

  assert_eq!(
    DualBalancedTernary::from_buffer(&ternary("&3445647.674").to_buffer()?),
    Ok(ternary("&3445647.674"))
  );
  assert_eq!(
    DualBalancedTernary::from_buffer(&ternary("&5553445647.674555").to_buffer()?),
    DualBalancedTernary::from_buffer(&ternary("&3445647.674").to_buffer()?),
  );

  assert_eq!(
    DualBalancedTernary {
      integral: vec![Dbt1, Dbt5],
      fractional: vec![Dbt1, Dbt5]
    }
    .to_buffer(),
    DualBalancedTernary {
      integral: vec![Dbt1],
      fractional: vec![Dbt1]
    }
    .to_buffer(),
  );

  Ok(())
}
