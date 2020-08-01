use crate::components::Physicsable;
use crate::components::PlayerControllable;
use crate::components::Transform;
use crate::entity::EntityId;
use crate::game::GameState;
use crate::geometry;
use crate::ship::build_bullet;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

use super::System;

pub struct PlayerInputSystem;

impl System for PlayerInputSystem {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult {
    let entity_ids: Vec<EntityId> = game
      .entities_with_mut::<PlayerControllable>()
      .iter_mut()
      .map(|e| e.id)
      .collect();

    for entity_id in entity_ids {
      apply_inputs_to(game, context, entity_id);
      handle_fire(game, context, entity_id);
    }

    Ok(())
  }
}

fn apply_inputs_to(game: &mut GameState, context: &mut Context, entity_id: EntityId) {
  let rotation = if let Some(transform) = game.get_component_mut::<Transform>(entity_id) {
    if keyboard::is_key_pressed(context, KeyCode::A) {
      turn_left(transform);
    }
    if keyboard::is_key_pressed(context, KeyCode::D) {
      turn_right(transform);
    }

    transform.rotation.clone()
  } else {
    0.
  };

  if let Some(physics) = game.get_component_mut::<Physicsable>(entity_id) {
    if keyboard::is_key_pressed(context, KeyCode::W) {
      accelerate(physics, rotation);
    } else {
      decelerate(physics);
    }
  }
}

fn handle_fire(game: &mut GameState, context: &mut Context, entity_id: EntityId) {
  let transform = game.get_component::<Transform>(entity_id);
  let position = transform.map(|t| t.position).unwrap_or(Point2::new(0., 0.));
  let rotation = transform.map(|t| t.rotation).unwrap_or(0.0);

  if let Some(controllable) = game.get_component_mut::<PlayerControllable>(entity_id) {
    if keyboard::is_key_pressed(context, KeyCode::Space) {
      if controllable.last_fired.elapsed().as_secs() > 1 {
        controllable.last_fired = Instant::now();
        if let Ok(bullet) = build_bullet(context, position.x, position.y, rotation) {
          game.entities.push(bullet);
        }
      }
    }
  }
}

pub fn accelerate(physics: &mut Physicsable, rotation: f32) {
  physics.acceleration += 0.01 * geometry::angle_to_vec(rotation);
}

pub fn decelerate(physics: &mut Physicsable) {
  physics.acceleration *= 0.0;
}

const ROTATION_SPEED: f32 = 3.0;

pub fn turn_left(transform: &mut Transform) {
  let mut rotation = transform.rotation - ROTATION_SPEED;
  if rotation < 0.0 {
    rotation += 360.0;
  }
  transform.rotation = rotation
}

pub fn turn_right(transform: &mut Transform) {
  transform.rotation = (transform.rotation + ROTATION_SPEED) % 360.0;
}
