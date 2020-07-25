use std::any::Any;
use std::fmt::Debug;
use std::fmt::Display;

mod collidable;
mod drawable;
mod expirable;
mod physicsable;
mod player_controllable;
mod positionable;
mod rotatable;

pub use collidable::{Collidable, CollisionBounds};
pub use drawable::Drawable;
pub use expirable::Expirable;
pub use physicsable::Physicsable;
pub use player_controllable::PlayerControllable;
pub use positionable::Positionable;
pub use rotatable::Rotatable;

pub trait Component: Any + Debug + Display {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
