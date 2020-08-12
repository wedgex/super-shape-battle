use super::System;
use crate::components::Expirable;
use crate::entity::EntityId;
use crate::world::World;
use ggez::Context;
use ggez::GameResult;

pub struct ExpirationSystem;

impl System for ExpirationSystem {
  fn update(world: &mut World, _context: &mut Context) -> GameResult {
    let expired: Vec<EntityId> = world
      .entities::<Expirable>()
      .into_iter()
      .filter(|e| !should_keep(world, &e))
      .collect();

    world.remove_all(expired);

    Ok(())
  }
}

fn should_keep(world: &World, entity: &EntityId) -> bool {
  world
    .get::<Expirable>(entity)
    .map_or(true, |e| !e.is_expired())
}
