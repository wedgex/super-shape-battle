use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

use super::shape::Shape;
use super::ship::Ship;
use super::systems::{PhysicsSystem, System};

pub struct GameState {
  pub ship: Ship,
  pub shapes: Vec<Shape>,
}

impl GameState {
  pub fn new() -> GameResult<GameState> {
    let mut octagon = Shape::octagon(Point2::new(100.0, 100.0));
    let mut hexagon = Shape::hexagon(Point2::new(500.0, 500.0));

    octagon.velocity = Vector2::new(1.0, 1.0);
    hexagon.velocity = Vector2::new(-1.0, -1.0);

    let s = GameState {
      ship: Ship::new(Point2::new(400.0, 400.0)),
      shapes: vec![octagon, hexagon],
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

    PhysicsSystem::update(self, ctx);

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
