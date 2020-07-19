use crate::components::drawable::Drawable;
use crate::components::positionable::Positionable;
use crate::components::rotatable::Rotatable;
use crate::components::Entity;
use crate::game::GameState;
use crate::geometry::rotation_transform;
use ggez::graphics;
use ggez::graphics::Mesh;
use ggez::nalgebra::Point2;
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
  if let (Some(drawable), Some(pos)) = (get_drawable(entity), get_position(entity)) {
    let rotation = get_rotation(entity);

    let vertices: Vec<Point2<f32>> = drawable
      .vertices
      .iter()
      .map(|v| rotation_transform(v, rotation))
      .collect();

    let mesh = Mesh::new_polygon(context, drawable.draw_mode, &vertices, drawable.color)?;

    graphics::draw(
      context,
      &mesh,
      graphics::DrawParam::default().dest(pos.clone()),
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

fn get_position(entity: &Entity) -> Option<&Point2<f32>> {
  if let Some(psoitionable) = entity.get_component::<Positionable>() {
    return Some(&psoitionable.position);
  }
  None
}

fn get_rotation(entity: &Entity) -> f32 {
  if let Some(rotatable) = entity.get_component::<Rotatable>() {
    rotatable.rotation
  } else {
    0.
  }
}
