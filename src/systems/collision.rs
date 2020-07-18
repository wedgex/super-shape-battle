use super::System;
use crate::GameState;
use geo::algorithm::intersects::Intersects;
use geo::{Coordinate, Line};
use ggez::nalgebra::Point2;
use ggez::Context;
use ggez::GameResult;

// TODO rework collisions into ECS

pub trait Collision {
  fn points(&self) -> Vec<Point2<f32>>;
  fn position(&self) -> Point2<f32>;
  fn collision(&mut self);
}

pub struct CollisionSystem;

impl System for CollisionSystem {
  fn update(game: &mut GameState, _context: &mut Context) -> GameResult {
    Ok(())
  }
}

fn overlaps(entity1: &impl Collision, entity2: &impl Collision) -> bool {
  intersects_centerline(entity1, entity2) || intersects_centerline(entity2, entity1)
}

fn intersects_centerline(entity1: &impl Collision, entity2: &impl Collision) -> bool {
  let center_lines = get_center_lines(entity1);
  let edges = get_edges(entity2);

  center_lines
    .iter()
    .any(|center_line| edges.iter().any(|edge| center_line.intersects(edge)))
}

fn get_edges(entity: &impl Collision) -> Vec<Line<f32>> {
  let points = entity.points();

  let mut closed_points: Vec<&Point2<f32>> = points.iter().collect();
  closed_points.push(points.first().unwrap());

  closed_points
    .windows(2)
    .map(|pair| {
      Line::new(
        Coordinate {
          x: pair[0].x,
          y: pair[0].y,
        },
        Coordinate {
          x: pair[1].x,
          y: pair[1].y,
        },
      )
    })
    .collect()
}

fn get_center_lines(entity: &impl Collision) -> Vec<Line<f32>> {
  let center = entity.position();
  entity
    .points()
    .iter()
    .map(|point| {
      Line::new(
        Coordinate {
          x: center.x,
          y: center.y,
        },
        Coordinate {
          x: point.x,
          y: point.y,
        },
      )
    })
    .collect()
}

#[cfg(test)]
mod test {
  use super::*;

  struct TestEntity {
    position: Point2<f32>,
  }

  impl Collision for TestEntity {
    fn points(&self) -> Vec<Point2<f32>> {
      vec![
        Point2::new(0.0 + self.position.x, 2.0 + self.position.y),
        Point2::new(2.0 + self.position.x, 2.0 + self.position.y),
        Point2::new(2.0 + self.position.x, 0.0 + self.position.y),
        Point2::new(0.0 + self.position.x, 0.0 + self.position.y),
      ]
    }

    fn position(&self) -> Point2<f32> {
      self.position
    }

    fn collision(&mut self) {}
  }

  #[test]
  fn overlaps_when_neither_are_colliding() {
    let entity1 = TestEntity {
      position: Point2::new(0.0, 0.0),
    };
    let entity2 = TestEntity {
      position: Point2::new(4.0, 4.0),
    };

    assert_eq!(overlaps(&entity1, &entity2), false);
  }

  #[test]
  fn overlaps_at_corners() {
    let entity1 = TestEntity {
      position: Point2::new(0.0, 0.0),
    };
    let entity2 = TestEntity {
      position: Point2::new(1.5, 1.5),
    };

    assert_eq!(overlaps(&entity1, &entity2), true);
  }

  #[test]
  fn overlaps_at_edge() {
    let entity1 = TestEntity {
      position: Point2::new(0.0, 0.0),
    };
    let entity2 = TestEntity {
      position: Point2::new(0.0, 2.0),
    };

    assert_eq!(overlaps(&entity1, &entity2), true);
  }

  #[test]
  fn overlaps_at_point() {
    let entity1 = TestEntity {
      position: Point2::new(0.0, 0.0),
    };
    let entity2 = TestEntity {
      position: Point2::new(2.0, 2.0),
    };

    assert_eq!(overlaps(&entity1, &entity2), true);
  }
}
