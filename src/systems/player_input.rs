use crate::components::Physicsable;
use crate::components::PlayerControllable;
use crate::components::Transform;
use crate::entity::{Bullet, EntityId};
use crate::geometry;
use crate::world::World;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use std::time::Instant;

use super::System;

pub struct PlayerInputSystem;

impl System for PlayerInputSystem {
  fn update(world: &mut World, context: &mut Context) -> GameResult {
    let entities: Vec<EntityId> = world.entities::<PlayerControllable>();

    for entity in entities {
      apply_inputs_to(world, context, &entity);
      handle_fire(world, context, &entity)?;
    }

    Ok(())
  }
}

fn apply_inputs_to(world: &mut World, context: &mut Context, entity: &EntityId) {
  let rotation = if let Some(transform) = world.get_mut::<Transform>(entity) {
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

  if let Some(physics) = world.get_mut::<Physicsable>(entity) {
    if keyboard::is_key_pressed(context, KeyCode::W) {
      accelerate(physics, rotation);
    } else {
      decelerate(physics);
    }
  }
}

fn handle_fire(world: &mut World, context: &mut Context, entity: &EntityId) -> GameResult {
  let transform = world.get::<Transform>(entity);
  let position = transform.map(|t| t.position).unwrap_or(Point2::new(0., 0.));
  let rotation = transform.map(|t| t.rotation).unwrap_or(0.0);

  if let Some(controllable) = world.get_mut::<PlayerControllable>(entity) {
    if keyboard::is_key_pressed(context, KeyCode::Space) {
      if controllable.last_fired.elapsed().as_secs() > 1 {
        controllable.last_fired = Instant::now();
        Bullet::create(world, context, position.x, position.y, rotation)?;
      }
    }
  }

  Ok(())
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
