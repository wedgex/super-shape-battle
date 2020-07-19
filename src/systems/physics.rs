use crate::components::Entity;
use crate::components::Physicsable;
use crate::components::Positionable;
use crate::game::GameState;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use super::System;

const MAX_VELOCITY: f32 = 5.0;

pub struct PhysicsSystem;

impl System for PhysicsSystem {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult {
    for entity in &mut game.entities {
      act_on(entity, context);
    }

    Ok(())
  }
}

fn act_on(entity: &mut Entity, context: &mut Context) {
  handle_acceleration(entity);
  handle_velocity(entity);
  wrap_position(entity, context);
}

fn handle_acceleration(entity: &mut Entity) {
  if let Some(physics) = entity.get_component_mut::<Physicsable>() {
    physics.velocity += physics.acceleration;
    if physics.velocity.norm_squared() > MAX_VELOCITY.powi(2) {
      physics.velocity = physics.velocity / physics.velocity.norm_squared().sqrt() * MAX_VELOCITY;
    }
  }
}

fn handle_velocity(entity: &mut Entity) {
  if let Some(physics) = entity.get_component::<Physicsable>() {
    let velocity = physics.velocity.clone();
    if let Some(position) = entity.get_component_mut::<Positionable>() {
      position.position += velocity;
    }
  }
}

fn wrap_position(entity: &mut Entity, context: &mut Context) {
  if let Some(position) = entity.get_component_mut::<Positionable>() {
    let (screen_width, screen_height) = graphics::drawable_size(context);

    if position.position.x < 0.0 {
      position.position.x += screen_width;
    }

    if position.position.x > screen_width {
      position.position.x -= screen_width;
    }

    if position.position.y < 0.0 {
      position.position.y += screen_height;
    }

    if position.position.y > screen_height {
      position.position.y -= screen_height;
    }
  }
}
