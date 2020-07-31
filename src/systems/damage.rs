use super::System;
use crate::components::Collision;
use crate::components::Damage;
use crate::components::Physicsable;
use crate::components::Vulnerable;
use crate::entity::EntityId;
use crate::game::GameState;
use ggez::Context;
use ggez::GameResult;

pub struct DamageSystem;

impl System for DamageSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    let collisions: Vec<(EntityId, EntityId)> = game
      .entities_with::<Collision>()
      .into_iter()
      .filter_map(|e| e.get_component::<Collision>())
      .map(|c| (c.entity1, c.entity2))
      .collect();

    for (e1, e2) in collisions {
      handle_collision(game, e1, e2);
    }

    Ok(())
  }
}

fn handle_collision(game: &mut GameState, entity1: EntityId, entity2: EntityId) {
  if is_vulnerable_to(game, entity1, entity2) {
    if let Some(e) = game.get_entity_mut(entity1) {
      e.remove_component::<Physicsable>();
    }
  }

  if is_vulnerable_to(game, entity2, entity1) {
    if let Some(e) = game.get_entity_mut(entity2) {
      e.remove_component::<Physicsable>();
    }
  }
}

fn is_vulnerable_to(game: &mut GameState, entity1: EntityId, entity2: EntityId) -> bool {
  let damage = game.get_component::<Damage>(entity2);
  let vulnerability = game.get_component::<Vulnerable>(entity1);

  if let (Some(damage), Some(vulnerability)) = (damage, vulnerability) {
    if vulnerability.damage_types.contains(&damage.damage_type) {
      return true;
    }
  }

  false
}
