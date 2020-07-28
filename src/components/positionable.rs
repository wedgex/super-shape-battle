use super::Component;
use ggez::nalgebra::Point2;
use std::any::Any;

pub struct Positionable {
  pub position: Point2<f32>,
  pub rotation: f32,
}

impl Positionable {
  pub fn new(x: f32, y: f32) -> Self {
    Positionable {
      position: Point2::new(x, y),
      rotation: 0.0,
    }
  }
}

impl Component for Positionable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
