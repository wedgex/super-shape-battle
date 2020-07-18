use ggez::graphics::{Color, DrawMode};
use ggez::nalgebra::Point2;
use std::any::Any;
use std::fmt::Debug;
use std::fmt::{self, Display, Formatter};

use super::Component;

#[derive(Debug)]
pub struct Drawable {
  pub vertices: Vec<Point2<f32>>,
  pub color: Color,
  pub draw_mode: DrawMode,
}

impl Drawable {
  pub fn new(vertices: Vec<Point2<f32>>, color: Color, draw_mode: DrawMode) -> Self {
    Drawable {
      vertices,
      color,
      draw_mode,
    }
  }
}

impl Component for Drawable {
  fn component_name() -> String {
    "Drawable".to_string()
  }

  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Display for Drawable {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "{}", self.name(),)
  }
}
