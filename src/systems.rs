use super::game::GameState;
use ggez::Context;
use ggez::GameResult;

pub mod collision;
pub mod draw;
pub mod physics;
pub mod player_input;

pub trait System {
  fn update(game: &mut GameState, context: &mut Context) -> GameResult;
}
