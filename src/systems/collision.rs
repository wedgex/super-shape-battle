use super::System;
use crate::components::Collision;
use crate::components::Tag;
use crate::components::Transform;
use crate::components::{Collidable, CollisionBounds};
use crate::entity::Entity;
use crate::entity::EntityId;
use crate::GameState;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::translate::Translate;
use ggez::Context;
use ggez::GameResult;

pub struct CollisionSystem;

impl System for CollisionSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    game.entities.retain(|e| !e.has_component::<Collision>());

    let entities = game.entities.as_slice();

    let collidable_entities: Vec<usize> = entities
      .iter()
      .enumerate()
      .filter(|(_, e)| e.has_component::<Collidable>() && e.has_component::<Transform>())
      .map(|(i, _)| i)
      .collect();

    let mut collisions: Vec<Entity> = vec![];

    for i in collidable_entities {
      let (head, tail) = entities.split_at(i + 1);
      let entity1 = head[i].id;
      for entity2 in tail {
        let entity2 = entity2.id;
        let c1_bounds = get_translated_bounds(game, entity1);
        let c2_bounds = get_translated_bounds(game, entity2);
        if let (Some(c1_bounds), Some(c2_bounds)) = (c1_bounds, c2_bounds) {
          if overlaps(&c1_bounds, &c2_bounds) {
            collisions.push(collision(entity1, entity2));
          }
        }
      }
    }

    game.entities.extend(collisions.into_iter());

    Ok(())
  }
}

fn overlaps(entity1: &CollisionBounds, entity2: &CollisionBounds) -> bool {
  entity1.intersects(entity2)
}

fn get_translated_bounds(game: &GameState, entity: EntityId) -> Option<CollisionBounds> {
  let transform = game.get_component::<Transform>(entity);
  let bounds = game.get_component::<Collidable>(entity);

  if let (Some(transform), Some(bounds)) = (transform, bounds) {
    let bounds = bounds.bounds.clone();

    return Some(
      bounds
        .translate(transform.position.x, transform.position.y)
        .rotate(transform.rotation),
    );
  }
  None
}

fn collision(entity1: EntityId, entity2: EntityId) -> Entity {
  let mut collision = Entity::new();
  collision.register_component(Collision::new(entity1, entity2));
  collision
}
