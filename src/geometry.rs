use ggez::nalgebra::Point2;
use ggez::nalgebra::Vector2;

pub fn translate_points(points: &mut Vec<Point2<f32>>, position: Point2<f32>) {
  for point in points {
    point.x += position.x;
    point.y += position.y;
  }
}

pub fn rotation_transform(point: &Point2<f32>, angle: f32) -> Point2<f32> {
  let radians = angle.to_radians();

  Point2::new(
    point.x * radians.cos() - point.y * radians.sin(),
    point.x * radians.sin() + point.y * radians.cos(),
  )
}

pub fn angle_to_vec(angle: f32) -> Vector2<f32> {
  Vector2::new(angle.to_radians().sin(), -angle.to_radians().cos())
}
