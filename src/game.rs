use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use super::ship::Ship;

pub struct GameState {
  ship: Ship,
}

impl GameState {
  pub fn new() -> GameResult<GameState> {
    let s = GameState {
      ship: Ship::new(Point2::new(400.0, 400.0)),
    };

    Ok(s)
  }
}

impl event::EventHandler for GameState {
  fn update(&mut self, ctx: &mut Context) -> GameResult {
    if keyboard::is_key_pressed(ctx, KeyCode::A) {
      self.ship.turn_left();
    }

    if keyboard::is_key_pressed(ctx, KeyCode::D) {
      self.ship.turn_right();
    }

    if keyboard::is_key_pressed(ctx, KeyCode::W) {
      self.ship.accelerate();
    } else {
      self.ship.decelerate();
    }

    handle_acceleration(&mut self.ship);
    handle_velocity(&mut self.ship);

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);
    self.ship.draw(ctx)?;
    graphics::present(ctx)?;

    Ok(())
  }
}

const MAX_VELOCITY: f32 = 5.0;

fn handle_acceleration(ship: &mut Ship) {
  ship.velocity += ship.acceleration;
  if ship.velocity.norm_squared() > MAX_VELOCITY.powi(2) {
    ship.velocity = ship.velocity / ship.velocity.norm_squared().sqrt() * MAX_VELOCITY;
  }
}

fn handle_velocity(ship: &mut Ship) {
  ship.position += ship.velocity;
}
