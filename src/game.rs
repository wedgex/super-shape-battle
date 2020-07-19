use crate::systems::draw::DrawSystem;
use crate::systems::expiration::ExpirationSystem;
use crate::systems::physics::PhysicsSystem;
use crate::systems::player_input::PlayerInputSystem;
use ggez::event;
use ggez::graphics;
use ggez::Context;
use ggez::GameResult;

use crate::components::Entity;
use crate::shape::{hexagon, octagon, square};
use crate::ship::build_ship;
use crate::systems::System;
use std::time::Instant;

pub struct GameState {
  pub entities: Vec<Entity>,
  pub last_fired: Instant,
}

impl GameState {
  pub fn new(context: &mut Context) -> GameResult<GameState> {
    let octagon = octagon(100.0, 100.0);
    let hexagon = hexagon(500.0, 500.0);
    let square = square(300.0, 200.0);

    let ship = build_ship();

    let s = GameState {
      entities: vec![ship, octagon, hexagon, square],
      last_fired: Instant::now(),
    };

    Ok(s)
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    PlayerInputSystem::update(self, ctx)?;
    PhysicsSystem::update(self, ctx)?;
    ExpirationSystem::update(self, ctx)?;

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);

    DrawSystem::update(self, ctx)?;

    graphics::present(ctx)?;

    Ok(())
  }
}
