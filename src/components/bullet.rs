use super::Component;
use std::any::Any;

pub struct Bullet {}

impl Bullet {
  pub fn new() -> Self {
    Bullet {}
  }
}

impl Component for Bullet {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
