use crate::components::Collidable;
use crate::components::Drawable;
use crate::components::Physicsable;
use crate::components::Transform;
use crate::components::Vulnerable;
use crate::components::{Damage, DamageType};
use crate::components::{Tag, TagType};
use crate::geometry::rotation_transform;
use crate::world::World;
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

pub fn octagon(world: &mut World, context: &mut Context, x: f32, y: f32) -> GameResult<()> {
  build_shape(
    world,
    context,
    x,
    y,
    octagon_points(),
    RED,
    Point2::new(30.18, 30.18), // https://www.omnicalculator.com/math/octagon
    3,
  )
}

pub fn hexagon(world: &mut World, context: &mut Context, x: f32, y: f32) -> GameResult<()> {
  build_shape(
    world,
    context,
    x,
    y,
    hexagon_points(),
    YELLOW,
    Point2::new(17.32, 17.32), // https://www.omnicalculator.com/math/hexagon
    2,
  )
}

pub fn square(world: &mut World, context: &mut Context, x: f32, y: f32) -> GameResult<()> {
  build_shape(
    world,
    context,
    x,
    y,
    square_points(),
    GREEN,
    Point2::new(15.0 / 2., 15.0 / 2.0),
    1,
  )
}

pub fn build_shape(
  world: &mut World,
  context: &mut Context,
  x: f32,
  y: f32,
  points: Vec<Point2<f32>>,
  color: Color,
  offset: Point2<f32>,
  level: u8,
) -> GameResult<()> {
  let entity = world.create_entity();

  let transform = Transform::new(x, y);
  let mesh = Mesh::new_polygon(context, graphics::DrawMode::stroke(2.0), &points, color)?;
  let drawable = Drawable::new(mesh, offset);
  let mut physics = Physicsable::new(0., 0.);
  physics.velocity = Vector2::new(1., 1.);

  world.add(&entity, Tag::new(TagType::Shape(level)));
  world.add(&entity, transform);
  world.add(&entity, drawable);
  world.add(&entity, physics);
  world.add(&entity, Collidable::new(points.clone()));
  world.add(&entity, Damage::new(DamageType::Smash));
  world.add(&entity, Vulnerable::new(vec![DamageType::Projectile]));

  Ok(())
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
