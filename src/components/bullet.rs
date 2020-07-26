use super::Component;
use std::any::Any;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
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

impl Display for Bullet {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Bullet")
  }
}
