use super::systems::Physics;
use ggez::graphics;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, GameResult};

const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);

pub struct Shape {
  pub position: Point2<f32>,
  pub velocity: Vector2<f32>,
  pub acceleration: Vector2<f32>,
}

impl Shape {
  pub fn new(position: Point2<f32>) -> Self {
    let velocity = Vector2::new(0.0, 0.0);
    let acceleration = Vector2::new(0.0, 0.0);

    Shape {
      position,
      velocity,
      acceleration,
    }
  }

  pub fn draw(&self, context: &mut Context) -> GameResult {
    let side_length: f32 = 25.0;

    // calculate the x & y offset of the diagonal sides
    let triangle_offset = (side_length.powi(2) / 2.0).sqrt();

    let w = side_length + 2.0 * triangle_offset;
    let h = side_length + 2.0 * triangle_offset;

    let shape = graphics::Mesh::new_polygon(
      context,
      graphics::DrawMode::stroke(2.0),
      &[
        Point2::new(0.0, triangle_offset),
        Point2::new(triangle_offset, 0.0),
        Point2::new(triangle_offset + side_length, 0.0),
        Point2::new(w, triangle_offset),
        Point2::new(w, triangle_offset + side_length),
        Point2::new(triangle_offset + side_length, h),
        Point2::new(triangle_offset, h),
        Point2::new(0.0, triangle_offset + side_length),
      ],
      RED,
    )?;

    graphics::draw(
      context,
      &shape,
      graphics::DrawParam::default()
        .offset(Point2::new(w / 2.0, h / 2.0))
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
