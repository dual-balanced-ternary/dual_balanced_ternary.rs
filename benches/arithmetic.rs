use std::{hint::black_box, time::Instant};

use dual_balanced_ternary::{DualBalancedTernary, try_ternary};

fn measure(name: &str, iterations: u32, mut operation: impl FnMut()) {
  for _ in 0..iterations.min(10) {
    operation();
  }

  let started = Instant::now();
  for _ in 0..iterations {
    operation();
  }
  let nanos = started.elapsed().as_nanos() / u128::from(iterations);
  println!("{name:>16}: {nanos:>10} ns/iteration ({iterations} iterations)");
}

fn patterned_value(digits: usize, offset: usize) -> DualBalancedTernary {
  const PATTERN: &[u8] = b"12346789";
  let mut literal = String::with_capacity(digits + 1);
  literal.push('&');
  for index in 0..digits {
    literal.push(PATTERN[(index + offset) % PATTERN.len()] as char);
  }
  try_ternary(&literal).unwrap()
}

fn main() {
  let parse_input = format!("&{}", "12346789".repeat(32));
  let add_left = patterned_value(128, 0);
  let add_right = patterned_value(128, 3);
  let mul_left = patterned_value(48, 0);
  let mul_right = patterned_value(48, 5);
  let divisor = patterned_value(20, 2);
  let dividend = divisor.clone() * patterned_value(20, 6);
  let shift_value = patterned_value(128, 1);

  measure("parse/256", 2_000, || {
    black_box(try_ternary(black_box(&parse_input)).unwrap());
  });
  measure("add/128", 2_000, || {
    black_box(black_box(add_left.clone()) + black_box(add_right.clone()));
  });
  measure("multiply/48", 500, || {
    black_box(black_box(mul_left.clone()) * black_box(mul_right.clone()));
  });
  measure("divide/exact20", 100, || {
    black_box(black_box(dividend.clone()) / black_box(divisor.clone()));
  });
  measure("shift/128x64", 2_000, || {
    black_box(black_box(&shift_value).move_by(64));
  });
}
