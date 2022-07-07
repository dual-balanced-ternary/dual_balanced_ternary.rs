extern crate dual_balanced_ternary;
use core::convert::TryInto;
use std::convert::TryFrom;

use dual_balanced_ternary::{ternary, DualBalancedTernary, DualBalancedTernaryDigit::*};

#[test]
fn to_buffer() -> Result<(), String> {
  assert_eq!(
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&."))?),
    Ok(ternary("&."))
  );
  assert_eq!(
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&1."))?),
    Ok(ternary("&1."))
  );
  assert_eq!(
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&.1"))?),
    Ok(ternary("&.1"))
  );

  assert_eq!(
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&12.12"))?),
    Ok(ternary("&12.12"))
  );

  assert_eq!(
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&3445647.674"))?),
    Ok(ternary("&3445647.674"))
  );
  assert_eq!(
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&5553445647.674555"))?),
    DualBalancedTernary::try_from(&TryInto::<Vec<u8>>::try_into(ternary("&3445647.674"))?),
  );

  assert_eq!(
    TryInto::<Vec<u8>>::try_into(DualBalancedTernary {
      integral: vec![Dbt1, Dbt5],
      fractional: vec![Dbt1, Dbt5]
    }),
    TryInto::<Vec<u8>>::try_into(DualBalancedTernary {
      integral: vec![Dbt1],
      fractional: vec![Dbt1]
    }),
  );

  Ok(())
}
