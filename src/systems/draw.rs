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
    let drawables: Vec<&Entity> = game.entities_with::<Drawable>().into_iter().collect();

    for drawable in drawables {
      draw(drawable, context)?
    }

    Ok(())
  }
}

fn draw(entity: &Entity, context: &mut Context) -> GameResult {
  let drawable = entity.get_component::<Drawable>();
  let transform = entity.get_component::<Transform>();

  if let (Some(drawable), Some(transform)) = (drawable, transform) {
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
