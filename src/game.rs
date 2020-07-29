use crate::systems::CollisionSystem;
use crate::systems::DrawSystem;
use crate::systems::ExpirationSystem;
use crate::systems::PhysicsSystem;
use crate::systems::PlayerInputSystem;
use ggez::event;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use crate::entity::Entity;
use crate::shape::{hexagon, octagon, square};
use crate::ship::build_ship;
use crate::systems::System;

pub struct GameState {
  pub entities: Vec<Entity>,
}

impl GameState {
  pub fn new(context: &mut Context) -> GameResult<GameState> {
    let octagon = octagon(context, 100.0, 100.0)?;
    let hexagon = hexagon(context, 500.0, 500.0)?;
    let square = square(context, 300.0, 200.0)?;

    let ship = build_ship(context)?;

    let s = GameState {
      entities: vec![ship, octagon, hexagon, square],
    };

    Ok(s)
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    PlayerInputSystem::update(self, ctx)?;
    PhysicsSystem::update(self, ctx)?;
    ExpirationSystem::update(self, ctx)?;
    CollisionSystem::update(self, ctx)?;

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);

    DrawSystem::update(self, ctx)?;

    graphics::present(ctx)?;

    Ok(())
  }
}
