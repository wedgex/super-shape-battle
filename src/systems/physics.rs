use crate::components::Physicsable;
use crate::components::Transform;
use crate::entity::EntityId;
use crate::world::World;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use super::System;

const MAX_VELOCITY: f32 = 5.0;

pub struct PhysicsSystem;

impl System for PhysicsSystem {
  fn update(world: &mut World, context: &mut Context) -> GameResult {
    let entities: Vec<EntityId> = world
      .entities::<Physicsable>()
      .into_iter()
      .cloned()
      .collect();

    for entity_id in entities {
      handle_acceleration(world, &entity_id);
      handle_velocity(world, &entity_id);
      wrap_position(world, context, &entity_id);
    }

    Ok(())
  }
}

fn handle_acceleration(world: &mut World, entity: &EntityId) {
  if let Some(physics) = world.get_mut::<Physicsable>(entity) {
    physics.velocity += physics.acceleration;
    if physics.velocity.norm_squared() > MAX_VELOCITY.powi(2) {
      physics.velocity = physics.velocity / physics.velocity.norm_squared().sqrt() * MAX_VELOCITY;
    }
  }
}

fn handle_velocity(world: &mut World, entity: &EntityId) {
  if let Some(physics) = world.get::<Physicsable>(entity) {
    let velocity = physics.velocity.clone();
    if let Some(position) = world.get_mut::<Transform>(entity) {
      position.position += velocity;
    }
  }
}

fn wrap_position(world: &mut World, context: &mut Context, entity: &EntityId) {
  if let Some(position) = world.get_mut::<Transform>(entity) {
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
