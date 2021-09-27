#![allow(clippy::unnecessary_wraps)]

mod vector;

use ggez::event;
use ggez::graphics::{self, Color, DrawParam, Mesh};
use ggez::{Context, GameResult};

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Vectors Board", "Igor Couto");
    let (mut ctx, events_loop) = cb.build()?;
    let state = MainState::new(&mut ctx).unwrap();
    event::run(ctx, events_loop, state)
}

struct MainState {
    meshes: Vec<graphics::Mesh>,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let mut meshes: Vec<Mesh> = Vec::new();
        meshes.push(create_vector(ctx, 0.0, 0.0)?);

        let main_state = MainState { meshes };
        Ok(main_state)
    }
}

fn create_vector(ctx: &mut Context, x: f32, y: f32) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            vector::Vector::origin().to_vec2(),
            vector::Vector::new(x + 1.0, y + 1.0).to_vec2(), // why only works when add + something or other vec?
        ],
        3.0,
        Color::new(0.37, 0.82, 0.95, 1.0),
    )?;

    mb.build(ctx)
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let mouse_position = ggez::input::mouse::position(&ctx);

        self.meshes[0] = create_vector(ctx, mouse_position.x + 1.0, mouse_position.y + 1.0)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.13, 0.15, 0.18, 1.0].into());

        for m in &self.meshes {
            graphics::draw(ctx, m, DrawParam::new())?;
        }

        graphics::present(ctx)?;
        Ok(())
    }
}
