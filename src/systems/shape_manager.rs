use super::System;
use crate::components::Damaged;
use crate::components::Transform;
use crate::components::{Tag, TagType};
use crate::entity::Entity;
use crate::entity::EntityId;
use crate::game::GameState;
use crate::shape;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

pub struct ShapeManager;

impl System for ShapeManager {
  fn update(game: &mut GameState, ctx: &mut Context) -> GameResult {
    let shapes: Vec<EntityId> = game
      .entities_with::<Tag>()
      .into_iter()
      .filter(|e| {
        if let Some(tag) = e.get_component::<Tag>() {
          if let TagType::Shape(_) = tag.tag_type {
            return true;
          };
        }
        false
      })
      .map(|d| d.id)
      .collect();

    let damaged: Vec<EntityId> = game
      .get_components::<Damaged>()
      .iter()
      .map(|d| d.entity_id)
      .collect();

    let damaged_shapes: Vec<EntityId> = shapes
      .into_iter()
      .filter(|eid| damaged.contains(eid))
      .collect();

    for eid in damaged_shapes {
      let tag = game.get_component::<Tag>(eid).map(|t| t.tag_type.clone());
      let transform = game.get_component::<Transform>(eid);
      if let (Some(TagType::Shape(level)), Some(transform)) = (tag, transform) {
        if let Some(shape) = build_shape(level - 1, transform.position, ctx) {
          game.entities.push(shape);
        }
      }
    }

    game.entities.retain(|e| !damaged.contains(&e.id));

    Ok(())
  }
}

fn build_shape(level: u8, position: Point2<f32>, context: &mut Context) -> Option<Entity> {
  match level {
    2 => shape::hexagon(context, position.x, position.y)
      .map(Some)
      .unwrap_or(None),
    1 => shape::square(context, position.x, position.y)
      .map(Some)
      .unwrap_or(None),
    _ => None,
  }
}
