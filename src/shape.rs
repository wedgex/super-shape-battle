use super::systems::Physics;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
const YELLOW: graphics::Color = graphics::Color::new(255.0, 255.0, 0.0, 1.0);
const GREEN: graphics::Color = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

pub struct Shape {
  pub position: Point2<f32>,
  pub velocity: Vector2<f32>,
  pub acceleration: Vector2<f32>,
  polygon_points: Vec<Point2<f32>>,
  height: f32,
  width: f32,
  color: graphics::Color,
}

impl Shape {
  pub fn new(
    x: f32,
    y: f32,
    height: f32,
    width: f32,
    polygon_points: Vec<Point2<f32>>,
    color: graphics::Color,
  ) -> Self {
    let position = Point2::new(x, y);
    let velocity = Vector2::new(0.0, 0.0);
    let acceleration = Vector2::new(0.0, 0.0);

    Shape {
      position,
      velocity,
      acceleration,
      height,
      width,
      polygon_points,
      color,
    }
  }

  pub fn octagon(x: f32, y: f32) -> Self {
    Shape::new(x, y, OCTAGON_HEIGHT, OCTAGON_WIDTH, octagon_points(), RED)
  }

  pub fn hexagon(x: f32, y: f32) -> Self {
    Shape::new(
      x,
      y,
      HEXAGON_HEIGHT,
      HEXAGON_WIDTH,
      hexagon_points(),
      YELLOW,
    )
  }

  pub fn square(x: f32, y: f32) -> Self {
    Shape::new(x, y, SQUARE_HEIGHT, SQUARE_WIDTH, square_points(), GREEN)
  }

  pub fn draw(&self, context: &mut Context) -> GameResult {
    let shape = graphics::Mesh::new_polygon(
      context,
      graphics::DrawMode::stroke(2.0),
      &self.polygon_points,
      self.color,
    )?;

    graphics::draw(
      context,
      &shape,
      graphics::DrawParam::default()
        .offset(Point2::new(self.width / 2.0, self.height / 2.0))
        .dest(self.position),
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

const OCTAGON_WIDTH: f32 = 60.355;
const OCTAGON_HEIGHT: f32 = 60.355;

fn octagon_points() -> Vec<Point2<f32>> {
  let side_length: f32 = 25.0;
  let triangle_offset = (side_length.powi(2) / 2.0).sqrt();

  let w = side_length + 2.0 * triangle_offset;
  let h = side_length + 2.0 * triangle_offset;

  vec![
    Point2::new(0.0, triangle_offset),
    Point2::new(triangle_offset, 0.0),
    Point2::new(triangle_offset + side_length, 0.0),
    Point2::new(w, triangle_offset),
    Point2::new(w, triangle_offset + side_length),
    Point2::new(triangle_offset + side_length, h),
    Point2::new(triangle_offset, h),
    Point2::new(0.0, triangle_offset + side_length),
  ]
}

const HEXAGON_HEIGHT: f32 = 20.0;
const HEXAGON_WIDTH: f32 = 20.0;

fn hexagon_points() -> Vec<Point2<f32>> {
  let side_length: f32 = 20.0;
  let center_distance = side_length / (60.0_f64.to_radians().sin() as f32);

  vec![
    Point2::new(-(side_length / 2.0), center_distance),
    Point2::new(side_length / 2.0, center_distance),
    Point2::new(center_distance, 0.0),
    Point2::new(side_length / 2.0, -center_distance),
    Point2::new(-(side_length / 2.0), -center_distance),
    Point2::new(-center_distance, 0.0),
  ]
}

const SQUARE_HEIGHT: f32 = 15.0;
const SQUARE_WIDTH: f32 = 15.0;

fn square_points() -> Vec<Point2<f32>> {
  let side_length: f32 = 15.0;

  vec![
    Point2::new(-(side_length / 2.0), 0.0),
    Point2::new(side_length / 2.0, 0.0),
    Point2::new(side_length / 2.0, side_length),
    Point2::new(-(side_length / 2.0), side_length),
  ]
}
