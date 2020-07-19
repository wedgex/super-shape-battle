use crate::components::Drawable;
use crate::components::Entity;
use crate::components::Physicsable;
use crate::components::Positionable;
use crate::geometry::rotation_transform;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::nalgebra::{Point2, Vector2};
use std::f32::consts::PI;

const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
const YELLOW: graphics::Color = graphics::Color::new(255.0, 255.0, 0.0, 1.0);
const GREEN: graphics::Color = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

pub fn octagon(x: f32, y: f32) -> Entity {
  build_shape(x, y, octagon_points(), RED)
}

pub fn hexagon(x: f32, y: f32) -> Entity {
  build_shape(x, y, hexagon_points(), YELLOW)
}

pub fn square(x: f32, y: f32) -> Entity {
  build_shape(x, y, square_points(), GREEN)
}

pub fn build_shape(x: f32, y: f32, points: Vec<Point2<f32>>, color: Color) -> Entity {
  let mut entity = Entity::new();

  let position = Positionable::new(x, y);
  let drawable = Drawable::new(points, color, graphics::DrawMode::stroke(2.0));
  let mut physics = Physicsable::new(0., 0.);
  physics.velocity = Vector2::new(1., 1.);

  entity.register_component::<Positionable>(position);
  entity.register_component::<Drawable>(drawable);
  entity.register_component(physics);

  entity
}

fn polygon_points(sides: i32, length: f32, rotation: f32) -> Vec<Point2<f32>> {
  let angle = 2.0 * PI / sides as f32;

  (0..=sides)
    .map(|i| {
      rotation_transform(
        &Point2::new(
          length * (angle * i as f32).cos(),
          length * (angle * i as f32).sin(),
        ),
        rotation,
      )
    })
    .collect()
}

fn octagon_points() -> Vec<Point2<f32>> {
  polygon_points(8, 25.0, 70.0)
}

fn hexagon_points() -> Vec<Point2<f32>> {
  polygon_points(6, 20.0, 60.0)
}

fn square_points() -> Vec<Point2<f32>> {
  polygon_points(4, 15.0, 45.0)
}
