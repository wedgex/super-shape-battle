use crate::components::drawable::Drawable;
use crate::components::expirable::Expirable;
use crate::components::physicsable::Physicsable;
use crate::components::player_controllable::PlayerControllable;
use crate::components::positionable::Positionable;
use crate::components::rotatable::Rotatable;
use crate::components::Entity;
use crate::geometry;
use ggez::graphics::{self};
use ggez::nalgebra::Point2;
use std::time::Duration;

pub fn build_ship() -> Entity {
  let mut entity = Entity::new();

  let position = Positionable::new(200., 200.);
  let drawable = Drawable::new(
    ship_points(),
    graphics::WHITE,
    graphics::DrawMode::stroke(2.0),
  );
  let physics = Physicsable::new(0., 0.);
  let rotation = Rotatable::new(0.);

  entity.register_component(position);
  entity.register_component(drawable);
  entity.register_component(physics);
  entity.register_component(rotation);
  entity.register_component(PlayerControllable::new());

  entity
}

pub fn build_bullet(x: f32, y: f32, angle: f32) -> Entity {
  let mut entity = Entity::new();
  let position = Positionable::new(x, y);
  let drawable = Drawable::new(
    vec![
      Point2::new(0.0, 0.0),
      Point2::new(2.0, 0.0),
      Point2::new(2.0, 2.0),
      Point2::new(0.0, 2.0),
    ],
    graphics::WHITE,
    graphics::DrawMode::stroke(2.0),
  );

  let velocity = 4. * geometry::angle_to_vec(angle);
  let physics = Physicsable::new(velocity.x, velocity.y);
  let expiration = Expirable::new(Duration::from_secs(3));

  entity.register_component(position);
  entity.register_component(drawable);
  entity.register_component(physics);
  entity.register_component(expiration);

  entity
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
