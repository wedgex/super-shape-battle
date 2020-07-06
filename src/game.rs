use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use super::shape::Shape;
use super::ship::Ship;
use super::systems::PhysicsSystem;

pub struct GameState {
  ship: Ship,
  shapes: Vec<Shape>,
}

impl GameState {
  pub fn new() -> GameResult<GameState> {
    let s = GameState {
      ship: Ship::new(Point2::new(400.0, 400.0)),
      shapes: vec![Shape::new(Point2::new(100.0, 100.0))],
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

    PhysicsSystem::update(&mut self.ship, ctx);

    println!(
      "position {}; acceleration {}; velocity {}",
      self.ship.position, self.ship.acceleration, self.ship.velocity
    );

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);
    self.ship.draw(ctx)?;
    for shape in &self.shapes {
      shape.draw(ctx)?
    }
    graphics::present(ctx)?;

    Ok(())
  }
}
