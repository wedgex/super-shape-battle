use crate::components::Collidable;
use crate::components::Drawable;
use crate::components::Expirable;
use crate::components::Physicsable;
use crate::components::PlayerControllable;
use crate::components::Positionable;
use crate::components::Rotatable;
use crate::entity::Entity;
use crate::geometry;
use ggez::graphics::{self};
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use graphics::Mesh;
use std::time::Duration;

pub fn build_ship(context: &mut Context) -> GameResult<Entity> {
  let mut entity = Entity::new();

  let points = ship_points();

  let position = Positionable::new(200., 200.);
  let mesh = Mesh::new_polygon(
    context,
    graphics::DrawMode::stroke(2.0),
    &points,
    graphics::WHITE,
  )?;

  let drawable = Drawable::new(mesh, Point2::new(25. / 2., 30. / 2.));
  let physics = Physicsable::new(0., 0.);
  let rotation = Rotatable::new(0.);

  entity.register_component(position);
  entity.register_component(drawable);
  entity.register_component(physics);
  entity.register_component(rotation);
  entity.register_component(Collidable::new(points.clone()));
  entity.register_component(PlayerControllable::new());

  Ok(entity)
}

pub fn build_bullet(context: &mut Context, x: f32, y: f32, angle: f32) -> GameResult<Entity> {
  let mut entity = Entity::new();
  let position = Positionable::new(x, y);
  let points = vec![
    Point2::new(0.0, 0.0),
    Point2::new(2.0, 0.0),
    Point2::new(2.0, 2.0),
    Point2::new(0.0, 2.0),
  ];
  let mesh = Mesh::new_polygon(
    context,
    graphics::DrawMode::stroke(2.0),
    &points,
    graphics::WHITE,
  )?;
  let drawable = Drawable::new(mesh, Point2::new(1., 1.));

  let velocity = 4. * geometry::angle_to_vec(angle);
  let physics = Physicsable::new(velocity.x, velocity.y);
  let expiration = Expirable::new(Duration::from_secs(3));

  entity.register_component(position);
  entity.register_component(drawable);
  entity.register_component(physics);
  entity.register_component(expiration);
  entity.register_component(Collidable::new(points.clone()));

  Ok(entity)
}

fn ship_points() -> Vec<Point2<f32>> {
  let w = 25.0;
  let h = 30.0;

  vec![
    Point2::new(0.0, h),
    Point2::new(w / 2.0, 0.0),
    Point2::new(w, h),
    Point2::new(w / 2.0, h - (h / 3.0)),
  ]
}
