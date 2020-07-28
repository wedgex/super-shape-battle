use super::Component;
use std::any::Any;

pub struct Ship {}

impl Ship {
  pub fn new() -> Self {
    Ship {}
  }
}

impl Component for Ship {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
