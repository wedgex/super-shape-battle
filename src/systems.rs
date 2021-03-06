use crate::world::World;
use ggez::Context;
use ggez::GameResult;

mod collision;
mod damage;
mod draw;
mod expiration;
mod physics;
mod player_input;
mod shape_manager;
mod ship_manager;

pub use collision::CollisionSystem;
pub use damage::DamageSystem;
pub use draw::DrawSystem;
pub use expiration::ExpirationSystem;
pub use physics::PhysicsSystem;
pub use player_input::PlayerInputSystem;
pub use shape_manager::ShapeManager;
pub use ship_manager::ShipManager;

pub trait System {
  fn update(world: &mut World, context: &mut Context) -> GameResult;
}
