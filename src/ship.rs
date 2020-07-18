use crate::components::drawable::Drawable;
use crate::components::physicsable::Physicsable;
use crate::components::positionable::Positionable;
use crate::components::rotatable::Rotatable;
use crate::components::Entity;
use ggez::graphics::{self};
use ggez::nalgebra::Point2;

const BULLET_TIME_SECS: u64 = 2;

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

  entity.register_component::<Positionable>(position);
  entity.register_component::<Drawable>(drawable);
  entity.register_component(physics);
  entity.register_component(rotation);

  entity
}

pub fn build_bullet(x: f32, y: f32) -> Entity {
  let mut entity = Entity::new();
  let position = Positionable::new(x, y);
  let drawable = Drawable::new(
    vec![Point2::new(0.0, 0.0)],
    graphics::WHITE,
    graphics::DrawMode::stroke(2.0),
  );
  let physics = Physicsable::new(0., 0.);

  entity.register_component::<Positionable>(position);
  entity.register_component::<Drawable>(drawable);
  entity.register_component(physics);

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
