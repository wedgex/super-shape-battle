use super::game::GameState;
use ggez::Context;
use ggez::GameResult;

mod collision;
mod damage;
mod draw;
mod expiration;
mod physics;
mod player_input;

pub use collision::CollisionSystem;
pub use damage::DamageSystem;
pub use draw::DrawSystem;
pub use expiration::ExpirationSystem;
pub use physics::PhysicsSystem;
pub use player_input::PlayerInputSystem;

pub trait System {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult;
}
