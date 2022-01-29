//Made by deca
#[allow(unused_imports)]
use ggez::{ContextBuilder, event::EventHandler, Context, error::GameResult, error::GameError};
use ggez::event;
use ggez::graphics;
use ggez::graphics::{Color, DrawParam};
use std::collections::LinkedList;



struct Apple {
    color: Color,
    width: i32,
    height: i32
}

struct Part {
    x: i32,
    y: i32,
}

struct Snake {
    parts: LinkedList<Part>
}

impl Snake {
    fn new() -> Self {
        let mut parts: LinkedList<Part> = LinkedList::new();
        let root_part = Part {x: 500, y: 500};
        parts.push_back(root_part);

        Self {
            parts: parts
        }
    }

    fn draw(&mut self,ctx: &mut Context) {
        let mut first: bool = false;
        for i  in &self.parts {
            let part_rect = graphics::Rect::new_i32(i.x, i.y, 20, 20);
            let part;
            if !first {
                part = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), part_rect, Color::RED).unwrap();
                first = true;
            }else {
                part = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), part_rect, Color::WHITE).unwrap();
            }
            

            if let Err(t) = graphics::draw(ctx, &part, DrawParam::default()) {
                println!("{:?}", t);
            }
        }
    }

    fn add_part(&mut self) {
        let last_part: &Part = self.parts.back().unwrap();

        self.parts.push_back(Part {x: last_part.x + 20, y: last_part.y});
    }
}

impl Apple {

    fn new() -> Self {
        Self {
            color: Color::from_rgb(0,200,0),
            width: 20,
            height: 20
        }
    }

    fn new_op(r: u8, g: u8, b: u8, size: i32) -> Self {
        Self {
            color: Color::from_rgb(r,g,b),
            width: size,
            height: size
        }
    }

    fn random_location(ctx: &mut Context) -> (i32, i32) {
        let (window_x, window_y) = graphics::size(ctx);
        let x = fastrand::i32(0..window_x as i32);
        let y = fastrand::i32(0..window_y as i32);

        (x, y)
    }
}

struct State {
	score: u64,
    snake: Snake
}

impl State {
    fn new() -> Self {
        Self {
            score: 0,
            snake: Snake::new()
        }
    }

}

impl EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.snake.add_part();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::from_rgb(0,0,128));
        //make snake 

        //make apple
        let apple_data: Apple =  Apple::new();
        let (apple_x,  apple_y) = Apple::random_location(ctx);
        let apple_rect = graphics::Rect::new_i32(apple_x, apple_y, apple_data.width, apple_data.height);
        let apple = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), apple_rect, apple_data.color).unwrap();

        self.snake.draw(ctx);

        if let Err(t) = graphics::draw(ctx, &apple, DrawParam::default()) {
            println!("{:?}", t);
        }

        graphics::present(ctx)
    }
}

fn main() {
    let game_state = State::new();
    let (context, eventloop) = ContextBuilder::new("Snake", "Me").build().unwrap();
    graphics::set_window_title(&context, "Snake");

    event::run(context, eventloop, game_state);
}
