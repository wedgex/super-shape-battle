use crate::systems::CollisionSystem;
use crate::systems::DamageSystem;
use crate::systems::DrawSystem;
use crate::systems::ExpirationSystem;
use crate::systems::PhysicsSystem;
use crate::systems::PlayerInputSystem;
use crate::systems::ShapeManager;
use crate::systems::ShipManager;
use crate::systems::System;
use crate::world::World;
use ggez::event;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

pub struct GameState {
  pub world: World,
}

impl GameState {
  pub fn new() -> Self {
    GameState {
      world: World::new(),
    }
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    PlayerInputSystem::update(&mut self.world, ctx)?;
    PhysicsSystem::update(&mut self.world, ctx)?;
    ExpirationSystem::update(&mut self.world, ctx)?;
    CollisionSystem::update(&mut self.world, ctx)?;
    DamageSystem::update(&mut self.world, ctx)?;
    ShipManager::update(&mut self.world, ctx)?;
    ShapeManager::update(&mut self.world, ctx)?;

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);

    DrawSystem::update(&mut self.world, ctx)?;

    graphics::present(ctx)?;

    Ok(())
  }
}
