use super::System;
use crate::components::Collision;
use crate::components::Damage;
use crate::components::Damaged;
use crate::components::Vulnerable;
use crate::entity::EntityId;
use crate::world::World;
use ggez::Context;
use ggez::GameResult;

pub struct DamageSystem;

impl System for DamageSystem {
  fn update(world: &mut World, _context: &mut Context) -> GameResult {
    world.remove_all(world.entities::<Damaged>());

    let mut damaged: Vec<Damaged> = vec![];
    let collisions: Vec<(EntityId, EntityId)> = world
      .components::<Collision>()
      .into_iter()
      .map(|c| (c.entity1, c.entity2))
      .collect();

    for (e1, e2) in collisions {
      if is_damaged_by(world, &e1, &e2) {
        damaged.push(Damaged::new(e1));
      }

      if is_damaged_by(world, &e2, &e1) {
        damaged.push(Damaged::new(e2));
      }
    }

    for d in damaged {
      let e = world.create_entity();
      world.add(&e, d);
    }

    Ok(())
  }
}

fn is_damaged_by(world: &mut World, entity1: &EntityId, entity2: &EntityId) -> bool {
  let damage = world.get::<Damage>(entity2);
  let vulnerability = world.get::<Vulnerable>(entity1);

  if let (Some(damage), Some(vulnerability)) = (damage, vulnerability) {
    if vulnerability.damage_types.contains(&damage.damage_type) {
      return true;
    }
  }

  false
}
