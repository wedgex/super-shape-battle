use super::Component;
use std::any::Any;
use std::fmt::{self, Debug, Display, Formatter};

#[derive(Debug)]
pub struct Rotatable {
  pub rotation: f32,
}

impl Rotatable {
  pub fn new(rotation: f32) -> Self {
    Rotatable { rotation }
  }
}

impl Component for Rotatable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Display for Rotatable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Rotatable: {}", self.rotation)
  }
}
