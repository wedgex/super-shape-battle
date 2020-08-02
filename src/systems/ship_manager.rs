use super::System;
use crate::components::Damaged;
use crate::components::{Tag, TagType};
use crate::entity::EntityId;
use crate::game::GameState;
use crate::ship;
use ggez::{Context, GameResult};

pub struct ShipManager;

impl System for ShipManager {
  fn update(game: &mut GameState, ctx: &mut Context) -> GameResult {
    let ships: Vec<EntityId> = game
      .entities_with::<Tag>()
      .into_iter()
      .filter(|e| {
        if let Some(tag) = e.get_component::<Tag>() {
          return tag.tag_type == TagType::Ship;
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

    let damaged_ships: Vec<EntityId> = ships
      .into_iter()
      .filter(|eid| damaged.contains(eid))
      .collect();

    // remove ship
    game.entities.retain(|e| !damaged_ships.contains(&e.id));

    // display death animation

    // delay

    // add new ship
    for _ in damaged_ships {
      game.entities.push(ship::build_ship(ctx)?);
    }

    Ok(())
  }
}
