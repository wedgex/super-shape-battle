use crate::entity::{Hexagon, Octagon, Ship, Square};
use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod components;
mod entity;
mod game;
mod geometry;
mod iter;
mod systems;
mod world;

use game::GameState;

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("super_space_shape_battle", "wedgex");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new();

    Octagon::create(&mut state.world, ctx, 100.0, 100.0)?;
    Hexagon::create(&mut state.world, ctx, 500.0, 500.0)?;
    Square::create(&mut state.world, ctx, 300.0, 200.0)?;
    Ship::create(&mut state.world, ctx)?;

    event::run(ctx, event_loop, state)
}
