use crate::components::{
  Collidable, Damage, DamageType, Drawable, Expirable, Physicsable, PlayerControllable, Tag,
  TagType, Transform, Vulnerable,
};
use crate::geometry;
use crate::geometry::rotation_transform;
use crate::world::World;
use ggez::graphics::Color;
use ggez::graphics::{self};
use ggez::nalgebra::{Point2, Vector2};
use ggez::Context;
use ggez::GameResult;
use graphics::Mesh;
use std::f32::consts::PI;
use std::time::Duration;
use uuid::Uuid;

pub type EntityId = Uuid;

pub struct Ship;

impl Ship {
  pub fn create(world: &mut World, context: &mut Context) -> GameResult<EntityId> {
    let entity = world.create_entity();

    world.add(&entity, Tag::new(TagType::Ship));
    world.add(&entity, Transform::new(200., 200.));
    world.add(
      &entity,
      Drawable::new(ship_mesh(context)?, Point2::new(25. / 2., 30. / 2.)),
    );
    world.add(&entity, Physicsable::new(0., 0.));
    world.add(&entity, Collidable::new(ship_points()));
    world.add(&entity, PlayerControllable::new());
    world.add(&entity, Vulnerable::new(vec![DamageType::Smash]));

    Ok(entity)
  }
}

fn ship_mesh(context: &mut Context) -> GameResult<Mesh> {
  let points = ship_points();
  Mesh::new_polygon(
    context,
    graphics::DrawMode::stroke(2.0),
    &points,
    graphics::WHITE,
  )
}

fn ship_points() -> Vec<Point2<f32>> {
  let w = 25.0;
  let h = 30.0;

  vec![
    Point2::new(0.0, h),
    Point2::new(w / 2.0, 0.0),
    Point2::new(w, h),
    Point2::new(w / 2.0, h - (h / 3.0)),
  ]
}

pub struct Bullet;

impl Bullet {
  pub fn create(
    world: &mut World,
    context: &mut Context,
    x: f32,
    y: f32,
    angle: f32,
  ) -> GameResult<EntityId> {
    let entity = world.create_entity();
    let transform = Transform::new(x, y);
    let points = vec![
      Point2::new(0.0, 0.0),
      Point2::new(2.0, 0.0),
      Point2::new(2.0, 2.0),
      Point2::new(0.0, 2.0),
    ];
    let mesh = Mesh::new_polygon(
      context,
      graphics::DrawMode::stroke(2.0),
      &points,
      graphics::WHITE,
    )?;
    let drawable = Drawable::new(mesh, Point2::new(1., 1.));

    let velocity = 4. * geometry::angle_to_vec(angle);
    let physics = Physicsable::new(velocity.x, velocity.y);
    let expiration = Expirable::new(Duration::from_secs(3));

    world.add(&entity, Tag::new(TagType::Bullet));
    world.add(&entity, transform);
    world.add(&entity, drawable);
    world.add(&entity, physics);
    world.add(&entity, expiration);
    world.add(&entity, Collidable::new(points.clone()));
    world.add(&entity, Damage::new(DamageType::Projectile));

    Ok(entity)
  }
}

const RED: graphics::Color = graphics::Color::new(255.0, 0.0, 0.0, 1.0);
const YELLOW: graphics::Color = graphics::Color::new(255.0, 255.0, 0.0, 1.0);
const GREEN: graphics::Color = graphics::Color::new(0.0, 255.0, 0.0, 1.0);

pub struct Octagon;

impl Octagon {
  pub fn create(world: &mut World, context: &mut Context, x: f32, y: f32) -> GameResult<EntityId> {
    create_shape(
      world,
      context,
      x,
      y,
      octagon_points(),
      RED,
      Point2::new(30.18, 30.18), // https://www.omnicalculator.com/math/octagon
      3,
    )
  }
}

pub struct Hexagon;

impl Hexagon {
  pub fn create(world: &mut World, context: &mut Context, x: f32, y: f32) -> GameResult<EntityId> {
    create_shape(
      world,
      context,
      x,
      y,
      hexagon_points(),
      YELLOW,
      Point2::new(17.32, 17.32), // https://www.omnicalculator.com/math/hexagon
      2,
    )
  }
}

pub struct Square;

impl Square {
  pub fn create(world: &mut World, context: &mut Context, x: f32, y: f32) -> GameResult<EntityId> {
    create_shape(
      world,
      context,
      x,
      y,
      square_points(),
      GREEN,
      Point2::new(15.0 / 2., 15.0 / 2.0),
      1,
    )
  }
}

pub fn create_shape(
  world: &mut World,
  context: &mut Context,
  x: f32,
  y: f32,
  points: Vec<Point2<f32>>,
  color: Color,
  offset: Point2<f32>,
  level: u8,
) -> GameResult<EntityId> {
  let entity = world.create_entity();

  let transform = Transform::new(x, y);
  let mesh = Mesh::new_polygon(context, graphics::DrawMode::stroke(2.0), &points, color)?;
  let drawable = Drawable::new(mesh, offset);
  let mut physics = Physicsable::new(0., 0.);
  physics.velocity = Vector2::new(1., 1.);

  world.add(&entity, Tag::new(TagType::Shape(level)));
  world.add(&entity, transform);
  world.add(&entity, drawable);
  world.add(&entity, physics);
  world.add(&entity, Collidable::new(points.clone()));
  world.add(&entity, Damage::new(DamageType::Smash));
  world.add(&entity, Vulnerable::new(vec![DamageType::Projectile]));

  Ok(entity)
}

fn polygon_points(sides: i32, length: f32, rotation: f32) -> Vec<Point2<f32>> {
  let angle = 2.0 * PI / sides as f32;

  (0..=sides)
    .map(|i| {
      rotation_transform(
        &Point2::new(
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
