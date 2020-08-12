use crate::components::Drawable;
use crate::components::Transform;
use crate::entity::EntityId;
use crate::world::World;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;
use std::any::TypeId;

use super::System;

pub struct DrawSystem;

impl System for DrawSystem {
  fn update(world: &mut World, context: &mut Context) -> GameResult {
    let drawables = world.entities_with(vec![TypeId::of::<Drawable>(), TypeId::of::<Transform>()]);

    for drawable in drawables {
      draw(world, &drawable, context)?
    }

    Ok(())
  }
}

fn draw(world: &World, entity: &EntityId, context: &mut Context) -> GameResult {
  let drawable = world.get::<Drawable>(entity);
  let transform = world.get::<Transform>(entity);

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
