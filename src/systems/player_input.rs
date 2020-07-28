use crate::components::Physicsable;
use crate::components::PlayerControllable;
use crate::components::Positionable;
use crate::entity::Entity;
use crate::game::GameState;
use crate::geometry;
use crate::ship::build_bullet;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

use super::System;

pub struct PlayerInputSystem;

impl System for PlayerInputSystem {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult {
    let mut bullet: Option<Entity> = None;

    for entity in game.entities.iter_mut() {
      if let Some(_) = entity.get_component::<PlayerControllable>() {
        apply_inputs_to(entity, context);
        bullet = handle_fire(entity, context);
      }
    }

    if let Some(b) = bullet {
      game.entities.push(b);
    }

    Ok(())
  }
}

fn apply_inputs_to(entity: &mut Entity, context: &mut Context) {
  let rotation = if let Some(rotatatable) = entity.get_component_mut::<Positionable>() {
    if keyboard::is_key_pressed(context, KeyCode::A) {
      turn_left(rotatatable);
    }
    if keyboard::is_key_pressed(context, KeyCode::D) {
      turn_right(rotatatable);
    }

    rotatatable.rotation.clone()
  } else {
    0.
  };
  if let Some(physics) = entity.get_component_mut::<Physicsable>() {
    if keyboard::is_key_pressed(context, KeyCode::W) {
      accelerate(physics, rotation);
    } else {
      decelerate(physics);
    }
  }
}

fn handle_fire(entity: &mut Entity, context: &mut Context) -> Option<Entity> {
  let position = entity.get_component::<Positionable>()?.position;

  let rotation = if let Some(rotatable) = entity.get_component::<Positionable>() {
    rotatable.rotation
  } else {
    0.
  };

  if let Some(controllable) = entity.get_component_mut::<PlayerControllable>() {
    if keyboard::is_key_pressed(context, KeyCode::Space) {
      if controllable.last_fired.elapsed().as_secs() > 1 {
        controllable.last_fired = Instant::now();
        if let Ok(bullet) = build_bullet(context, position.x, position.y, rotation) {
          return Some(bullet);
        }
      }
    }
  }

  None
}

pub fn accelerate(physics: &mut Physicsable, rotation: f32) {
  physics.acceleration += 0.01 * geometry::angle_to_vec(rotation);
}

pub fn decelerate(physics: &mut Physicsable) {
  physics.acceleration *= 0.0;
}

const ROTATION_SPEED: f32 = 3.0;

pub fn turn_left(rotatable: &mut Positionable) {
  let mut rotation = rotatable.rotation - ROTATION_SPEED;
  if rotation < 0.0 {
    rotation += 360.0;
  }
  rotatable.rotation = rotation
}

pub fn turn_right(rotatable: &mut Positionable) {
  rotatable.rotation = (rotatable.rotation + ROTATION_SPEED) % 360.0;
}
