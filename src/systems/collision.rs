use super::System;
use crate::components::Bullet;
use crate::components::Expirable;
use crate::components::Physicsable;
use crate::components::PlayerControllable;
use crate::components::Positionable;
use crate::components::Rotatable;
use crate::components::Shape;
use crate::components::Ship;
use crate::components::{Collidable, CollisionBounds};
use crate::entity::Entity;
use crate::GameState;
use geo::algorithm::intersects::Intersects;
use geo::algorithm::rotate::Rotate;
use geo::algorithm::translate::Translate;
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;
use std::time::Duration;

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

fn handle_collision(entity1: &mut Entity, entity2: &mut Entity) {
  if is_ship(entity1) {
    if is_shape(entity2) {
      // destroy the ship
      entity1.remove_component::<PlayerControllable>();
      entity1.remove_component::<Physicsable>();
      // add an explosion at the player position
    }
  }

  if is_bullet(entity1) {
    if is_shape(entity2) {
      // destroy the shape & bullet
      entity1.remove_component::<Expirable>();
      entity1.register_component(Expirable::new(Duration::from_secs(0)));
      entity2.register_component(Expirable::new(Duration::from_secs(0)));
    }
  }
}

fn is_ship(entity: &Entity) -> bool {
  entity.has_component::<Ship>()
}

fn is_shape(entity: &Entity) -> bool {
  entity.has_component::<Shape>()
}

fn is_bullet(entity: &Entity) -> bool {
  entity.has_component::<Bullet>()
}
