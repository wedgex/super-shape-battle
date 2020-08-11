use super::Component;
use ggez::nalgebra::Point2;
use std::any::Any;

#[derive(Clone)]
pub struct Transform {
  pub position: Point2<f32>,
  pub rotation: f32,
}

impl Transform {
  pub fn new(x: f32, y: f32) -> Self {
    Transform {
      position: Point2::new(x, y),
      rotation: 0.0,
    }
  }
}

impl Component for Transform {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}

impl Default for Transform {
  fn default() -> Self {
    Transform::new(0., 0.)
  }
}
