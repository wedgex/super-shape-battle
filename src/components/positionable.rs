use super::Component;
use ggez::nalgebra::Point2;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Positionable {
  pub position: Point2<f32>,
}

impl Positionable {
  pub fn new(x: f32, y: f32) -> Self {
    Positionable {
      position: Point2::new(x, y),
    }
  }
}

impl Component for Positionable {
  fn component_name() -> String {
    "Positionable".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }
}

impl Display for Positionable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(
      f,
      "{}: (x: {}, y: {})",
      self.name(),
      self.position.x,
      self.position.y
    )
  }
}
