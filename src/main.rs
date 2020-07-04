use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::Point2;
use ggez::{Context, ContextBuilder, GameResult};

struct Ship {
    position: Point2<f32>,
    rotation: f32,
}

impl Ship {
    fn new(position: Point2<f32>) -> Self {
        Ship {
            position,
            rotation: 0.0,
        }
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        let w = 25.0;
        let h = 30.0;

        let ship = graphics::Mesh::new_polygon(
            context,
            graphics::DrawMode::stroke(2.0),
            &[
                Point2::<f32>::new(0.0, h),
                Point2::<f32>::new(w / 2.0, 0.0),
                Point2::<f32>::new(w, h),
                Point2::<f32>::new(w / 2.0, h - (h / 3.0)),
            ],
            graphics::WHITE,
        )?;

        graphics::draw(
            context,
            &ship,
            graphics::DrawParam::default()
                .rotation(self.rotation)
                .offset(Point2::<f32>::new(w / 2.0, h / 2.0))
                .dest(self.position),
        )?;

        Ok(())
    }
}

struct GameState {
    ship: Ship,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState {
            ship: Ship::new(Point2::<f32>::new(400.0, 400.0)),
        };

        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        println!("rotation: {}", self.ship.rotation);
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            let mut rotation = self.ship.rotation - 0.1;
            if rotation < 0.0 {
                rotation = rotation + 360.0;
            }
            self.ship.rotation = rotation;
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.ship.rotation = (self.ship.rotation + 0.1) % 360.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.ship.draw(ctx)?;
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
