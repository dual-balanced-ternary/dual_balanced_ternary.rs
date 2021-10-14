use dual_balanced_ternary::{ternary, DualBalancedTernary};

pub fn main() -> Result<(), String> {
  println!("{:?}", ternary("&1.1").to_buffer());
  println!("{:?}", ternary("&14.14").to_buffer());

  println!("{:?}", DualBalancedTernary::from_buffer(&[1, 21, 21]));
  println!("{:?}", DualBalancedTernary::from_buffer(&[1, 65, 20]));

  println!("TODO {:?}", ternary("&12.12").to_buffer()?);

  Ok(())
}
