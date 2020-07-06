use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod game;
mod shape;
mod ship;
mod systems;

use game::GameState;

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("super_space_shape_battle", "wedgex");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
