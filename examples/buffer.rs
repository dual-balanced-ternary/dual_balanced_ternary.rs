use dual_balanced_ternary::{ternary, DualBalancedTernary};
use std::convert::{TryFrom, TryInto};

pub fn main() -> Result<(), String> {
  println!("{:?}", TryInto::<Vec<u8>>::try_into(ternary("&1.1"))?);
  println!("{:?}", TryInto::<Vec<u8>>::try_into(ternary("&14.14"))?);

  println!("{:?}", DualBalancedTernary::try_from(&vec![1, 21, 21])?);
  println!("{:?}", DualBalancedTernary::try_from(&vec![1, 65, 20]));

  println!("TODO {:?}", TryInto::<Vec<u8>>::try_into(ternary("&12.12"))?);

  Ok(())
}
