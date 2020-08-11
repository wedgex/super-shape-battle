use super::System;
use crate::components::Damaged;
use crate::components::Transform;
use crate::components::{Tag, TagType};
use crate::entity::EntityId;
use crate::shape;
use crate::world::World;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

pub struct ShapeManager;

impl System for ShapeManager {
  fn update(world: &mut World, ctx: &mut Context) -> GameResult {
    let is_shape = |e: &EntityId| {
      if let Some(tag) = world.get::<Tag>(e) {
        if let TagType::Shape(_) = tag.tag_type {
          return true;
        };
      }
      false
    };

    // TODO this either needs some sort of query or to be moved to an event system
    let damaged_shapes: Vec<EntityId> = world
      .components::<Damaged>()
      .into_iter()
      .map(|d| d.entity_id.clone())
      .filter(is_shape)
      .collect();

    for eid in damaged_shapes {
      let tag = world.get::<Tag>(&eid).map(|t| t.tag_type.clone());
      let transform = world.get::<Transform>(&eid).map(|t| t.position.clone());
      if let (Some(TagType::Shape(level)), Some(position)) = (tag, transform) {
        build_shape(world, level - 1, position, ctx)?;
      }
      world.remove(&eid);
    }

    Ok(())
  }
}

fn build_shape(
  world: &mut World,
  level: u8,
  position: Point2<f32>,
  context: &mut Context,
) -> GameResult<()> {
  match level {
    2 => shape::hexagon(world, context, position.x, position.y),
    1 => shape::square(world, context, position.x, position.y),
    _ => Ok(()),
  }
}
