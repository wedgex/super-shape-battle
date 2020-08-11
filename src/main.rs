use crate::shape::hexagon;
use crate::shape::octagon;
use crate::shape::square;
use crate::ship::build_ship;
use ggez::event;
use ggez::{ContextBuilder, GameResult};

mod components;
mod entity;
mod game;
mod geometry;
mod shape;
mod ship;
mod systems;
mod world;

use game::GameState;

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("super_space_shape_battle", "wedgex");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new();

    octagon(&mut state.world, ctx, 100.0, 100.0)?;
    hexagon(&mut state.world, ctx, 500.0, 500.0)?;
    square(&mut state.world, ctx, 300.0, 200.0)?;
    build_ship(&mut state.world, ctx)?;

    event::run(ctx, event_loop, state)
}
