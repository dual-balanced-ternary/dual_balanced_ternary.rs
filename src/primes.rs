use std::convert::{TryFrom, TryInto};
use std::fmt;
use std::hash::{Hash, Hasher};
use std::ops::{Add, Div, Mul, Neg, Sub};
use std::str::FromStr;

use crate::complex::ComplexXy;
use crate::digit::{DualBalancedTernaryDigit, DualBalancedTernaryDigit::*};

/// how many digits in fractional part, when it's not divisible
pub const DIV_PRECISION: usize = 10;

const ZERO: DualBalancedTernary = DualBalancedTernary {
  integral: vec![],
  fractional: vec![],
};

/// Dual Balanced Ternary represented in limited accuracy.
#[derive(Debug, Clone)]
pub struct DualBalancedTernary {
  /// integral part, digits near 0 are placed first
  pub integral: Vec<DualBalancedTernaryDigit>,
  /// fractional part, digits near 0 are placed first
  pub fractional: Vec<DualBalancedTernaryDigit>,
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
        for x in &self.fractional {
          write!(f, "{}", x)?;
        }
      }
    }
    Ok(())
  }
}

impl TryFrom<f64> for DualBalancedTernary {
  type Error = String;

  fn try_from(x: f64) -> Result<Self, Self::Error> {
    let mut result = DualBalancedTernary {
      integral: vec![],
      fractional: vec![],
    };

    let negative_value = x < 0.0;

    let mut integral_part = x.floor() as i64;
    let mut fractional_part = x - x.floor();
    if negative_value {
      integral_part = 0 - integral_part;
    }

    let mut idx = 0;

    while integral_part > 0 {
      let left = integral_part % 3;
      if left == 0 {
        // nothing
      } else if left == 1 {
        result = result.add_at(idx, Dbt3);
      } else if left == 2 {
        result = result.add_at(idx + 1, Dbt3);
        result = result.add_at(idx, Dbt7);
      } else {
        unreachable!("unexpected reminder: {} from {}", left, x)
      }
      integral_part = (integral_part - left) / 3;
      idx += 1;
    }

    if negative_value {
      for (idx, item) in result.integral.to_owned().iter().enumerate() {
        result.integral[idx] = -*item;
      }
    }

    let mut f_idx = -1;
    let mut precision = DIV_PRECISION; // TODO
    while fractional_part > 0.0 && precision > 0 {
      fractional_part *= 3.0;
      let left = fractional_part.floor();
      if left == 0.0 {
        // nothing
      } else if (left - 1.0).abs() < std::f64::EPSILON {
        result = result.add_at(f_idx, Dbt3);
      } else if (left - 2.0).abs() < std::f64::EPSILON {
        result = result.add_at(f_idx + 1, Dbt3);
        result = result.add_at(f_idx, Dbt7);
      } else {
        return Err(format!("unexpected carry: {} from {}", left, fractional_part));
      }
      fractional_part -= left;
      f_idx -= 1;
      precision -= 1;
    }
    Ok(result)
  }
}

impl TryFrom<(f64, f64)> for DualBalancedTernary {
  type Error = String;
  fn try_from(pair: (f64, f64)) -> Result<Self, Self::Error> {
    let (x, y) = pair;
    let a: DualBalancedTernary = x.try_into()?;
    let mut b: DualBalancedTernary = y.try_into()?;
    for (idx, item) in b.integral.to_owned().iter().enumerate() {
      b.integral[idx] = item.flip_xy();
    }
    for (idx, item) in b.fractional.to_owned().iter().enumerate() {
      b.fractional[idx] = item.flip_xy();
    }
    Ok(a + b)
  }
}

impl Neg for DualBalancedTernary {
  type Output = Self;
  fn neg(self) -> Self {
    let mut result: DualBalancedTernary = self;
    for i in 0..result.integral.len() {
      result.integral[i] = -result.integral[i];
    }
    for i in 0..result.fractional.len() {
      result.fractional[i] = -result.fractional[i];
    }
    result
  }
}

// convert to x,y value, which is a complex number
impl From<DualBalancedTernary> for ComplexXy {
  fn from(value: DualBalancedTernary) -> Self {
    let mut result = ComplexXy { x: 0.0, y: 0.0 };
    let mut unit: f64 = 1.0;
    for item in &value.integral {
      let v: ComplexXy = (*item).into();
      result.x += v.x * unit;
      result.y += v.y * unit;
      unit *= 3.0;
    }
    unit = 1.0;
    for item in &value.fractional {
      unit /= 3.0;
      let v: ComplexXy = (*item).into();
      result.x += v.x * unit;
      result.y += v.y * unit;
    }
    result
  }
}

impl TryFrom<DualBalancedTernary> for Vec<u8> {
  type Error = String;

  /// buffer format
  /// [integral length]+[integral pairs]+[fractional pairs]
  fn try_from(value: DualBalancedTernary) -> Result<Self, String> {
    // make sure no extra `5`s is generated into buffer
    let v = value.strip_empty_tails();
    let int_len = v.integral.len();
    if int_len < 256 {
      let mut buf: Vec<u8> = vec![int_len as u8];
      // for integral part, put space 5 at head
      let mut halfed = false;
      let mut prev: u8 = 0;
      for x in &v.integral {
        if halfed {
          prev += u8::from(*x);
          buf.push(prev.to_owned());
          halfed = false;
        } else {
          prev = u8::from(*x) << 4;
          halfed = true;
        }
      }
      if halfed {
        prev += 5;
        buf.push(prev.to_owned());
        halfed = false;
      }

      // expected handled by pair
      assert_eq!(buf.len(), ((int_len + 1) >> 1) + 1);

      // for integral part, put space 5 at tail
      for x in &v.fractional {
        if halfed {
          prev += u8::from(*x);
          buf.push(prev.to_owned());
          halfed = false;
        } else {
          prev = u8::from(*x) << 4;
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
}

impl TryFrom<&Vec<u8>> for DualBalancedTernary {
  type Error = String;
  /// buffer format
  /// [integral length]+[integral pairs]+[fractional pairs]
  fn try_from(buf: &Vec<u8>) -> Result<Self, Self::Error> {
    if buf.is_empty() {
      return Err(String::from("dbt buffer expected >=2 u8 numbers"));
    }

    let int_range = (buf[0] + 1) as usize >> 1;

    if buf.len() < (int_range + 1) {
      return Err(String::from("dbt buffer length smaller than integral size"));
    }
    let mut integral: Vec<DualBalancedTernaryDigit> = vec![];
    let mut fractional: Vec<DualBalancedTernaryDigit> = vec![];

    // println!("buffer: {:?}", buf);
    for (idx, x) in buf.iter().enumerate() {
      if idx < 1 {
        continue;
      }
      // println!("reading: {} {}", idx, x);
      if idx < (int_range + 1) as usize {
        integral.push(DualBalancedTernaryDigit::try_from((x & 0b11110000) >> 4)?);
        integral.push(DualBalancedTernaryDigit::try_from(x & 0b00001111)?);
      } else {
        fractional.push(DualBalancedTernaryDigit::try_from((x & 0b11110000) >> 4)?);
        fractional.push(DualBalancedTernaryDigit::try_from(x & 0b00001111)?);
      }
    }

    Ok(Self { integral, fractional }.strip_empty_tails())
  }
}

impl DualBalancedTernary {
  /// created like a complex number, but notice DBT has main direction at `1`
  pub fn new(x: f64, y: f64) -> Self {
    (x, y).try_into().unwrap()
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
        let (carry, unit) = self.integral[idx as usize] + d;
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
        let (carry, unit) = self.fractional[f_idx as usize] + d;
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

  /// clockwise rotation
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

  /// anti-clockwise rotation
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
      return (a2.integral.last().unwrap().to_owned(), a2.integral.len() as i64 - 1);
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
      unreachable!("only linear ternary values allowed for a: {}", self)
    }
    if !other.is_linear_ternary() {
      unreachable!("only linear ternary values allowed for b: {}", other)
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
      let try_digit: DualBalancedTernaryDigit = if (a_digit == Dbt1 && b_digit == Dbt1) || (a_digit == Dbt9 && b_digit == Dbt9) {
        Dbt1
      } else if (a_digit == Dbt1 && b_digit == Dbt9) || (a_digit == Dbt9 && b_digit == Dbt1) {
        Dbt9
      } else {
        unreachable!("TODO, unknown case")
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

  /// drop fractional part
  pub fn round(&self) -> Self {
    DualBalancedTernary {
      integral: self.integral.to_owned(),
      fractional: vec![],
    }
  }

  /// drop fractional part but leave at least n digits
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
      && (self.fractional.is_empty() || self.fractional.to_owned()[self.fractional.len() - 1] != Dbt5)
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

  /// internally it relies on 1-directional arithmetic for calculation
  pub fn is_linear_ternary(&self) -> bool {
    for item in &self.integral {
      if item != &Dbt1 && item != &Dbt5 && item != &Dbt9 {
        return false;
      }
    }
    for item in &self.fractional {
      if item != &Dbt1 && item != &Dbt5 && item != &Dbt9 {
        return false;
      }
    }
    true
  }
}

impl TryFrom<char> for DualBalancedTernaryDigit {
  type Error = String;
  fn try_from(value: char) -> Result<Self, Self::Error> {
    match value {
      '1' => Ok(DualBalancedTernaryDigit::Dbt1),
      '2' => Ok(DualBalancedTernaryDigit::Dbt2),
      '3' => Ok(DualBalancedTernaryDigit::Dbt3),
      '4' => Ok(DualBalancedTernaryDigit::Dbt4),
      '5' => Ok(DualBalancedTernaryDigit::Dbt5),
      '6' => Ok(DualBalancedTernaryDigit::Dbt6),
      '7' => Ok(DualBalancedTernaryDigit::Dbt7),
      '8' => Ok(DualBalancedTernaryDigit::Dbt8),
      '9' => Ok(DualBalancedTernaryDigit::Dbt9),
      _ => Err(format!("{} is not valid ternary digit representation", value)),
    }
  }
}

impl FromStr for DualBalancedTernary {
  type Err = String;
  fn from_str(s: &str) -> Result<Self, Self::Err> {
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
        result.integral.insert(0, c.try_into()?);
      }
    }
    if pieces.len() == 2 {
      let chunk = pieces[1];
      for c in chunk.chars() {
        result.fractional.push(c.try_into()?);
      }
    }
    if pieces.len() > 2 {
      return Err(format!("invalid format for a ternary value: {}", s));
    }
    result = result.strip_empty_tails();
    Ok(result)
  }
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
    self.add(-other)
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
        let (carry, unit) = a_item * b_item;
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
