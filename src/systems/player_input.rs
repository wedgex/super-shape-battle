use crate::components::physicsable::Physicsable;
use crate::components::rotatable::Rotatable;
use crate::game::GameState;
use ggez::event::KeyCode;
use ggez::input::keyboard;
use ggez::nalgebra::Vector2;
use ggez::Context;
use ggez::GameResult;

use super::System;

pub struct PlayerInputSystem;

impl System for PlayerInputSystem {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult {
    let ship = &mut game.ship;
    let rotation = if let Some(rotatatable) = ship.get_component_mut::<Rotatable>() {
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
    if let Some(physics) = ship.get_component_mut::<Physicsable>() {
      if keyboard::is_key_pressed(context, KeyCode::W) {
        accelerate(physics, rotation);
      } else {
        decelerate(physics);
      }
    }

    Ok(())
  }
}

pub fn accelerate(physics: &mut Physicsable, rotation: f32) {
  physics.acceleration +=
    0.01 * Vector2::new(rotation.to_radians().sin(), -rotation.to_radians().cos());
}

pub fn decelerate(physics: &mut Physicsable) {
  physics.acceleration *= 0.0;
}

pub fn turn_left(rotatable: &mut Rotatable) {
  let mut rotation = rotatable.rotation - 1.0;
  if rotation < 0.0 {
    rotation += 360.0;
  }
  rotatable.rotation = rotation
}

pub fn turn_right(rotatable: &mut Rotatable) {
  rotatable.rotation = (rotatable.rotation + 1.0) % 360.0;
}
