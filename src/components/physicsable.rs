use ggez::nalgebra::Vector2;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

use super::Component;

#[derive(Clone, Debug)]
pub struct Physicsable {
  pub velocity: Vector2<f32>,
  pub acceleration: Vector2<f32>,
}

impl Physicsable {
  pub fn new(x: f32, y: f32) -> Self {
    Physicsable {
      velocity: Vector2::new(x, y),
      acceleration: Vector2::new(0., 0.),
    }
  }
}

impl Component for Physicsable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Display for Physicsable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "Physicsable: (x: {}, y: {})",
      self.velocity.x, self.velocity.y
    )
  }
}
