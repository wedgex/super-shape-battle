use super::System;
use crate::components::Collision;
use crate::components::Transform;
use crate::components::{Collidable, CollisionBounds};
use crate::entity::EntityId;
use crate::world::World;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::translate::Translate;
use ggez::Context;
use ggez::GameResult;
use std::any::TypeId;

pub struct CollisionSystem;

impl System for CollisionSystem {
  fn update(world: &mut World, _context: &mut Context) -> GameResult {
    let old_collisions: Vec<EntityId> = world.entities::<Collision>();
    for old_collision in old_collisions {
      world.remove(&old_collision);
    }

    let mut entities: Vec<EntityId> =
      world.entities_with(vec![TypeId::of::<Collidable>(), TypeId::of::<Transform>()]);

    while let Some(entity1) = entities.pop() {
      for entity2 in &entities {
        let c1_bounds = get_translated_bounds(world, &entity1);
        let c2_bounds = get_translated_bounds(world, &entity2);
        if let (Some(c1_bounds), Some(c2_bounds)) = (c1_bounds, c2_bounds) {
          if overlaps(&c1_bounds, &c2_bounds) {
            add_collision(world, &entity1, &entity2);
          }
        }
      }
    }

    Ok(())
  }
}

fn overlaps(entity1: &CollisionBounds, entity2: &CollisionBounds) -> bool {
  entity1.intersects(entity2)
}

fn get_translated_bounds(world: &World, entity: &EntityId) -> Option<CollisionBounds> {
  let transform = world.get::<Transform>(entity);
  let bounds = world.get::<Collidable>(entity);

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

fn add_collision(world: &mut World, entity1: &EntityId, entity2: &EntityId) {
  let collision = world.create_entity();
  world.add(&collision, Collision::new(*entity1, *entity2));
}
