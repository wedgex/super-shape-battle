use super::System;
use crate::components::Expirable;
use crate::entity::Entity;
use crate::game::GameState;
use ggez::Context;
use ggez::GameResult;

pub struct ExpirationSystem;

impl System for ExpirationSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    game.entities.retain(|e| should_keep(e));

    Ok(())
  }
}

fn should_keep(entity: &Entity) -> bool {
  if let Some(expirable) = entity.get_component::<Expirable>() {
    if expirable.created.elapsed() > expirable.expiration {
      return false;
    }
  }
  true
}
