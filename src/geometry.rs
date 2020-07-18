use ggez::nalgebra::Point2;

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
