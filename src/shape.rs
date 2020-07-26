use crate::components::Collidable;
use crate::components::Drawable;
use crate::components::Physicsable;
use crate::components::Positionable;
use crate::components::{Shape, ShapeType};
use crate::entity::Entity;
use crate::geometry::rotation_transform;
use ggez::graphics;
use ggez::graphics::Color;
use ggez::graphics::Mesh;
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;
use ggez::GameResult;
use std::f32::consts::PI;

const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
const YELLOW: graphics::Color = graphics::Color::new(255.0, 255.0, 0.0, 1.0);
const GREEN: graphics::Color = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

pub fn octagon(context: &mut Context, x: f32, y: f32) -> GameResult<Entity> {
  build_shape(
    context,
    x,
    y,
    octagon_points(),
    RED,
    ShapeType::Octagon,
    Point2::new(30.18, 30.18), // https://www.omnicalculator.com/math/octagon
  )
}

pub fn hexagon(context: &mut Context, x: f32, y: f32) -> GameResult<Entity> {
  build_shape(
    context,
    x,
    y,
    hexagon_points(),
    YELLOW,
    ShapeType::Hexagon,
    Point2::new(17.32, 17.32), // https://www.omnicalculator.com/math/hexagon
  )
}

pub fn square(context: &mut Context, x: f32, y: f32) -> GameResult<Entity> {
  build_shape(
    context,
    x,
    y,
    square_points(),
    GREEN,
    ShapeType::Square,
    Point2::new(15.0 / 2., 15.0 / 2.0),
  )
}

pub fn build_shape(
  context: &mut Context,
  x: f32,
  y: f32,
  points: Vec<Point2<f32>>,
  color: Color,
  shape_type: ShapeType,
  offset: Point2<f32>,
) -> GameResult<Entity> {
  let mut entity = Entity::new();

  let position = Positionable::new(x, y);
  let mesh = Mesh::new_polygon(context, graphics::DrawMode::stroke(2.0), &points, color)?;
  let drawable = Drawable::new(mesh, offset);
  let mut physics = Physicsable::new(0., 0.);
  physics.velocity = Vector2::new(1., 1.);

  entity.register_component::<Positionable>(position);
  entity.register_component::<Drawable>(drawable);
  entity.register_component(physics);
  entity.register_component(Collidable::new(points.clone()));
  entity.register_component(Shape::new(shape_type));

  Ok(entity)
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
