//Made by deca


#[allow(unused_imports)]
use ggez::{ContextBuilder, event::EventHandler, Context, error::GameResult, error::GameError};
use ggez::event;
use ggez::graphics;
use ggez::graphics::Color;

struct State {
	score: u64,
}

impl State {
    fn new() -> Self {
        Self {
            score: 0
        }
    }
}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0,0,128));


        graphics::present(ctx)
    }
}

fn main() {
    let game_state = State::new();
    let (context, eventloop) = ContextBuilder::new("Snake", "Me").build().unwrap();
    graphics::set_window_title(&context, "Snake");

    event::run(context, eventloop, game_state);
}
