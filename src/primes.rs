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

fn canonical_len(digits: &[DualBalancedTernaryDigit]) -> usize {
  digits.iter().rposition(|digit| *digit != Dbt5).map_or(0, |index| index + 1)
}

/// Reduces an arbitrary component to `3 * carry + unit`, with a centered unit.
fn balanced_reduce(value: i64) -> (i64, i8) {
  let unit = (value + 1).rem_euclid(3) - 1;
  ((value - unit) / 3, unit as i8)
}

/// One exact balanced-ternary step without overflowing on `i64::MIN`.
fn balanced_integer_step(value: i64) -> (i64, i8) {
  match value % 3 {
    -2 => (value / 3 - 1, 1),
    -1 => (value / 3, -1),
    0 => (value / 3, 0),
    1 => (value / 3, 1),
    2 => (value / 3 + 1, -1),
    _ => unreachable!("remainder modulo 3 is outside -2..=2"),
  }
}

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
    if !x.is_finite() {
      return Err(format!("DBT conversion requires a finite number, got {x}"));
    }
    if x.abs() >= i64::MAX as f64 {
      return Err(format!("DBT conversion is out of range for i64: {x}"));
    }
    let mut result = DualBalancedTernary {
      integral: vec![],
      fractional: vec![],
    };

    let negative_value = x.is_sign_negative();
    let magnitude = x.abs();
    let mut integral_part = magnitude.floor() as i64;
    let mut fractional_part = magnitude - magnitude.floor();

    let mut idx = 0;

    while integral_part > 0 {
      let left = integral_part % 3;
      if left == 0 {
        // nothing
      } else if left == 1 {
        result.add_assign_at(idx, Dbt3);
      } else if left == 2 {
        result.add_assign_at(idx + 1, Dbt3);
        result.add_assign_at(idx, Dbt7);
      } else {
        unreachable!("unexpected reminder: {} from {}", left, x)
      }
      integral_part = (integral_part - left) / 3;
      idx += 1;
    }

    let mut f_idx = -1;
    let mut precision = DIV_PRECISION; // TODO
    while fractional_part > 0.0 && precision > 0 {
      fractional_part *= 3.0;
      let left = fractional_part.floor();
      if left == 0.0 {
        // nothing
      } else if (left - 1.0).abs() < f64::EPSILON {
        result.add_assign_at(f_idx, Dbt3);
      } else if (left - 2.0).abs() < f64::EPSILON {
        result.add_assign_at(f_idx + 1, Dbt3);
        result.add_assign_at(f_idx, Dbt7);
      } else {
        return Err(format!("unexpected carry: {} from {}", left, fractional_part));
      }
      fractional_part -= left;
      f_idx -= 1;
      precision -= 1;
    }
    if negative_value {
      result = -result;
    }
    Ok(result.strip_empty_tails())
  }
}

impl TryFrom<(f64, f64)> for DualBalancedTernary {
  type Error = String;
  fn try_from(pair: (f64, f64)) -> Result<Self, Self::Error> {
    let (x, y) = pair;
    let a: DualBalancedTernary = x.try_into()?;
    let mut b: DualBalancedTernary = y.try_into()?;
    for item in &mut b.integral {
      *item = item.flip_xy();
    }
    for item in &mut b.fractional {
      *item = item.flip_xy();
    }
    Ok(a + b)
  }
}

impl Neg for DualBalancedTernary {
  type Output = Self;
  fn neg(self) -> Self {
    let mut result: DualBalancedTernary = self;
    for item in &mut result.integral {
      *item = -*item;
    }
    for item in &mut result.fractional {
      *item = -*item;
    }
    result
  }
}

// convert to x,y value, which is a complex number
impl From<&DualBalancedTernary> for ComplexXy {
  fn from(value: &DualBalancedTernary) -> Self {
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

impl From<DualBalancedTernary> for ComplexXy {
  fn from(value: DualBalancedTernary) -> Self {
    Self::from(&value)
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

impl TryFrom<&[u8]> for DualBalancedTernary {
  type Error = String;
  /// buffer format
  /// [integral length]+[integral pairs]+[fractional pairs]
  fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
    if buf.is_empty() {
      return Err(String::from("DBT buffer must contain a length byte"));
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
      if idx < int_range + 1 {
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

impl TryFrom<&Vec<u8>> for DualBalancedTernary {
  type Error = String;

  fn try_from(buf: &Vec<u8>) -> Result<Self, Self::Error> {
    Self::try_from(buf.as_slice())
  }
}

impl DualBalancedTernary {
  fn normalize_mut(&mut self) {
    self.integral.truncate(canonical_len(&self.integral));
    self.fractional.truncate(canonical_len(&self.fractional));
  }

  fn digit_at(&self, exponent: i64) -> DualBalancedTernaryDigit {
    if exponent >= 0 {
      self.integral.get(exponent as usize).copied().unwrap_or(Dbt5)
    } else {
      self.fractional.get((-1 - exponent) as usize).copied().unwrap_or(Dbt5)
    }
  }

  fn set_digit(&mut self, exponent: i64, digit: DualBalancedTernaryDigit) {
    if exponent >= 0 {
      let index = exponent as usize;
      if self.integral.len() <= index {
        self.integral.resize(index + 1, Dbt5);
      }
      self.integral[index] = digit;
    } else {
      let index = (-1 - exponent) as usize;
      if self.fractional.len() <= index {
        self.fractional.resize(index + 1, Dbt5);
      }
      self.fractional[index] = digit;
    }
  }

  fn add_assign_at(&mut self, mut exponent: i64, mut digit: DualBalancedTernaryDigit) {
    while digit != Dbt5 {
      let (x, y) = self.digit_at(exponent).coordinates();
      let (dx, dy) = digit.coordinates();
      let (carry_x, unit_x) = balanced_reduce(i64::from(x + dx));
      let (carry_y, unit_y) = balanced_reduce(i64::from(y + dy));
      self.set_digit(exponent, DualBalancedTernaryDigit::from_coordinates(unit_x, unit_y));
      digit = DualBalancedTernaryDigit::from_coordinates(carry_x as i8, carry_y as i8);
      exponent += 1;
    }
  }

  fn digit_pairs(&self) -> impl Iterator<Item = (i64, DualBalancedTernaryDigit)> + '_ {
    self
      .integral
      .iter()
      .enumerate()
      .map(|(index, digit)| (index as i64, *digit))
      .chain(self.fractional.iter().enumerate().map(|(index, digit)| (-1 - index as i64, *digit)))
  }

  fn exponent_bounds(&self) -> Option<(i64, i64)> {
    self
      .digit_pairs()
      .filter(|(_, digit)| *digit != Dbt5)
      .map(|(exponent, _)| exponent)
      .fold(None, |bounds, exponent| match bounds {
        None => Some((exponent, exponent)),
        Some((minimum, maximum)) => Some((minimum.min(exponent), maximum.max(exponent))),
      })
  }

  fn from_exponent_digits(minimum: i64, digits: Vec<DualBalancedTernaryDigit>) -> Self {
    let mut result = Self {
      integral: vec![],
      fractional: vec![],
    };
    for (offset, digit) in digits.into_iter().enumerate() {
      if digit != Dbt5 {
        result.set_digit(minimum + offset as i64, digit);
      }
    }
    result.normalize_mut();
    result
  }

  /// Creates a DBT value from Cartesian coordinates.
  ///
  /// # Panics
  ///
  /// Panics for non-finite or out-of-range inputs. Use [`Self::try_new`] for
  /// untrusted values.
  pub fn new(x: f64, y: f64) -> Self {
    Self::try_new(x, y).expect("invalid Cartesian coordinates for DBT")
  }

  /// Tries to create a DBT value from Cartesian coordinates.
  pub fn try_new(x: f64, y: f64) -> Result<Self, String> {
    (x, y).try_into()
  }

  /// Creates a DBT value exactly from integer Cartesian coordinates.
  ///
  /// This avoids the precision limit of converting through `f64`.
  pub fn from_i64_coordinates(mut x: i64, mut y: i64) -> Self {
    let mut integral = Vec::new();
    while x != 0 || y != 0 {
      let (next_x, unit_x) = balanced_integer_step(x);
      let (next_y, unit_y) = balanced_integer_step(y);
      integral.push(DualBalancedTernaryDigit::from_coordinates(unit_x, unit_y));
      x = next_x;
      y = next_y;
    }
    Self {
      integral,
      fractional: vec![],
    }
  }

  /// Raises this value to a non-negative integer power exactly.
  pub fn pow(&self, mut exponent: u32) -> Self {
    let mut base = self.clone();
    let mut result = Self::from_i64_coordinates(0, 1);
    while exponent > 0 {
      if exponent & 1 == 1 {
        result = result * base.clone();
      }
      exponent >>= 1;
      if exponent > 0 {
        base = base.clone() * base;
      }
    }
    result
  }

  /// Multiplies by an integer power of the radix: `self * 3^n`.
  pub fn move_by(&self, n: i64) -> DualBalancedTernary {
    if n == 0 {
      return self.clone();
    }

    let amount = usize::try_from(n.unsigned_abs()).expect("shift does not fit in memory");
    let mut result = self.clone();
    if n > 0 {
      let moved_count = amount.min(result.fractional.len());
      let moved: Vec<_> = result.fractional.drain(..moved_count).collect();
      let mut integral = Vec::with_capacity(result.integral.len().saturating_add(amount));
      integral.resize(amount - moved_count, Dbt5);
      integral.extend(moved.into_iter().rev());
      integral.append(&mut result.integral);
      result.integral = integral;
    } else {
      let moved_count = amount.min(result.integral.len());
      let moved: Vec<_> = result.integral.drain(..moved_count).collect();
      let mut fractional = Vec::with_capacity(result.fractional.len().saturating_add(amount));
      fractional.resize(amount - moved_count, Dbt5);
      fractional.extend(moved.into_iter().rev());
      fractional.append(&mut result.fractional);
      result.fractional = fractional;
    }
    result.normalize_mut();
    result
  }

  // 0 for unit position, -1 for first fractional position
  pub fn add_at(&self, idx: i64, d: DualBalancedTernaryDigit) -> DualBalancedTernary {
    let mut result = self.clone();
    result.add_assign_at(idx, d);
    result.normalize_mut();
    result
  }

  /// keep value of 1 direction and flip 3 direction
  pub fn conjugate(&self) -> DualBalancedTernary {
    let mut result = self.to_owned();
    for item in &mut result.integral {
      *item = item.flip_left_right();
    }
    for item in &mut result.fractional {
      *item = item.flip_left_right();
    }
    result
  }

  /// Gaussian norm `z * conjugate(z)`, represented on the identity axis.
  pub fn norm(&self) -> DualBalancedTernary {
    self.to_owned() * self.conjugate()
  }

  /// Divides by `other`, returning an error instead of panicking on zero.
  pub fn checked_div(&self, other: &DualBalancedTernary) -> Result<DualBalancedTernary, String> {
    if other.is_zero() {
      return Err(String::from("cannot divide a DBT value by zero (&5)"));
    }

    let cj = other.conjugate();
    let numerator = self.to_owned() * cj.to_owned();
    let denominator = other.to_owned() * cj;
    let (x, y) = numerator.split_yx();
    Ok(y.linear_divide(denominator.to_owned()) + x.rotate7().linear_divide(denominator).rotate3())
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
    for item in &mut result.integral {
      *item = item.rotate3();
    }
    for item in &mut result.fractional {
      *item = item.rotate3();
    }
    result
  }

  /// anti-clockwise rotation
  pub fn rotate7(&self) -> DualBalancedTernary {
    let mut result = self.to_owned();
    for item in &mut result.integral {
      *item = item.rotate7();
    }
    for item in &mut result.fractional {
      *item = item.rotate7();
    }
    result
  }

  pub fn get_first_digit(&self) -> (DualBalancedTernaryDigit, i64) {
    if let Some(index) = self.integral.iter().rposition(|digit| *digit != Dbt5) {
      return (self.integral[index], index as i64);
    }
    if let Some(index) = self.fractional.iter().position(|digit| *digit != Dbt5) {
      return (self.fractional[index], -1 - index as i64);
    }
    (Dbt5, 0)
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
    let mut result = self.clone();
    result.normalize_mut();
    result
  }

  pub fn pairs(&self) -> Vec<(i64, DualBalancedTernaryDigit)> {
    let mut result = Vec::with_capacity(self.integral.len() + self.fractional.len());
    for (idx, item) in self.integral.iter().enumerate() {
      result.push((idx as i64, *item));
    }
    for (idx, item) in self.fractional.iter().enumerate() {
      result.push((-1 - idx as i64, *item));
    }
    result
  }

  pub fn is_zero(&self) -> bool {
    self.integral.iter().chain(&self.fractional).all(|digit| *digit == Dbt5)
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
    let content = s.strip_prefix('&').ok_or_else(|| format!("DBT literal must start with '&': {s}"))?;
    if content.is_empty() {
      return Err(String::from("DBT literal requires at least one digit or a radix point"));
    }
    let mut pieces = content.split('.');
    let integral = pieces.next().expect("split always returns the first piece");
    result.integral = integral
      .chars()
      .rev()
      .map(DualBalancedTernaryDigit::try_from)
      .collect::<Result<_, _>>()?;
    if let Some(fractional) = pieces.next() {
      result.fractional = fractional
        .chars()
        .map(DualBalancedTernaryDigit::try_from)
        .collect::<Result<_, _>>()?;
    }
    if pieces.next().is_some() {
      return Err(format!("invalid format for a ternary value: {}", s));
    }
    result.normalize_mut();
    Ok(result)
  }
}

impl PartialEq for DualBalancedTernary {
  fn eq(&self, other: &Self) -> bool {
    let self_integral_len = canonical_len(&self.integral);
    let other_integral_len = canonical_len(&other.integral);
    let self_fractional_len = canonical_len(&self.fractional);
    let other_fractional_len = canonical_len(&other.fractional);
    self.integral[..self_integral_len] == other.integral[..other_integral_len]
      && self.fractional[..self_fractional_len] == other.fractional[..other_fractional_len]
  }
}
impl Eq for DualBalancedTernary {}

impl Hash for DualBalancedTernary {
  fn hash<H: Hasher>(&self, state: &mut H) {
    "DualBalancedTernary".hash(state);

    for item in &self.integral[..canonical_len(&self.integral)] {
      item.hash(state)
    }
    (".").hash(state);
    for item in &self.fractional[..canonical_len(&self.fractional)] {
      item.hash(state)
    }
  }
}

impl Add for DualBalancedTernary {
  type Output = Self;

  fn add(self, b: Self) -> Self {
    let mut result = self;
    for (idx, item) in b.integral.into_iter().enumerate() {
      result.add_assign_at(idx as i64, item);
    }
    for (idx, item) in b.fractional.into_iter().enumerate() {
      result.add_assign_at(-1 - idx as i64, item);
    }
    result.normalize_mut();
    result
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
    let Some((self_minimum, self_maximum)) = self.exponent_bounds() else {
      return ZERO;
    };
    let Some((other_minimum, other_maximum)) = other.exponent_bounds() else {
      return ZERO;
    };
    let minimum = self_minimum + other_minimum;
    let maximum = self_maximum + other_maximum;
    let accumulator_len = usize::try_from(maximum - minimum + 1).expect("DBT product is too large");
    let mut xs = vec![0_i64; accumulator_len];
    let mut ys = vec![0_i64; accumulator_len];

    for (left_exponent, left) in self.digit_pairs().filter(|(_, digit)| *digit != Dbt5) {
      let (left_x, left_y) = left.coordinates();
      for (right_exponent, right) in other.digit_pairs().filter(|(_, digit)| *digit != Dbt5) {
        let (right_x, right_y) = right.coordinates();
        let index = usize::try_from(left_exponent + right_exponent - minimum).expect("product exponent is negative");
        xs[index] += i64::from(left_y * right_x + left_x * right_y);
        ys[index] += i64::from(left_y * right_y - left_x * right_x);
      }
    }

    let mut digits = Vec::with_capacity(accumulator_len + 1);
    let mut index = 0;
    while index < xs.len() {
      let (carry_x, unit_x) = balanced_reduce(xs[index]);
      let (carry_y, unit_y) = balanced_reduce(ys[index]);
      digits.push(DualBalancedTernaryDigit::from_coordinates(unit_x, unit_y));
      if carry_x != 0 || carry_y != 0 {
        if index + 1 == xs.len() {
          xs.push(carry_x);
          ys.push(carry_y);
        } else {
          xs[index + 1] += carry_x;
          ys[index + 1] += carry_y;
        }
      }
      index += 1;
    }
    Self::from_exponent_digits(minimum, digits)
  }
}

impl Div for DualBalancedTernary {
  type Output = Self;

  fn div(self, other: DualBalancedTernary) -> Self {
    self.checked_div(&other).expect("attempted to divide a DBT value by zero")
  }
}
