use super::System;
use crate::components::Damaged;
use crate::components::{Tag, TagType};
use crate::entity::EntityId;
use crate::ship;
use crate::world::World;
use ggez::{Context, GameResult};

pub struct ShipManager;

impl System for ShipManager {
  fn update(world: &mut World, ctx: &mut Context) -> GameResult {
    let damaged_ships: Vec<EntityId> = world
      .components::<Damaged>()
      .into_iter()
      .map(|e| e.entity.clone())
      .filter(|e| {
        if let Some(tag) = world.get::<Tag>(e) {
          return tag.tag_type == TagType::Ship;
        }
        false
      })
      .collect();

    for entity in damaged_ships.iter() {
      world.remove(entity);
    }

    // display death animation

    // delay

    // add new ship
    if damaged_ships.len() > 0 {
      ship::build_ship(world, ctx)?;
    }

    Ok(())
  }
}
