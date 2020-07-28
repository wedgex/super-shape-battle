use ggez::nalgebra::Vector2;
use std::any::Any;

use super::Component;

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
