use ggez::graphics::Mesh;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

use super::Component;

#[derive(Debug)]
pub struct Drawable {
  pub mesh: Mesh,
}

impl Component for Drawable {
  fn component_name() -> String {
    "Drawable".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for Drawable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name(),)
  }
}
