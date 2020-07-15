use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

use crate::shape::Shape;
use crate::ship::{Bullet, Ship};
use crate::systems::{collision::CollisionSystem, PhysicsSystem, System};
use std::time::Instant;

const BULLET_TIME_SECS: u64 = 2;

pub struct GameState {
  pub ship: Ship,
  pub shapes: Vec<Shape>,
  pub last_fired: Instant,
  pub bullets: Vec<Bullet>,
}

impl GameState {
  pub fn new() -> GameResult<GameState> {
    let mut octagon = Shape::octagon(100.0, 100.0);
    let mut hexagon = Shape::hexagon(500.0, 500.0);
    let mut square = Shape::square(300.0, 200.0);

    octagon.velocity = Vector2::new(1.0, 1.0);
    hexagon.velocity = Vector2::new(-1.0, -1.0);
    square.velocity = Vector2::new(1.0, -1.0);

    let s = GameState {
      ship: Ship::new(Point2::new(400.0, 400.0)),
      shapes: vec![octagon, hexagon, square],
      last_fired: Instant::now(),
      bullets: vec![],
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

    if keyboard::is_key_pressed(ctx, KeyCode::Space) {
      if self.last_fired.elapsed().as_secs() > 1 {
        let bullet = Bullet::new(
          self.ship.position.x,
          self.ship.position.y,
          self.ship.rotation,
        );

        self.bullets.push(bullet);
        self.last_fired = Instant::now();
      }
    }

    PhysicsSystem::update(self, ctx);
    CollisionSystem::update(self, ctx);

    self.bullets = self
      .bullets
      .drain(0..)
      .filter(|b| b.created_at.elapsed().as_secs() < BULLET_TIME_SECS)
      .collect();

    Ok(())
  }

  fn draw(&mut self, ctx: &mut Context) -> GameResult {
    graphics::clear(ctx, graphics::BLACK);
    self.ship.draw(ctx)?;
    for shape in &self.shapes {
      shape.draw(ctx)?
    }
    for bullet in &self.bullets {
      bullet.draw(ctx)?
    }
    graphics::present(ctx)?;

    Ok(())
  }
}
