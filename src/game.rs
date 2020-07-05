use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};

use super::shape::Shape;
use super::ship::Ship;

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

    handle_acceleration(&mut self.ship);
    handle_velocity(&mut self.ship);
    wrap_position(&mut self.ship, ctx);

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

fn wrap_position(ship: &mut Ship, context: &Context) {
  let (screen_width, screen_height) = graphics::drawable_size(context);

  if ship.position.x < 0.0 {
    ship.position.x += screen_width;
  }

  if ship.position.x > screen_width {
    ship.position.x -= screen_width;
  }

  if ship.position.y < 0.0 {
    ship.position.y += screen_height;
  }

  if ship.position.y > screen_height {
    ship.position.y -= screen_height;
  }
}
