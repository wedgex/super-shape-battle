use geo::{LineString, Polygon};
use ggez::nalgebra::Point2;
use std::any::Any;

use super::Component;

pub type CollisionBounds = Polygon<f32>;

#[derive(Clone)]
pub struct Collidable {
  pub bounds: CollisionBounds,
}

impl Collidable {
  pub fn new(bounds: Vec<Point2<f32>>) -> Self {
    let line_points: Vec<(f32, f32)> = bounds.iter().map(|p| (p.x, p.y)).collect();
    Collidable {
      bounds: Polygon::new(LineString::from(line_points), vec![]),
    }
  }
}

impl Component for Collidable {
  fn as_any(&self) -> &dyn Any {
    self
  }

  fn as_any_mut(&mut self) -> &mut dyn Any {
    self
  }
}
