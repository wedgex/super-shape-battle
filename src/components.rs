use std::any::Any;

mod bullet;
mod collidable;
mod drawable;
mod expirable;
mod physicsable;
mod player_controllable;
mod positionable;
mod shape;
mod ship;

pub use bullet::Bullet;
pub use collidable::{Collidable, CollisionBounds};
pub use drawable::Drawable;
pub use expirable::Expirable;
pub use physicsable::Physicsable;
pub use player_controllable::PlayerControllable;
pub use positionable::Positionable;
pub use shape::{Shape, ShapeType};
pub use ship::Ship;

pub trait Component: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
