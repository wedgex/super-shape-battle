use ggez::event;
use ggez::graphics;
use ggez::nalgebra as na;
use ggez::{Context, ContextBuilder, GameResult};

struct GameState {
    pos_x: f32,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState { pos_x: 0.0 };
        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        self.pos_x = self.pos_x % 800.0 + 1.0;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let w = 25.0;
        let h = 30.0;

        let ship = graphics::Mesh::new_polygon(
            ctx,
            graphics::DrawMode::stroke(2.0),
            &[
                na::Point2::<f32>::new(0.0, h),
                na::Point2::<f32>::new(w / 2.0, 0.0),
                na::Point2::<f32>::new(w, h),
                na::Point2::<f32>::new(w / 2.0, h - (h / 3.0)),
            ],
            graphics::WHITE,
        )?;

        graphics::draw(
            ctx,
            &ship,
            graphics::DrawParam::default()
                .rotation(0.0)
                .dest(na::Point2::new(100.0, 100.0)),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("super_space_shape_battle", "wedgex");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
