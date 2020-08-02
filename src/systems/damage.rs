use super::System;
use crate::components::Collision;
use crate::components::Damage;
use crate::components::Damaged;
use crate::components::Vulnerable;
use crate::entity::Entity;
use crate::entity::EntityId;
use crate::game::GameState;
use ggez::Context;
use ggez::GameResult;

pub struct DamageSystem;

impl System for DamageSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    game.entities.retain(|e| !e.has_component::<Damaged>());

    let mut damaged: Vec<Damaged> = vec![];
    let collisions: Vec<(EntityId, EntityId)> = game
      .get_components::<Collision>()
      .into_iter()
      .map(|c| (c.entity1, c.entity2))
      .collect();

    for (e1, e2) in collisions {
      if is_damaged_by(game, e1, e2) {
        damaged.push(Damaged::new(e1));
      }

      if is_damaged_by(game, e2, e1) {
        damaged.push(Damaged::new(e2));
      }
    }

    for d in damaged {
      let mut e = Entity::new();
      e.register_component(d);
      game.entities.push(e);
    }

    Ok(())
  }
}

fn is_damaged_by(game: &mut GameState, entity1: EntityId, entity2: EntityId) -> bool {
  let damage = game.get_component::<Damage>(entity2);
  let vulnerability = game.get_component::<Vulnerable>(entity1);

  if let (Some(damage), Some(vulnerability)) = (damage, vulnerability) {
    if vulnerability.damage_types.contains(&damage.damage_type) {
      return true;
    }
  }

  false
}
