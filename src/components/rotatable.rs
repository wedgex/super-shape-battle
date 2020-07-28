use super::Component;
use std::any::Any;

#[derive(Clone, Debug)]
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
