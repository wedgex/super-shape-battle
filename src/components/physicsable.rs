use ggez::nalgebra::Vector2;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

use super::Component;

#[derive(Debug)]
pub struct Physicsable {
  pub velocity: Vector2<f32>,
}

impl Physicsable {
  pub fn new(x: f32, y: f32) -> Self {
    Physicsable {
      velocity: Vector2::new(x, y),
    }
  }
}

impl Component for Physicsable {
  fn component_name() -> String {
    "Physicsable".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for Physicsable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}: (x: {}, y: {})",
      self.name(),
      self.velocity.x,
      self.velocity.y
    )
  }
}
