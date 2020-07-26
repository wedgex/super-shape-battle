use super::Component;
use std::any::Any;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub enum ShapeType {
  Octagon,
  Hexagon,
  Square,
}

#[derive(Clone, Debug)]
pub struct Shape {
  shape_type: ShapeType,
}

impl Shape {
  pub fn new(shape_type: ShapeType) -> Self {
    Shape { shape_type }
  }
}

impl Component for Shape {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Display for Shape {
  fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
    write!(f, "Shape")
  }
}
