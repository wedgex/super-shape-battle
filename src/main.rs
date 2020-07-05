use ggez::event;
use ggez::event::KeyCode;
use ggez::graphics;
use ggez::input::keyboard;
use ggez::nalgebra::{Point2, Vector2};
use ggez::{Context, ContextBuilder, GameResult};

struct Ship {
    position: Point2<f32>,
    rotation: f32,
    velocity: Vector2<f32>,
    acceleration: Vector2<f32>,
}

impl Ship {
    fn new(position: Point2<f32>) -> Self {
        let velocity = Vector2::new(0.0, 0.0);
        let acceleration = Vector2::new(0.0, 0.0);

        Ship {
            position,
            rotation: 0.0,
            velocity,
            acceleration,
        }
    }

    fn draw(&self, context: &mut Context) -> GameResult {
        let w = 25.0;
        let h = 30.0;

        let ship = graphics::Mesh::new_polygon(
            context,
            graphics::DrawMode::stroke(2.0),
            &[
                Point2::new(0.0, h),
                Point2::new(w / 2.0, 0.0),
                Point2::new(w, h),
                Point2::new(w / 2.0, h - (h / 3.0)),
            ],
            graphics::WHITE,
        )?;

        graphics::draw(
            context,
            &ship,
            graphics::DrawParam::default()
                .rotation(self.rotation)
                .offset(Point2::new(w / 2.0, h / 2.0))
                .dest(self.position),
        )?;

        Ok(())
    }

    fn accelerate(&mut self) {
        self.acceleration += 0.01 * Vector2::new(self.rotation.sin(), -self.rotation.cos());
    }

    fn decelerate(&mut self) {
        self.acceleration *= 0.0;
    }

    // turning occasionally seems jumpy
    fn turn_left(&mut self) {
        let mut rotation = self.rotation - 0.1;
        if rotation < 0.0 {
            rotation += 360.0;
        }
        self.rotation = rotation;
    }

    fn turn_right(&mut self) {
        self.rotation = (self.rotation + 0.1) % 360.0;
    }
}

struct GameState {
    ship: Ship,
}

impl GameState {
    fn new() -> GameResult<GameState> {
        let s = GameState {
            ship: Ship::new(Point2::new(400.0, 400.0)),
        };

        Ok(s)
    }
}

impl event::EventHandler for GameState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        if keyboard::is_key_pressed(ctx, KeyCode::A) {
            self.ship.turn_left();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::D) {
            self.ship.turn_right();
        }

        if keyboard::is_key_pressed(ctx, KeyCode::W) {
            self.ship.accelerate();
        } else {
            self.ship.decelerate();
        }

        handle_acceleration(&mut self.ship);
        handle_velocity(&mut self.ship);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        self.ship.draw(ctx)?;
        graphics::present(ctx)?;

        Ok(())
    }
}

const MAX_VELOCITY: f32 = 5.0;

fn handle_acceleration(ship: &mut Ship) {
    ship.velocity += ship.acceleration;
    if ship.velocity.norm_squared() > MAX_VELOCITY.powi(2) {
        ship.velocity = ship.velocity / ship.velocity.norm_squared().sqrt() * MAX_VELOCITY;
    }
}

fn handle_velocity(ship: &mut Ship) {
    ship.position += ship.velocity;
}

pub fn main() -> GameResult {
    let cb = ContextBuilder::new("super_space_shape_battle", "wedgex");
    let (ctx, event_loop) = &mut cb.build()?;
    let state = &mut GameState::new()?;
    event::run(ctx, event_loop, state)
}
