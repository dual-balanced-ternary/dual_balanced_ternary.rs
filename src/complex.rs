/// simple complex number struct
#[derive(PartialEq, Debug, Clone, Copy, Default)]
pub struct ComplexXy {
  pub x: f64,
  pub y: f64,
}

impl ComplexXy {
  /// Creates a Cartesian coordinate pair.
  pub const fn new(x: f64, y: f64) -> Self {
    Self { x, y }
  }

  /// Squared Euclidean length, equal to the Gaussian norm.
  pub fn norm_squared(self) -> f64 {
    self.x.mul_add(self.x, self.y * self.y)
  }

  pub fn flip_xy(&self) -> ComplexXy {
    ComplexXy { x: self.y, y: self.x }
  }
}
