use std::any::Any;

mod collidable;
mod damage;
mod drawable;
mod expirable;
mod physicsable;
mod player_controllable;
mod transform;
mod vulnerable;

pub use collidable::{Collidable, CollisionBounds};
pub use damage::{Damage, DamageType};
pub use drawable::Drawable;
pub use expirable::Expirable;
pub use physicsable::Physicsable;
pub use player_controllable::PlayerControllable;
pub use transform::Transform;
pub use vulnerable::Vulnerable;

pub trait Component: Any {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}
