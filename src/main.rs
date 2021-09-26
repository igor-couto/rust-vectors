#![allow(clippy::unnecessary_wraps)]

mod vector;

use ggez::event;
use ggez::graphics::{self, Color, DrawParam};
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
        let meshes = vec![build_mesh(ctx)?];
        let s = MainState { meshes };

        Ok(s)
    }
}

fn build_mesh(ctx: &mut Context) -> GameResult<graphics::Mesh> {
    let mb = &mut graphics::MeshBuilder::new();

    mb.line(
        &[
            vector::Vector::origin().toVec2(),
            vector::Vector::new(400.0, 200.0).toVec2(),
        ],
        4.0,
        Color::new(0.37, 0.82, 0.95, 1.0),
    )?;

    mb.line(
        &[
            vector::Vector::origin().toVec2(),
            vector::Vector::new(10.0, 30.0).toVec2(),
        ],
        4.0,
        Color::new(1.00, 0.45, 0.45, 1.0),
    )?;

    mb.build(ctx)
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        ctx.mouse_context.reset_delta();

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
