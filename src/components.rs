use std::any::Any;

mod collidable;
mod collision;
mod damage;
mod damaged;
mod drawable;
mod expirable;
mod physicsable;
mod player_controllable;
mod tag;
mod transform;
mod vulnerable;

pub use collidable::{Collidable, CollisionBounds};
pub use collision::Collision;
pub use damage::{Damage, DamageType};
pub use damaged::Damaged;
pub use drawable::Drawable;
pub use expirable::Expirable;
pub use physicsable::Physicsable;
pub use player_controllable::PlayerControllable;
pub use tag::{Tag, TagType};
pub use transform::Transform;
pub use vulnerable::Vulnerable;

pub trait Component: Any + 'static {
  fn as_any(&self) -> &dyn Any;
  fn as_any_mut(&mut self) -> &mut dyn Any;
}

pub fn downcast<T: Component>(c: &Box<dyn Component>) -> Option<&T> {
  c.as_any().downcast_ref::<T>()
}

pub fn downcast_mut<T: Component>(c: &mut Box<dyn Component>) -> Option<&mut T> {
  c.as_any_mut().downcast_mut::<T>()
}
