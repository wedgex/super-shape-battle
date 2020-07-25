use super::System;
use crate::components::Positionable;
use crate::components::Rotatable;
use crate::components::{Collidable, CollisionBounds};
use crate::entity::Entity;
use crate::GameState;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::translate::Translate;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;

pub struct CollisionSystem;

impl System for CollisionSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    let entities = game.entities.as_mut_slice();

    let collidable_entities: Vec<usize> = entities
      .iter()
      .enumerate()
      .filter(|(_, e)| e.has_component::<Collidable>() && e.has_component::<Positionable>())
      .map(|(i, _)| i)
      .collect();

    for i in collidable_entities.iter().cloned() {
      // clear previous collisions
      let entity = &mut entities[i];
      entity.get_component_mut::<Collidable>().unwrap().colliding = vec![];
    }

    for i in collidable_entities {
      let (head, tail) = entities.split_at_mut(i + 1);
      let entity1 = &mut head[i];
      for entity2 in tail {
        let eid1 = entity1.id.clone();
        let eid2 = entity2.id.clone();
        let c1_bounds = get_translated_bounds(entity1);
        let c2_bounds = get_translated_bounds(entity2);
        let c1 = entity1.get_component_mut::<Collidable>().unwrap();
        let c2 = entity2.get_component_mut::<Collidable>().unwrap();
        if overlaps(&c1_bounds, &c2_bounds) {
          c1.colliding.push(eid2);
          c2.colliding.push(eid1);
        }
      }
    }

    Ok(())
  }
}

fn overlaps(entity1: &CollisionBounds, entity2: &CollisionBounds) -> bool {
  entity1.intersects(entity2)
}

fn get_translated_bounds(entity: &Entity) -> CollisionBounds {
  let position = get_position(entity);
  let rotation = get_rotation(entity);
  let bounds = get_bounds(entity);

  bounds.translate(position.x, position.y).rotate(rotation)
}

fn get_position(entity: &Entity) -> Point2<f32> {
  if let Some(p) = entity.get_component::<Positionable>() {
    p.position
  } else {
    Point2::new(0.0f32, 0.0f32)
  }
}

fn get_rotation(entity: &Entity) -> f32 {
  if let Some(r) = entity.get_component::<Rotatable>() {
    r.rotation
  } else {
    0.
  }
}

fn get_bounds(entity: &Entity) -> CollisionBounds {
  entity.get_component::<Collidable>().unwrap().bounds.clone()
}
