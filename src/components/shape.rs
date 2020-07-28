use super::Component;
use std::any::Any;

pub enum ShapeType {
  Octagon,
  Hexagon,
  Square,
}

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
