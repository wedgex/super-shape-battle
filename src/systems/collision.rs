use super::System;
use crate::components::Damage;
use crate::components::Physicsable;
use crate::components::Transform;
use crate::components::Vulnerable;
use crate::components::{Collidable, CollisionBounds};
use crate::entity::Entity;
use crate::GameState;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::translate::Translate;
use ggez::Context;
use ggez::GameResult;

pub struct CollisionSystem;

impl System for CollisionSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    let entities = game.entities.as_mut_slice();

    let collidable_entities: Vec<usize> = entities
      .iter()
      .enumerate()
      .filter(|(_, e)| e.has_component::<Collidable>() && e.has_component::<Transform>())
      .map(|(i, _)| i)
      .collect();

    for i in collidable_entities {
      let (head, tail) = entities.split_at_mut(i + 1);
      let entity1 = &mut head[i];
      for entity2 in tail {
        let c1_bounds = get_translated_bounds(entity1);
        let c2_bounds = get_translated_bounds(entity2);
        if overlaps(&c1_bounds, &c2_bounds) {
          handle_collision(entity1, entity2);
          handle_collision(entity2, entity1);
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
  let transform = get_transform(entity).unwrap();
  let bounds = get_bounds(entity);

  bounds
    .translate(transform.position.x, transform.position.y)
    .rotate(transform.rotation)
}

fn get_transform(entity: &Entity) -> Option<&Transform> {
  entity.get_component::<Transform>()
}

fn get_bounds(entity: &Entity) -> CollisionBounds {
  entity.get_component::<Collidable>().unwrap().bounds.clone()
}

fn handle_collision(entity1: &mut Entity, entity2: &mut Entity) {
  if is_vulnerable_to(entity1, entity2) {
    entity1.remove_component::<Physicsable>();
  }

  if is_vulnerable_to(entity2, entity1) {
    entity2.remove_component::<Physicsable>();
  }
}

fn is_vulnerable_to(entity1: &Entity, entity2: &Entity) -> bool {
  if let Some(vulnerability) = entity1.get_component::<Vulnerable>() {
    if let Some(damage) = entity2.get_component::<Damage>() {
      if vulnerability.damage_types.contains(&damage.damage_type) {
        return true;
      }
    }
  }
  false
}
