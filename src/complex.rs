/// simple complex number struct
#[derive(PartialEq, Debug, Clone)]
pub struct ComplexXy {
  pub x: f64,
  pub y: f64,
}

impl ComplexXy {
  pub fn flip_xy(&self) -> ComplexXy {
    ComplexXy { x: self.y, y: self.x }
  }
}
