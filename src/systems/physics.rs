use crate::components::Physicsable;
use crate::components::Transform;
use crate::entity::EntityId;
use crate::game::GameState;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use super::System;

const MAX_VELOCITY: f32 = 5.0;

pub struct PhysicsSystem;

impl System for PhysicsSystem {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult {
    let entities: Vec<EntityId> = game
      .entities_with::<Physicsable>()
      .iter()
      .map(|e| e.id)
      .collect();

    for entity_id in entities {
      handle_acceleration(game, entity_id);
      handle_velocity(game, entity_id);
      wrap_position(game, context, entity_id);
    }

    Ok(())
  }
}

fn handle_acceleration(game: &mut GameState, entity_id: EntityId) {
  if let Some(physics) = game.get_component_mut::<Physicsable>(entity_id) {
    physics.velocity += physics.acceleration;
    if physics.velocity.norm_squared() > MAX_VELOCITY.powi(2) {
      physics.velocity = physics.velocity / physics.velocity.norm_squared().sqrt() * MAX_VELOCITY;
    }
  }
}

fn handle_velocity(game: &mut GameState, entity_id: EntityId) {
  if let Some(physics) = game.get_component::<Physicsable>(entity_id) {
    let velocity = physics.velocity.clone();
    if let Some(position) = game.get_component_mut::<Transform>(entity_id) {
      position.position += velocity;
    }
  }
}

fn wrap_position(game: &mut GameState, context: &mut Context, entity_id: EntityId) {
  if let Some(position) = game.get_component_mut::<Transform>(entity_id) {
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
