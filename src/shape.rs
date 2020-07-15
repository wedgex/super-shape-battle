use super::systems::Physics;
use crate::geometry;
use crate::systems::collision::Collision;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};
use std::f32::consts::PI;

const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
const YELLOW: graphics::Color = graphics::Color::new(255.0, 255.0, 0.0, 1.0);
const GREEN: graphics::Color = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

pub struct Shape {
  pub position: Point2<f32>,
  pub velocity: Vector2<f32>,
  pub acceleration: Vector2<f32>,
  points: Vec<Point2<f32>>,
  color: graphics::Color,
}

impl Shape {
  pub fn new(x: f32, y: f32, points: Vec<Point2<f32>>, color: graphics::Color) -> Self {
    let position = Point2::new(x, y);
    let velocity = Vector2::new(0.0, 0.0);
    let acceleration = Vector2::new(0.0, 0.0);

    Shape {
      position,
      velocity,
      acceleration,
      points,
      color,
    }
  }

  pub fn octagon(x: f32, y: f32) -> Self {
    Shape::new(x, y, octagon_points(), RED)
  }

  pub fn hexagon(x: f32, y: f32) -> Self {
    Shape::new(x, y, hexagon_points(), YELLOW)
  }

  pub fn square(x: f32, y: f32) -> Self {
    Shape::new(x, y, square_points(), GREEN)
  }

  pub fn draw(&self, context: &mut Context) -> GameResult {
    let shape = graphics::Mesh::new_polygon(
      context,
      graphics::DrawMode::stroke(2.0),
      &self.points,
      self.color,
    )?;

    graphics::draw(
      context,
      &shape,
      graphics::DrawParam::default().dest(self.position),
    )?;

    Ok(())
  }
}

impl Physics for Shape {
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

fn rotation_transform(point: Point2<f32>, angle: f32) -> Point2<f32> {
  let radians = angle.to_radians();

  Point2::new(
    point.x * radians.cos() - point.y * radians.sin(),
    point.x * radians.sin() + point.y * radians.cos(),
  )
}

fn polygon_points(sides: i32, length: f32, rotation: f32) -> Vec<Point2<f32>> {
  let angle = 2.0 * PI / sides as f32;

  (0..=sides)
    .map(|i| {
      rotation_transform(
        Point2::new(
          length * (angle * i as f32).cos(),
          length * (angle * i as f32).sin(),
        ),
        rotation,
      )
    })
    .collect()
}

fn octagon_points() -> Vec<Point2<f32>> {
  polygon_points(8, 25.0, 70.0)
}

fn hexagon_points() -> Vec<Point2<f32>> {
  polygon_points(6, 20.0, 60.0)
}

fn square_points() -> Vec<Point2<f32>> {
  polygon_points(4, 15.0, 45.0)
}

impl Collision for Shape {
  fn points(&self) -> Vec<Point2<f32>> {
    let mut points = self.points.clone();
    geometry::translate_points(&mut points, self.position);
    points
  }

  fn position(&self) -> Point2<f32> {
    self.position
  }

  fn collision(&mut self) {
    self.color.a -= 0.1;
  }
}
