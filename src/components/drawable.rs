use ggez::graphics::Mesh;
use ggez::nalgebra::Point2;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

use super::Component;

#[derive(Debug)]
pub struct Drawable {
  pub mesh: Mesh,
  pub offset: Point2<f32>,
}

impl Drawable {
  pub fn new(mesh: Mesh, offset: Point2<f32>) -> Self {
    Drawable { mesh, offset }
  }
}

impl Component for Drawable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Display for Drawable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Drawable")
  }
}
