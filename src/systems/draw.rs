use crate::components::Drawable;
use crate::components::Transform;
use crate::entity::Entity;
use crate::game::GameState;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use super::System;

pub struct DrawSystem;

impl System for DrawSystem {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult {
    for entity in &mut game.entities {
      draw(entity, context)?
    }

    Ok(())
  }
}

fn draw(entity: &mut Entity, context: &mut Context) -> GameResult {
  if let (Some(drawable), Some(transform)) = (get_drawable(entity), get_transform(entity)) {
    graphics::draw(
      context,
      &drawable.mesh,
      graphics::DrawParam::default()
        .dest(transform.position.clone())
        .rotation(transform.rotation.to_radians())
        .offset(drawable.offset),
    )?;
  }

  Ok(())
}

fn get_drawable(entity: &Entity) -> Option<&Drawable> {
  if let Some(drawable) = entity.get_component::<Drawable>() {
    return Some(drawable);
  }
  None
}

fn get_transform(entity: &Entity) -> Option<&Transform> {
  if let Some(transform) = entity.get_component::<Transform>() {
    return Some(transform);
  }
  None
}
