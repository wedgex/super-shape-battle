use ggez::nalgebra::Point2;

pub fn translate_points(points: &mut Vec<Point2<f32>>, position: Point2<f32>) {
  for point in points {
    point.x += position.x;
    point.y += position.y;
  }
}
