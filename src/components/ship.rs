use super::Component;
use std::any::Any;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
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

impl Display for Ship {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Ship")
  }
}
