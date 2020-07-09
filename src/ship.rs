use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use std::time::Instant;

use super::systems::Physics;

pub struct Ship {
  pub position: Point2<f32>,
  pub rotation: f32,
  pub velocity: Vector2<f32>,
  pub acceleration: Vector2<f32>,
}

impl Ship {
  pub fn new(position: Point2<f32>) -> Self {
    let velocity = Vector2::new(0.0, 0.0);
    let acceleration = Vector2::new(0.0, 0.0);

    Ship {
      position,
      rotation: 0.0,
      velocity,
      acceleration,
    }
  }

  pub fn draw(&self, context: &mut Context) -> GameResult {
    let w = 25.0;
    let h = 30.0;

    let ship = graphics::Mesh::new_polygon(
      context,
      graphics::DrawMode::stroke(2.0),
      &[
        Point2::new(0.0, h),
        Point2::new(w / 2.0, 0.0),
        Point2::new(w, h),
        Point2::new(w / 2.0, h - (h / 3.0)),
      ],
      graphics::WHITE,
    )?;

    graphics::draw(
      context,
      &ship,
      graphics::DrawParam::default()
        .rotation(self.rotation)
        .offset(Point2::new(w / 2.0, h / 2.0))
        .dest(self.position),
    )?;

    Ok(())
  }

  pub fn accelerate(&mut self) {
    self.acceleration += 0.01 * Vector2::new(self.rotation.sin(), -self.rotation.cos());
  }

  pub fn decelerate(&mut self) {
    self.acceleration *= 0.0;
  }

  // turning occasionally seems jumpy
  pub fn turn_left(&mut self) {
    let mut rotation = self.rotation - 0.1;
    if rotation < 0.0 {
      rotation += 360.0;
    }
    self.rotation = rotation;
  }

  pub fn turn_right(&mut self) {
    self.rotation = (self.rotation + 0.1) % 360.0;
  }
}

impl Physics for Ship {
  fn get_position(&self) -> Point2<f32> {
    self.position
  }

  fn get_acceleration(&self) -> Vector2<f32> {
    self.acceleration
  }

  fn get_velocity(&self) -> Vector2<f32> {
    self.velocity
  }

  fn set_velocity(&mut self, velcoity: Vector2<f32>) {
    self.velocity = velcoity;
  }

  fn move_to(&mut self, position: Point2<f32>) {
    self.position = position;
  }
}

pub struct Bullet {
  pub position: Point2<f32>,
  pub velocity: Vector2<f32>,
  pub created_at: Instant,
}

impl Bullet {
  pub fn new(x: f32, y: f32, angle: f32) -> Self {
    let position = Point2::new(x, y);
    let velocity = 5.0 * Vector2::new(angle.sin(), -angle.cos());
    let created_at = Instant::now();

    Bullet {
      position,
      velocity,
      created_at,
    }
  }

  pub fn draw(&self, context: &mut Context) -> GameResult {
    let bullet = graphics::Mesh::new_circle(
      context,
      graphics::DrawMode::stroke(2.0),
      Point2::new(0., 0.),
      5.0,
      1.0,
      graphics::WHITE,
    )?;

    graphics::draw(
      context,
      &bullet,
      graphics::DrawParam::default().dest(self.position),
    )?;

    Ok(())
  }
}

impl Physics for Bullet {
  fn get_position(&self) -> Point2<f32> {
    self.position
  }

  fn get_acceleration(&self) -> Vector2<f32> {
    Vector2::new(0.0, 0.0)
  }

  fn get_velocity(&self) -> Vector2<f32> {
    self.velocity
  }

  fn set_velocity(&mut self, velcoity: Vector2<f32>) {
    self.velocity = velcoity;
  }

  fn move_to(&mut self, position: Point2<f32>) {
    self.position = position;
  }
}
