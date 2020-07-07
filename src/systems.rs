use super::game::GameState;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;

pub trait System {
  fn update(game: &mut GameState, context: &Context);
}

pub trait Physics {
  fn get_position(&self) -> Point2<f32>;
  fn move_to(&mut self, location: Point2<f32>);
  fn get_acceleration(&self) -> Vector2<f32>;
  fn get_velocity(&self) -> Vector2<f32>;
  fn set_velocity(&mut self, velocity: Vector2<f32>);
}

const MAX_VELOCITY: f32 = 5.0;

pub struct PhysicsSystem {}

impl System for PhysicsSystem {
  fn update(game: &mut GameState, context: &Context) {
    update_entity(&mut game.ship, context);

    for shape in &mut game.shapes {
      update_entity(shape, context);
    }
  }
}

fn update_entity(entity: &mut dyn Physics, context: &Context) {
  handle_acceleration(entity);
  handle_velocity(entity);
  wrap_position(entity, context);
}

fn handle_acceleration(entity: &mut dyn Physics) {
  let mut velocity = entity.get_velocity();
  velocity += entity.get_acceleration();
  if velocity.norm_squared() > MAX_VELOCITY.powi(2) {
    velocity = velocity / velocity.norm_squared().sqrt() * MAX_VELOCITY;
  }
  entity.set_velocity(velocity);
}

fn handle_velocity(entity: &mut dyn Physics) {
  entity.move_to(entity.get_position() + entity.get_velocity());
}

fn wrap_position(entity: &mut dyn Physics, context: &Context) {
  let mut position = entity.get_position();

  let (screen_width, screen_height) = graphics::drawable_size(context);

  if position.x < 0.0 {
    position.x += screen_width;
  }

  if position.x > screen_width {
    position.x -= screen_width;
  }

  if position.y < 0.0 {
    position.y += screen_height;
  }

  if position.y > screen_height {
    position.y -= screen_height;
  }

  entity.move_to(position);
}
