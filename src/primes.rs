use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Sub};

use crate::complex::ComplextXy;
use crate::digit::{DualBalancedTernaryDigit, DualBalancedTernaryDigit::*};

// places of digits
//
//   6 1 8
//   7 5 3
//   2 9 4

// how many digits in fractional part, when it's not divisible
pub const DIV_PRECISION: usize = 10;

const ZERO: DualBalancedTernary = DualBalancedTernary {
  integral: vec![],
  fractional: vec![],
};

#[derive(Debug, Clone)]
pub struct DualBalancedTernary {
  // digits near 0 are placed first
  pub integral: Vec<DualBalancedTernaryDigit>,
  pub fractional: Vec<DualBalancedTernaryDigit>,
}

impl fmt::Display for DualBalancedTernaryDigit {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      DualBalancedTernaryDigit::Dbt1 => write!(f, "1"),
      DualBalancedTernaryDigit::Dbt2 => write!(f, "2"),
      DualBalancedTernaryDigit::Dbt3 => write!(f, "3"),
      DualBalancedTernaryDigit::Dbt4 => write!(f, "4"),
      DualBalancedTernaryDigit::Dbt5 => write!(f, "5"),
      DualBalancedTernaryDigit::Dbt6 => write!(f, "6"),
      DualBalancedTernaryDigit::Dbt7 => write!(f, "7"),
      DualBalancedTernaryDigit::Dbt8 => write!(f, "8"),
      DualBalancedTernaryDigit::Dbt9 => write!(f, "9"),
    }
  }
}

/// uses `&1.2` to write. notice `5` is the zero point
impl fmt::Display for DualBalancedTernary {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    if self.integral.is_empty() && self.fractional.is_empty() {
      write!(f, "&5")?;
    } else {
      write!(f, "&")?;
      for i in 0..self.integral.len() {
        write!(f, "{}", self.integral[self.integral.len() - i - 1])?;
      }
      if !self.fractional.is_empty() {
        write!(f, ".")?;
        for x in self.fractional.to_owned() {
          write!(f, "{}", x)?;
        }
      }
    }
    Ok(())
  }
}

impl DualBalancedTernary {
  pub fn negate(&self) -> DualBalancedTernary {
    let mut result: DualBalancedTernary = self.to_owned();
    for i in 0..result.integral.len() {
      result.integral[i] = result.integral[i].negate();
    }
    for i in 0..result.fractional.len() {
      result.fractional[i] = result.fractional[i].negate();
    }
    result
  }

  // TODO positive number to make value larger, not in use yet
  pub fn move_by(&self, n: i64) -> DualBalancedTernary {
    let mut b = self.to_owned();
    match n {
      0 => return self.to_owned(),
      n if n > 0 => {
        for _i in 0..n {
          if b.fractional.is_empty() {
            b.integral.insert(0, Dbt5);
          } else {
            b.integral.insert(0, b.fractional.to_owned()[0]);
            b.fractional.remove(0);
          }
        }
      }
      _ => {
        // n < 0
        for _i in 0..(-n) {
          if b.integral.is_empty() {
            b.fractional.insert(0, Dbt5);
          } else {
            b.fractional.insert(0, b.integral[0]);
            b.integral.remove(0);
          }
        }
      }
    };
    b
  }

  // 0 for unit position, -1 for first fractional position
  pub fn add_at(&self, idx: i64, d: DualBalancedTernaryDigit) -> DualBalancedTernary {
    // echo "to add: ", a, " ", d, " at: ", idx
    if d == Dbt5 {
      return self.to_owned();
    }
    let mut b = self.to_owned();
    if idx >= 0 {
      if idx > self.integral.len() as i64 - 1 {
        let mut times = idx - self.integral.len() as i64 + 1;
        while times > 0 {
          b.integral.push(Dbt5);
          times -= 1;
        }
        // # echo b.integral, " ", idx, " times ", times

        b.integral[idx as usize] = d;
        b
      } else {
        let (carry, unit) = self.integral[idx as usize].add_digits(d);
        b.integral[idx as usize] = unit;
        // echo "sum: ", sum
        if carry != Dbt5 {
          // echo "has carry in integral: ", sum
          b.add_at(idx + 1, carry)
        } else {
          b
        }
      }
    } else {
      let f_idx = -1 - idx;
      if f_idx > self.fractional.len() as i64 - 1 {
        let mut times = f_idx - self.fractional.len() as i64 + 1;
        while times > 0 {
          b.fractional.push(Dbt5);
          times -= 1;
        }
        b.fractional[f_idx as usize] = d;
        b
      } else {
        let (carry, unit) = self.fractional[f_idx as usize].add_digits(d);
        b.fractional[f_idx as usize] = unit;
        if carry != Dbt5 {
          // echo "has carry in fractional: ", sum
          b.add_at(idx + 1, carry)
        } else {
          b
        }
      }
    }
  }

  /// keep value of 1 direction and flip 3 direction
  pub fn conjugate(&self) -> DualBalancedTernary {
    let mut result = self.to_owned();
    for (idx, item) in result.integral.to_owned().iter().enumerate() {
      result.integral[idx] = item.flip_left_right();
    }
    for (idx, item) in result.fractional.to_owned().iter().enumerate() {
      result.fractional[idx] = item.flip_left_right();
    }
    result
  }

  /// value at y direction only contains 1, 5, 9,
  /// value at x direction only contains 7, 5, 3.
  pub fn split_yx(&self) -> (DualBalancedTernary, DualBalancedTernary) {
    let mut x: DualBalancedTernary = self.to_owned();
    let mut y: DualBalancedTernary = self.to_owned();
    for (idx, item) in self.integral.iter().enumerate() {
      let v = item.split_yx();
      let (v_x, v_y) = v;
      x.integral[idx] = v_x;
      y.integral[idx] = v_y;
    }
    for (idx, item) in self.fractional.iter().enumerate() {
      let v = item.split_yx();
      let (v_x, v_y) = v;
      x.fractional[idx] = v_x;
      y.fractional[idx] = v_y;
    }
    (y.strip_empty_tails(), x.strip_empty_tails())
  }

  pub fn rotate3(&self) -> DualBalancedTernary {
    let mut result = self.to_owned();
    for (idx, item) in result.integral.to_owned().iter().enumerate() {
      result.integral[idx] = item.rotate3();
    }
    for (idx, item) in result.fractional.to_owned().iter().enumerate() {
      result.fractional[idx] = item.rotate3();
    }
    result
  }

  pub fn rotate7(&self) -> DualBalancedTernary {
    let mut result = self.to_owned();
    for (idx, item) in result.integral.to_owned().iter().enumerate() {
      result.integral[idx] = item.rotate7();
    }
    for (idx, item) in result.fractional.to_owned().iter().enumerate() {
      result.fractional[idx] = item.rotate7();
    }
    result
  }

  pub fn get_first_digit(&self) -> (DualBalancedTernaryDigit, i64) {
    let a2 = self.strip_empty_tails();
    if !a2.integral.is_empty() {
      return (
        a2.integral.last().unwrap().to_owned(),
        a2.integral.len() as i64 - 1,
      );
    } else if a2.fractional.is_empty() {
      (Dbt5, 0)
    } else {
      for (idx, item) in a2.fractional.iter().enumerate() {
        if item != &Dbt5 {
          return (item.to_owned(), -1 - idx as i64);
        }
      }
      unreachable!("TODO get first")
    }
  }

  /// only works for paths containing 1,5,9
  pub fn linear_greater_than(self, b: DualBalancedTernary) -> bool {
    let delta = self - b;
    let (digit, _) = delta.get_first_digit();
    digit == Dbt1
  }

  /// only works for paths containing 1,5,9
  pub fn linear_littler_than(self, b: DualBalancedTernary) -> bool {
    let delta = self - b;
    let (digit, _) = delta.get_first_digit();
    digit == Dbt9
  }

  /// ternary divide only handles values consisted of 1,5,9
  pub fn linear_divide(&self, other: DualBalancedTernary) -> DualBalancedTernary {
    let mut result = DualBalancedTernary {
      integral: vec![],
      fractional: vec![],
    };
    // echo fmt"dividing: a b {a} {b}"
    if self.is_zero() {
      return self.to_owned();
    }
    if other.is_zero() {
      unreachable!("&5 is not a valid divisor as divisor")
    }
    if !self.is_linear_ternary() {
      unreachable!(format!(
        "only linear ternary values allowed for a: {}",
        self
      ))
    }
    if !other.is_linear_ternary() {
      unreachable!(format!(
        "only linear ternary values allowed for b: {}",
        other
      ))
    }

    let mut reminder = self.to_owned();
    let mut precision = DIV_PRECISION * 2;
    // echo fmt"initial: {reminder} {b}"
    while !reminder.is_zero() && precision > 0 {
      // echo fmt"loop with reminder:{reminder} divisor:{b} result:{result}"
      let (a_digit, a_idx) = reminder.get_first_digit();
      let (b_digit, b_idx) = other.get_first_digit();
      let try_position = a_idx - b_idx;
      // echo fmt"guessing {try_digit} at {try_position}, with cond {a_head} {b_head}"
      let try_digit: DualBalancedTernaryDigit =
        if (a_digit == Dbt1 && b_digit == Dbt1) || (a_digit == Dbt9 && b_digit == Dbt9) {
          Dbt1
        } else if (a_digit == Dbt1 && b_digit == Dbt9) || (a_digit == Dbt9 && b_digit == Dbt1) {
          Dbt9
        } else {
          unreachable!(String::from("TODO, unknown case"))
        };
      let v = ZERO.add_at(try_position, try_digit);
      let step = v.to_owned() * other.to_owned();
      reminder = reminder.to_owned() - step;
      result = result + v;
      precision -= 1;
    }
    // echo fmt"temp result: {result}"
    result
  }

  pub fn round(&self) -> Self {
    DualBalancedTernary {
      integral: self.integral.to_owned(),
      fractional: vec![],
    }
  }

  pub fn round_n(&self, n: usize) -> Self {
    if n > self.fractional.len() {
      self.to_owned()
    } else {
      let mut fractional = vec![];
      let mut i = 0;
      while i < n {
        fractional.push(self.fractional[i]);
        i += 1;
      }
      DualBalancedTernary {
        integral: self.integral.to_owned(),
        fractional,
      }
    }
  }

  // 5 is the zero point of digits, can be removed at end
  pub fn strip_empty_tails(&self) -> DualBalancedTernary {
    if (self.integral.is_empty() || self.integral.to_owned()[self.integral.len() - 1] != Dbt5)
      && (self.fractional.is_empty()
        || self.fractional.to_owned()[self.fractional.len() - 1] != Dbt5)
    {
      return self.to_owned();
    }
    let mut y = self.to_owned();
    while !y.integral.is_empty() && y.integral.to_owned()[y.integral.len() - 1] == Dbt5 {
      y.integral.pop();
    }
    while !y.fractional.is_empty() && y.fractional.to_owned()[y.fractional.len() - 1] == Dbt5 {
      y.fractional.pop();
    }
    y
  }

  pub fn pairs(&self) -> Vec<(i64, DualBalancedTernaryDigit)> {
    let mut result: Vec<(i64, DualBalancedTernaryDigit)> = vec![];
    for (idx, item) in self.integral.to_owned().iter().enumerate() {
      result.push((idx as i64, item.to_owned()));
    }
    for (idx, item) in self.fractional.to_owned().iter().enumerate() {
      result.push((-1 - idx as i64, item.to_owned()));
    }
    result
  }

  pub fn is_zero(&self) -> bool {
    let a2 = self.strip_empty_tails();
    a2.integral.is_empty() && a2.fractional.is_empty()
  }

  pub fn is_linear_ternary(&self) -> bool {
    for item in self.integral.to_owned() {
      if item != Dbt1 && item != Dbt5 && item != Dbt9 {
        return false;
      }
    }
    for item in self.fractional.to_owned() {
      if item != Dbt1 && item != Dbt5 && item != Dbt9 {
        return false;
      }
    }
    true
  }

  // convert to x,y values
  pub fn to_float(&self) -> ComplextXy {
    let mut result = ComplextXy { x: 0.0, y: 0.0 };
    let mut unit: f64 = 1.0;
    for item in self.integral.to_owned() {
      let v = item.to_float();
      result.x += v.x * unit;
      result.y += v.y * unit;
      unit *= 3.0;
    }
    unit = 1.0;
    for item in self.fractional.to_owned() {
      unit /= 3.0;
      let v = item.to_float();
      result.x += v.x * unit;
      result.y += v.y * unit;
    }
    result
  }

  /// buffer format
  /// [magic 3]+[integral length]+[integral pairs]+[fractional pairs]
  pub fn to_buffer(&self) -> Result<Vec<u8>, String> {
    let int_len = self.integral.len();
    if int_len < 256 {
      let mut buf: Vec<u8> = vec![3, int_len as u8];
      // for integral part, put space 5 at head
      let mut halfed = false;
      let mut prev: u8 = 0;
      for x in &self.integral {
        if halfed {
          prev += x.to_u8();
          buf.push(prev.to_owned());
          halfed = false;
        } else {
          prev = x.to_u8() << 4;
          halfed = true;
        }
      }
      if halfed {
        prev += 5;
        buf.push(prev.to_owned());
        halfed = false;
      }

      // expected handled by pair
      assert_eq!(buf.len(), ((int_len + 1) >> 1) + 2);

      // for integral part, put space 5 at tail
      for x in &self.fractional {
        if halfed {
          prev += x.to_u8();
          buf.push(prev.to_owned());
          halfed = false;
        } else {
          prev = x.to_u8() << 4;
          halfed = true;
        }
      }
      if halfed {
        prev += 5;
        buf.push(prev.to_owned());
      }

      Ok(buf)
    } else {
      Err(format!("integral part too long: {}", int_len))
    }
  }

  /// buffer format
  /// [magic 3]+[integral length]+[integral pairs]+[fractional pairs]
  pub fn from_buffer(buf: Vec<u8>) -> Result<Self, String> {
    if buf.len() < 2 {
      return Err(String::from("dbt buffer expected >=2 u8 numbers"));
    }
    if buf[0] != 3 {
      return Err(String::from("dbt magic number should be 3"));
    }

    let int_range = (buf[1] + 1) as usize >> 1;

    if buf.len() < (int_range + 2) {
      return Err(String::from("dbt buffer length smaller than integral size"));
    }
    let mut integral: Vec<DualBalancedTernaryDigit> = vec![];
    let mut fractional: Vec<DualBalancedTernaryDigit> = vec![];

    // println!("buffer: {:?}", buf);
    for (idx, x) in buf.iter().enumerate() {
      if idx < 2 {
        continue;
      }
      // println!("reading: {} {}", idx, x);
      if idx < (int_range + 2) as usize {
        integral.push(DualBalancedTernaryDigit::from_u8((x & 0b11110000) >> 4)?);
        integral.push(DualBalancedTernaryDigit::from_u8(x & 0b00001111)?);
      } else {
        fractional.push(DualBalancedTernaryDigit::from_u8((x & 0b11110000) >> 4)?);
        fractional.push(DualBalancedTernaryDigit::from_u8(x & 0b00001111)?);
      }
    }

    Ok(
      Self {
        integral,
        fractional,
      }
      .strip_empty_tails(),
    )
  }
}

pub fn parse_ternary_digit(s: char) -> Result<DualBalancedTernaryDigit, String> {
  match s {
    '1' => Ok(DualBalancedTernaryDigit::Dbt1),
    '2' => Ok(DualBalancedTernaryDigit::Dbt2),
    '3' => Ok(DualBalancedTernaryDigit::Dbt3),
    '4' => Ok(DualBalancedTernaryDigit::Dbt4),
    '5' => Ok(DualBalancedTernaryDigit::Dbt5),
    '6' => Ok(DualBalancedTernaryDigit::Dbt6),
    '7' => Ok(DualBalancedTernaryDigit::Dbt7),
    '8' => Ok(DualBalancedTernaryDigit::Dbt8),
    '9' => Ok(DualBalancedTernaryDigit::Dbt9),
    _ => Err(format!("{} is not valid ternary digit representation", s)),
  }
}

pub fn parse_ternary_digit_from_char(s: char) -> Result<DualBalancedTernaryDigit, String> {
  match s {
    '1' => Ok(Dbt1),
    '2' => Ok(Dbt2),
    '3' => Ok(Dbt3),
    '4' => Ok(Dbt4),
    '5' => Ok(Dbt5),
    '6' => Ok(Dbt6),
    '7' => Ok(Dbt7),
    '8' => Ok(Dbt8),
    '9' => Ok(Dbt9),
    _ => Err(format!("{} is not valid ternary digit representation", s)),
  }
}

pub fn parse_ternary(s: &str) -> Result<DualBalancedTernary, String> {
  let mut result = DualBalancedTernary {
    integral: vec![],
    fractional: vec![],
  };
  if s.is_empty() {
    return Err(String::from("ternary requires & symbol"));
  }
  let content = s[1..].to_string();
  if content.is_empty() {
    return Err(String::from("ternary requires a number, at least &5"));
  }
  let pieces = content.split('.').collect::<Vec<&str>>();
  if !pieces.is_empty() {
    let chunk = pieces[0];
    for c in chunk.chars() {
      result.integral.insert(0, parse_ternary_digit(c)?);
    }
  }
  if pieces.len() == 2 {
    let chunk = pieces[1];
    for c in chunk.chars() {
      result.fractional.push(parse_ternary_digit(c)?);
    }
  }
  if pieces.len() > 2 {
    return Err(format!("invalid format for a ternary value: {}", s));
  }
  result = result.strip_empty_tails();
  Ok(result)
}

/// a creator function
pub fn ternary(s: &str) -> DualBalancedTernary {
  parse_ternary(s).unwrap()
}

impl PartialEq for DualBalancedTernary {
  fn eq(&self, other: &Self) -> bool {
    let a2 = self.strip_empty_tails();
    let b2 = other.strip_empty_tails();
    if a2.integral.len() != b2.integral.len() {
      return false;
    }
    if a2.fractional.len() != b2.fractional.len() {
      return false;
    }
    for (idx, item) in a2.integral.iter().enumerate() {
      if &b2.integral[idx] != item {
        return false;
      }
    }
    for (idx, item) in a2.fractional.iter().enumerate() {
      if &b2.fractional[idx] != item {
        return false;
      }
    }

    true
  }
}
impl Eq for DualBalancedTernary {}

impl Hash for DualBalancedTernary {
  fn hash<H: Hasher>(&self, state: &mut H) {
    "DualBalancedTernary".hash(state);

    let a2 = self.strip_empty_tails();
    for item in a2.integral {
      item.hash(state)
    }
    (".").hash(state);
    for item in a2.fractional {
      item.hash(state)
    }
  }
}

impl Add for DualBalancedTernary {
  type Output = Self;

  fn add(self, b: Self) -> Self {
    let mut a2 = self;
    for (idx, item) in b.integral.iter().enumerate() {
      // echo "adding: ", a2, " ", idx, " ", item
      a2 = a2.add_at(idx as i64, item.to_owned());
    }
    for (idx, item) in b.fractional.iter().enumerate() {
      // echo "f adding: ", a2, " ", idx, " ", item
      a2 = a2.add_at(-1 - idx as i64, item.to_owned());
    }
    // echo "result: ", a2
    a2.strip_empty_tails();
    a2
  }
}

impl Sub for DualBalancedTernary {
  type Output = Self;
  fn sub(self, other: Self) -> Self::Output {
    self.add(other.negate())
  }
}

impl Mul for DualBalancedTernary {
  type Output = Self;

  fn mul(self, other: Self) -> Self::Output {
    let mut result = DualBalancedTernary {
      integral: vec![],
      fractional: vec![],
    };
    for (a_idx, a_item) in self.pairs() {
      for (b_idx, b_item) in other.pairs() {
        let (carry, unit) = a_item.mutiply_digits(b_item);
        result = result.add_at(a_idx + b_idx, unit);
        if carry != Dbt5 {
          result = result.add_at(a_idx + b_idx + 1, carry);
        }
        // echo fmt"multiply a:{a_item} b:{b_item}, v:{v}, result:{result}"
      }
    }
    result
  }
}

impl Div for DualBalancedTernary {
  type Output = Self;

  fn div(self, other: DualBalancedTernary) -> Self {
    let cj = other.conjugate();
    let a2 = self * cj.to_owned();
    let b2 = other * cj; // support only 1,5,9 in value now
    let (x, y) = a2.split_yx();
    let ay = y;
    let ax = x;
    // echo fmt"b.. {b} {cj} => {b2}"
    // echo fmt"splitted: {splitted} from {a2}, b2 is {b2}"
    ay.linear_divide(b2.to_owned()) + (ax.rotate7().linear_divide(b2).rotate3())
  }
}
